//! Native and WASM `main` for `EmulatorApp`.
//! Native calls `parse_rom_arg` (optional `.sfc` path) and `eframe::run_native`.
//! WASM starts `eframe::WebRunner` on `#emulator_canvas` with no cartridge.

pub mod app;
pub mod audio;
pub mod debug;
pub mod embedded_roms;
pub mod home;
#[cfg(test)]
mod test_utils;
pub mod util;

use app::EmulatorApp;

#[cfg(not(target_arch = "wasm32"))]
fn parse_rom_arg() -> Option<std::path::PathBuf> {
    let mut rom = None;
    for arg in std::env::args_os().skip(1) {
        if arg == "-h" || arg == "--help" {
            println!("Usage: sres_egui [ROM]");
            println!();
            println!("Options:");
            println!("  -h, --help  Show this help message and exit");
            std::process::exit(0);
        }
        if arg.to_str().is_some_and(|s| s.starts_with('-')) {
            eprintln!("Unknown option: {}", arg.to_string_lossy());
            std::process::exit(2);
        }
        if rom.is_some() {
            eprintln!("Expected at most one positional ROM path");
            std::process::exit(2);
        }
        rom = Some(std::path::PathBuf::from(arg));
    }
    rom
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use egui::vec2;
    use egui::ViewportBuilder;
    use sres_emulator::common::logging;
    use sres_emulator::components::cartridge::Cartridge;

    logging::init();
    let rom = parse_rom_arg();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(vec2(1024.0, 768.0)),
        ..Default::default()
    };

    let cartridge = rom.map(|path| Cartridge::with_sfc_file(&path).unwrap());

    eframe::run_native(
        "Super Rust Entertainment System",
        native_options,
        Box::new(|cc| Ok(Box::new(EmulatorApp::new(cc, cartridge)))),
    )
    .unwrap();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("emulator_canvas")
            .expect("Failed to find emulator_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("emulator_canvas was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(EmulatorApp::new(cc, None)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
