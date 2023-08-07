use crate::uint::U16Ext;

pub struct Ppu {
    pub vram: Vec<u8>,

    vram_address: u16,
    vram_address_latch: bool,
}

impl Ppu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            vram: vec![0; 0x10000],
            vram_address: 0,
            vram_address_latch: false,
        }
    }

    /// Writes to 0x2100..0x213F are handled by the PPU
    pub fn write_ppu_register(&mut self, addr: u8, value: u8) {
        //println!("PPU Write: ${:04X} = {:02X}", 0x2100 + addr as u16, value);
        match addr {
            0x16 => {
                self.vram_address.set_low_byte(value);
                self.vram_address_latch = true;
            }
            0x17 => {
                self.vram_address.set_high_byte(value);
                self.vram_address_latch = true;
            }
            0x18 => {
                self.vram[self.vram_address as usize] = value;
            }
            0x19 => {
                self.vram[self.vram_address as usize + 1] = value;
                self.vram_address = self.vram_address.wrapping_add(2);
            }
            _ => (),
        }
    }

    /// Reads from 0x2100..0x213F are handled by the PPU
    pub fn read_ppu_register(&mut self, addr: u8) -> u8 {
        //println!("PPU Read: ${:04X}", 0x2100 + addr as u16);
        match addr {
            0x39 => self.vram[self.vram_address as usize],
            0x3A => {
                let value = self.vram[self.vram_address as usize + 1];
                if self.vram_address_latch {
                    self.vram_address_latch = false;
                } else {
                    self.vram_address = self.vram_address.wrapping_add(2);
                }
                value
            }
            _ => 0,
        }
    }
}
