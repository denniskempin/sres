//! Simple fuzzer that executes random programs on the emulator.
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
