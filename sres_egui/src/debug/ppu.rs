use eframe::CreationContext;
use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;
use egui::TextureOptions;
use egui::Ui;
use egui::Vec2;
use sres_emulator::ppu::Background;
use sres_emulator::ppu::BackgroundId;
use sres_emulator::ppu::Ppu;
use sres_emulator::System;

use crate::util::EguiImageImpl;

pub struct PpuDebugWindow {
    pub open: bool,
    background_widget: PpuBackgroundWidget,
}

impl PpuDebugWindow {
    pub fn new(cc: &CreationContext) -> Self {
        PpuDebugWindow {
            open: false,
            background_widget: PpuBackgroundWidget::new(cc),
        }
    }

    pub fn show(&mut self, ctx: &Context, emulator: &System) {
        egui::Window::new("PPU")
            .open(&mut self.open)
            .show(ctx, |ui| {
                ppu_status_widget(ui, &emulator.cpu.bus.ppu);
                ui.separator();
                self.background_widget.show(ui, &emulator.cpu.bus.ppu);
            });
    }
}

pub fn ppu_status_widget(ui: &mut Ui, ppu: &Ppu) {
    ui.label(format!("V, H: ({}, {})", ppu.timer.v, ppu.timer.hdot()));
}

struct PpuBackgroundWidget {
    selected_bg: BackgroundId,
    tilemap_texture: TextureHandle,
    tileset_texture: TextureHandle,
}

impl PpuBackgroundWidget {
    pub fn new(cc: &CreationContext) -> Self {
        PpuBackgroundWidget {
            selected_bg: BackgroundId::BG0,
            tilemap_texture: cc.egui_ctx.load_texture(
                "Tilemap",
                ColorImage::example(),
                Default::default(),
            ),
            tileset_texture: cc.egui_ctx.load_texture(
                "Tileset",
                ColorImage::example(),
                Default::default(),
            ),
        }
    }

    pub fn update_textures(&mut self, ppu: &Ppu) {
        self.tilemap_texture.set(
            ppu.debug_render_background::<EguiImageImpl>(self.selected_bg),
            TextureOptions::default(),
        );
        self.tileset_texture.set(
            ppu.debug_render_tileset::<EguiImageImpl>(self.selected_bg),
            TextureOptions::default(),
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &Ppu) {
        self.update_textures(ppu);

        tabs_widget(
            ui,
            &[
                BackgroundId::BG0,
                BackgroundId::BG1,
                BackgroundId::BG2,
                BackgroundId::BG3,
            ],
            &mut self.selected_bg,
        );
        let background = &ppu.backgrounds[self.selected_bg as usize];
        ui.label(format!(
            "Scroll: ({}, {})",
            background.h_offset, background.v_offset
        ));
        ui.horizontal(|ui| {
            tilemap_widget(ui, background, &self.tilemap_texture);
            tileset_widget(ui, background, &self.tileset_texture);
        });
    }
}

fn tabs_widget<T: ToString + PartialEq + Copy>(ui: &mut Ui, tabs: &[T], selected: &mut T) {
    ui.horizontal(|ui| {
        for tab in tabs.iter() {
            ui.selectable_value(selected, *tab, tab.to_string());
        }
    });
}

fn tilemap_widget(ui: &mut Ui, background: &Background, tilemap_texture: &TextureHandle) {
    ui.vertical(|ui| {
        ui.label(format!(
            "Tilemap (0x{:04X}, {})",
            background.tilemap_addr, background.tilemap_size
        ));
        ui.image(tilemap_texture, Vec2::new(512.0, 512.0));
    });
}

fn tileset_widget(ui: &mut Ui, background: &Background, tileset_texture: &TextureHandle) {
    ui.vertical(|ui| {
        ui.label(format!(
            "Tileset (0x{:04X}, {}, {})",
            background.tileset_addr, background.bit_depth, background.tile_size
        ));
        ui.image(tileset_texture, Vec2::new(512.0, 512.0));
    });
}
