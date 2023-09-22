mod cgram;
mod timer;
mod vram;

use std::fmt::Display;
use std::fmt::Formatter;

use intbits::Bits;
use log::error;

use self::cgram::CgRam;
pub use self::timer::fvh_to_master_clock;
use self::timer::PpuTimer;
use self::vram::Vram;
use crate::util::image::Image;
use crate::util::image::Rgb15;
use crate::util::memory::Address;
use crate::util::uint::U16Ext;
use crate::util::uint::U8Ext;

pub struct Ppu {
    pub timer: PpuTimer,
    pub vram: Vram,
    pub backgrounds: [Background; 4],

    pub framebuffer: Vec<Rgb15>,
    pub cgram: CgRam,
    pub last_drawn_scanline: u64,

    pub bgofs_latch: u8,
    pub bghofs_latch: u8,
}

impl Ppu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            timer: PpuTimer::default(),
            vram: Vram::new(),
            backgrounds: [Background::default(); 4],
            framebuffer: vec![Rgb15(0); 256 * 224],
            cgram: CgRam::new(),
            last_drawn_scanline: 0,
            bgofs_latch: 0,
            bghofs_latch: 0,
        }
    }

    pub fn bus_read(&mut self, addr: Address) -> u8 {
        match addr.offset {
            0x2139 => self.vram.read_vmdatalread(),
            0x213A => self.vram.read_vmdatahread(),
            0x213B => self.cgram.read_cgdataread(),
            _ => 0,
        }
    }

    pub fn bus_peek(&self, addr: Address) -> Option<u8> {
        match addr.offset {
            0x2139 => Some(self.vram.peek_vmdatalread()),
            0x213A => Some(self.vram.peek_vmdatahread()),
            0x213B => Some(self.cgram.peek_cgdataread()),
            _ => None,
        }
    }

    pub fn bus_write(&mut self, addr: Address, value: u8) {
        match addr.offset {
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
            _ => (),
        }
    }

    pub fn advance_master_clock(&mut self, cycles: u64) {
        self.timer.advance_master_clock(cycles);
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
        for (idx, pixel) in self.framebuffer.iter().enumerate() {
            image.set_pixel((idx as u32 % 256, idx as u32 / 256), (*pixel).into());
        }
        image
    }

    fn draw_scanline(&mut self, scanline: u32) {
        if scanline >= 224 {
            return;
        }

        let bg = &self.backgrounds[0];
        let framebuffer_idx = scanline as usize * 256;
        let coarse_y = scanline / 8;
        let fine_y = scanline % 8;

        for coarse_x in 0..32 {
            let tile = bg.get_tile(coarse_x, coarse_y, &self.vram);
            for (fine_x, pixel) in tile.row(fine_y as u16, &self.vram).pixels().enumerate() {
                let color = self.cgram.memory[pixel as usize];
                self.framebuffer[framebuffer_idx + coarse_x as usize * 8 + fine_x] = color;
            }
        }
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
        let bg_mode = value.bits(0..=2);
        use BitDepth::*;
        let bit_depths = match bg_mode {
            0 => (Bpp2, Bpp2, Bpp2, Bpp2),
            1 => (Bpp4, Bpp4, Bpp2, Disabled),
            2 => (Bpp4, Bpp4, Opt, Disabled),
            3 => (Bpp8, Bpp4, Disabled, Disabled),
            4 => (Bpp8, Bpp2, Opt, Disabled),
            5 => (Bpp4, Bpp2, Disabled, Disabled),
            6 => (Bpp4, Disabled, Opt, Disabled),
            7 => (Bpp8, Disabled, Disabled, Disabled),
            _ => unreachable!(),
        };
        self.backgrounds[0].bit_depth = bit_depths.0;
        self.backgrounds[1].bit_depth = bit_depths.1;
        self.backgrounds[2].bit_depth = bit_depths.2;
        self.backgrounds[3].bit_depth = bit_depths.3;

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
        self.backgrounds[bg_id].tilemap_addr = ((value.bits(2..=7) as u16) << 10) & 0x7FFF;
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
        self.backgrounds[0].tileset_addr = (value.low_nibble() as u16) << 12;
        self.backgrounds[1].tileset_addr = (value.high_nibble() as u16) << 12;
    }

    /// Register 210C: BG34NBA - Tileset base address for BG3 and BG4
    /// 7  bit  0
    /// ---- ----
    /// DDDD CCCC
    /// |||| ||||
    /// |||| ++++- BG3 CHR word base address (word address = CCCC << 12)
    /// ++++------ BG4 CHR word base address (word address = DDDD << 12)
    fn write_bg34nba(&mut self, value: u8) {
        self.backgrounds[2].tileset_addr = (value.low_nibble() as u16) << 12;
        self.backgrounds[3].tileset_addr = (value.high_nibble() as u16) << 12;
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
        self.backgrounds[bg_id].h_offset = ((value as u16) << 8)
            | ((self.bgofs_latch as u16) & !7)
            | ((self.bghofs_latch as u16) & 7);
        error!(
            "BGNHOFS: {} = {:04X} (write {})",
            bg_id, self.backgrounds[bg_id].h_offset, value
        );
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
        self.backgrounds[bg_id].v_offset = ((value as u16) << 8) | (self.bgofs_latch as u16);
        self.bgofs_latch = value;
    }

    // Debug APIs

    pub fn debug_render_tileset<ImageT: Image>(&self, background_id: BackgroundId) -> ImageT {
        let mut image = ImageT::new(16 * 8, 16 * 8);
        let bg = &self.backgrounds[background_id as usize];
        for tile_idx in 0..256_u16 {
            let tile_x: u32 = (tile_idx as u32 % 16) * 8;
            let tile_y: u32 = (tile_idx as u32 / 16) * 8;
            let tile = bg.get_tileset_tile(tile_idx);
            for (row_idx, row) in tile.rows(&self.vram).enumerate() {
                for (col_idx, pixel) in row.pixels().enumerate() {
                    let color = self.cgram.memory[pixel as usize];
                    image.set_pixel(
                        (tile_x + col_idx as u32, tile_y + row_idx as u32),
                        color.into(),
                    );
                }
            }
        }
        image
    }

    pub fn debug_render_background<ImageT: Image>(&self, background_id: BackgroundId) -> ImageT {
        let bg = &self.backgrounds[background_id as usize];
        let mut image = ImageT::new(bg.coarse_width() * 8, bg.coarse_height() * 8);
        for coarse_y in 0..bg.coarse_height() {
            for coarse_x in 0..bg.coarse_width() {
                let tile = bg.get_tile(coarse_x, coarse_y, &self.vram);
                for (fine_y, row) in tile.rows(&self.vram).enumerate() {
                    for (fine_x, pixel) in row.pixels().enumerate() {
                        let color = self.cgram.memory[pixel as usize];
                        image.set_pixel(
                            (coarse_x * 8 + fine_x as u32, coarse_y * 8 + fine_y as u32),
                            color.into(),
                        );
                    }
                }
            }
        }
        image
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Background {
    pub bit_depth: BitDepth,
    pub tile_size: TileSize,
    pub tilemap_addr: u16,
    pub tileset_addr: u16,
    pub tilemap_size: TilemapSize,
    pub h_offset: u16,
    pub v_offset: u16,
}

impl Background {
    pub fn get_tile(&self, coarse_x: u32, coarse_y: u32, vram: &Vram) -> Tile {
        let tilemap_idx = match self.tilemap_size {
            TilemapSize::Size32x32 => 0,
            TilemapSize::Size64x32 => coarse_x / 32,
            TilemapSize::Size32x64 => coarse_y / 32,
            TilemapSize::Size64x64 => coarse_x / 32 + (coarse_y / 32) * 2,
        };
        let tile_idx = tilemap_idx * 1024 + (coarse_y % 32) * 32 + (coarse_x % 32);
        let tilemap_entry = vram.memory[self.tilemap_addr as usize + tile_idx as usize];
        Tile::from_tilemap_entry(self.tileset_addr, tilemap_entry)
    }

    pub fn get_tileset_tile(&self, index: u16) -> Tile {
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

#[derive(Default, Copy, Clone, Debug)]
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

#[derive(Default, Copy, Clone, Debug)]
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

#[derive(Default, Copy, Clone, Debug)]
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

#[derive(Default, Copy, Clone, Debug, PartialEq)]
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

pub struct Tile {
    tile_addr: u16,
    flip_v: bool,
    flip_h: bool,
}

impl Tile {
    pub fn from_tilemap_entry(tileset_addr: u16, tilemap_entry: u16) -> Self {
        let tile_idx = tilemap_entry.bits(0..=9);
        Self {
            tile_addr: tileset_addr + tile_idx * 8,
            flip_v: tilemap_entry.bit(15),
            flip_h: tilemap_entry.bit(14),
        }
    }

    pub fn from_tileset_index(tileset_addr: u16, index: u16) -> Self {
        Self {
            tile_addr: tileset_addr + index * 8,
            flip_v: false,
            flip_h: false,
        }
    }

    pub fn row(&self, row_idx: u16, vram: &Vram) -> TileRow {
        let flipped_idx = if self.flip_v { 7 - row_idx } else { row_idx };
        TileRow::new(
            vram.memory[(self.tile_addr + flipped_idx) as usize],
            self.flip_h,
        )
    }

    pub fn rows<'a>(&'a self, vram: &'a Vram) -> impl Iterator<Item = TileRow> + 'a {
        (0..8).map(move |row_idx| self.row(row_idx, vram))
    }
}

pub struct TileRow {
    low: u8,
    high: u8,
    flip: bool,
}

impl TileRow {
    pub fn new(row_data: u16, flip: bool) -> Self {
        Self {
            low: row_data.low_byte(),
            high: row_data.high_byte(),
            flip,
        }
    }

    pub fn pixel(&self, pixel_idx: usize) -> u8 {
        let flipped_idx = if self.flip { pixel_idx } else { 7 - pixel_idx };
        self.low.bit(flipped_idx) as u8 + ((self.high.bit(flipped_idx) as u8) << 1)
    }

    pub fn pixels<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        (0..8).map(|pixel_idx| self.pixel(pixel_idx))
    }
}
