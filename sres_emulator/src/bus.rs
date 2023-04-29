use crate::cartridge::Cartridge;
use crate::memory::Memory;
use crate::memory::ToAddress;

pub trait Bus: Memory {}

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

impl Memory for SresBus {
    fn peek<Addr: ToAddress>(&self, addr: Addr) -> Option<u8> {
        let addr = addr.to_address();
        match addr.offset {
            0x8000..=0xFFFF => {
                let rom_addr = (addr.offset as usize - 0x8000) + addr.bank as usize * 0x8000;
                if rom_addr < self.cartridge.rom.len() {
                    Some(self.cartridge.rom[rom_addr])
                } else {
                    #[cfg(feature = "debug_log")]
                    println!("Invalid read from ${addr} (rom addr ${rom_addr:06X})");
                    None
                }
            }
            // Fake RDNMI register. NMI is always true.
            0x4210 => Some(0xC2),
            _ => {
                #[cfg(feature = "debug_log")]
                println!("Invalid read from ${addr}");
                None
            }
        }
    }

    fn read<Addr: ToAddress>(&mut self, addr: Addr) -> u8 {
        self.peek(addr).unwrap_or(0)
    }

    fn write<Addr: ToAddress>(&mut self, addr: Addr, val: u8) {
        let addr = addr.to_address();
        #[allow(clippy::single_match)]
        match addr.offset {
            0x8000..=0xFFFF => {
                let rom_addr = (addr.offset as usize - 0x8000) + addr.bank as usize * 0x8000;
                if rom_addr < self.cartridge.rom.len() {
                    self.cartridge.rom[rom_addr] = val;
                } else {
                    #[cfg(feature = "debug_log")]
                    println!("Invalid write to ${addr} (rom addr ${rom_addr:06X})");
                }
            }
            _ => {
                #[cfg(feature = "debug_log")]
                println!("Invalid write to ${addr}");
            }
        }
    }
}

impl Bus for SresBus {}

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
