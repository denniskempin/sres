use std::collections::HashMap;
use std::ffi::OsStr;
use std::time::Duration;

use eframe::CreationContext;
use eframe::Frame;
use egui::Color32;
use egui::ColorImage;
use egui::Context;
use egui::DroppedFile;
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

    fn load_dropped_file(&mut self, drop: &DroppedFile) {
        if let Some(path) = &drop.path {
            match path.extension().and_then(OsStr::to_str) {
                Some("sfc") => {
                    self.load_cartridge(Cartridge::with_sfc_file(path).unwrap());
                }
                _ => {
                    panic!("Unknown file type");
                }
            }
        } else if let Some(bytes) = &drop.bytes {
            //#[cfg(target_arch = "wasm32")]
            //crate::wasm::save_rom_in_local_storage(bytes);
            self.load_cartridge(Cartridge::with_sfc_data(bytes, None).unwrap());
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

    fn emulator_ui(&mut self, ctx: &Context) {
        puffin::profile_function!();
        puffin::GlobalProfiler::lock().new_frame();
        let start = Instant::now();

        // Load new program if a file is dropped on the app
        ctx.input(|input| {
            if !input.raw.dropped_files.is_empty() {
                self.load_dropped_file(&input.raw.dropped_files[0]);
            }
        });

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu_bar(ui);
        });
        if self.loaded_cartridge.is_none() {
            return;
        }
        ctx.input(|input| {
            self.update_keys(input);
        });

        let stable_dt = ctx.input(|input| input.stable_dt as f64);

        if !self.emulator.debugger().enabled() {
            puffin::set_scopes_on(false);
            self.emulator
                .execute_for_audio_samples(self.audio_output.samples_needed_to_maintain_buffer());
        } else {
            puffin::set_scopes_on(self.debug_ui.show_profiler);
            self.debug_ui.run_emulator(&mut self.emulator, stable_dt);

            egui::SidePanel::right("right_debug_panel")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.style_mut().override_font_id = Some(FontId::monospace(12.0));
                    self.debug_ui.right_debug_panel(ui, &self.emulator);
                });

            egui::TopBottomPanel::bottom("bottom_debug_panel").show(ctx, |ui| {
                self.debug_ui.bottom_debug_panel(ui, &self.emulator);
            });
        }

        // Update audio output with new samples from the APU
        self.audio_output.update(&mut self.emulator);

        // Render emulator display
        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_display(ui);
        });

        if self.emulator.debugger().enabled() {
            self.debug_ui.modals(ctx, &mut self.emulator);
        }

        self.past_frame_times.push(start.elapsed());

        // Always repaint to keep rendering at 60Hz.
        ctx.request_repaint()
    }
}

impl eframe::App for EmulatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.loaded_cartridge.is_none() {
            home::home_screen(ctx, |cartridge| {
                self.load_cartridge(cartridge);
            });
        } else {
            self.emulator_ui(ctx);
        }
    }
}
