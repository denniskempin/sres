//! High level testing focused on the APU.

use std::path::PathBuf;

use bilge::prelude::*;
use pretty_assertions::assert_eq;
use sres_emulator::apu::s_dsp::Adsr1;
use sres_emulator::apu::s_dsp::Adsr2;
use sres_emulator::apu::s_dsp::Voice;
use sres_emulator::cartridge::Cartridge;
use sres_emulator::debugger::EventFilter;
use sres_emulator::util::memory::format_memory;
use sres_emulator::System;

#[test]
pub fn test_play_brr_sample() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rom_path = root_dir.join("tests/apu_tests/play_brr_sample.sfc");

    // Load rom and execute enough instructions to finish initialization
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());

    // Run until spc reaches infinite loop of the program.
    system.debug_until(EventFilter::Spc700ProgramCounter(0x02e9..0x02ea));
    let actual_voice0 = system.cpu.bus.apu.spc700.bus.dsp.voices[0];
    let expected_voice0 = Voice {
        vol_l: 127,
        vol_r: 127,
        pitch: 0x1000,
        sample_source: 0,
        adsr1: Adsr1::new(u4::new(10), u3::new(7), true),
        adsr2: Adsr2::new(u5::new(0), u3::new(7)),
        gain: 127,
        envx: 0,
        outx: 0,
    };
    assert_eq!(actual_voice0, expected_voice0);
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
    let actual_program = &system.cpu.bus.apu.spc700.bus.ram[0x0200..(0x0200 + spc_program.len())];
    assert_eq!(format_memory(actual_program), format_memory(&spc_program));

    // Run until "Kick" info has been written into Voice 0
    system.debug_until(EventFilter::Spc700ProgramCounter(0x02dd..0x02de));
    let actual_voice0 = system.cpu.bus.apu.spc700.bus.dsp.voices[0];
    let expected_voice0 = Voice {
        vol_l: 127,
        vol_r: 127,
        pitch: 0,
        sample_source: 0,
        adsr1: Adsr1::new(u4::new(0b1110), u3::new(0), true),
        adsr2: Adsr2::new(u5::new(0b10110), u3::new(0b111)),
        gain: 127,
        envx: 0,
        outx: 0,
    };
    assert_eq!(actual_voice0, expected_voice0);
}
