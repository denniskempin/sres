use egui::Context;
use sres_emulator::System;

pub struct ApuDebugWindow {
    open: bool,
}

impl ApuDebugWindow {
    pub fn new() -> Self {
        ApuDebugWindow { open: false }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn show(&mut self, ctx: &Context, emulator: &System) {
        egui::Window::new("PPU")
            .open(&mut self.open)
            .show(ctx, |ui| {
                let debug = emulator.debug();
                let dsp = &debug.apu().dsp();
                for i in 0..6 {
                    let voice = dsp.voice(i);
                    ui.label(format!("Voice {}: {}", i, voice));
                }
            });
    }
}
