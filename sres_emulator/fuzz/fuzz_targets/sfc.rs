//! libfuzzer bin `sfc`: feed input bytes to `Cartridge::with_sfc_data`.
//! Discards the loader `Result`; a parse `Err` is a successful iteration.
#![no_main]

use libfuzzer_sys::fuzz_target;
use sres_emulator::components::cartridge::Cartridge;

fuzz_target!(|data: &[u8]| {
    // This will likely fail, but should never panic!
    let _ = Cartridge::with_sfc_data(data, None);
});
