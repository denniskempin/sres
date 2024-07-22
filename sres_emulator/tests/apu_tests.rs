//! High level testing focused on the APU.

use std::path::PathBuf;

use pretty_assertions::assert_eq;
use sres_emulator::common::util::format_memory;
use sres_emulator::components::cartridge::Cartridge;
use sres_emulator::debugger::EventFilter;
use sres_emulator::System;

#[test]
pub fn test_play_brr_sample() {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rom_path = root_dir.join("tests/apu_tests/play_brr_sample.sfc");

    // Load rom and execute enough instructions to finish initialization
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());

    // Run until spc reaches infinite loop of the program.
    system.debug_until(EventFilter::Spc700ProgramCounter(0x02e9..0x02ea));

    assert_eq!(
        system.cpu.bus.apu.debug().dsp().voice(0),
        "vol:127/127 pitch:4096 adsr:(10,7,7,0) src:$00 env:0 out:0".to_string()
    );
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
    let debug = system.cpu.bus.apu.debug();
    let actual_program = &debug.ram()[0x0200..(0x0200 + spc_program.len())];
    assert_eq!(format_memory(actual_program), format_memory(&spc_program));

    // Run until "Kick" info has been written into Voice 0
    system.debug_until(EventFilter::Spc700ProgramCounter(0x02dd..0x02de));

    assert_eq!(
        system.debug().apu().dsp().voice(0),
        "vol:127/127 pitch:0 adsr:(14,0,7,22) src:$00 env:0 out:0".to_string()
    );
}
