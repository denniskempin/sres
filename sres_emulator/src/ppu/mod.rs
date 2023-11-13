mod cgram;
pub mod oam;
mod timer;
mod vram;

use std::fmt::Display;
use std::fmt::Formatter;
use std::marker::PhantomData;

use intbits::Bits;
use serde::Deserialize;
use serde::Serialize;

use self::cgram::CgRam;
use self::oam::Oam;
pub use self::timer::fvh_to_master_clock;
use self::timer::PpuTimer;
use self::vram::Vram;
pub use self::vram::VramAddr;
use crate::debugger::DebuggerRef;
use crate::util::image::Image;
use crate::util::image::Rgb15;
use crate::util::memory::Address;
use crate::util::uint::U16Ext;
use crate::util::uint::U8Ext;

pub struct Ppu {
    pub disabled: bool,
    pub timer: PpuTimer,
    pub vram: Vram,
    pub bgmode: BgMode,
    pub bg3_priority: bool,
    pub backgrounds: [Background; 4],

    pub framebuffer: Framebuffer,
    pub cgram: CgRam,
    pub oam: Oam,
    pub last_drawn_scanline: u64,

    pub bgofs_latch: u8,
    pub bghofs_latch: u8,

    pub debugger: DebuggerRef,
}

impl Ppu {
    #[allow(clippy::new_without_default)]
    pub fn new(debugger: DebuggerRef) -> Self {
        Self {
            disabled: false,
            timer: PpuTimer::default(),
            vram: Vram::new(),
            bgmode: BgMode::Mode0,
            bg3_priority: false,
            backgrounds: [Background::default(); 4],
            framebuffer: Framebuffer::default(),
            cgram: CgRam::new(),
            oam: Oam::new(),
            last_drawn_scanline: 0,
            bgofs_latch: 0,
            bghofs_latch: 0,
            debugger,
        }
    }

    pub fn bus_read(&mut self, addr: Address) -> u8 {
        match addr.offset {
            0x2138 => self.oam.read_oamdataread(),
            0x2139 => self.vram.read_vmdatalread(),
            0x213A => self.vram.read_vmdatahread(),
            0x213B => self.cgram.read_cgdataread(),
            _ => 0,
        }
    }

    pub fn bus_peek(&self, addr: Address) -> Option<u8> {
        match addr.offset {
            0x2138 => Some(self.oam.peek_oamdataread()),
            0x2139 => Some(self.vram.peek_vmdatalread()),
            0x213A => Some(self.vram.peek_vmdatahread()),
            0x213B => Some(self.cgram.peek_cgdataread()),
            _ => None,
        }
    }

    pub fn bus_write(&mut self, addr: Address, value: u8) {
        match addr.offset {
            0x2100 => self.write_inidisp(value),
            0x2101 => self.oam.write_objsel(value),
            0x2102 => self.oam.write_oamaddl(value),
            0x2103 => self.oam.write_oamaddh(value),
            0x2104 => self.oam.write_oamdata(value),
            0x2105 => self.write_bgmode(value),
            0x2107..=0x210A => self.write_bgnsc(addr, value),
            0x210B => self.write_bg12nba(value),
            0x210C => self.write_bg34nba(value),
            0x210D | 0x210F | 0x2111 | 0x2113 => self.write_bgnhofs(addr, value),
            0x210E | 0x2110 | 0x2112 | 0x2114 => self.write_bgnvofs(addr, value),
            0x2115 => self.vram.write_vmain(value),
            0x2116 => self.vram.write_vmaddl(value),
            0x2117 => self.vram.write_vmaddh(value),
            0x2118 => self.vram.write_vmdatal(value),
            0x2119 => self.vram.write_vmdatah(value),
            0x2121 => self.cgram.write_cgadd(value),
            0x2122 => self.cgram.write_cgdata(value),
            0x212C => self.write_tm(value),
            _ => (),
        }
    }

    pub fn advance_master_clock(&mut self, cycles: u64) {
        self.timer.advance_master_clock(cycles);
        if self.disabled {
            return;
        }
        if self.timer.v != self.last_drawn_scanline {
            self.draw_scanline(self.timer.v as u32);
            self.last_drawn_scanline = self.timer.v;
        }
    }

    pub fn reset(&mut self) {
        self.last_drawn_scanline = 0;
        self.timer = PpuTimer::default();
    }

    pub fn get_rgba_framebuffer<ImageT: Image>(&self) -> ImageT {
        let mut image = ImageT::new(256, 224);
        for (x, y, pixel) in self.framebuffer.iter() {
            image.set_pixel((x, y), (*pixel).into());
        }
        image
    }

    pub fn draw_scanline(&mut self, scanline: u32) {
        if scanline >= 224 {
            return;
        }

        let default_color = self.cgram[0];
        for x in 0..256 {
            self.framebuffer[(x, scanline)] = default_color;
        }

        match self.bgmode {
            BgMode::Mode0 => {
                self.draw_scanline_bgmode0(scanline);
            }
            BgMode::Mode1 => {
                if self.bg3_priority {
                    self.draw_scanline_bgmode1::<true>(scanline)
                } else {
                    self.draw_scanline_bgmode1::<false>(scanline)
                }
            }
            BgMode::Mode2 => {
                self.draw_scanline_bgmode2(scanline);
            }
            BgMode::Mode3 => {
                self.draw_scanline_bgmode3(scanline);
            }
            BgMode::Mode4 => {
                self.draw_scanline_bgmode4(scanline);
            }
            BgMode::Mode5 => {
                self.draw_scanline_bgmode5(scanline);
            }
            BgMode::Mode6 => {
                self.draw_scanline_bgmode6(scanline);
            }
            _ => panic!("Unsupported BG mode: {}", self.bgmode),
        }
    }

    pub fn draw_scanline_bgmode0(&mut self, scanline: u32) {
        self.draw_background_scanline::<Bpp2Decoder, false>(scanline, 3);
        self.draw_background_scanline::<Bpp2Decoder, false>(scanline, 2);
        self.draw_sprite_layer_scanline(scanline, 0);

        self.draw_background_scanline::<Bpp2Decoder, true>(scanline, 3);
        self.draw_background_scanline::<Bpp2Decoder, true>(scanline, 2);
        self.draw_sprite_layer_scanline(scanline, 1);

        self.draw_background_scanline::<Bpp2Decoder, false>(scanline, 1);
        self.draw_background_scanline::<Bpp2Decoder, false>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 2);

        self.draw_background_scanline::<Bpp2Decoder, true>(scanline, 1);
        self.draw_background_scanline::<Bpp2Decoder, true>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 3);
    }

    pub fn draw_scanline_bgmode1<const BG3_PRIORITY: bool>(&mut self, scanline: u32) {
        self.draw_background_scanline::<Bpp2Decoder, false>(scanline, 2);
        self.draw_sprite_layer_scanline(scanline, 0);

        if !BG3_PRIORITY {
            self.draw_background_scanline::<Bpp2Decoder, true>(scanline, 2);
        }
        self.draw_sprite_layer_scanline(scanline, 1);

        self.draw_background_scanline::<Bpp4Decoder, false>(scanline, 1);
        self.draw_background_scanline::<Bpp4Decoder, false>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 2);

        self.draw_background_scanline::<Bpp4Decoder, true>(scanline, 1);
        self.draw_background_scanline::<Bpp4Decoder, true>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 3);

        if BG3_PRIORITY {
            self.draw_background_scanline::<Bpp2Decoder, true>(scanline, 2);
        }
    }

    pub fn draw_scanline_bgmode2(&mut self, scanline: u32) {
        self.draw_background_scanline::<Bpp4Decoder, false>(scanline, 1);
        self.draw_sprite_layer_scanline(scanline, 0);

        self.draw_background_scanline::<Bpp4Decoder, false>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 1);

        self.draw_background_scanline::<Bpp4Decoder, true>(scanline, 1);
        self.draw_sprite_layer_scanline(scanline, 2);

        self.draw_background_scanline::<Bpp4Decoder, true>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 3);
    }

    pub fn draw_scanline_bgmode3(&mut self, scanline: u32) {
        self.draw_background_scanline::<Bpp4Decoder, false>(scanline, 1);
        self.draw_sprite_layer_scanline(scanline, 0);

        self.draw_background_scanline::<Bpp8Decoder, false>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 1);

        self.draw_background_scanline::<Bpp4Decoder, true>(scanline, 1);
        self.draw_sprite_layer_scanline(scanline, 2);

        self.draw_background_scanline::<Bpp8Decoder, true>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 3);
    }

    pub fn draw_scanline_bgmode4(&mut self, scanline: u32) {
        self.draw_background_scanline::<Bpp2Decoder, false>(scanline, 1);
        self.draw_sprite_layer_scanline(scanline, 0);

        self.draw_background_scanline::<Bpp8Decoder, false>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 1);

        self.draw_background_scanline::<Bpp2Decoder, true>(scanline, 1);
        self.draw_sprite_layer_scanline(scanline, 2);

        self.draw_background_scanline::<Bpp8Decoder, true>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 3);
    }

    pub fn draw_scanline_bgmode5(&mut self, scanline: u32) {
        self.draw_background_scanline::<Bpp2Decoder, false>(scanline, 1);
        self.draw_sprite_layer_scanline(scanline, 0);

        self.draw_background_scanline::<Bpp4Decoder, false>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 1);

        self.draw_background_scanline::<Bpp2Decoder, true>(scanline, 1);
        self.draw_sprite_layer_scanline(scanline, 2);

        self.draw_background_scanline::<Bpp4Decoder, true>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 3);
    }

    pub fn draw_scanline_bgmode6(&mut self, scanline: u32) {
        self.draw_sprite_layer_scanline(scanline, 0);

        self.draw_background_scanline::<Bpp4Decoder, false>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 1);

        self.draw_sprite_layer_scanline(scanline, 2);

        self.draw_background_scanline::<Bpp4Decoder, true>(scanline, 0);
        self.draw_sprite_layer_scanline(scanline, 3);
    }

    fn draw_sprite_layer_scanline(&mut self, scanline: u32, priority: u32) {
        let sprites = self.oam.get_sprites_on_scanline(scanline, priority);
        for (sprite, row) in sprites {
            let row_coarse = row / 8;
            let row_fine = row % 8;
            if sprite.x >= 256 {
                continue;
            }
            for coarse_x in 0..sprite.coarse_width() {
                let tile_idx = sprite.tile + coarse_x + row_coarse * 16;
                let tile = Tile::<Bpp4Decoder>::from_tileset_index(sprite.nametable, tile_idx);
                for (fine_x, pixel) in tile.row(row_fine, &self.vram).pixels() {
                    if pixel > 0 {
                        let color = self.cgram[sprite.palette_addr() + pixel];
                        self.framebuffer[(sprite.x + coarse_x * 8 + fine_x, scanline)] = color;
                    }
                }
            }
        }
    }

    fn draw_background_scanline<TileDecoderT: TileDecoder, const PRIORITY: bool>(
        &mut self,
        scanline: u32,
        background_id: usize,
    ) {
        let bg = self.backgrounds[background_id];
        if bg.bit_depth == BitDepth::Disabled || !bg.enabled {
            return;
        }

        let coarse_y = scanline / 8;
        let fine_y = scanline % 8;

        for coarse_x in 0..32 {
            let tile_x = coarse_x + bg.h_offset / 8;
            let tile_y = coarse_y + bg.v_offset / 8;
            let tile = bg.get_tile::<TileDecoderT>(tile_x, tile_y, &self.vram);
            if tile.priority != PRIORITY {
                continue;
            }
            for (fine_x, pixel) in tile.row(fine_y, &self.vram).pixels() {
                if pixel > 0 {
                    let color = self.cgram[bg.palette_addr + pixel];
                    self.framebuffer[(coarse_x * 8 + fine_x, scanline)] = color;
                }
            }
        }
    }

    /// Register 2100: INIDISP
    /// 7  bit  0
    /// ---- ----
    /// F... BBBB
    /// |    ||||
    /// |    ++++- Screen brightness (linear steps from 0 = none to $F = full)
    /// +--------- Force blanking
    fn write_inidisp(&mut self, value: u8) {
        log::info!("INIDISP = {:08b}", value);
        self.disabled = value.bit(7);
    }

    /// Register 2105: BGMODE
    /// 7  bit  0
    /// ---- ----
    /// 4321 PMMM
    /// |||| ||||
    /// |||| |+++- BG mode
    /// |||| +---- Mode 1 BG3 priority (0 = normal, 1 = high)
    /// |||+------ BG1 character size (0 = 8x8, 1 = 16x16)
    /// ||+------- BG2 character size (0 = 8x8, 1 = 16x16)
    /// |+-------- BG3 character size (0 = 8x8, 1 = 16x16)
    /// +--------- BG4 character size (0 = 8x8, 1 = 16x16)
    fn write_bgmode(&mut self, value: u8) {
        self.bgmode = match value.bits(0..=2) {
            0 => BgMode::Mode0,
            1 => BgMode::Mode1,
            2 => BgMode::Mode2,
            3 => BgMode::Mode3,
            4 => BgMode::Mode4,
            5 => BgMode::Mode5,
            6 => BgMode::Mode6,
            7 => BgMode::Mode7,
            _ => unreachable!(),
        };
        self.bg3_priority = value.bit(3);

        use BitDepth::*;
        let bit_depths = match self.bgmode {
            BgMode::Mode0 => (Bpp2, Bpp2, Bpp2, Bpp2),
            BgMode::Mode1 => (Bpp4, Bpp4, Bpp2, Disabled),
            BgMode::Mode2 => (Bpp4, Bpp4, Opt, Disabled),
            BgMode::Mode3 => (Bpp8, Bpp4, Disabled, Disabled),
            BgMode::Mode4 => (Bpp8, Bpp2, Opt, Disabled),
            BgMode::Mode5 => (Bpp4, Bpp2, Disabled, Disabled),
            BgMode::Mode6 => (Bpp4, Disabled, Opt, Disabled),
            BgMode::Mode7 => (Bpp8, Disabled, Disabled, Disabled),
        };
        self.backgrounds[0].bit_depth = bit_depths.0;
        self.backgrounds[1].bit_depth = bit_depths.1;
        self.backgrounds[2].bit_depth = bit_depths.2;
        self.backgrounds[3].bit_depth = bit_depths.3;

        let palette_addr = match self.bgmode {
            BgMode::Mode0 => (0, 32, 64, 96),
            BgMode::Mode1 => (0, 32, 0, 0),
            BgMode::Mode2 => (0, 32, 0, 0),
            BgMode::Mode3 => (0, 0, 0, 0),
            BgMode::Mode4 => (0, 0, 0, 0),
            BgMode::Mode5 => (0, 0, 0, 0),
            BgMode::Mode6 => (0, 0, 0, 0),
            BgMode::Mode7 => (0, 0, 0, 0),
        };
        self.backgrounds[0].palette_addr = palette_addr.0;
        self.backgrounds[1].palette_addr = palette_addr.1;
        self.backgrounds[2].palette_addr = palette_addr.2;
        self.backgrounds[3].palette_addr = palette_addr.3;

        fn to_tile_size(value: bool) -> TileSize {
            if value {
                TileSize::Size16x16
            } else {
                TileSize::Size8x8
            }
        }
        for i in 0..4 {
            self.backgrounds[i].tile_size = to_tile_size(value.bit(4 + i));
        }
    }

    /// Register 2107..210A: BGNSC - BG1..BG4 tilemap base address
    /// 7  bit  0
    /// ---- ----
    /// AAAA AAYX
    /// |||| ||||
    /// |||| |||+- Horizontal tilemap count (0 = 1 tilemap, 1 = 2 tilemaps)
    /// |||| ||+-- Vertical tilemap count (0 = 1 tilemap, 1 = 2 tilemaps)
    /// ++++-++--- Tilemap VRAM address (word address = AAAAAA << 10)
    fn write_bgnsc(&mut self, addr: Address, value: u8) {
        let bg_id = (addr.offset - 0x2107) as usize;
        self.backgrounds[bg_id].tilemap_addr = VramAddr((value.bits(2..=7) as u16) << 10);
        self.backgrounds[bg_id].tilemap_size = match value.bits(0..=1) {
            0 => TilemapSize::Size32x32,
            1 => TilemapSize::Size64x32,
            2 => TilemapSize::Size32x64,
            3 => TilemapSize::Size64x64,
            _ => unreachable!(),
        }
    }

    /// Register 210B: BG12NBA - Tileset base address for BG1 and BG2
    /// 7  bit  0
    /// ---- ----
    /// BBBB AAAA
    /// |||| ||||
    /// |||| ++++- BG1 CHR word base address (word address = AAAA << 12)
    /// ++++------ BG2 CHR word base address (word address = BBBB << 12)
    fn write_bg12nba(&mut self, value: u8) {
        self.backgrounds[0].tileset_addr = VramAddr((value.low_nibble() as u16) << 12);
        self.backgrounds[1].tileset_addr = VramAddr((value.high_nibble() as u16) << 12);
    }

    /// Register 210C: BG34NBA - Tileset base address for BG3 and BG4
    /// 7  bit  0
    /// ---- ----
    /// DDDD CCCC
    /// |||| ||||
    /// |||| ++++- BG3 CHR word base address (word address = CCCC << 12)
    /// ++++------ BG4 CHR word base address (word address = DDDD << 12)
    fn write_bg34nba(&mut self, value: u8) {
        self.backgrounds[2].tileset_addr = VramAddr((value.low_nibble() as u16) << 12);
        self.backgrounds[3].tileset_addr = VramAddr((value.high_nibble() as u16) << 12);
    }

    /// Register 210D, 210F, 2111, 2113: BGNHOFS - Background N horizontal scroll
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  .... ..XX   XXXX XXXX
    ///         ||   |||| ||||
    ///         ++---++++-++++- BGn horizontal scroll
    ///
    /// On write: BGnHOFS = (value << 8) | (bgofs_latch & ~7) | (bghofs_latch & 7)
    ///           bgofs_latch = value
    ///           bghofs_latch = value
    pub fn write_bgnhofs(&mut self, addr: Address, value: u8) {
        let bg_id = ((addr.offset - 0x210D) / 2) as usize;
        self.backgrounds[bg_id].h_offset = ((value as u32) << 8)
            | ((self.bgofs_latch as u32) & !7)
            | ((self.bghofs_latch as u32) & 7);
        self.bgofs_latch = value;
        self.bghofs_latch = value;
    }

    /// Register 210E, 2110, 2112, 2114: BGNVOFS - Background N vertical scroll
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  .... ..YY   YYYY YYYY
    ///         ||   |||| ||||
    ///         ++---++++-++++- BGn vertical scroll
    ///
    /// On write: BGnVOFS = (value << 8) | bgofs_latch
    ///           bgofs_latch = value
    ///
    /// Note: BG1VOFS uses the same address as M7VOFS
    pub fn write_bgnvofs(&mut self, addr: Address, value: u8) {
        let bg_id = ((addr.offset - 0x210E) / 2) as usize;
        self.backgrounds[bg_id].v_offset = ((value as u32) << 8) | (self.bgofs_latch as u32);
        self.bgofs_latch = value;
    }

    /// Register 212C: TM - Main screen layer enable
    /// 7  bit  0
    /// ---- ----
    /// ...O 4321
    ///    | ||||
    ///    | |||+- Enable BG1 on main screen
    ///    | ||+-- Enable BG2 on main screen
    ///    | |+--- Enable BG3 on main screen
    ///    | +---- Enable BG4 on main screen
    ///    +------ Enable OBJ on main screen
    pub fn write_tm(&mut self, value: u8) {
        for i in 0..4 {
            self.backgrounds[i].enabled = value.bit(i);
        }
    }

    // Debug APIs

    pub fn debug_render_tileset<ImageT: Image>(&self, background_id: BackgroundId) -> ImageT {
        let mut image = ImageT::new(16 * 8, 16 * 8);
        let background = &self.backgrounds[background_id as usize];
        match background.bit_depth {
            BitDepth::Bpp2 => self.debug_render_tileset_impl::<Bpp2Decoder>(&mut image, background),
            BitDepth::Bpp4 => self.debug_render_tileset_impl::<Bpp4Decoder>(&mut image, background),
            BitDepth::Bpp8 => self.debug_render_tileset_impl::<Bpp8Decoder>(&mut image, background),
            _ => (),
        };
        image
    }

    fn debug_render_tileset_impl<TileDecoderT: TileDecoder>(
        &self,
        image: &mut impl Image,
        background: &Background,
    ) {
        for tile_idx in 0..256 {
            let tile_x: u32 = (tile_idx % 16) * 8;
            let tile_y: u32 = (tile_idx / 16) * 8;
            let tile = background.get_tileset_tile::<TileDecoderT>(tile_idx);
            for (row_idx, row) in tile.rows(&self.vram) {
                for (col_idx, pixel) in row.pixels() {
                    let color = self.cgram[background.palette_addr + pixel];
                    image.set_pixel((tile_x + col_idx, tile_y + row_idx), color.into());
                }
            }
        }
    }

    pub fn debug_render_vram<ImageT: Image>(
        &self,
        addr: VramAddr,
        num_rows: u32,
        bit_depth: BitDepth,
        palette_addr: u8,
    ) -> ImageT {
        let mut image = ImageT::new(16 * 8, num_rows * 8);
        match bit_depth {
            BitDepth::Bpp2 => {
                self.debug_render_vram_impl::<Bpp2Decoder>(&mut image, addr, num_rows, palette_addr)
            }
            BitDepth::Bpp4 => {
                self.debug_render_vram_impl::<Bpp4Decoder>(&mut image, addr, num_rows, palette_addr)
            }
            BitDepth::Bpp8 => {
                self.debug_render_vram_impl::<Bpp8Decoder>(&mut image, addr, num_rows, palette_addr)
            }
            _ => (),
        };
        image
    }

    fn debug_render_vram_impl<TileDecoderT: TileDecoder>(
        &self,
        image: &mut impl Image,
        addr: VramAddr,
        num_rows: u32,
        palette_addr: u8,
    ) {
        for coarse_x in 0..16 {
            for coarse_y in 0..num_rows {
                let tile_idx = coarse_y * 16 + coarse_x;
                let tile = Tile::<TileDecoderT>::from_tileset_index(addr, tile_idx);
                for (row_idx, row) in tile.rows(&self.vram) {
                    for (col_idx, pixel) in row.pixels() {
                        let color = self.cgram[palette_addr + pixel];
                        image.set_pixel(
                            (coarse_x * 8 + col_idx, coarse_y * 8 + row_idx),
                            color.into(),
                        );
                    }
                }
            }
        }
    }

    pub fn debug_render_background<ImageT: Image>(&self, background_id: BackgroundId) -> ImageT {
        let background = &self.backgrounds[background_id as usize];
        let mut image = ImageT::new(
            background.coarse_width() * 8,
            background.coarse_height() * 8,
        );
        match background.bit_depth {
            BitDepth::Bpp2 => {
                self.debug_render_background_impl::<Bpp2Decoder>(&mut image, background)
            }
            BitDepth::Bpp4 => {
                self.debug_render_background_impl::<Bpp4Decoder>(&mut image, background)
            }
            BitDepth::Bpp8 => {
                self.debug_render_background_impl::<Bpp8Decoder>(&mut image, background)
            }
            _ => (),
        };
        image
    }

    fn debug_render_background_impl<TileDecoderT: TileDecoder>(
        &self,
        image: &mut impl Image,
        background: &Background,
    ) {
        for coarse_y in 0..background.coarse_height() {
            for coarse_x in 0..background.coarse_width() {
                let tile = background.get_tile::<TileDecoderT>(coarse_x, coarse_y, &self.vram);
                for (fine_y, row) in tile.rows(&self.vram) {
                    for (fine_x, pixel) in row.pixels() {
                        let color = self.cgram[background.palette_addr + pixel];
                        image.set_pixel(
                            (coarse_x * 8 + fine_x, coarse_y * 8 + fine_y),
                            color.into(),
                        );
                    }
                }
            }
        }
    }

    pub fn debug_render_sprite<ImageT: Image>(&self, sprite_id: u32) -> ImageT {
        let sprite = self.oam.get_sprite(sprite_id);
        let mut image = ImageT::new(sprite.width(), sprite.height());
        for coarse_y in 0..(sprite.coarse_height()) {
            for coarse_x in 0..(sprite.coarse_width()) {
                let tile_idx = sprite.tile + coarse_x + coarse_y * 16;
                let tile = Tile::<Bpp4Decoder>::from_tileset_index(sprite.nametable, tile_idx);
                for (fine_y, row) in tile.rows(&self.vram) {
                    for (fine_x, pixel) in row.pixels() {
                        let color = self.cgram[sprite.palette_addr() + pixel];
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
}

pub struct Framebuffer(Vec<Rgb15>);

impl Framebuffer {
    pub fn iter(&self) -> impl Iterator<Item = (u32, u32, &Rgb15)> {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, pixel)| (idx as u32 % 256, idx as u32 / 256, pixel))
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self(vec![Rgb15(0); 256 * 224])
    }
}

impl std::ops::Index<(u32, u32)> for Framebuffer {
    type Output = Rgb15;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.0[index.0 as usize + index.1 as usize * 256]
    }
}

impl std::ops::IndexMut<(u32, u32)> for Framebuffer {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        &mut self.0[index.0 as usize + index.1 as usize * 256]
    }
}

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum BgMode {
    #[default]
    Mode0,
    Mode1,
    Mode2,
    Mode3,
    Mode4,
    Mode5,
    Mode6,
    Mode7,
}

impl Display for BgMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BgMode::Mode0 => write!(f, "Mode 0"),
            BgMode::Mode1 => write!(f, "Mode 1"),
            BgMode::Mode2 => write!(f, "Mode 2"),
            BgMode::Mode3 => write!(f, "Mode 3"),
            BgMode::Mode4 => write!(f, "Mode 4"),
            BgMode::Mode5 => write!(f, "Mode 5"),
            BgMode::Mode6 => write!(f, "Mode 6"),
            BgMode::Mode7 => write!(f, "Mode 7"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Background {
    pub enabled: bool,
    pub bit_depth: BitDepth,
    pub palette_addr: u8,
    pub tile_size: TileSize,
    pub tilemap_addr: VramAddr,
    pub tileset_addr: VramAddr,
    pub tilemap_size: TilemapSize,
    pub h_offset: u32,
    pub v_offset: u32,
}

impl Background {
    pub fn get_tile<TileDecoderT: TileDecoder>(
        &self,
        coarse_x: u32,
        coarse_y: u32,
        vram: &Vram,
    ) -> Tile<TileDecoderT> {
        let tilemap_idx = match self.tilemap_size {
            TilemapSize::Size32x32 => 0,
            TilemapSize::Size64x32 => coarse_x / 32,
            TilemapSize::Size32x64 => coarse_y / 32,
            TilemapSize::Size64x64 => coarse_x / 32 + (coarse_y / 32) * 2,
        };
        let tile_idx = tilemap_idx * 1024 + (coarse_y % 32) * 32 + (coarse_x % 32);
        let tilemap_entry = vram[self.tilemap_addr + tile_idx];
        Tile::from_tilemap_entry(self.tileset_addr, tilemap_entry)
    }

    pub fn get_tileset_tile<TileDecoderT: TileDecoder>(&self, index: u32) -> Tile<TileDecoderT> {
        Tile::from_tileset_index(self.tileset_addr, index)
    }

    pub fn coarse_width(&self) -> u32 {
        match self.tilemap_size {
            TilemapSize::Size32x32 => 32,
            TilemapSize::Size64x32 => 64,
            TilemapSize::Size32x64 => 32,
            TilemapSize::Size64x64 => 64,
        }
    }

    pub fn coarse_height(&self) -> u32 {
        match self.tilemap_size {
            TilemapSize::Size32x32 => 32,
            TilemapSize::Size64x32 => 32,
            TilemapSize::Size32x64 => 64,
            TilemapSize::Size64x64 => 64,
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BitDepth {
    #[default]
    Disabled,
    Bpp2,
    Bpp4,
    Bpp8,
    Opt,
}

impl Display for BitDepth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BitDepth::Disabled => write!(f, "Disabled"),
            BitDepth::Bpp2 => write!(f, "2bpp"),
            BitDepth::Bpp4 => write!(f, "4bpp"),
            BitDepth::Bpp8 => write!(f, "8bpp"),
            BitDepth::Opt => write!(f, "Opt"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TileSize {
    #[default]
    Size8x8,
    Size16x16,
}

impl Display for TileSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileSize::Size8x8 => write!(f, "8x8"),
            TileSize::Size16x16 => write!(f, "16x16"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TilemapSize {
    #[default]
    Size32x32,
    Size64x32,
    Size32x64,
    Size64x64,
}

impl Display for TilemapSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TilemapSize::Size32x32 => write!(f, "32x32"),
            TilemapSize::Size64x32 => write!(f, "64x32"),
            TilemapSize::Size32x64 => write!(f, "32x64"),
            TilemapSize::Size64x64 => write!(f, "64x64"),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BackgroundId {
    #[default]
    BG0 = 0,
    BG1 = 1,
    BG2 = 2,
    BG3 = 3,
}

impl Display for BackgroundId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BackgroundId::BG0 => write!(f, "BG0"),
            BackgroundId::BG1 => write!(f, "BG1"),
            BackgroundId::BG2 => write!(f, "BG2"),
            BackgroundId::BG3 => write!(f, "BG3"),
        }
    }
}

pub struct Tile<TileDecoderT: TileDecoder> {
    tile_addr: VramAddr,
    palette: u8,
    priority: bool,
    flip_v: bool,
    flip_h: bool,
    _decoder: PhantomData<TileDecoderT>,
}

impl<TileDecoderT: TileDecoder> Tile<TileDecoderT> {
    pub fn from_tilemap_entry(tileset_addr: VramAddr, tilemap_entry: u16) -> Self {
        let tile_idx = tilemap_entry.bits(0..=9);
        Self {
            tile_addr: tileset_addr + tile_idx * TileDecoderT::WORDS_PER_ROW as u16 * 8,
            palette: tilemap_entry.bits(10..=12) as u8,
            priority: tilemap_entry.bit(13),
            flip_v: tilemap_entry.bit(15),
            flip_h: tilemap_entry.bit(14),
            _decoder: PhantomData,
        }
    }

    pub fn from_tileset_index(tileset_addr: VramAddr, tile_idx: u32) -> Self {
        Self {
            tile_addr: tileset_addr + tile_idx * TileDecoderT::WORDS_PER_ROW * 8,
            palette: 0,
            priority: false,
            flip_v: false,
            flip_h: false,
            _decoder: PhantomData,
        }
    }

    pub fn row(&self, row_idx: u32, vram: &Vram) -> TileRow<TileDecoderT> {
        let flipped_idx = if self.flip_v { 7 - row_idx } else { row_idx };
        TileRow::new(
            TileDecoderT::new(self.tile_addr + flipped_idx, vram),
            self.palette,
            self.flip_h,
        )
    }

    pub fn rows<'a>(
        &'a self,
        vram: &'a Vram,
    ) -> impl Iterator<Item = (u32, TileRow<TileDecoderT>)> + 'a {
        (0..8).map(move |row_idx| (row_idx, self.row(row_idx, vram)))
    }
}

pub struct TileRow<TileDecoderT: TileDecoder> {
    decoder: TileDecoderT,
    palette: u8,
    flip: bool,
}

impl<TileDecoderT: TileDecoder> TileRow<TileDecoderT> {
    pub fn new(decoder: TileDecoderT, palette: u8, flip: bool) -> Self {
        Self {
            decoder,
            palette,
            flip,
        }
    }

    pub fn pixel(&self, pixel_idx: u32) -> u8 {
        let flipped_idx = if self.flip { pixel_idx } else { 7 - pixel_idx };
        let raw_pixel = self.decoder.pixel(flipped_idx);
        if raw_pixel == 0 {
            0
        } else {
            raw_pixel + self.palette * TileDecoderT::NUM_COLORS
        }
    }

    pub fn pixels(&self) -> impl Iterator<Item = (u32, u8)> + '_ {
        (0..8).map(|pixel_idx| (pixel_idx, self.pixel(pixel_idx)))
    }
}

pub trait TileDecoder {
    const WORDS_PER_ROW: u32;
    const NUM_COLORS: u8;

    fn new(tile_addr: VramAddr, vram: &Vram) -> Self;
    fn pixel(&self, pixel_idx: u32) -> u8;
}

pub struct Bpp2Decoder {
    planes: [u8; 2],
}

impl TileDecoder for Bpp2Decoder {
    const WORDS_PER_ROW: u32 = 1;
    const NUM_COLORS: u8 = 4;

    fn new(row_addr: VramAddr, vram: &Vram) -> Self {
        let data = vram[row_addr];
        Self {
            planes: [data.low_byte(), data.high_byte()],
        }
    }

    fn pixel(&self, pixel_idx: u32) -> u8 {
        self.planes[0].bit(pixel_idx) as u8 + ((self.planes[1].bit(pixel_idx) as u8) << 1)
    }
}

pub struct Bpp4Decoder {
    planes: [u8; 4],
}

impl TileDecoder for Bpp4Decoder {
    const WORDS_PER_ROW: u32 = 2;
    const NUM_COLORS: u8 = 16;

    fn new(row_addr: VramAddr, vram: &Vram) -> Self {
        let low_word = vram[row_addr];
        let high_word = vram[row_addr + 8_u16];
        Self {
            planes: [
                low_word.low_byte(),
                low_word.high_byte(),
                high_word.low_byte(),
                high_word.high_byte(),
            ],
        }
    }

    fn pixel(&self, pixel_idx: u32) -> u8 {
        self.planes[0].bit(pixel_idx) as u8
            + ((self.planes[1].bit(pixel_idx) as u8) << 1)
            + ((self.planes[2].bit(pixel_idx) as u8) << 2)
            + ((self.planes[3].bit(pixel_idx) as u8) << 3)
    }
}

pub struct Bpp8Decoder {
    planes: [u8; 8],
}

impl TileDecoder for Bpp8Decoder {
    const WORDS_PER_ROW: u32 = 4;
    const NUM_COLORS: u8 = 255;

    fn new(row_addr: VramAddr, vram: &Vram) -> Self {
        let word0 = vram[row_addr];
        let word1 = vram[row_addr + 8_u16];
        let word2 = vram[row_addr + 16_u16];
        let word3 = vram[row_addr + 24_u16];
        Self {
            planes: [
                word0.low_byte(),
                word0.high_byte(),
                word1.low_byte(),
                word1.high_byte(),
                word2.low_byte(),
                word2.high_byte(),
                word3.low_byte(),
                word3.high_byte(),
            ],
        }
    }

    fn pixel(&self, pixel_idx: u32) -> u8 {
        self.planes[0].bit(pixel_idx) as u8
            + ((self.planes[1].bit(pixel_idx) as u8) << 1)
            + ((self.planes[2].bit(pixel_idx) as u8) << 2)
            + ((self.planes[3].bit(pixel_idx) as u8) << 3)
            + ((self.planes[4].bit(pixel_idx) as u8) << 4)
            + ((self.planes[5].bit(pixel_idx) as u8) << 5)
            + ((self.planes[6].bit(pixel_idx) as u8) << 6)
            + ((self.planes[7].bit(pixel_idx) as u8) << 7)
    }
}
