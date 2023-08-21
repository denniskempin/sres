use image::Rgba;
use image::RgbaImage;
use intbits::Bits;
use log::debug;

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

    /// Writes to 0x2100..0x213F are handled by the PPU
    pub fn write_ppu_register(&mut self, addr: u8, value: u8) {
        match addr {
            0x07..=0x0A => {
                let bg_id = (addr - 0x07) as usize;
                self.backgrounds[bg_id].tilemap_addr = (((value as usize) << 9) & 0xFFFF) >> 1;
            }
            0x0B => {
                self.backgrounds[0].tileset_addr = (value.low_nibble() as usize) << 12;
                self.backgrounds[1].tileset_addr = (value.high_nibble() as usize) << 12;
            }
            0x0C => {
                self.backgrounds[2].tileset_addr = (value.low_nibble() as usize) << 12;
                self.backgrounds[3].tileset_addr = (value.high_nibble() as usize) << 12;
            }
            0x15 => self.vram.increment_mode = value.bit(7),
            0x16 => self.vram.set_address_low(value),
            0x17 => self.vram.set_address_high(value),
            0x18 => self.vram.write_selected_low(value),
            0x19 => self.vram.write_selected_high(value),
            _ => (),
        }
    }

    /// Reads from 0x2100..0x213F are handled by the PPU
    pub fn read_ppu_register(&mut self, addr: u8) -> u8 {
        match addr {
            0x39 => self.vram.read_selected_low(),
            0x3A => self.vram.read_selected_high(),
            _ => 0,
        }
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
    pub fn new() -> Self {
        Self {
            memory: vec![0; 0x20000],
            current_addr: 0,
            read_latch: false,
            increment_mode: false,
        }
    }

    pub fn set_address_low(&mut self, value: u8) {
        self.current_addr.set_low_byte(value);
        self.read_latch = true;
    }

    pub fn set_address_high(&mut self, value: u8) {
        self.current_addr.set_high_byte(value);
        self.read_latch = true;
    }

    pub fn read_selected_low(&mut self) -> u8 {
        let value = self.memory[self.current_addr as usize].low_byte();
        if !self.increment_mode {
            if self.read_latch {
                self.read_latch = false;
            } else {
                self.current_addr = self.current_addr.wrapping_add(1);
            }
        }
        value
    }

    pub fn read_selected_high(&mut self) -> u8 {
        let value = self.memory[self.current_addr as usize].high_byte();
        if self.increment_mode {
            if self.read_latch {
                self.read_latch = false;
            } else {
                self.current_addr = self.current_addr.wrapping_add(1);
            }
        }
        value
    }

    pub fn write_selected_low(&mut self, value: u8) {
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

    pub fn write_selected_high(&mut self, value: u8) {
        debug!("VRAM[{:04X}].high = {}", self.current_addr, value);
        self.memory[self.current_addr as usize].set_high_byte(value);
        if self.increment_mode {
            self.current_addr = self.current_addr.wrapping_add(1);
        }
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
