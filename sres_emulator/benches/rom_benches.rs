//! Nightly `#[bench]` per-frame `execute_frames(1)` benches via `test::Bencher`.
//! ROMs: `tests/rom_tests/krom_adc.sfc` (`SyncSystem` / `BatchedSystem` / `AsyncSystem` / headless) and `tests/ppu_tests/krom_blend_hicolor_3840.sfc` (`System` / headless).
#![feature(test)]

extern crate test;

use std::path::PathBuf;

use sres_emulator::components::cartridge::Cartridge;
use sres_emulator::AsyncSystem;
use sres_emulator::BatchedSystem;
use sres_emulator::SyncSystem;
use sres_emulator::System;
use test::Bencher;

const ADC_ROM: &str = "tests/rom_tests/krom_adc.sfc";
const BLEND_ROM: &str = "tests/ppu_tests/krom_blend_hicolor_3840.sfc";

fn rom_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[bench]
fn krom_adc_frame_time_sync(b: &mut Bencher) {
    let mut system =
        SyncSystem::with_cartridge(&Cartridge::with_sfc_file(&rom_path(ADC_ROM)).unwrap());
    b.iter(|| system.execute_frames(1));
}

#[bench]
fn krom_adc_frame_time_batched(b: &mut Bencher) {
    let mut system =
        BatchedSystem::with_cartridge(&Cartridge::with_sfc_file(&rom_path(ADC_ROM)).unwrap());
    b.iter(|| system.execute_frames(1));
}

#[bench]
fn krom_adc_frame_time_async(b: &mut Bencher) {
    let mut system =
        AsyncSystem::with_cartridge(&Cartridge::with_sfc_file(&rom_path(ADC_ROM)).unwrap());
    b.iter(|| system.execute_frames(1));
}

#[bench]
fn krom_adc_frame_time_headless(b: &mut Bencher) {
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path(ADC_ROM)).unwrap());
    system.force_headless();
    b.iter(|| system.execute_frames(1));
}

#[bench]
fn krom_blend_frame_time(b: &mut Bencher) {
    let mut system =
        System::with_cartridge(&Cartridge::with_sfc_file(&rom_path(BLEND_ROM)).unwrap());
    b.iter(|| system.execute_frames(1));
}

#[bench]
fn krom_blend_frame_time_headless(b: &mut Bencher) {
    let mut system =
        System::with_cartridge(&Cartridge::with_sfc_file(&rom_path(BLEND_ROM)).unwrap());
    system.force_headless();
    b.iter(|| system.execute_frames(1));
}
