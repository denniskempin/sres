mod cgram;
mod timer;
mod vram;

use std::fmt::Display;
use std::fmt::Formatter;

use intbits::Bits;

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
}

impl Ppu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            timer: PpuTimer::default(),
            vram: Vram::new(),
            backgrounds: [Background::default(); 4],
            framebuffer: vec![Rgb15(0); 256 * 256],
            cgram: CgRam::new(),
            last_drawn_scanline: 0,
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
        self.draw_scanline(self.timer.v);
    }

    pub fn reset(&mut self) {
        self.last_drawn_scanline = 0;
        self.timer = PpuTimer::default();
    }

    pub fn get_rgba_framebuffer<ImageT: Image>(&self) -> ImageT {
        let mut image = ImageT::new(256, 256);
        for (idx, pixel) in self.framebuffer.iter().enumerate() {
            image.set_pixel((idx as u32 % 256, idx as u32 / 256), (*pixel).into());
        }
        image
    }

    fn draw_scanline(&mut self, scanline: u64) {
        if scanline > 160 {
            return;
        }

        let bg = &self.backgrounds[0];
        let tileset_data = &self.vram.memory[bg.tileset_addr..bg.tileset_addr + 0x2000];
        let tilemap_data = &self.vram.memory[bg.tilemap_addr..bg.tilemap_addr + 0x2000];

        let framebuffer_idx = scanline as usize * 256;

        let coarse_y = scanline / 8;
        let fine_y = scanline % 8;
        for coarse_x in 0..32 {
            let tilemap_entry = tilemap_data[(coarse_y as usize) * 32 + coarse_x as usize];
            let tile_idx = tilemap_entry.bits(0..=9) as u32;
            let tile_addr = (tile_idx * 8) as usize;
            let tile_row_addr = tile_addr + (fine_y as usize);

            let row = tileset_data[tile_row_addr];
            let low = row.low_byte();
            let high = row.high_byte();
            for fine_x in 0..8 {
                let pixel = low.bit(7 - fine_x) as u8 + ((high.bit(7 - fine_x) as u8) << 1);
                let color = self.cgram.memory[pixel as usize];
                self.framebuffer[framebuffer_idx + coarse_x as usize * 8 + fine_x as usize] = color;
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
        self.backgrounds[bg_id].tilemap_addr = (((value as usize) << 9) & 0xFFFF) >> 1;
    }

    /// Register 210B: BG12NBA - Tileset base address for BG1 and BG2
    /// 7  bit  0
    /// ---- ----
    /// BBBB AAAA
    /// |||| ||||
    /// |||| ++++- BG1 CHR word base address (word address = AAAA << 12)
    /// ++++------ BG2 CHR word base address (word address = BBBB << 12)
    fn write_bg12nba(&mut self, value: u8) {
        self.backgrounds[0].tileset_addr = (value.low_nibble() as usize) << 12;
        self.backgrounds[1].tileset_addr = (value.high_nibble() as usize) << 12;
    }

    /// Register 210C: BG34NBA - Tileset base address for BG3 and BG4
    /// 7  bit  0
    /// ---- ----
    /// DDDD CCCC
    /// |||| ||||
    /// |||| ++++- BG3 CHR word base address (word address = CCCC << 12)
    /// ++++------ BG4 CHR word base address (word address = DDDD << 12)
    fn write_bg34nba(&mut self, value: u8) {
        self.backgrounds[2].tileset_addr = (value.low_nibble() as usize) << 12;
        self.backgrounds[3].tileset_addr = (value.high_nibble() as usize) << 12;
    }

    // Debug APIs

    pub fn debug_render_tileset<ImageT: Image>(&self, background_id: BackgroundId) -> ImageT {
        let bg = &self.backgrounds[background_id as usize];
        let tileset_data = &self.vram.memory[bg.tileset_addr..bg.tileset_addr + 0x2000];
        let mut image = ImageT::new(16 * 8, 16 * 8);
        for tile_idx in 0..256 {
            let tile_x: u32 = (tile_idx % 16) * 8;
            let tile_y: u32 = (tile_idx / 16) * 8;
            let tile_addr = (tile_idx * 8) as usize;
            for (row_idx, row) in tileset_data[tile_addr..(tile_addr + 8)].iter().enumerate() {
                let low = row.low_byte();
                let high = row.high_byte();
                for col_idx in 0..8 {
                    let pixel = low.bit(col_idx) as u8 + ((high.bit(col_idx) as u8) << 1);
                    let color = self.cgram.memory[pixel as usize];
                    image.set_pixel(
                        (tile_x + (7 - col_idx), tile_y + row_idx as u32),
                        color.into(),
                    );
                }
            }
        }
        image
    }

    pub fn debug_render_tilemap<ImageT: Image>(&self, background_id: BackgroundId) -> ImageT {
        let bg = &self.backgrounds[background_id as usize];
        let tileset_data = &self.vram.memory[bg.tileset_addr..bg.tileset_addr + 0x2000];
        let tilemap_data = &self.vram.memory[bg.tilemap_addr..bg.tilemap_addr + 0x2000];
        let mut image = ImageT::new(32 * 8, 32 * 8);
        for tile_y_idx in 0..32_u32 {
            for tile_x_idx in 0..32_u32 {
                let entry = tilemap_data[(tile_y_idx as usize) * 32 + tile_x_idx as usize];
                let tile_idx = entry.bits(0..=9) as u32;
                let tile_addr = (tile_idx * 8) as usize;
                let tile_x: u32 = tile_x_idx * 8;
                let tile_y: u32 = tile_y_idx * 8;
                for (row_idx, row) in tileset_data[tile_addr..(tile_addr + 8)].iter().enumerate() {
                    let low = row.low_byte();
                    let high = row.high_byte();
                    for col_idx in 0..8 {
                        let pixel = low.bit(col_idx) as u8 + ((high.bit(col_idx) as u8) << 1);
                        let color = self.cgram.memory[pixel as usize];
                        image.set_pixel(
                            (tile_x + (7 - col_idx), tile_y + row_idx as u32),
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
    pub tilemap_addr: usize,
    pub tileset_addr: usize,
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
