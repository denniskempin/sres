use std::path::PathBuf;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BatchSize;
use criterion::Criterion;
use sres_emulator::bus::SresBus;
use sres_emulator::cpu::Cpu;

fn criterion_benchmark(c: &mut Criterion) {
    let rom_path = PathBuf::from("tests/cpu/CPUADC.sfc");
    c.bench_function("krom_speed", |b| {
        b.iter_batched_ref(
            || {
                let mut bus = SresBus::new();
                bus.cartridge.load_sfc(&rom_path).unwrap();
                Cpu::new(bus)
            },
            |cpu: &mut Cpu<SresBus>| {
                for _ in 0..100 {
                    cpu.reset();
                    // Execute first 985 instructions (That's all that's supported so far)
                    for _ in 0..985 {
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
