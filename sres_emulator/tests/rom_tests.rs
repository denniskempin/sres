use std::collections::VecDeque;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use sres_emulator::bus::Bus;
use sres_emulator::bus::SresBus;
use sres_emulator::cpu::Cpu;
use sres_emulator::memory::Wrap;
use sres_emulator::timer::fvh_to_master_clock;
use sres_emulator::trace::Trace;

#[test]
pub fn test_nmi_sub_cycle_accuracy() {
    static TEST_CASES: &[(u64, u64, bool, bool)] = &[
        // The `bit $4210` instruction is often used to check the NMI signal, to wait for VSYNC.
        // This makes the instruction very sensitive to sub-cpu-cycle timing, as the result will
        // depend on when exactly the signal is read.
        //
        // The list below is the result of `bit $4210` executed at various points in the frame. This
        // matches the behavior of BSNES.
        //
        // Starting 1334, the bit instruction will end after NMI and the internal NMI flag will
        // be set after the instruction is executed.
        //
        // Starting 1340, NMI will be high by the time the bit instruction reads the state. Usually
        // reads from $4210 will reset the NMI flag, but not for the first 4 cycles.
        //
        // (V, H, nmi returned by `bit`, internal nmi flag)
        (224, 1330, false, false),
        (224, 1332, false, false),
        (224, 1334, false, true),
        (224, 1336, false, true),
        (224, 1338, false, true),
        (224, 1340, true, true),
        (224, 1342, true, true),
        (224, 1344, true, false),
        (224, 1346, true, false),
        (224, 1348, true, false),
        (224, 1350, true, false),
        (224, 1352, true, false),
        (224, 1354, true, false),
        (224, 1356, true, false),
        (224, 1358, true, false),
        (224, 1360, true, false),
        (224, 1362, true, false),
        (225, 0, true, false),
    ];
    for (v, h, expected_nmi, expected_internal_nmi) in TEST_CASES {
        // Create CPU with `bit $4210` program in memory
        let mut bus = SresBus::default();
        bus.cycle_write_u16(0x00.into(), 0x2C, Wrap::NoWrap);
        bus.cycle_write_u16(0x01.into(), 0x4210, Wrap::NoWrap);
        let mut cpu = Cpu::new(bus);
        cpu.reset();

        // Advance PPU timer until (v, h) is reached
        while cpu.bus.ppu_timer.v != *v || cpu.bus.ppu_timer.h_counter != *h {
            cpu.bus.ppu_timer.advance_master_clock(2);
        }

        // Execute `bit $4210` instruction
        println!("before: {}", Trace::from_sres_cpu(&cpu));
        cpu.step();
        println!("after: {}", Trace::from_sres_cpu(&cpu));

        // If the NMI bit is set, the negative status bit will be true.
        assert_eq!(cpu.status.negative, *expected_nmi);
        // For the first 4 cycles NMI will remain high, so the internal nmi_flag will still be set.
        assert_eq!(cpu.bus.ppu_timer.nmi_flag, *expected_internal_nmi);
    }
}

#[test]
pub fn test_krom_adc() {
    run_rom_test("krom_adc");
}

#[test]
pub fn test_krom_and() {
    run_rom_test("krom_and");
}

#[test]
pub fn test_krom_asl() {
    run_rom_test("krom_asl");
}

#[test]
pub fn test_krom_bit() {
    run_rom_test("krom_bit");
}

#[test]
pub fn test_krom_bra() {
    run_rom_test("krom_bra");
}

#[test]
pub fn test_krom_cmp() {
    run_rom_test("krom_cmp");
}

#[test]
pub fn test_krom_dec() {
    run_rom_test("krom_dec");
}

#[test]
pub fn test_krom_eor() {
    run_rom_test("krom_eor");
}

#[test]
pub fn test_krom_inc() {
    run_rom_test("krom_inc");
}

#[test]
pub fn test_krom_jmp() {
    run_rom_test("krom_jmp");
}

#[test]
pub fn test_krom_ldr() {
    run_rom_test("krom_ldr");
}

#[test]
pub fn test_krom_lsr() {
    run_rom_test("krom_lsr");
}

#[test]
pub fn test_krom_mov() {
    run_rom_test("krom_mov");
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_krom_msc() {
    run_rom_test("krom_msc");
}

#[test]
pub fn test_krom_ora() {
    run_rom_test("krom_ora");
}

#[test]
pub fn test_krom_phl() {
    run_rom_test("krom_phl");
}

#[test]
pub fn test_krom_psr() {
    run_rom_test("krom_psr");
}

#[test]
pub fn test_krom_ret() {
    run_rom_test("krom_ret");
}

#[test]
pub fn test_krom_rol() {
    run_rom_test("krom_rol");
}

#[test]
pub fn test_krom_ror() {
    run_rom_test("krom_ror");
}

#[test]
pub fn test_krom_sbc() {
    run_rom_test("krom_sbc");
}

#[test]
pub fn test_krom_str() {
    run_rom_test("krom_str");
}

#[test]
pub fn test_krom_trn() {
    run_rom_test("krom_trn");
}

#[test]
pub fn test_ppu_timing() {
    run_rom_test("ppu_timing");
}

fn run_rom_test(test_name: &str) {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let trace_path = root_dir.join(format!("tests/rom_tests/{test_name}-trace.log.xz"));
    let rom_path = root_dir.join(format!("tests/rom_tests/{test_name}.sfc"));

    let mut bus = SresBus::with_sfc(&rom_path).unwrap();
    // CPUMSC reads 0x20 from $000000 at the first instruction. I cannot figure out why, it
    // should be mapped to RAM.
    bus.cycle_write_u8(0x000000.into(), 0x20);

    let mut cpu = Cpu::new(bus);
    cpu.reset();

    let mut previous_master_cycle = 0;
    let mut previous_lines: VecDeque<Trace> = VecDeque::new();
    for (i, expected_line) in Trace::from_xz_file(&trace_path).unwrap().enumerate() {
        let mut expected_line = expected_line.unwrap();
        if i == 0 {
            assert_eq!(
                expected_line.h, 186,
                "Trace file is using dots not H-position"
            );
        }

        let mut actual_line = Trace::from_sres_cpu(&cpu);
        previous_lines.push_front(actual_line.clone());
        previous_lines.truncate(50);

        // Fix some BSNES trace inconsistencies:

        // Disassembly for branch instructions prints the absolute operand address, not the
        // relative address.
        if expected_line.instruction.starts_with('b') && expected_line.instruction != "bit" {
            actual_line.operand = "".to_string();
            expected_line.operand = "".to_string();
        }
        // `per` instruction prints relative address as effective address, not the calculated
        // absolute address.
        if expected_line.instruction == "per" {
            actual_line.operand = "".to_string();
            expected_line.operand = "".to_string();
            actual_line.operand_addr = None;
            expected_line.operand_addr = None;
        }
        // `jmp` instructions in bsnes print an inconsistent effective address. Skip comparison.
        if expected_line.instruction.starts_with('j') {
            expected_line.operand_addr = None;
            actual_line.operand_addr = None;
        }

        if actual_line != expected_line {
            println!("Assertion failure at instruction {i}");
            for line in previous_lines.iter().rev() {
                println!("{line}");
            }

            // Convert F: V: H: from BSNES trace to master cycles to make it easier to compare how
            // many cycles each instruction takes (or should take).
            let expected_master_cycle =
                fvh_to_master_clock(expected_line.f, expected_line.v, expected_line.h);
            let expected_duration = expected_master_cycle.saturating_sub(previous_master_cycle);
            let actual_duration = cpu
                .bus
                .ppu_timer
                .master_clock
                .saturating_sub(previous_master_cycle);
            if expected_duration != actual_duration {
                println!(
                    "Expected duration: {} - Actual: {}, diff: {}",
                    expected_duration,
                    actual_duration,
                    (expected_duration as i64) - (actual_duration as i64),
                );
            }

            // Compare as strings to get a nice diff.
            assert_eq!(actual_line.to_string(), expected_line.to_string())
        }

        previous_master_cycle = cpu.bus.ppu_timer.master_clock;
        cpu.step();
    }
}

#[test]
pub fn test_dma_vram() {
    // This rom will generate a test sequence 0x00..0xFF in WRAM at 0x0000, then copies it into VRAM via
    // a DMA transfer and copies it back into WRAM at 0x0100.
    let cpu = run_test_rom("dma_vram");
    let expected: Vec<u8> = (0x00..=0xFF).collect();

    // Validate the test sequence at 0x0000
    assert_eq!(
        format_memory(&cpu.bus.memory[0x0000..=0x00FF]),
        format_memory(&expected),
    );

    // Validate the test sequence in VRAM
    assert_eq!(
        format_memory(&cpu.bus.ppu.vram[0x0000..=0x00FF]),
        format_memory(&expected),
    );

    // Validate the test sequence after it's copied back into WRAM at 0x0100
    assert_eq!(
        format_memory(&cpu.bus.memory[0x0100..=0x01FF]),
        format_memory(&expected),
    );
}

fn run_test_rom(test_name: &str) -> Cpu<SresBus> {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rom_path = root_dir.join(format!("tests/rom_tests/{test_name}.sfc"));

    let mut cpu = Cpu::new(SresBus::with_sfc(&rom_path).unwrap());
    cpu.reset();

    while !cpu.halt {
        cpu.step();
        println!("{}", Trace::from_cpu(&cpu));
    }
    cpu
}

fn format_memory(memory: &[u8]) -> String {
    let mut writer = BufWriter::new(Vec::new());
    for chunks in memory.chunks(16) {
        for chunk in chunks {
            write!(&mut writer, "{:02X} ", *chunk).unwrap();
        }
        writeln!(&mut writer).unwrap();
    }

    let bytes = writer.into_inner().unwrap();
    String::from_utf8(bytes).unwrap()
}