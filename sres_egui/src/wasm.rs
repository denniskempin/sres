#![cfg(target_arch = "wasm32")]

use base64;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use eframe::wasm_bindgen;
use eframe::wasm_bindgen::prelude::*;
use sres_emulator::components::cartridge::Cartridge;
use web_sys;

use crate::EmulatorApp;

#[wasm_bindgen]
pub fn start_app(canvas_id: &str) {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let canvas_id_owned = canvas_id.to_owned();
    wasm_bindgen_futures::spawn_local(async move {
        eframe::WebRunner::new()
            .start(
                &canvas_id_owned,
                Default::default(),
                Box::new(|cc| {
                    let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
                    let initial_rom = storage.get_item("rom").unwrap();
                    let initial_rom = initial_rom.map(|raw| {
                        Cartridge::with_sfc_data(&STANDARD.decode(raw).unwrap(), None).unwrap()
                    });
                    Box::new(EmulatorApp::new(cc, initial_rom))
                }),
            )
            .await
            .expect("failed to start eframe");
    });
}
