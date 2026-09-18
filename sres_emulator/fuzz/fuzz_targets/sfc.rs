//! libfuzzer bin `sfc`: feed input bytes to the cartridge SFC loader.
//! Discards the loader `Result`; a parse `Err` is a successful iteration.
#![no_main]

use libfuzzer_sys::fuzz_target;
use sres_emulator::cartridge::Cartridge;

fuzz_target!(|data: &[u8]| {
    let mut cartridge = Cartridge::new();
    // This will likely fail, but should never panic!
    let _ = cartridge.load_sfc_data(data);
});
