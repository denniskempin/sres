use std::fmt::Display;
use std::fmt::Formatter;

use eframe::CreationContext;
use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;
use egui::TextureOptions;
use egui::Ui;
use egui::Vec2;
use sres_emulator::ppu::Background;
use sres_emulator::ppu::BackgroundId;
use sres_emulator::ppu::BitDepth;
use sres_emulator::ppu::Ppu;
use sres_emulator::ppu::VramAddr;
use sres_emulator::System;

use crate::util::EguiImageImpl;

#[derive(PartialEq, Copy, Clone)]

enum PpuDebugTabs {
    Background,
    Sprites,
    Vram,
}

impl Display for PpuDebugTabs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PpuDebugTabs::Background => write!(f, "Background"),
            PpuDebugTabs::Sprites => write!(f, "Sprites"),
            PpuDebugTabs::Vram => write!(f, "Vram"),
        }
    }
}

pub struct PpuDebugWindow {
    pub open: bool,
    selected_tab: PpuDebugTabs,
    background_widget: PpuBackgroundWidget,
    vram_widget: PpuVramWidget,
}

impl PpuDebugWindow {
    pub fn new(cc: &CreationContext) -> Self {
        PpuDebugWindow {
            open: false,
            selected_tab: PpuDebugTabs::Background,
            background_widget: PpuBackgroundWidget::new(cc),
            vram_widget: PpuVramWidget::new(cc),
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
                    ],
                    &mut self.selected_tab,
                );
                ui.separator();
                match self.selected_tab {
                    PpuDebugTabs::Background => {
                        self.background_widget.show(ui, &emulator.cpu.bus.ppu);
                    }
                    PpuDebugTabs::Sprites => {
                        ui.label("Sprites");
                        for i in 0..16 {
                            ui.label(emulator.cpu.bus.ppu.oam.get_sprite(i).to_string());
                        }
                    }
                    PpuDebugTabs::Vram => self.vram_widget.show(ui, &emulator.cpu.bus.ppu),
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
        ui.image(tilemap_texture, Vec2::new(512.0, 512.0));
    });
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
                BackgroundId::BG0,
                BackgroundId::BG1,
                BackgroundId::BG2,
                BackgroundId::BG3,
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
            ui.image(&mut self.vram_texture, Vec2::new(512.0, 512.0));
        });
    }
}
