mod debug;
mod wasm;

use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use eframe::CreationContext;
use eframe::Frame;
use egui::ColorImage;
use egui::Context;
use egui::DroppedFile;
use egui::FontId;
use egui::Image;
use egui::InputState;
use egui::Layout;
use egui::Sense;
use egui::TextureHandle;
use egui::TextureOptions;
use egui::Ui;
use sres_emulator::System;
use tracing::instrument;

use self::debug::DebugUi;

const PROGRAMS: &[(&str, &[u8])] = &[];

const GAMES: &[(&str, &[u8])] = &[];

pub struct Rom {
    sfc_data: Vec<u8>,
}

impl Rom {
    pub fn load_from_file(path: &Path) -> Rom {
        let sfc_data = fs::read(path).unwrap();
        Rom { sfc_data }
    }

    pub fn load_from_bytes(_name: &str, sfc_data: &[u8]) -> Rom {
        Rom {
            sfc_data: sfc_data.to_owned(),
        }
    }
}

pub struct EmulatorApp {
    emulator: System,
    loaded_rom: Option<Rom>,
    framebuffer_texture: TextureHandle,
    debug_ui: DebugUi,
}

impl EmulatorApp {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext<'_>, rom: Option<Rom>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut app = EmulatorApp {
            emulator: System::new(),
            loaded_rom: None,
            framebuffer_texture: cc.egui_ctx.load_texture(
                "Framebuffer",
                ColorImage::example(),
                Default::default(),
            ),
            debug_ui: DebugUi::new(cc),
        };

        if let Some(rom) = rom {
            app.load_rom(rom);
        }
        app
    }

    fn load_rom(&mut self, rom: Rom) {
        self.emulator = System::with_sfc_bytes(&rom.sfc_data).unwrap();
        self.emulator.enable_debugger();
        self.loaded_rom = Some(rom);
    }

    fn load_dropped_file(&mut self, drop: &DroppedFile) {
        if let Some(path) = &drop.path {
            match path.extension().and_then(OsStr::to_str) {
                Some("sfc") => {
                    self.load_rom(Rom::load_from_file(path));
                }
                _ => {
                    panic!("Unknown file type");
                }
            }
        } else if let Some(bytes) = &drop.bytes {
            #[cfg(target_arch = "wasm32")]
            crate::wasm::save_rom_in_local_storage(bytes);
            self.load_rom(Rom::load_from_bytes(&drop.name, bytes));
        }
    }

    fn update_keys(&mut self, _input: &InputState) {}

    fn menu_bar(&mut self, ui: &mut Ui) {
        ui.columns(2, |columns| {
            columns[0].with_layout(Layout::left_to_right(egui::Align::Min), |ui| {
                ui.menu_button("Programs", |ui| {
                    for program in PROGRAMS {
                        if ui.button(program.0).clicked() {
                            self.load_rom(Rom::load_from_bytes(program.0, program.1));
                        }
                    }
                });
                ui.menu_button("Games", |ui| {
                    for program in GAMES {
                        if ui.button(program.0).clicked() {
                            self.load_rom(Rom::load_from_bytes(program.0, program.1));
                        }
                    }
                });
                ui.label("(Or drop an .sfc file to load it)");
            });
            columns[1].with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                if ui.button("Debug").clicked() {
                    if self.emulator.is_debugger_enabled() {
                        self.emulator.disable_debugger()
                    } else {
                        self.emulator.enable_debugger()
                    }
                }
            });
        });
    }

    fn main_display(&mut self, ui: &mut Ui) {
        let framebuffer = self.emulator.cpu.bus.ppu.backgrounds[0]
            .debug_render_tilemap(&self.emulator.cpu.bus.ppu.vram);

        self.framebuffer_texture.set(
            ColorImage::from_rgba_unmultiplied(
                [framebuffer.width() as usize, framebuffer.height() as usize],
                framebuffer.as_raw(),
            ),
            TextureOptions::default(),
        );

        let desired_size = ui.available_size();
        let (whole_rect, _) =
            ui.allocate_exact_size(desired_size, Sense::focusable_noninteractive());

        let image = Image::new(
            &self.framebuffer_texture,
            self.framebuffer_texture.size_vec2(),
        );
        image.paint_at(ui, whole_rect);
    }
}

impl eframe::App for EmulatorApp {
    #[instrument(skip_all)]
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        // Load new program if a file is dropped on the app
        if !ctx.input().raw.dropped_files.is_empty() {
            self.load_dropped_file(&ctx.input().raw.dropped_files[0]);
        }

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            self.menu_bar(ui);
        });

        if self.loaded_rom.is_none() {
            return;
        }

        self.update_keys(&ctx.input());

        if !self.emulator.is_debugger_enabled() {
            self.emulator
                .execute_for_duration(ctx.input().stable_dt as f64);
        } else {
            self.debug_ui
                .run_emulator(&mut self.emulator, ctx.input().unstable_dt as f64);

            egui::SidePanel::right("right_debug_panel")
                .resizable(false)
                .show(ctx, |ui| {
                    ui.style_mut().override_font_id = Some(FontId::monospace(12.0));
                    self.debug_ui.right_debug_panel(ui, &self.emulator);
                });
        }

        // Render emulator display
        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_display(ui);
        });

        // Always repaint to keep rendering at 60Hz.
        ctx.request_repaint()
    }
}
