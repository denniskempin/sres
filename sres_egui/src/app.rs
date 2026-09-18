//! `EmulatorApp` (`eframe::App::ui`): home screen until a cartridge is loaded, then `emulator_ui`.
//! `load_cartridge` builds `System::with_cartridge` and enables the debugger.
//! File drops load `.sfc` from `ui()` (native path/bytes, WASM `bytes_async`).
//! Debugger-off run uses `execute_for_audio_samples`; present via `swap_video_frame` / `AudioOutput::update`.

use std::collections::HashMap;
use std::ffi::OsStr;
use std::time::Duration;

use eframe::CreationContext;
use eframe::Frame;
use egui::Color32;
use egui::ColorImage;
use egui::DroppedFileHandle;
use egui::FontId;
use egui::Image;
use egui::InputState;
use egui::Key;
use egui::Layout;
use egui::Sense;
use egui::TextureHandle;
use egui::TextureOptions;
use egui::Ui;
use sres_emulator::components::cartridge::Cartridge;
use sres_emulator::components::ppu::Framebuffer;
use sres_emulator::controller::StandardController;
use sres_emulator::System;

use crate::audio::AudioOutput;
use crate::debug::DebugUi;
use crate::home;
use crate::util::EguiImageImpl;
use crate::util::Instant;
use crate::util::RingBuffer;

pub struct EmulatorApp {
    emulator: System,
    loaded_cartridge: Option<Cartridge>,
    framebuffer_texture: TextureHandle,
    debug_ui: DebugUi,
    past_frame_times: RingBuffer<Duration, 60>,
    audio_output: AudioOutput,
    video_frame_buffer: Framebuffer,

    input_recording_active: bool,
    input_recording_last: u16,
    input_recording: HashMap<u64, u16>,
    #[cfg(target_arch = "wasm32")]
    pending_dropped_rom: std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>,
}

impl EmulatorApp {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext<'_>, cartridge: Option<Cartridge>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut app = EmulatorApp {
            emulator: System::new(),
            loaded_cartridge: None,
            framebuffer_texture: cc.egui_ctx.load_texture(
                "Framebuffer",
                ColorImage::filled([32, 32], Color32::BLACK),
                Default::default(),
            ),
            debug_ui: DebugUi::new(cc),
            past_frame_times: RingBuffer::default(),
            audio_output: AudioOutput::new(),
            video_frame_buffer: Framebuffer::default(),
            input_recording: HashMap::new(),
            input_recording_last: 0,
            input_recording_active: false,
            #[cfg(target_arch = "wasm32")]
            pending_dropped_rom: std::sync::Arc::new(std::sync::Mutex::new(None)),
        };

        if let Some(rom) = cartridge {
            app.load_cartridge(rom);
        }
        app
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.emulator = System::with_cartridge(&cartridge);
        self.emulator.debugger().enable();
        self.loaded_cartridge = Some(cartridge);
        // Start audio output when a cartridge is loaded
        self.audio_output.start();
    }

    fn consume_dropped_rom(&mut self, ui: &Ui) {
        let dropped = ui.input(|input| input.raw.dropped_files.first().cloned());
        if let Some(drop) = dropped {
            self.load_dropped_file(&drop);
        }
        #[cfg(target_arch = "wasm32")]
        self.load_pending_dropped_rom();
    }

    fn load_dropped_file(&mut self, drop: &DroppedFileHandle) {
        let path = drop.path();
        if path.extension().and_then(OsStr::to_str) != Some("sfc") {
            log::warn!("Ignoring dropped file {path:?}: not an .sfc ROM");
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let result = if path.is_file() {
                Cartridge::with_sfc_file(path)
            } else {
                match drop.bytes() {
                    Ok(bytes) => Cartridge::with_sfc_data(&bytes, None),
                    Err(err) => {
                        log::error!("Failed to read dropped file {path:?}: {err}");
                        return;
                    }
                }
            };
            match result {
                Ok(cartridge) => self.load_cartridge(cartridge),
                Err(err) => log::error!("Failed to load dropped ROM {path:?}: {err}"),
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let pending = self.pending_dropped_rom.clone();
            let drop = drop.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match drop.bytes_async().await {
                    Ok(bytes) => *pending.lock().unwrap() = Some(bytes),
                    Err(err) => log::error!("Failed to read dropped file: {err}"),
                }
            });
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn load_pending_dropped_rom(&mut self) {
        // Take bytes first so the MutexGuard drops before `&mut self` in `load_cartridge`.
        let Some(bytes) = self.pending_dropped_rom.lock().unwrap().take() else {
            return;
        };
        match Cartridge::with_sfc_data(&bytes, None) {
            Ok(cartridge) => self.load_cartridge(cartridge),
            Err(err) => log::error!("Failed to load dropped ROM: {err}"),
        }
    }

    fn update_keys(&mut self, input: &InputState) {
        let joy1 = StandardController {
            right: input.key_down(Key::ArrowRight),
            left: input.key_down(Key::ArrowLeft),
            up: input.key_down(Key::ArrowUp),
            down: input.key_down(Key::ArrowDown),
            b: input.key_down(Key::Z),
            a: input.key_down(Key::X),
            y: input.key_down(Key::A),
            x: input.key_down(Key::S),
            start: input.key_down(Key::Enter),
            select: input.key_down(Key::Backspace),
            ..Default::default()
        };
        if self.input_recording_active && joy1.to_u16() != self.input_recording_last {
            self.input_recording_last = joy1.to_u16();
            self.input_recording
                .insert(self.emulator.clock_info().f, joy1.to_u16());
        }
        self.emulator.update_joypads(joy1.to_u16(), 0)
    }

    fn menu_bar(&mut self, ui: &mut Ui) {
        ui.columns(2, |columns| {
            columns[0].with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                if ui.link("Super Rust Entertainment System").clicked() {
                    // Unload cartridge to return to home screen
                    self.loaded_cartridge = None;
                }
            });
            columns[1].with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                let avg_duration = self
                    .past_frame_times
                    .iter()
                    .map(|d| d.as_secs_f64())
                    .sum::<f64>()
                    / self.past_frame_times.len() as f64;
                ui.label(format!("{:.2}ms", avg_duration * 1000.0));

                if ui.button("Debug").clicked() {
                    if self.emulator.debugger().enabled() {
                        self.emulator.debugger().disable()
                    } else {
                        self.emulator.debugger().enable()
                    }
                }
                if self.input_recording_active {
                    if ui.button("Save Recording").clicked() {
                        self.input_recording_active = false;
                        let mut file = std::fs::File::create("input_recording.json").unwrap();
                        serde_json::to_writer(&mut file, &self.input_recording).unwrap();
                        self.input_recording.clear();
                    }
                } else if ui.button("Record Input").clicked() {
                    self.input_recording_active = true;
                    self.input_recording.clear();
                }
            });
        });
    }

    fn main_display(&mut self, ui: &mut Ui) {
        if self.emulator.swap_video_frame(&mut self.video_frame_buffer) {
            let video_frame = self.video_frame_buffer.to_rgba::<EguiImageImpl>();
            self.framebuffer_texture
                .set(video_frame, TextureOptions::default());
        }

        let desired_size = ui.available_size();
        let (whole_rect, _) =
            ui.allocate_exact_size(desired_size, Sense::focusable_noninteractive());
        Image::new((
            self.framebuffer_texture.id(),
            self.framebuffer_texture.size_vec2(),
        ))
        .paint_at(ui, whole_rect);
    }

    fn emulator_ui(&mut self, ui: &mut Ui) {
        puffin::profile_function!();
        puffin::GlobalProfiler::lock().new_frame();
        let start = Instant::now();

        egui::Panel::top("menu_bar").show(ui, |ui| {
            self.menu_bar(ui);
        });
        if self.loaded_cartridge.is_none() {
            return;
        }
        ui.input(|input| {
            self.update_keys(input);
        });

        let stable_dt = ui.input(|input| input.stable_dt as f64);

        if !self.emulator.debugger().enabled() {
            puffin::set_scopes_on(false);
            self.emulator
                .execute_for_audio_samples(self.audio_output.samples_needed_to_maintain_buffer());
        } else {
            puffin::set_scopes_on(self.debug_ui.show_profiler);
            self.debug_ui.run_emulator(&mut self.emulator, stable_dt);

            egui::Panel::right("right_debug_panel")
                .resizable(false)
                .show(ui, |ui| {
                    ui.style_mut().override_font_id = Some(FontId::monospace(12.0));
                    self.debug_ui.right_debug_panel(ui, &self.emulator);
                });

            egui::Panel::bottom("bottom_debug_panel").show(ui, |ui| {
                self.debug_ui.bottom_debug_panel(ui, &self.emulator);
            });
        }

        // Update audio output with new samples from the APU
        self.audio_output.update(&mut self.emulator);

        // Render emulator display
        egui::CentralPanel::default().show(ui, |ui| {
            self.main_display(ui);
        });

        if self.emulator.debugger().enabled() {
            self.debug_ui.modals(ui.ctx(), &mut self.emulator);
        }

        self.past_frame_times.push(start.elapsed());

        // Always repaint to keep rendering at 60Hz.
        ui.ctx().request_repaint()
    }
}

impl eframe::App for EmulatorApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        self.consume_dropped_rom(ui);
        if self.loaded_cartridge.is_none() {
            home::home_screen(ui, |cartridge| {
                self.load_cartridge(cartridge);
            });
        } else {
            self.emulator_ui(ui);
        }
    }
}
