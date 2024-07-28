use egui::Context;
use sres_emulator::System;

pub struct ApuDebugWindow {
    pub open: bool,
}

impl ApuDebugWindow {
    pub fn new() -> Self {
        ApuDebugWindow { open: false }
    }

    pub fn show(&mut self, ctx: &Context, emulator: &System) {
        egui::Window::new("PPU")
            .open(&mut self.open)
            .show(ctx, |ui| {
                let dsp = &emulator.cpu.bus.apu.debug().dsp();
                for i in 0..6 {
                    let voice = dsp.voice(i);
                    ui.label(format!("Voice {}: {}", i, voice));
                }
            });
    }
}
