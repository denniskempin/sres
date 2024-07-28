use eframe::CreationContext;
use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;
use egui::TextureOptions;
use egui::Ui;
use egui::Vec2;
use sres_emulator::common::clock::ClockInfo;
use sres_emulator::components::ppu::BackgroundId;
use sres_emulator::components::ppu::PpuDebug;
use sres_emulator::components::ppu::VramRenderSelection;
use sres_emulator::System;

use crate::util::EguiImageImpl;

#[derive(PartialEq, Copy, Clone, strum::Display)]
enum PpuDebugTabs {
    Background,
    Sprites,
    Vram,
    Palette,
}

pub struct PpuDebugWindow {
    pub open: bool,
    selected_tab: PpuDebugTabs,
    background_widget: PpuBackgroundWidget,
    sprites_widget: PpuSpritesWidget,
    vram_widget: PpuVramWidget,
    palette_widget: PpuPaletteWidget,
}

impl PpuDebugWindow {
    pub fn new(cc: &CreationContext) -> Self {
        PpuDebugWindow {
            open: false,
            selected_tab: PpuDebugTabs::Background,
            background_widget: PpuBackgroundWidget::new(cc),
            sprites_widget: PpuSpritesWidget::new(cc),
            vram_widget: PpuVramWidget::new(cc),
            palette_widget: PpuPaletteWidget::new(cc),
        }
    }

    pub fn show(&mut self, ctx: &Context, emulator: &System) {
        egui::Window::new("PPU")
            .open(&mut self.open)
            .show(ctx, |ui| {
                clock_info_widget(ui, emulator.clock_info());
                tabs_widget(
                    ui,
                    &[
                        PpuDebugTabs::Background,
                        PpuDebugTabs::Sprites,
                        PpuDebugTabs::Vram,
                        PpuDebugTabs::Palette,
                    ],
                    &mut self.selected_tab,
                );
                ui.separator();
                match self.selected_tab {
                    PpuDebugTabs::Background => {
                        self.background_widget.show(ui, &emulator.debug().ppu());
                    }
                    PpuDebugTabs::Sprites => self.sprites_widget.show(ui, &emulator.debug().ppu()),
                    PpuDebugTabs::Vram => self.vram_widget.show(ui, &emulator.debug().ppu()),
                    PpuDebugTabs::Palette => self.palette_widget.show(ui, &emulator.debug().ppu()),
                }
            });
    }
}

pub fn clock_info_widget(ui: &mut Ui, clock_info: ClockInfo) {
    ui.label(format!("V, H: ({}, {})", clock_info.v, clock_info.hdot()));
}

struct PpuBackgroundWidget {
    selected_bg: BackgroundId,
    tilemap_texture: TextureHandle,
}

impl PpuBackgroundWidget {
    pub fn new(cc: &CreationContext) -> Self {
        PpuBackgroundWidget {
            selected_bg: BackgroundId::BG1,
            tilemap_texture: cc.egui_ctx.load_texture(
                "Tilemap",
                ColorImage::example(),
                Default::default(),
            ),
        }
    }

    pub fn update_textures(&mut self, ppu: &PpuDebug<'_>) {
        self.tilemap_texture.set(
            ppu.render_background::<EguiImageImpl>(self.selected_bg),
            TextureOptions::default(),
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &PpuDebug<'_>) {
        self.update_textures(ppu);

        tabs_widget(
            ui,
            &[
                BackgroundId::BG1,
                BackgroundId::BG2,
                BackgroundId::BG3,
                BackgroundId::BG4,
            ],
            &mut self.selected_bg,
        );
        ui.label(ppu.background_info(self.selected_bg));
        ui.horizontal(|ui| {
            tilemap_widget(ui, &self.tilemap_texture);
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

fn tilemap_widget(ui: &mut Ui, tilemap_texture: &TextureHandle) {
    ui.image((tilemap_texture.id(), Vec2::new(512.0, 512.0)));
}

struct PpuSpritesWidget {
    sprite_id: usize,
    sprite_texture: TextureHandle,
}

impl PpuSpritesWidget {
    pub fn new(cc: &CreationContext) -> Self {
        PpuSpritesWidget {
            sprite_id: 0,
            sprite_texture: cc.egui_ctx.load_texture(
                "Sprite",
                ColorImage::example(),
                Default::default(),
            ),
        }
    }

    pub fn update_textures(&mut self, ppu: &PpuDebug<'_>) {
        self.sprite_texture.set(
            ppu.render_sprite::<EguiImageImpl>(self.sprite_id),
            TextureOptions::default(),
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &PpuDebug<'_>) {
        self.update_textures(ppu);

        ui.horizontal(|ui| {
            ui.label("Sprite:".to_string());
            if ui.button("-").clicked() && self.sprite_id > 0 {
                self.sprite_id -= 1;
            }
            ui.label(format!("{}", self.sprite_id));
            if ui.button("+").clicked() && self.sprite_id < 255 {
                self.sprite_id += 1;
            }
        });

        ui.horizontal(|ui| {
            ui.label(ppu.sprite_info(self.sprite_id));
            ui.image((
                self.sprite_texture.id(),
                Vec2::new(
                    self.sprite_texture.size_vec2().x * 4.0,
                    self.sprite_texture.size_vec2().y * 4.0,
                ),
            ));
        });
    }
}

struct PpuVramWidget {
    offset: i32,
    selection: VramRenderSelection,
    vram_texture: TextureHandle,
}

impl PpuVramWidget {
    pub fn new(cc: &CreationContext) -> Self {
        PpuVramWidget {
            offset: 0,
            selection: VramRenderSelection::Background(BackgroundId::BG1),
            vram_texture: cc.egui_ctx.load_texture(
                "Vram",
                ColorImage::example(),
                Default::default(),
            ),
        }
    }

    pub fn update_textures(&mut self, ppu: &PpuDebug<'_>) {
        self.vram_texture.set(
            ppu.render_vram::<EguiImageImpl>(32, self.offset, self.selection),
            TextureOptions::default(),
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &PpuDebug<'_>) {
        self.update_textures(ppu);

        ui.horizontal(|ui| {
            for selection in &[
                VramRenderSelection::Background(BackgroundId::BG1),
                VramRenderSelection::Background(BackgroundId::BG2),
                VramRenderSelection::Background(BackgroundId::BG3),
                VramRenderSelection::Background(BackgroundId::BG4),
                VramRenderSelection::Sprite0,
                VramRenderSelection::Sprite1,
            ] {
                if ui.button(selection.to_string()).clicked() {
                    self.selection = *selection;
                    self.offset = 0;
                }
            }
        });
        ui.image((self.vram_texture.id(), Vec2::new(512.0, 512.0)));
    }
}

struct PpuPaletteWidget {
    palette_texture: TextureHandle,
}

impl PpuPaletteWidget {
    pub fn new(cc: &CreationContext) -> Self {
        PpuPaletteWidget {
            palette_texture: cc.egui_ctx.load_texture(
                "Palette",
                ColorImage::example(),
                Default::default(),
            ),
        }
    }

    pub fn update_textures(&mut self, ppu: &PpuDebug<'_>) {
        self.palette_texture.set(
            ppu.render_palette::<EguiImageImpl>(),
            TextureOptions::default(),
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &PpuDebug<'_>) {
        self.update_textures(ppu);

        ui.image((self.palette_texture.id(), Vec2::new(256.0, 256.0)));
    }
}
