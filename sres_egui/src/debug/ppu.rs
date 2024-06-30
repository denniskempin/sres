use eframe::CreationContext;
use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;
use egui::TextureOptions;
use egui::Ui;
use egui::Vec2;
use sres_emulator::components::ppu::Background;
use sres_emulator::components::ppu::BackgroundId;
use sres_emulator::components::ppu::BitDepth;
use sres_emulator::components::ppu::Ppu;
use sres_emulator::components::ppu::VramAddr;
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
                ppu_status_widget(ui, &emulator.cpu.bus.ppu);
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
                        self.background_widget.show(ui, &emulator.cpu.bus.ppu);
                    }
                    PpuDebugTabs::Sprites => self.sprites_widget.show(ui, &emulator.cpu.bus.ppu),
                    PpuDebugTabs::Vram => self.vram_widget.show(ui, &emulator.cpu.bus.ppu),
                    PpuDebugTabs::Palette => self.palette_widget.show(ui, &emulator.cpu.bus.ppu),
                }
            });
    }
}

pub fn ppu_status_widget(ui: &mut Ui, ppu: &Ppu) {
    ui.label(format!("V, H: ({}, {})", ppu.timer.v, ppu.timer.hdot()));
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

    pub fn update_textures(&mut self, ppu: &Ppu) {
        self.tilemap_texture.set(
            ppu.debug_render_background::<EguiImageImpl>(self.selected_bg),
            TextureOptions::default(),
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &Ppu) {
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
        let background = &ppu.backgrounds[self.selected_bg as usize];
        ui.label(format!(
            "Scroll: ({}, {})",
            background.h_offset, background.v_offset
        ));
        ui.horizontal(|ui| {
            tilemap_widget(ui, background, &self.tilemap_texture);
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
            "Tilemap ({}, {})",
            background.tilemap_addr, background.tilemap_size
        ));
        ui.image((tilemap_texture.id(), Vec2::new(512.0, 512.0)));
    });
}

struct PpuSpritesWidget {
    sprite_id: u32,
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

    pub fn update_textures(&mut self, ppu: &Ppu) {
        self.sprite_texture.set(
            ppu.debug_render_sprite::<EguiImageImpl>(self.sprite_id),
            TextureOptions::default(),
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &Ppu) {
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

        let sprite = ppu.oam.get_sprite(self.sprite_id);
        ui.horizontal(|ui| {
            ui.vertical(|ui| ui.label(format!("Position: ({}, {})", sprite.x, sprite.y)));
            ui.image((
                self.sprite_texture.id(),
                Vec2::new(sprite.width() as f32 * 4.0, sprite.height() as f32 * 4.0),
            ));
        });
    }
}

struct PpuVramWidget {
    addr: VramAddr,
    bit_depth: BitDepth,
    palette_addr: u8,
    vram_texture: TextureHandle,
}

impl PpuVramWidget {
    pub fn new(cc: &CreationContext) -> Self {
        PpuVramWidget {
            addr: VramAddr(0),
            bit_depth: BitDepth::Bpp2,
            palette_addr: 0,
            vram_texture: cc.egui_ctx.load_texture(
                "Vram",
                ColorImage::example(),
                Default::default(),
            ),
        }
    }

    pub fn update_textures(&mut self, ppu: &Ppu) {
        self.vram_texture.set(
            ppu.debug_render_vram::<EguiImageImpl>(
                self.addr,
                32,
                self.bit_depth,
                self.palette_addr,
            ),
            TextureOptions::default(),
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &Ppu) {
        self.update_textures(ppu);

        ui.horizontal(|ui| {
            for bgid in &[
                BackgroundId::BG1,
                BackgroundId::BG2,
                BackgroundId::BG3,
                BackgroundId::BG4,
            ] {
                if ui.button(bgid.to_string()).clicked() {
                    let bg = ppu.backgrounds[*bgid as usize];
                    self.bit_depth = bg.bit_depth;
                    self.addr = bg.tileset_addr;
                    self.palette_addr = bg.palette_addr;
                }
            }
            if ui.button("Sprites0").clicked() {
                self.bit_depth = BitDepth::Bpp4;
                self.addr = ppu.oam.nametables.0;
            }
            if ui.button("Sprites1").clicked() {
                self.bit_depth = BitDepth::Bpp4;
                self.addr = ppu.oam.nametables.1;
                self.palette_addr = 128;
            }
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Bit Depth: ".to_string());
                tabs_widget(
                    ui,
                    &[BitDepth::Bpp2, BitDepth::Bpp4, BitDepth::Bpp8],
                    &mut self.bit_depth,
                );
            });
            ui.horizontal(|ui| {
                ui.label("Addr: {}".to_string());
                if ui.button("-").clicked() {
                    self.addr = self.addr - 0x400_u16;
                }
                ui.label(format!("{}", self.addr));
                if ui.button("+").clicked() {
                    self.addr = self.addr + 0x400_u16;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Palette: {}".to_string());
                if ui.button("-").clicked() {
                    self.palette_addr = self.palette_addr.wrapping_sub(0x10);
                }
                ui.label(format!("{:02X}", self.palette_addr));
                if ui.button("+").clicked() {
                    self.palette_addr = self.palette_addr.wrapping_add(0x10);
                }
            });
            ui.image((self.vram_texture.id(), Vec2::new(512.0, 512.0)));
        });
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

    pub fn update_textures(&mut self, ppu: &Ppu) {
        self.palette_texture.set(
            ppu.cgram.debug_render_palette::<EguiImageImpl>(),
            TextureOptions::default(),
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &Ppu) {
        self.update_textures(ppu);

        ui.image((self.palette_texture.id(), Vec2::new(256.0, 256.0)));
    }
}
