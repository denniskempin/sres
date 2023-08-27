use image::Rgba;
use image::RgbaImage;
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

    /// Register 2107..210A: BGNSC - BG1..BG4 tilemap base address
    fn write_bgnsc(&mut self, addr: Address, value: u8) {
        let bg_id = (addr.offset - 0x2107) as usize;
        self.backgrounds[bg_id].tilemap_addr = (((value as usize) << 9) & 0xFFFF) >> 1;
    }

    /// Register 210B: BG12NBA - Tileset base address for BG1 and BG2
    fn write_bg12nba(&mut self, value: u8) {
        self.backgrounds[0].tileset_addr = (value.low_nibble() as usize) << 12;
        self.backgrounds[1].tileset_addr = (value.high_nibble() as usize) << 12;
    }

    /// Register 210C: BG34NBA - Tileset base address for BG3 and BG4
    fn write_bg34nba(&mut self, value: u8) {
        self.backgrounds[2].tileset_addr = (value.low_nibble() as usize) << 12;
        self.backgrounds[3].tileset_addr = (value.high_nibble() as usize) << 12;
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
        debug!(
            "VRAM[{:04X}, {:04X}].low = {}",
            self.current_addr,
            self.current_addr << 1,
            value
        );
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

#[derive(Default, Copy, Clone, Debug)]
pub struct Background {
    tilemap_addr: usize,
    tileset_addr: usize,
}

impl Background {
    pub fn debug_render_tileset(&self, vram: &Vram) -> RgbaImage {
        let tileset_data = &vram.memory[self.tileset_addr..self.tileset_addr + 0x2000];
        let mut image = RgbaImage::new(16 * 8, 16 * 8);
        for tile_idx in 0..256 {
            let tile_x: u32 = (tile_idx % 16) * 8;
            let tile_y: u32 = (tile_idx / 16) * 8;
            let tile_addr = (tile_idx * 8) as usize;
            for (row_idx, row) in tileset_data[tile_addr..(tile_addr + 8)].iter().enumerate() {
                let low = row.low_byte();
                let high = row.high_byte();
                for col_idx in 0..8 {
                    let pixel = low.bit(col_idx) as u8 + ((high.bit(col_idx) as u8) << 1);
                    image[(tile_x + (7 - col_idx), tile_y + row_idx as u32)] = if pixel > 0 {
                        Rgba([255, 255, 255, 255])
                    } else {
                        Rgba([0, 0, 0, 255])
                    };
                }
            }
        }
        image
    }

    pub fn debug_render_tilemap(&self, vram: &Vram) -> RgbaImage {
        let tileset_data = &vram.memory[self.tileset_addr..self.tileset_addr + 0x2000];
        let tilemap_data = &vram.memory[self.tilemap_addr..self.tilemap_addr + 0x2000];
        let mut image = RgbaImage::new(32 * 8, 32 * 8);
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
                        image[(tile_x + (7 - col_idx), tile_y + row_idx as u32)] = if pixel > 0 {
                            Rgba([255, 255, 255, 255])
                        } else {
                            Rgba([0, 0, 0, 255])
                        };
                    }
                }
            }
        }
        image
    }
}
