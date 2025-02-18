#![cfg(not(target_arch = "wasm32"))]

use std::path::PathBuf;

use argh::FromArgs;
use egui::{vec2, ViewportBuilder};
use sres_egui::EmulatorApp;
use sres_emulator::common::logging;
use sres_emulator::components::cartridge::Cartridge;
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::prelude::*;

/// Rust Entertainment System
#[derive(FromArgs)]
struct ResArgs {
    /// rom file to load
    #[argh(positional)]
    rom: Option<String>,

    /// enable generation of trace files
    #[argh(option)]
    trace_file: Option<PathBuf>,
}

fn main() {
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
        let path = PathBuf::from(path);
        Cartridge::with_sfc_file(&path).unwrap()
    });

    eframe::run_native(
        "Super Rust Entertainment System",
        native_options,
        Box::new(|cc| Box::new(EmulatorApp::new(cc, cartridge))),
    )
    .unwrap();
}
