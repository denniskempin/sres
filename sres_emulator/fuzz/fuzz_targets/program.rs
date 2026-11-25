//! libfuzzer bin `program`: build headless `System` from `Cartridge::with_program` and step `Cpu`.
//! Caps execution at 1000 `cpu.step()` calls and skips inputs larger than `0x4000000`.
#![no_main]

use libfuzzer_sys::fuzz_target;
use sres_emulator::components::cartridge::Cartridge;
use sres_emulator::System;

fuzz_target!(|data: &[u8]| {
    if data.len() > 0x4000000 {
        return;
    }
    // Load a random program into the emulator.
    let mut cartridge = Cartridge::with_program(data);
    cartridge.sram = vec![0; 0x70000];
    let mut system = System::with_cartridge(&cartridge);
    system.force_headless();
    // This can fail in all kinds of ways, but it should never ever panic!
    for _ in 0..1000 {
        system.cpu.step();
    }
});
