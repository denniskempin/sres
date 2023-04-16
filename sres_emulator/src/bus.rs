use intbits::Bits;

use crate::cartridge::Cartridge;

pub trait Bus {
    fn read(&mut self, addr: u32) -> u8;
    fn write(&mut self, addr: u32, val: u8);
}

pub struct SresBus {
    pub cartridge: Cartridge,
}

impl SresBus {
    pub fn new() -> Self {
        Self {
            cartridge: Cartridge::new(),
        }
    }
}

impl Default for SresBus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus for SresBus {
    fn read(&mut self, addr: u32) -> u8 {
        let bank = addr.bits(16..24);
        let offset = addr.bits(0..16);
        match offset {
            0x8000..=0xFFFF => {
                let rom_addr = ((offset - 0x8000) + bank * 0x8000) as usize;
                if rom_addr < self.cartridge.rom.len() {
                    self.cartridge.rom[rom_addr]
                } else {
                    println!("Invalid read from ${addr:06X} (rom addr ${rom_addr:06X})");
                    0
                }
            }
            _ => {
                println!("Invalid read from ${addr:06X}");
                0
            }
        }
    }

    fn write(&mut self, addr: u32, val: u8) {
        let bank = addr.bits(16..24);
        let offset = addr.bits(0..16);
        match offset {
            0x8000..=0xFFFF => {
                let rom_addr = ((offset - 0x8000) + bank * 0x8000) as usize;
                if rom_addr < self.cartridge.rom.len() {
                    self.cartridge.rom[rom_addr] = val;
                } else {
                    println!("Invalid write to ${addr:06X} (rom addr ${rom_addr:06X})");
                }
            }
            _ => {
                println!("Invalid write to ${addr:06X}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cartridge::MappingMode;

    #[test]
    fn test_read_rom() {
        let mut bus = SresBus {
            cartridge: Cartridge {
                // Two pages of memory
                rom: vec![0x00; 0x10000],
                mapping_mode: MappingMode::LoRom,
            },
        };
        // Access first byte of page 0
        bus.cartridge.rom[0x000000] = 0x12;
        assert_eq!(bus.read(0x008000), 0x12);

        // Access first byte of page 1
        bus.cartridge.rom[0x008000] = 0x23;
        assert_eq!(bus.read(0x018000), 0x23);
    }

    #[test]
    fn test_write_rom() {
        let mut bus = SresBus {
            cartridge: Cartridge {
                // Two pages of memory
                rom: vec![0x00; 0x10000],
                mapping_mode: MappingMode::LoRom,
            },
        };

        // Access first byte of page 0
        bus.write(0x008000, 0x12);
        assert_eq!(bus.cartridge.rom[0x000000], 0x12);

        // Access first byte of page 1
        bus.write(0x018000, 0x23);
        assert_eq!(bus.cartridge.rom[0x008000], 0x23);
    }
}
