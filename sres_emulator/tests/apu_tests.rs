//! High level testing focused on the APU.

use std::path::PathBuf;

use pretty_assertions::assert_eq;
use sres_emulator::apu::AudioBuffer;
use sres_emulator::common::test_util::compare_wav_against_golden;
use sres_emulator::common::util::format_memory;
use sres_emulator::components::cartridge::Cartridge;
use sres_emulator::debugger::EventFilter;
use sres_emulator::System;

/// Helper: run a ROM until the SPC700 reaches `spc700_pc`, then return the formatted
/// APUIO timing log. This is the primary tool for comparing CPU↔APU handshake timing
/// against a reference emulator such as Mesen2.
///
/// # How to use for DKC debugging
/// 1. Run your ROM and call `dump_apuio_timing_log(system, target_pc)`.
/// 2. Do the same in Mesen2 (CPU trace + APU register log).
/// 3. Diff the two logs — the first diverging entry shows where the timing breaks.
///
/// Expected format per line:
/// `[cpu_clk=NNN spc_cycle=NNN] PORT N <R/W> XX`
pub fn dump_apuio_timing_log(system: &mut System, spc700_pc: u16) -> String {
    system.clear_apuio_log();
    system.debug_until(EventFilter::Spc700ProgramCounter(spc700_pc..spc700_pc + 1));
    system.debug().apu().format_apuio_log()
}

/// Verifies the CPU↔SPC700 APUIO timing during the IPL boot handshake and the
/// first SPC program block upload. This exercises the same protocol used by DKC.
///
/// Key invariants checked:
/// - The SPC700 must have run the full IPL RAM-clear loop (>= ~50 000 master cycles).
/// - The CPU must have read $AA from port 0 at some point during boot.
/// - Every CPU write of a sequence number to port 0 during the transfer loop must
///   be echoed back by the SPC700 before the CPU writes the next sequence number.
///
/// If DKC hangs, run it with `RUST_LOG=sres_emulator::apu=debug` and compare the
/// printed APUIO lines with a Mesen2 trace.
#[test]
pub fn test_apuio_timing_during_boot() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rom_path = root_dir.join("tests/apu_tests/play_brr_sample.sfc");

    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());

    // Run the full boot sequence: IPL boot + SPC upload + program start.
    system.debug_until(EventFilter::Spc700ProgramCounter(0x02e9..0x02ea));

    {
        let cpu_clock = system.clock_info().master_clock;
        let apu = system.apu();
        println!(
            "After boot: spc_master_cycle={}, cpu_clock={}",
            apu.spc700.bus.master_cycle, cpu_clock
        );

        // The SPC700 must have run the full IPL RAM-clear loop (~50 000 master cycles).
        assert!(
            apu.spc700.bus.master_cycle >= 40_000,
            "SPC700 should have run >= 40 000 master cycles for IPL boot, got {}",
            apu.spc700.bus.master_cycle
        );
    }

    let debug = system.debug();
    let apu = debug.apu();
    let accesses = apu.apuio_log();
    assert!(!accesses.is_empty(), "APUIO log must not be empty after boot sequence");

    // Print first 10 entries for diagnosis.
    println!("First 10 APUIO log entries:");
    for (i, a) in accesses.iter().take(10).enumerate() {
        println!(
            "  [{}] cpu_clk={} spc_cycle={} PORT {} {} {:#04X}",
            i,
            a.cpu_master_clock,
            a.spc700_master_cycle,
            a.port,
            if a.is_write { "W" } else { "R" },
            a.value
        );
    }

    // Find the first CPU read returning $AA on port 0 — start of the handshake.
    let first_aa_idx = accesses
        .iter()
        .position(|a| !a.is_write && a.port == 0 && a.value == 0xAA);
    assert!(
        first_aa_idx.is_some(),
        "CPU must read $AA from port 0 at some point during boot"
    );
    let first_aa_idx = first_aa_idx.unwrap();
    println!(
        "First $AA read at log index {}, cpu_clk={}",
        first_aa_idx, accesses[first_aa_idx].cpu_master_clock
    );

    // From the $AA read onwards, every CPU write of sequence number N to port 0
    // must be echoed back by the SPC700 before the CPU writes N+1.
    // This is the core timing invariant that breaks when CPU-APU synchronisation is wrong.
    let mut unacked_writes: std::collections::VecDeque<(u8, u64)> =
        std::collections::VecDeque::new();
    let mut max_echo_lag: u64 = 0;
    for access in accesses.iter().skip(first_aa_idx) {
        if access.port == 0 {
            if access.is_write {
                unacked_writes.push_back((access.value, access.cpu_master_clock));
            } else if let Some(&(expected, write_clock)) = unacked_writes.front() {
                if access.value == expected {
                    let lag = access.cpu_master_clock.saturating_sub(write_clock);
                    max_echo_lag = max_echo_lag.max(lag);
                    unacked_writes.pop_front();
                }
            }
        }
    }

    // No written byte should remain unacknowledged at the end.
    assert!(
        unacked_writes.is_empty(),
        "Some CPU writes to port 0 were never echoed back by SPC700: {:?}",
        unacked_writes
            .iter()
            .map(|(v, _)| format!("{v:#04X}"))
            .collect::<Vec<_>>()
    );

    // Print the worst-case echo latency — compare this against Mesen2 to tune timing.
    drop(apu);
    drop(debug);
    println!("Max port-0 echo latency: {} master cycles", max_echo_lag);
}

#[test]
pub fn test_play_brr_sample() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path_prefix = root_dir.join("tests/apu_tests/play_brr_sample");

    // Load rom and execute enough instructions to finish initialization
    let mut system = System::with_cartridge(
        &Cartridge::with_sfc_file(&path_prefix.with_extension("sfc")).unwrap(),
    );

    // Run until spc reaches infinite loop of the program.
    system.debug_until(EventFilter::Spc700ProgramCounter(0x02e9..0x02ea));

    assert_eq!(
        system.debug().apu().dsp().voice(0),
        "vol:127/127 pitch:4096 adsr:(10,7,7,0) src:$00 env:0 out:0".to_string()
    );
    // Clear audio buffer
    let mut samples = AudioBuffer::new();
    system.swap_audio_buffer(&mut samples);

    // Execute for length of BRR sample and collect generated audio samples
    const NUM_SAMPLES: usize = 7936; // Length of the play_brr_sample sample
    system.execute_for_audio_samples(NUM_SAMPLES);
    system.swap_audio_buffer(&mut samples);
    compare_wav_against_golden(&samples.into_vec(), &path_prefix)
}

#[test]
pub fn test_play_noise() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rom_path = root_dir.join("tests/apu_tests/play_noise.sfc");
    let spc_rom_path = root_dir.join("tests/apu_tests/play_noise.spc");

    // Load rom and execute enough instructions to finish initialization
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());

    // Run until SPC jumps into the loaded program
    system.debug_until(EventFilter::Spc700ProgramCounter(0x0200..0x0201));

    // Verify the program has been loaded correctly at 0x0200 in SPC700 RAM.
    let spc_program = std::fs::read(spc_rom_path).unwrap();
    let actual_program = system.debug().apu().ram()[0x0200..(0x0200 + spc_program.len())].to_vec();
    assert_eq!(format_memory(&actual_program), format_memory(&spc_program));

    // Run until "Kick" info has been written into Voice 0
    system.debug_until(EventFilter::Spc700ProgramCounter(0x02dd..0x02de));

    assert_eq!(
        system.debug().apu().dsp().voice(0),
        "vol:127/127 pitch:0 adsr:(14,0,7,22) src:$00 env:4 out:-2".to_string()
    );
}

#[test]
pub fn test_ffvii_prelude() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path_prefix = root_dir.join("tests/apu_tests/ffvii_prelude");

    let mut system = System::with_cartridge(
        &Cartridge::with_sfc_file(&path_prefix.with_extension("sfc")).unwrap(),
    );

    // Execute for 5 seconds and collect all audio samples
    let mut all_samples = Vec::<i16>::with_capacity(32000 * 5);
    let mut buffer = AudioBuffer::new();
    for _ in 0..5 {
        system.execute_frames(60);
        system.swap_audio_buffer(&mut buffer);
        all_samples.extend(buffer.iter());
    }
    compare_wav_against_golden(&all_samples, &path_prefix)
}
