//! libfuzzer bin `program`: load input as a 65816 program and step `Cpu`.
//! Caps the loop at 1000 `cpu.step()` calls; does not run until halt.
#![no_main]

use crate::components::cpu::Cpu;
use libfuzzer_sys::fuzz_target;
use sres_emulator::bus::SresBus;

fuzz_target!(|data: &[u8]| {
    // Load a random program into the emulator.
    let mut cpu = Cpu::new(SresBus::with_program(data));
    // This can fail in all kinds of ways, but it should never ever panic!
    for _ in 0..1000 {
        cpu.step();
    }
});
