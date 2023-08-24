use std::path::PathBuf;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BatchSize;
use criterion::Criterion;
use sres_emulator::bus::Bus;
use sres_emulator::bus::SresBus;
use sres_emulator::cpu::Cpu;
use sres_emulator::System;

fn criterion_benchmark(c: &mut Criterion) {
    let rom_path = PathBuf::from("tests/krom_tests/CPUADC.sfc");
    c.bench_function("krom_speed", |b| {
        b.iter_batched_ref(
            || {
                let mut system = System::with_sfc(&rom_path).unwrap();
                // Fake RDNMI register. NMI is always true.
                system.cpu.bus.cycle_write_u8(0x004210.into(), 0xC2);
                system.cpu
            },
            |cpu: &mut Cpu<SresBus>| {
                for _ in 0..10 {
                    cpu.reset();
                    // The rom will eventually loop indefinitely, so limit to a fixed number of steps.
                    for _ in 0..100000 {
                        cpu.step();
                    }
                }
            },
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
