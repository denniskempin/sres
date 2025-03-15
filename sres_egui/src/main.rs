pub mod app;
pub mod audio;
pub mod debug;
pub mod embedded_roms;
pub mod home;
pub mod util;

use app::EmulatorApp;

/// Rust Entertainment System
#[cfg(not(target_arch = "wasm32"))]
#[derive(argh::FromArgs)]
struct ResArgs {
    /// rom file to load
    #[argh(positional)]
    rom: Option<String>,

    /// enable generation of trace files
    #[argh(option)]
    trace_file: Option<std::path::PathBuf>,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use egui::vec2;
    use egui::ViewportBuilder;
    use sres_emulator::common::logging;
    use sres_emulator::components::cartridge::Cartridge;
    use tracing_chrome::ChromeLayerBuilder;
    use tracing_subscriber::prelude::*;

    logging::init();
    let args: ResArgs = argh::from_env();
    let _tracing_guard = if let Some(trace_file) = args.trace_file {
        let (chrome_layer, guard) = ChromeLayerBuilder::new().file(trace_file).build();
        tracing_subscriber::registry().with(chrome_layer).init();
        Some(guard)
    } else {
        None
    };

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size(vec2(1024.0, 768.0)),
        ..Default::default()
    };

    let cartridge = args.rom.map(|path| {
        let path = std::path::PathBuf::from(path);
        Cartridge::with_sfc_file(&path).unwrap()
    });

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
