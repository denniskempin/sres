use std::path::PathBuf;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BatchSize;
use criterion::Criterion;
use sres_emulator::System;

fn criterion_benchmark(c: &mut Criterion) {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rom_path = root_dir.join("tests/rom_tests/krom_adc.sfc");
    c.bench_function("krom_frame_time", |b| {
        b.iter_batched_ref(
            || System::with_sfc(&rom_path).unwrap(),
            |system: &mut System| system.execute_one_frame(),
            BatchSize::LargeInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
