//! Criterion micro-bench of `Clock::advance_master_clock`.
//! One iteration is `44671` steps of `8` master cycles (`357368` = `262 * 1364`, one NTSC frame without the short scanline).
use std::hint::black_box;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sres_emulator::components::clock::Clock;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("advance_master_clock", |b| {
        let mut timer = Clock::default();
        b.iter(|| {
            // Simulate how often the timer is advanced in one frame
            for _ in 0..44671 {
                timer.advance_master_clock(black_box(8))
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
