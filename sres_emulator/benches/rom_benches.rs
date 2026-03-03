//! Benchmarks that measure the time it takes to execute each frame of executing a ROM.
use std::path::PathBuf;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;
use sres_emulator::components::cartridge::Cartridge;
use sres_emulator::AsyncSystem;
use sres_emulator::BatchedSystem;
use sres_emulator::SyncSystem;
use sres_emulator::System;

fn criterion_benchmark(c: &mut Criterion) {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let adc_rom_path = root_dir.join("tests/rom_tests/krom_adc.sfc");
    let blend_rom_path = root_dir.join("tests/ppu_tests/krom_blend_hicolor_3840.sfc");

    c.bench_function("krom_adc_frame_time_sync", |b| {
        let mut system =
            SyncSystem::with_cartridge(&Cartridge::with_sfc_file(&adc_rom_path).unwrap());
        b.iter(|| system.execute_frames(1));
    });
    c.bench_function("krom_adc_frame_time_batched", |b| {
        let mut system =
            BatchedSystem::with_cartridge(&Cartridge::with_sfc_file(&adc_rom_path).unwrap());
        b.iter(|| system.execute_frames(1));
    });
    c.bench_function("krom_adc_frame_time_async", |b| {
        let mut system =
            AsyncSystem::with_cartridge(&Cartridge::with_sfc_file(&adc_rom_path).unwrap());
        b.iter(|| system.execute_frames(1));
    });

    c.bench_function("krom_adc_frame_time_headless", |b| {
        let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&adc_rom_path).unwrap());
        system.force_headless();
        b.iter(|| system.execute_frames(1));
    });

    c.bench_function("krom_blend_frame_time", |b| {
        let mut system =
            System::with_cartridge(&Cartridge::with_sfc_file(&blend_rom_path).unwrap());
        b.iter(|| system.execute_frames(1));
    });

    c.bench_function("krom_blend_frame_time_headless", |b| {
        let mut system =
            System::with_cartridge(&Cartridge::with_sfc_file(&blend_rom_path).unwrap());
        system.force_headless();
        b.iter(|| system.execute_frames(1));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
