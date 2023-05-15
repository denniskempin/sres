#![no_main]

use libfuzzer_sys::fuzz_target;
use sres_emulator::bus::TestBus;
use sres_emulator::cpu::Cpu;

fuzz_target!(|data: &[u8]| {
    // Load a random program into the emulator.
    let mut cpu = Cpu::new(TestBus::with_program(data));
    // This can fail in all kinds of ways, but it should never ever panic!
    for _ in 0..100000 {
        println!("{}", cpu.trace());
        cpu.step();
    }
});
