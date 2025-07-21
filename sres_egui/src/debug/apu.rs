use egui::Color32;
use egui::Context;
use egui::Stroke;
use egui::Ui;
use sres_emulator::components::s_dsp::voice::GainMode;
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
        egui::Window::new("APU Debug")
            .open(&mut self.open)
            .default_width(800.0)
            .default_height(600.0)
            .show(ctx, |ui| {
                let debug = emulator.debug();
                let apu_debug = debug.apu();

                ui.heading("S-DSP Voice Status");
                ui.separator();

                ui.vertical(|ui| {
                    for i in 0..8 {
                        voice_detail_widget(ui, i, &apu_debug.dsp(), apu_debug.ram());
                        if i % 2 == 1 {
                            ui.end_row();
                        }
                    }
                });

                ui.separator();
                ui.heading("Global DSP Status");
                global_dsp_state_widget(ui, &apu_debug.dsp(), apu_debug.ram());
            });
    }
}

fn voice_detail_widget(
    ui: &mut Ui,
    voice_id: usize,
    dsp: &sres_emulator::components::s_dsp::SDspDebug,
    ram: &[u8],
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.heading(format!("Voice {}", voice_id));

            // Get voice data directly from the Voice struct
            let voice = &dsp.voices()[voice_id];
            let vol_l = voice.vol_l;
            let vol_r = voice.vol_r;
            let pitch = voice.pitch;
            let sample_source = voice.sample_source;
            let adsr1 = voice.adsr1;
            let adsr2 = voice.adsr2;
            let gain = voice.gain;
            let envx = voice.envx;
            let outx = voice.outx;

            // Voice activity indicator
            let is_active = envx > 0 || outx != 0;
            let activity_color = if is_active {
                Color32::GREEN
            } else {
                Color32::GRAY
            };
            ui.colored_label(activity_color, if is_active { "●" } else { "○" });

            // Volume settings
            ui.horizontal(|ui| {
                ui.label("Vol:");
                // Volume bars
                let vol_l_norm = (vol_l.abs() as f32) / 127.0;
                let vol_r_norm = (vol_r.abs() as f32) / 127.0;
                volume_bar_widget(ui, vol_l_norm, vol_l < 0);
                volume_bar_widget(ui, vol_r_norm, vol_r < 0);
            });

            // Pitch
            ui.horizontal(|ui| {
                ui.label("Pitch:");
                ui.label(format!(
                    "${:04X} ({:.1} Hz)",
                    pitch,
                    pitch_to_frequency(pitch)
                ));
            });

            // Sample source and envelope
            ui.horizontal(|ui| {
                ui.label("Sample:");
                ui.label(format!("${:02X}", sample_source));
                ui.label("Env:");
                ui.label(format!("{:3}", envx));
                ui.label("Out:");
                ui.label(format!("{:+4}", outx));
            });

            // ADSR/Gain settings
            if adsr1.enable() {
                ui.horizontal(|ui| {
                    ui.label("ADSR:");
                    ui.label(format!(
                        "A:{} D:{} S:{} R:{}",
                        adsr1.attack_rate().value(),
                        adsr1.decay_rate().value(),
                        adsr2.sustain_level().value(),
                        adsr2.release_rate().value()
                    ));
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("GAIN:");
                    match gain.mode() {
                        GainMode::Fixed(value) => {
                            ui.label(format!("Fixed {}", value));
                        }
                        GainMode::LinearDecay(rate) => {
                            ui.label(format!("Lin Dec {}", rate));
                        }
                        GainMode::ExponentialDecay(rate) => {
                            ui.label(format!("Exp Dec {}", rate));
                        }
                        GainMode::LinearIncrease(rate) => {
                            ui.label(format!("Lin Inc {}", rate));
                        }
                        GainMode::BentIncrease(rate) => {
                            ui.label(format!("Bent Inc {}", rate));
                        }
                    }
                });
            }

            // Envelope visualization
            envelope_widget(ui, envx);

            // Sample directory info
            if sample_source != 0 {
                let dir_offset = dsp.sample_directory() as usize * 0x100;
                let source_addr = dir_offset + sample_source as usize * 4;
                if source_addr + 3 < ram.len() {
                    let start_addr = u16::from_le_bytes([ram[source_addr], ram[source_addr + 1]]);
                    let loop_addr =
                        u16::from_le_bytes([ram[source_addr + 2], ram[source_addr + 3]]);
                    ui.horizontal(|ui| {
                        ui.label("Sample:");
                        ui.label(format!("Start:${:04X} Loop:${:04X}", start_addr, loop_addr));
                    });
                }
            }
        });
    });
}

fn global_dsp_state_widget(
    ui: &mut Ui,
    dsp: &sres_emulator::components::s_dsp::SDspDebug,
    _ram: &[u8],
) {
    ui.horizontal(|ui| {
        ui.label("Sample Directory:");
        ui.label(format!("${:04X}", dsp.sample_directory() as u16 * 0x100));
    });

    ui.horizontal(|ui| {
        ui.label("Flags:");
        let flags = dsp.flags();
        ui.label(format!(
            "Reset:{} Mute:{} Echo:{} Noise:{:02X}",
            flags.reset(),
            flags.mute(),
            !flags.echo_disable(),
            flags.noise_frequency().value()
        ));
    });

    // Voice key on/off status
    ui.horizontal(|ui| {
        ui.label("Key On:");
        let kon = dsp.key_on();
        for i in 0..8 {
            let on = (kon & (1 << i)) != 0;
            ui.colored_label(
                if on { Color32::GREEN } else { Color32::GRAY },
                format!("{}", i),
            );
        }
    });

    ui.horizontal(|ui| {
        ui.label("Key Off:");
        let kof = dsp.key_off();
        for i in 0..8 {
            let off = (kof & (1 << i)) != 0;
            ui.colored_label(
                if off { Color32::RED } else { Color32::GRAY },
                format!("{}", i),
            );
        }
    });

    // Noise enable
    ui.horizontal(|ui| {
        ui.label("Noise:");
        let noise = dsp.noise_enable();
        for i in 0..8 {
            let noise_on = (noise & (1 << i)) != 0;
            ui.colored_label(
                if noise_on {
                    Color32::YELLOW
                } else {
                    Color32::GRAY
                },
                format!("{}", i),
            );
        }
    });
}

fn volume_bar_widget(ui: &mut Ui, level: f32, negative: bool) {
    let size = egui::Vec2::new(40.0, 8.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

    let bg_color = Color32::from_gray(50);
    let fg_color = if negative {
        Color32::RED
    } else {
        Color32::GREEN
    };

    ui.painter().rect_filled(rect, 2.0, bg_color);

    let fill_width = rect.width() * level;
    let fill_rect = egui::Rect::from_min_size(rect.min, egui::Vec2::new(fill_width, rect.height()));
    ui.painter().rect_filled(fill_rect, 2.0, fg_color);

    ui.painter().rect_stroke(
        rect,
        2.0,
        Stroke::new(1.0, Color32::WHITE),
        egui::StrokeKind::Inside,
    );
}

fn envelope_widget(ui: &mut Ui, envx: u8) {
    let size = egui::Vec2::new(60.0, 20.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

    let bg_color = Color32::from_gray(30);
    let fg_color = Color32::LIGHT_BLUE;

    ui.painter().rect_filled(rect, 2.0, bg_color);

    let level = (envx as f32) / 127.0;
    let fill_height = rect.height() * level;
    let fill_rect = egui::Rect::from_min_size(
        egui::Pos2::new(rect.min.x, rect.max.y - fill_height),
        egui::Vec2::new(rect.width(), fill_height),
    );
    ui.painter().rect_filled(fill_rect, 2.0, fg_color);

    ui.painter().rect_stroke(
        rect,
        2.0,
        Stroke::new(1.0, Color32::WHITE),
        egui::StrokeKind::Inside,
    );

    // Add envelope value text
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{}", envx),
        egui::FontId::monospace(10.0),
        Color32::WHITE,
    );
}

fn pitch_to_frequency(pitch: u16) -> f32 {
    // SNES pitch calculation: frequency = (pitch / 4096) * 32000 Hz
    (pitch as f32 / 4096.0) * 32000.0
}
