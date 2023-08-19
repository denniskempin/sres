use intbits::Bits;
use log::{debug, info};

use crate::uint::{U16Ext, U8Ext};

pub struct Ppu {
    pub vram: Vec<u16>,

    vram_address: u16,
    vram_address_latch: bool,
    vram_increment_mode: bool,

    pub backgrounds: [Background; 4],
}

impl Ppu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            vram: vec![0; 0x10000],
            vram_address: 0,
            vram_address_latch: false,
            vram_increment_mode: false,
            backgrounds: [Background::default(); 4],
        }
    }

    /// Writes to 0x2100..0x213F are handled by the PPU
    pub fn write_ppu_register(&mut self, addr: u8, value: u8) {
        //println!("PPU Write: ${:04X} = {:02X}", 0x2100 + addr as u16, value);
        match addr {
            0x07..=0x0A => {
                let bg_id = (addr - 0x07) as usize;
                self.backgrounds[bg_id].tilemap_addr = (value.bits(2..=7) as u16) << 10;
            }
            0x0B => {
                self.backgrounds[0].tileset_addr = (value.low_nibble() as u16) << 12;
                self.backgrounds[1].tileset_addr = (value.high_nibble() as u16) << 12;
            }
            0x0C => {
                self.backgrounds[2].tileset_addr = (value.low_nibble() as u16) << 12;
                self.backgrounds[3].tileset_addr = (value.high_nibble() as u16) << 12;
            }
            0x15 => {
                self.vram_increment_mode = value.bit(7);
            }
            0x16 => {
                self.vram_address.set_low_byte(value);
                self.vram_address_latch = true;
            }
            0x17 => {
                self.vram_address.set_high_byte(value);
                self.vram_address_latch = true;
            }
            0x18 => {
                debug!("VRAM[{:04X}].low = {}", self.vram_address, value);
                self.vram[self.vram_address as usize].set_low_byte(value);
                if !self.vram_increment_mode {
                    self.vram_address = self.vram_address.wrapping_add(1);
                }
            }
            0x19 => {
                debug!("VRAM[{:04X}].high = {}", self.vram_address, value);
                self.vram[self.vram_address as usize].set_high_byte(value);
                if self.vram_increment_mode {
                    self.vram_address = self.vram_address.wrapping_add(1);
                }
            }
            _ => (),
        }
    }

    /// Reads from 0x2100..0x213F are handled by the PPU
    pub fn read_ppu_register(&mut self, addr: u8) -> u8 {
        //println!("PPU Read: ${:04X}", 0x2100 + addr as u16);
        match addr {
            0x39 => {
                let value = self.vram[self.vram_address as usize].low_byte();
                if !self.vram_increment_mode {
                    if self.vram_address_latch {
                        self.vram_address_latch = false;
                    } else {
                        self.vram_address = self.vram_address.wrapping_add(1);
                    }
                }
                value
            }
            0x3A => {
                let value = self.vram[self.vram_address as usize].high_byte();
                if self.vram_increment_mode {
                    if self.vram_address_latch {
                        self.vram_address_latch = false;
                    } else {
                        self.vram_address = self.vram_address.wrapping_add(1);
                    }
                }
                value
            }
            _ => 0,
        }
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Background {
    tilemap_addr: u16,
    tileset_addr: u16,
}
