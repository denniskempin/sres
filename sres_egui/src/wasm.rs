#![cfg(target_arch = "wasm32")]

use base64;
use eframe::wasm_bindgen;
use eframe::wasm_bindgen::prelude::*;
use web_sys;

use crate::EmulatorApp;
use crate::Rom;

pub fn save_rom_in_local_storage(rom: &[u8]) {
    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    storage.set_item("rom", &base64::encode(rom)).unwrap();
}

#[wasm_bindgen]
pub fn start_app(canvas_id: &str) {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let canvas_id_owned = canvas_id.to_owned();
    wasm_bindgen_futures::spawn_local(async move {
        eframe::start_web(
            &canvas_id_owned,
            Default::default(),
            Box::new(|cc| {
                let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                let initial_rom = storage.get_item("rom").unwrap();
                let initial_rom = initial_rom
                    .map(|raw| Rom::load_from_bytes("last_rom", &base64::decode(raw).unwrap()));
                Box::new(EmulatorApp::new(cc, initial_rom))
            }),
        )
        .await
        .expect("failed to start eframe");
    });
}
