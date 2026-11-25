//! Nightly `#[bench]` micro-bench of `Clock::advance_master_clock` via `test::Bencher`.
//! One iteration runs `44671` steps of `8` master cycles (`357368` = `262 * 1364`, one NTSC frame without the short scanline).
#![feature(test)]

extern crate test;

use std::hint::black_box;

use sres_emulator::components::clock::Clock;
use test::Bencher;

#[bench]
fn advance_master_clock(b: &mut Bencher) {
    let mut timer = Clock::default();
    b.iter(|| {
        // Simulate how often the timer is advanced in one frame
        for _ in 0..44671 {
            timer.advance_master_clock(black_box(8));
        }
    });
}
