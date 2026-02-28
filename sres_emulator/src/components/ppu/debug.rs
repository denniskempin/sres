use super::Background;
use super::BackgroundId;
use super::BitDepth;
use super::Bpp2Decoder;
use super::Bpp4Decoder;
use super::Bpp8Decoder;
use super::Ppu;
use super::Tile;
use super::TileDecoder;
use crate::common::address::AddressU15;
use crate::common::image::Image;

pub struct PpuDebug<'a>(pub &'a Ppu);

impl PpuDebug<'_> {
    pub fn background_info(&self, background_id: BackgroundId) -> String {
        let background = self.0.state.backgrounds[background_id as usize];
        format!("Scroll ({}, {})", background.h_offset, background.v_offset)
    }

    pub fn sprite_info(&self, sprite_id: usize) -> String {
        self.0.state.oam.get_sprite(sprite_id as u32).to_string()
    }

    pub fn sprites(&self) -> Vec<super::oam::Sprite> {
        (0..128).map(|id| self.0.state.oam.get_sprite(id)).collect()
    }

    pub fn render_sprite<ImageT: Image>(&self, sprite_id: usize) -> ImageT {
        let sprite = self.0.state.oam.get_sprite(sprite_id as u32);
        let mut image = ImageT::new(sprite.width(), sprite.height());
        for coarse_y in 0..(sprite.coarse_height()) {
            for coarse_x in 0..(sprite.coarse_width()) {
                let tile_idx = sprite.tile + coarse_x + coarse_y * 16;
                let tile = Tile::<Bpp4Decoder>::from_tileset_index(
                    sprite.nametable,
                    tile_idx,
                    false,
                    false,
                );
                for (fine_y, row) in tile.rows(&self.0.state.vram) {
                    for (fine_x, pixel) in row.pixels() {
                        let color = self.0.state.cgram[sprite.palette_addr() + pixel];
                        image.set_pixel(
                            (coarse_x * 8 + fine_x, coarse_y * 8 + fine_y),
                            color.into(),
                        );
                    }
                }
            }
        }
        image
    }

    pub fn render_vram<ImageT: Image>(&self, selection: VramRenderSelection) -> ImageT {
        let (bit_depth, tileset_addr, palette_addr) = self.vram_selection_params(selection);
        let num_rows = Self::vram_num_rows(bit_depth, tileset_addr);
        let mut image = ImageT::new(16 * 8, num_rows * 8);
        match bit_depth {
            BitDepth::Bpp2 => self.debug_render_vram_impl::<Bpp2Decoder>(
                &mut image,
                num_rows,
                tileset_addr,
                palette_addr,
            ),
            BitDepth::Bpp4 => self.debug_render_vram_impl::<Bpp4Decoder>(
                &mut image,
                num_rows,
                tileset_addr,
                palette_addr,
            ),
            BitDepth::Bpp8 => self.debug_render_vram_impl::<Bpp8Decoder>(
                &mut image,
                num_rows,
                tileset_addr,
                palette_addr,
            ),
            _ => (),
        };
        image
    }

    pub fn vram_tile_info(&self, selection: VramRenderSelection, tile_idx: u32) -> String {
        let (bit_depth, tileset_addr, _) = self.vram_selection_params(selection);
        let words_per_tile = Self::words_per_tile(bit_depth);
        let tile_word_addr = tileset_addr.0 as u32 + tile_idx * words_per_tile;
        format!("Tile #{tile_idx}\nVRAM word addr: 0x{tile_word_addr:04X}")
    }

    fn vram_selection_params(&self, selection: VramRenderSelection) -> (BitDepth, AddressU15, u8) {
        match selection {
            VramRenderSelection::Background(id) => {
                let background = self.0.state.backgrounds[id as usize];
                (
                    background.bit_depth,
                    background.tileset_addr,
                    background.palette_addr,
                )
            }
            VramRenderSelection::Sprite0 => (BitDepth::Bpp4, self.0.state.oam.nametables.0, 128),
            VramRenderSelection::Sprite1 => (BitDepth::Bpp4, self.0.state.oam.nametables.1, 128),
        }
    }

    fn words_per_tile(bit_depth: BitDepth) -> u32 {
        match bit_depth {
            BitDepth::Bpp2 => 8,
            BitDepth::Bpp4 => 16,
            BitDepth::Bpp8 => 32,
            _ => 16,
        }
    }

    fn vram_num_rows(bit_depth: BitDepth, tileset_addr: AddressU15) -> u32 {
        const VRAM_SIZE: u32 = 0x8000;
        let words_per_tile = Self::words_per_tile(bit_depth);
        let remaining_words = VRAM_SIZE.saturating_sub(tileset_addr.0 as u32);
        let total_tiles = remaining_words / words_per_tile;
        (total_tiles + 15) / 16
    }

    fn debug_render_vram_impl<TileDecoderT: TileDecoder>(
        &self,
        image: &mut impl Image,
        num_rows: u32,
        tileset_addr: AddressU15,
        palette_addr: u8,
    ) {
        for coarse_x in 0..16 {
            for coarse_y in 0..num_rows {
                let tile_idx = coarse_y * 16 + coarse_x;
                let tile =
                    Tile::<TileDecoderT>::from_tileset_index(tileset_addr, tile_idx, false, false);
                for (row_idx, row) in tile.rows(&self.0.state.vram) {
                    for (col_idx, pixel) in row.pixels() {
                        let color = self.0.state.cgram[palette_addr + pixel];
                        image.set_pixel(
                            (coarse_x * 8 + col_idx, coarse_y * 8 + row_idx),
                            color.into(),
                        );
                    }
                }
            }
        }
    }

    pub fn render_background<ImageT: Image>(&self, background_id: BackgroundId) -> ImageT {
        let background = &self.0.state.backgrounds[background_id as usize];
        let mut image = ImageT::new(
            background.coarse_width() * 8,
            background.coarse_height() * 8,
        );
        match background.bit_depth {
            BitDepth::Bpp2 => self.render_background_impl::<Bpp2Decoder>(&mut image, background),
            BitDepth::Bpp4 => self.render_background_impl::<Bpp4Decoder>(&mut image, background),
            BitDepth::Bpp8 => self.render_background_impl::<Bpp8Decoder>(&mut image, background),
            _ => (),
        };
        image
    }

    fn render_background_impl<TileDecoderT: TileDecoder>(
        &self,
        image: &mut impl Image,
        background: &Background,
    ) {
        for coarse_y in 0..background.coarse_height() {
            for coarse_x in 0..background.coarse_width() {
                let tile =
                    background.get_tile::<TileDecoderT>(coarse_x, coarse_y, &self.0.state.vram);
                for (fine_y, row) in tile.rows(&self.0.state.vram) {
                    for (fine_x, pixel) in row.pixels() {
                        let color = self.0.state.cgram[background.palette_addr + pixel];
                        image.set_pixel(
                            (coarse_x * 8 + fine_x, coarse_y * 8 + fine_y),
                            color.into(),
                        );
                    }
                }
            }
        }
    }

    pub fn render_palette<ImageT: Image>(&self) -> ImageT {
        let mut image = ImageT::new(128, 128);
        for y in 0..16_u32 {
            for x in 0..16_u32 {
                let color = self.0.state.cgram.memory[(y * 16 + x) as usize];
                for fine_y in 0..8_u32 {
                    for fine_x in 0..8_u32 {
                        image.set_pixel((x * 8 + fine_x, y * 8 + fine_y), color.into());
                    }
                }
            }
        }
        image
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum VramRenderSelection {
    Background(BackgroundId),
    Sprite0,
    Sprite1,
}

impl std::fmt::Display for VramRenderSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VramRenderSelection::Background(id) => write!(f, "{id}"),
            VramRenderSelection::Sprite0 => write!(f, "Sprite0"),
            VramRenderSelection::Sprite1 => write!(f, "Sprite1"),
        }
    }
}
