use eframe::CreationContext;
use egui::ColorImage;
use egui::Context;
use egui::TextureHandle;
use egui::TextureOptions;
use egui::Ui;
use egui::Vec2;
use egui_extras::Column;
use egui_extras::TableBuilder;
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
    open: bool,
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

    pub fn toggle(&mut self) {
        self.open = !self.open;
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

#[cfg(test)]
mod tests {
    use sres_emulator::common::clock::ClockInfo;

    use super::*;

    /// All stateless PPU widgets in one combined snapshot.
    #[test]
    fn ppu_widgets() {
        crate::test_utils::widget_snapshot("ppu/ppu_widgets", |ui| {
            ui.vertical(|ui| {
                ui.label("── clock_info_widget ──");
                clock_info_widget(ui, ClockInfo::default()); // V=0, H=0
                clock_info_widget(ui, ClockInfo::from_master_clock(500_000)); // mid-frame

                ui.label("── tabs_widget ──");
                let mut first = "Alpha";
                tabs_widget(ui, &["Alpha", "Beta", "Gamma"], &mut first); // first selected
                let mut last = "Gamma";
                tabs_widget(ui, &["Alpha", "Beta", "Gamma"], &mut last); // last selected
            });
        });
    }
}

struct PpuSpritesWidget {
    sprite_thumbnails: Vec<TextureHandle>,
}

const SPRITE_THUMBNAIL_SIZE: f32 = 32.0;

impl PpuSpritesWidget {
    pub fn new(cc: &CreationContext) -> Self {
        let placeholder = ColorImage::example();
        let sprite_thumbnails = (0..128)
            .map(|i| {
                cc.egui_ctx.load_texture(
                    format!("SpriteThumbnail{i}"),
                    placeholder.clone(),
                    TextureOptions::NEAREST,
                )
            })
            .collect();
        PpuSpritesWidget { sprite_thumbnails }
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &PpuDebug<'_>) {
        let sprites = ppu.sprites();

        let row_height = SPRITE_THUMBNAIL_SIZE + 4.0;
        TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto()) // thumbnail
            .column(Column::auto()) // #
            .column(Column::auto()) // X
            .column(Column::auto()) // Y
            .column(Column::auto()) // Size
            .column(Column::auto()) // Tile
            .column(Column::auto()) // Pal
            .column(Column::auto()) // Pri
            .column(Column::auto()) // H
            .column(Column::remainder()) // V
            .header(20.0, |mut header| {
                for label in &["", "#", "X", "Y", "Size", "Tile", "Pal", "Pri", "H", "V"] {
                    header.col(|ui| {
                        ui.strong(*label);
                    });
                }
            })
            .body(|body| {
                // Use rows() for virtual scrolling: only visible rows call the closure.
                body.rows(row_height, sprites.len(), |mut row| {
                    let row_index = row.index();
                    let sprite = &sprites[row_index];
                    let sprite_id = sprite.id as usize;

                    // Update only this row's thumbnail (lazy, visible rows only).
                    self.sprite_thumbnails[sprite_id].set(
                        ppu.render_sprite::<EguiImageImpl>(sprite_id),
                        TextureOptions::NEAREST,
                    );

                    row.col(|ui| {
                        let tex = &self.sprite_thumbnails[sprite_id];
                        let tex_size = tex.size_vec2();
                        let max_dim = tex_size.x.max(tex_size.y).max(1.0);
                        let scale = SPRITE_THUMBNAIL_SIZE / max_dim;
                        let display_size = Vec2::new(tex_size.x * scale, tex_size.y * scale);
                        ui.add(egui::Image::new((tex.id(), display_size)));
                    });
                    row.col(|ui| {
                        ui.label(format!("{:3}", sprite.id));
                    });
                    row.col(|ui| {
                        ui.label(format!("{:3}", sprite.x));
                    });
                    row.col(|ui| {
                        ui.label(format!("{:3}", sprite.y));
                    });
                    row.col(|ui| {
                        ui.label(sprite.size.to_string());
                    });
                    row.col(|ui| {
                        ui.label(format!("${:02X}", sprite.tile));
                    });
                    row.col(|ui| {
                        ui.label(format!("{}", sprite.palette));
                    });
                    row.col(|ui| {
                        ui.label(format!("{}", sprite.priority));
                    });
                    row.col(|ui| {
                        ui.label(if sprite.hflip { "H" } else { "" });
                    });
                    row.col(|ui| {
                        ui.label(if sprite.vflip { "V" } else { "" });
                    });
                });
            });
    }
}

const VRAM_TILE_SCALE: f32 = 3.0;
const VRAM_TILES_PER_ROW: f32 = 16.0;

struct PpuVramWidget {
    selection: VramRenderSelection,
    vram_texture: TextureHandle,
}

impl PpuVramWidget {
    pub fn new(cc: &CreationContext) -> Self {
        PpuVramWidget {
            selection: VramRenderSelection::Background(BackgroundId::BG1),
            vram_texture: cc.egui_ctx.load_texture(
                "Vram",
                ColorImage::example(),
                TextureOptions::NEAREST,
            ),
        }
    }

    pub fn update_textures(&mut self, ppu: &PpuDebug<'_>) {
        self.vram_texture.set(
            ppu.render_vram::<EguiImageImpl>(self.selection),
            TextureOptions::NEAREST,
        );
    }

    pub fn show(&mut self, ui: &mut Ui, ppu: &PpuDebug<'_>) {
        self.update_textures(ppu);

        ui.horizontal(|ui| {
            for &sel in &[
                VramRenderSelection::Background(BackgroundId::BG1),
                VramRenderSelection::Background(BackgroundId::BG2),
                VramRenderSelection::Background(BackgroundId::BG3),
                VramRenderSelection::Background(BackgroundId::BG4),
                VramRenderSelection::Sprite0,
                VramRenderSelection::Sprite1,
            ] {
                if ui
                    .selectable_label(self.selection == sel, sel.to_string())
                    .clicked()
                {
                    self.selection = sel;
                }
            }
        });

        let tex_size = self.vram_texture.size_vec2();
        let display_size = tex_size * VRAM_TILE_SCALE;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let response = ui.image((self.vram_texture.id(), display_size));

                if response.hovered() {
                    if let Some(pos) = ui.input(|i| i.pointer.hover_pos()) {
                        let rel = pos - response.rect.min;
                        let tile_px = 8.0 * VRAM_TILE_SCALE;
                        if rel.x >= 0.0
                            && rel.y >= 0.0
                            && rel.x < display_size.x
                            && rel.y < display_size.y
                        {
                            let tile_x = (rel.x / tile_px) as u32;
                            let tile_y = (rel.y / tile_px) as u32;
                            let tile_idx =
                                tile_y * VRAM_TILES_PER_ROW as u32 + tile_x;
                            let info = ppu.vram_tile_info(self.selection, tile_idx);
                            egui::show_tooltip_at_pointer(
                                ui.ctx(),
                                ui.layer_id(),
                                egui::Id::new("vram_tile_tooltip"),
                                |ui| {
                                    ui.label(info);
                                },
                            );
                        }
                    }
                }
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
