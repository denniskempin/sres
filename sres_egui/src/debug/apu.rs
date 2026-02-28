use egui::Color32;
use egui::Context;
use egui::Stroke;
use egui::Ui;
use sres_emulator::components::s_dsp::voice::AudioRingBuffer;
use sres_emulator::components::s_dsp::voice::GainMode;
use sres_emulator::components::s_dsp::voice::OUTX_BUFFER_SIZE;
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
            .default_width(1200.0)
            .default_height(800.0)
            .show(ctx, |ui| {
                let debug = emulator.debug();
                let apu_debug = debug.apu();

                ui.heading("S-DSP Voice Status");
                ui.separator();

                egui::Grid::new("voice_grid")
                    .num_columns(4)
                    .spacing([5.0, 5.0])
                    .show(ui, |ui| {
                        for i in 0..8 {
                            voice_detail_widget(ui, i, &apu_debug.dsp(), apu_debug.ram());
                            if i % 4 == 3 {
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
        ui.set_width(200.0);
        ui.vertical(|ui| {
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

            // Voice header with activity indicator
            ui.horizontal(|ui| {
                ui.heading(format!("Voice {voice_id}"));
                let is_active = envx > 0 || outx != 0;
                let activity_color = if is_active {
                    Color32::GREEN
                } else {
                    Color32::GRAY
                };
                ui.colored_label(activity_color, if is_active { "●" } else { "○" });
            });

            // Volume bars
            ui.horizontal(|ui| {
                ui.label("Vol:");
                let vol_l_norm = (vol_l.abs() as f32) / 127.0;
                let vol_r_norm = (vol_r.abs() as f32) / 127.0;
                volume_bar_widget(ui, vol_l_norm, vol_l < 0);
                volume_bar_widget(ui, vol_r_norm, vol_r < 0);
            });

            // Pitch (shortened)
            ui.label(format!(
                "Pitch: ${:04X} ({:.2} Hz)",
                pitch,
                pitch_to_frequency(pitch)
            ));

            // Sample, envelope, output in compact format
            ui.label(format!("Src:${sample_source:02X} Env:{envx} Out:{outx:+4}"));

            // ADSR/Gain settings (compact)
            if adsr1.enable() {
                ui.label(format!(
                    "ADSR: {}/{}/{}/{}",
                    adsr1.attack_rate().value(),
                    adsr1.decay_rate().value(),
                    adsr2.sustain_level().value(),
                    adsr2.release_rate().value()
                ));
            } else {
                ui.label(format!(
                    "GAIN: {}",
                    match gain.mode() {
                        GainMode::Fixed(value) => format!("Fix {value}"),
                        GainMode::LinearDecay(rate) => format!("LDec {rate}"),
                        GainMode::ExponentialDecay(rate) => format!("EDec {rate}"),
                        GainMode::LinearIncrease(rate) => format!("LInc {rate}"),
                        GainMode::BentIncrease(rate) => format!("BInc {rate}"),
                    }
                ));
            }

            // Envelope visualization
            envelope_widget(ui, envx);

            // Waveform visualization
            ui.label("Waveform:");
            waveform_widget(ui, &voice.outx_buffer);

            // Sample directory info (compact)
            if sample_source != 0 {
                let dir_offset = dsp.sample_directory() as usize * 0x100;
                let source_addr = dir_offset + sample_source as usize * 4;
                if source_addr + 3 < ram.len() {
                    let start_addr = u16::from_le_bytes([ram[source_addr], ram[source_addr + 1]]);
                    let loop_addr =
                        u16::from_le_bytes([ram[source_addr + 2], ram[source_addr + 3]]);
                    ui.label(format!("Start:${start_addr:04X}"));
                    ui.label(format!("Loop:${loop_addr:04X}"));
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
                format!("{i}"),
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
                format!("{i}"),
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
                format!("{i}"),
            );
        }
    });
}

fn volume_bar_widget(ui: &mut Ui, level: f32, negative: bool) {
    let size = egui::Vec2::new(30.0, 6.0);
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
    let size = egui::Vec2::new(50.0, 16.0);
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
        format!("{envx}"),
        egui::FontId::monospace(10.0),
        Color32::WHITE,
    );
}

fn waveform_widget(ui: &mut Ui, buffer: &AudioRingBuffer<OUTX_BUFFER_SIZE>) {
    let size = egui::Vec2::new(160.0, 30.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

    let bg_color = Color32::from_gray(20);
    let line_color = Color32::LIGHT_GREEN;

    ui.painter().rect_filled(rect, 2.0, bg_color);

    // Draw center line
    let center_y = rect.center().y;
    ui.painter().line_segment(
        [
            egui::Pos2::new(rect.min.x, center_y),
            egui::Pos2::new(rect.max.x, center_y),
        ],
        Stroke::new(1.0, Color32::from_gray(60)),
    );

    // Draw waveform
    let mut prev_point: Option<egui::Pos2> = None;

    for (i, &sample) in buffer.iter().enumerate() {
        let x = rect.min.x + (i as f32 / (OUTX_BUFFER_SIZE - 1) as f32) * rect.width();
        let normalized = (sample as f32) / 32768.0; // Normalize i16 to [-1, 1]
        let y = center_y - normalized * (rect.height() / 2.0);
        let point = egui::Pos2::new(x, y.clamp(rect.min.y, rect.max.y));

        if let Some(prev) = prev_point {
            ui.painter()
                .line_segment([prev, point], Stroke::new(1.0, line_color));
        }
        prev_point = Some(point);
    }

    ui.painter().rect_stroke(
        rect,
        2.0,
        Stroke::new(1.0, Color32::WHITE),
        egui::StrokeKind::Inside,
    );
}

fn pitch_to_frequency(pitch: u16) -> f32 {
    // SNES pitch calculation: frequency = (pitch / 4096) * 32000 Hz
    (pitch as f32 / 4096.0) * 32000.0
}

#[cfg(test)]
mod tests {
    use sres_emulator::components::s_dsp::voice::AudioRingBuffer;
    use sres_emulator::components::s_dsp::voice::OUTX_BUFFER_SIZE;

    use super::*;

    /// All APU visualization widgets in one combined snapshot.
    #[test]
    fn apu_widgets() {
        let silent = AudioRingBuffer::<OUTX_BUFFER_SIZE>::default();
        let mut sine = AudioRingBuffer::<OUTX_BUFFER_SIZE>::default();
        for i in 0..OUTX_BUFFER_SIZE {
            let phase = i as f32 / OUTX_BUFFER_SIZE as f32;
            let sample = ((phase * 2.0 * std::f32::consts::PI).sin() * 16000.0) as i16;
            sine.push(sample);
        }

        crate::test_utils::widget_snapshot("apu/apu_widgets", move |ui| {
            ui.vertical(|ui| {
                ui.label("── volume_bar_widget ──");
                ui.horizontal(|ui| {
                    volume_bar_widget(ui, 0.0, false); // empty
                    volume_bar_widget(ui, 0.5, true); // negative
                    volume_bar_widget(ui, 0.75, false); // positive
                    volume_bar_widget(ui, 1.0, false); // full
                });

                ui.label("── envelope_widget ──");
                ui.horizontal(|ui| {
                    envelope_widget(ui, 0); // empty
                    envelope_widget(ui, 64); // half
                    envelope_widget(ui, 127); // full
                });

                ui.label("── waveform_widget ──");
                waveform_widget(ui, &silent); // flat line
                waveform_widget(ui, &sine); // sine wave
            });
        });
    }
}
