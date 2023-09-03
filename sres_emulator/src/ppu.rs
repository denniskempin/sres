use std::fmt::Display;
use std::fmt::Formatter;

use intbits::Bits;
use log::debug;

use crate::memory::Address;
use crate::uint::U16Ext;
use crate::uint::U8Ext;

pub struct Ppu {
    pub vram: Vram,
    pub backgrounds: [Background; 4],
}
impl Ppu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            vram: Vram::new(),
            backgrounds: [Background::default(); 4],
        }
    }

    pub fn bus_read(&mut self, addr: Address) -> u8 {
        match addr.offset {
            0x2139 => self.vram.read_vmdatalread(),
            0x213A => self.vram.read_vmdatahread(),
            _ => 0,
        }
    }

    pub fn bus_peek(&self, addr: Address) -> Option<u8> {
        match addr.offset {
            0x2139 => Some(self.vram.peek_vmdatalread()),
            0x213A => Some(self.vram.peek_vmdatahread()),
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
            _ => (),
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

    pub fn debug_render_tileset<ImageT: ImageBackend>(
        &self,
        background_id: BackgroundId,
    ) -> ImageT {
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
                    image.set_pixel(
                        (tile_x + (7 - col_idx), tile_y + row_idx as u32),
                        if pixel > 0 {
                            [255, 255, 255, 255]
                        } else {
                            [0, 0, 0, 255]
                        },
                    );
                }
            }
        }
        image
    }

    pub fn debug_render_tilemap<ImageT: ImageBackend>(
        &self,
        background_id: BackgroundId,
    ) -> ImageT {
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
                        image.set_pixel(
                            (tile_x + (7 - col_idx), tile_y + row_idx as u32),
                            if pixel > 0 {
                                [255, 255, 255, 255]
                            } else {
                                [0, 0, 0, 255]
                            },
                        );
                    }
                }
            }
        }
        image
    }
}

pub struct Vram {
    pub memory: Vec<u16>,
    pub current_addr: u16,
    pub read_latch: bool,
    pub increment_mode: bool,
}

impl Vram {
    #[allow(clippy::new_without_default)]
    fn new() -> Self {
        Self {
            memory: vec![0; 0x20000],
            current_addr: 0,
            read_latch: false,
            increment_mode: false,
        }
    }

    /// Register 2115: VMAIN - Video port control
    /// 7  bit  0
    /// ---- ----
    /// M... RRII
    /// |    ||||
    /// |    ||++- Address increment amount:
    /// |    ||     0: Increment by 1 word
    /// |    ||     1: Increment by 32 words
    /// |    ||     2: Increment by 128 words
    /// |    ||     3: Increment by 128 words
    /// |    ++--- Address remapping: (VMADD -> Internal)
    /// |           0: None
    /// |           1: Remap rrrrrrrr YYYccccc -> rrrrrrrr cccccYYY (2bpp)
    /// |           2: Remap rrrrrrrY YYcccccP -> rrrrrrrc ccccPYYY (4bpp)
    /// |           3: Remap rrrrrrYY YcccccPP -> rrrrrrcc cccPPYYY (8bpp)
    /// +--------- Address increment mode:
    ///             0: Increment after writing $2118 or reading $2139
    ///             1: Increment after writing $2119 or reading $213A
    fn write_vmain(&mut self, value: u8) {
        self.increment_mode = value.bit(7)
    }

    /// Register 2116: VMADDL - VRAM word address low
    fn write_vmaddl(&mut self, value: u8) {
        self.current_addr.set_low_byte(value);
        self.read_latch = true;
    }

    /// Register 2117: VMADDH - VRAM word address high
    fn write_vmaddh(&mut self, value: u8) {
        self.current_addr.set_high_byte(value);
        self.read_latch = true;
    }

    /// Register 2118: VMDATAL - VRAM data write low
    fn write_vmdatal(&mut self, value: u8) {
        debug!("VRAM[{:04X}].low = {}", self.current_addr, value);
        self.memory[self.current_addr as usize].set_low_byte(value);
        if !self.increment_mode {
            self.current_addr = self.current_addr.wrapping_add(1);
        }
    }

    /// Register 2119: VMDATAH - VRAM data write high
    fn write_vmdatah(&mut self, value: u8) {
        debug!("VRAM[{:04X}].high = {}", self.current_addr, value);
        self.memory[self.current_addr as usize].set_high_byte(value);
        if self.increment_mode {
            self.current_addr = self.current_addr.wrapping_add(1);
        }
    }

    /// Register 2139: VMDATALREAD - VRAM data read low
    fn read_vmdatalread(&mut self) -> u8 {
        let value = self.peek_vmdatalread();
        if !self.increment_mode {
            if self.read_latch {
                self.read_latch = false;
            } else {
                self.current_addr = self.current_addr.wrapping_add(1);
            }
        }
        value
    }

    fn peek_vmdatalread(&self) -> u8 {
        self.memory[self.current_addr as usize].low_byte()
    }

    /// Register 213A: VMDATAHREAD - VRAM data read high
    fn read_vmdatahread(&mut self) -> u8 {
        let value = self.peek_vmdatahread();
        if self.increment_mode {
            if self.read_latch {
                self.read_latch = false;
            } else {
                self.current_addr = self.current_addr.wrapping_add(1);
            }
        }
        value
    }

    fn peek_vmdatahread(&self) -> u8 {
        self.memory[self.current_addr as usize].high_byte()
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

/// Abstract interface for image::RgbaImage or egui::ColorImage.
pub trait ImageBackend {
    fn new(width: u32, height: u32) -> Self;
    fn set_pixel(&mut self, index: (u32, u32), value: [u8; 4]);
}
