//! Main bus used by the 65816 CPU.
mod dma;
mod multiplication;

use dma::DmaController;
use log::trace;

use self::multiplication::MultiplicationUnit;
use crate::apu::Apu;
use crate::common::address::AddressU24;
use crate::common::bus::Bus;
use crate::common::clock::ClockInfo;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::common::uint::U16Ext;
use crate::components::cartridge::Cartridge;
use crate::components::cartridge::MappingMode;
use crate::components::cpu::MainBus;
use crate::components::ppu::Ppu;
use crate::debugger::DebuggerRef;

#[derive(Debug, Clone, PartialEq)]
pub enum MainBusEvent {
    Read(AddressU24, u8),
    Write(AddressU24, u8),
}

pub struct MainBusImpl {
    pub debug_event_collector: DebugEventCollectorRef<MainBusEvent>,

    pub wram: Vec<u8>,
    pub sram: Vec<u8>,
    pub rom: Vec<u8>,
    pub clock_speed: u64,
    pub dma_controller: DmaController,
    pub ppu: Ppu,
    pub apu: Apu,
    pub multiplication: MultiplicationUnit,
    pub joy1: u16,
    pub joy2: u16,
    pub mapping_mode: MappingMode,
}

impl MainBusImpl {
    pub fn new(cartridge: &Cartridge, debugger: DebuggerRef) -> Self {
        let mut rom = vec![0; 0x4000000];
        for (i, byte) in cartridge.rom.iter().enumerate() {
            rom[i] = *byte;
        }

        Self {
            wram: vec![0; 0x4000000],
            sram: cartridge.sram.clone(),
            rom,
            clock_speed: 8,
            dma_controller: DmaController::new(DebugEventCollectorRef(debugger.clone())),
            ppu: Ppu::new(),
            apu: Apu::new(debugger.clone()),
            multiplication: MultiplicationUnit::new(),
            debug_event_collector: DebugEventCollectorRef(debugger.clone()),
            joy1: 0,
            joy2: 0,
            mapping_mode: cartridge.mapping_mode,
        }
    }

    pub fn bus_peek(&self, addr: AddressU24) -> Option<u8> {
        match self.memory_map(addr) {
            MemoryBlock::Ram(offset) => Some(self.wram[offset]),
            MemoryBlock::Rom(offset) => Some(self.rom[offset]),
            MemoryBlock::Sram(offset) => Some(self.sram[offset]),
            MemoryBlock::Register => match addr.offset {
                0x2100..=0x213F => self.ppu.bus_peek(addr),
                0x2140..=0x217F => self.apu.bus_peek(addr),
                0x420B | 0x420C | 0x4300..=0x43FF => self.dma_controller.bus_peek(addr),
                0x4200 | 0x4207..=0x420A | 0x4210..=0x4212 => self.ppu.bus_peek(addr),
                0x4214..=0x4217 => self.multiplication.bus_peek(addr),
                0x4218 => Some(self.joy1.low_byte()),
                0x4219 => Some(self.joy1.high_byte()),
                0x421A => Some(self.joy2.low_byte()),
                0x421B => Some(self.joy2.high_byte()),
                _ => None,
            },
            MemoryBlock::Unmapped => None,
        }
    }

    pub fn bus_read(&mut self, addr: AddressU24) -> u8 {
        let value = match self.memory_map(addr) {
            MemoryBlock::Ram(offset) => self.wram[offset],
            MemoryBlock::Rom(offset) => self.rom[offset],
            MemoryBlock::Sram(offset) => self.sram[offset],
            MemoryBlock::Register => match addr.offset {
                0x2100..=0x213F => self.ppu.bus_read(addr),
                0x2140..=0x217F => self.apu.bus_read(addr),
                0x420B | 0x420C | 0x4300..=0x43FF => self.dma_controller.bus_read(addr),
                0x4200 | 0x4207..=0x420A | 0x4210..=0x4212 => self.ppu.bus_read(addr),
                0x4214..=0x4217 => self.multiplication.bus_read(addr),
                0x4016..=0x4017 => {
                    log::warn!("Serial Joypad not implemented");
                    0
                }
                0x4218 => self.joy1.low_byte(),
                0x4219 => self.joy1.high_byte(),
                0x421A => self.joy2.low_byte(),
                0x421B => self.joy2.high_byte(),
                _ => {
                    self.debug_event_collector
                        .on_error(format!("Read from unimplemented register {}", addr));
                    0
                }
            },
            MemoryBlock::Unmapped => {
                self.debug_event_collector
                    .on_error(format!("Read from unmapped memory region {}", addr));
                0
            }
        };
        self.debug_event_collector
            .on_event(MainBusEvent::Read(addr, value));
        value
    }

    #[allow(clippy::single_match)]
    fn bus_write(&mut self, addr: AddressU24, value: u8) {
        self.debug_event_collector
            .on_event(MainBusEvent::Write(addr, value));
        match self.memory_map(addr) {
            MemoryBlock::Ram(offset) => self.wram[offset] = value,
            MemoryBlock::Rom(offset) => self.rom[offset] = value,
            MemoryBlock::Sram(offset) => self.sram[offset] = value,
            MemoryBlock::Register => match addr.offset {
                0x2100..=0x213F => self.ppu.bus_write(addr, value),
                0x2140..=0x217F => self.apu.bus_write(addr, value),
                0x420B | 0x420C | 0x4300..=0x43FF => self.dma_controller.bus_write(addr, value),
                0x4202..=0x4206 => self.multiplication.bus_write(addr, value),
                0x4200 | 0x4207..=0x420A | 0x4210..=0x4212 => self.ppu.bus_write(addr, value),
                _ => {
                    self.debug_event_collector.on_error(format!(
                        "Write to unimplemented register {} = {}",
                        addr, value
                    ));
                }
            },
            MemoryBlock::Unmapped => {
                self.debug_event_collector
                    .on_error(format!("Write to unmapped region {}", addr));
            }
        }
    }

    #[inline]
    fn memory_map(&self, addr: AddressU24) -> MemoryBlock {
        // TODO: Unnecessary branch on each cpu cycle
        match self.mapping_mode {
            MappingMode::LoRom => lorom_memory_map(addr),
            MappingMode::HiRom => hirom_memory_map(addr),
        }
    }

    fn advance_master_clock(&mut self, cycles: u64) {
        self.ppu.advance_master_clock(cycles);

        if let Some((transfers, duration)) = self
            .dma_controller
            .pending_transfers(self.clock_info().master_clock, self.clock_speed)
        {
            self.ppu.advance_master_clock(duration);
            for (source, destination) in transfers {
                let value = self.bus_read(source);
                self.bus_write(destination, value);
            }
        }
        self.dma_controller.update_state();
    }
}

impl Bus<AddressU24> for MainBusImpl {
    fn peek_u8(&self, addr: AddressU24) -> Option<u8> {
        self.bus_peek(addr)
    }

    fn cycle_read_u8(&mut self, addr: AddressU24) -> u8 {
        self.clock_speed = memory_access_speed(addr);
        trace!(target: "cycles", "cycle read {addr} ({} cycles)", self.clock_speed);
        self.ppu.advance_master_clock(self.clock_speed - 6);
        let value = self.bus_read(addr);
        self.advance_master_clock(6);
        value
    }

    #[allow(clippy::single_match)]
    fn cycle_write_u8(&mut self, addr: AddressU24, val: u8) {
        self.clock_speed = memory_access_speed(addr);
        self.advance_master_clock(self.clock_speed);
        trace!(
            target: "cycles",
            "cycle write {addr} = {val:02x} ({} cycles)",
            self.clock_speed
        );

        self.bus_write(addr, val);
    }

    fn cycle_io(&mut self) {
        trace!(target: "cycles", "cycle io (6 cycles)");
        self.clock_speed = 6;
        self.advance_master_clock(self.clock_speed);
    }

    fn reset(&mut self) {
        self.ppu.reset();
        self.advance_master_clock(186);
    }
}

impl MainBus for MainBusImpl {
    fn consume_nmi_interrupt(&mut self) -> bool {
        self.ppu.consume_nmi_interrupt()
    }

    fn consume_timer_interrupt(&mut self) -> bool {
        self.ppu.consume_timer_interrupt()
    }

    fn clock_info(&self) -> ClockInfo {
        self.ppu.clock_info()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum MemoryBlock {
    Ram(usize),
    Rom(usize),
    Sram(usize),
    Register,
    Unmapped,
}

/// Memory access speed as per memory map. See:
/// https://wiki.superfamicom.org/memory-mapping#memory-map-67
fn memory_access_speed(addr: AddressU24) -> u64 {
    static FAST: u64 = 6;
    static SLOW: u64 = 8;
    static XSLOW: u64 = 12;

    match addr.bank {
        0x00..=0x3F => match addr.offset {
            0x0000..=0x1FFF => SLOW,
            0x2000..=0x3FFF => FAST,
            0x4000..=0x41FF => XSLOW,
            0x4200..=0x5FFF => FAST,
            0x6000..=0xFFFF => SLOW,
        },
        0x40..=0x7F => SLOW,
        0x80..=0xBF => {
            match addr.offset {
                0x0000..=0x1FFF => SLOW,
                0x2000..=0x3FFF => FAST,
                0x4000..=0x41FF => XSLOW,
                0x4200..=0x5FFF => FAST,
                0x6000..=0xFFFF => SLOW, // TODO fastrom support
            }
        }
        0xC0..=0xFF => SLOW, // TODO fastrom support
    }
}

#[inline]
fn lorom_memory_map(addr: AddressU24) -> MemoryBlock {
    match addr.bank {
        0x00..=0x3F => match addr.offset {
            0x0000..=0x1FFF => MemoryBlock::Ram(addr.offset as usize),
            0x2000..=0x7FFF => MemoryBlock::Register,
            0x8000..=0xFFFF => {
                MemoryBlock::Rom(addr.bank as usize * 0x8000 + (addr.offset as usize - 0x8000))
            }
        },
        0x40..=0x6F => match addr.offset {
            0x0000..=0x7FFF => MemoryBlock::Unmapped,
            0x8000..=0xFFFF => {
                MemoryBlock::Rom(addr.bank as usize * 0x8000 + (addr.offset as usize - 0x8000))
            }
        },
        0x70..=0x7D => match addr.offset {
            0x0000..=0x7FFF => {
                MemoryBlock::Sram((addr.bank as usize - 0x70) * 0x8000 + addr.offset as usize)
            }
            0x8000..=0xFFFF => {
                MemoryBlock::Rom(addr.bank as usize * 0x8000 + (addr.offset as usize - 0x8000))
            }
        },
        0x7E..=0x7F => {
            MemoryBlock::Ram((addr.bank as usize - 0x7E) * 0x10000 + addr.offset as usize)
        }
        0x80..=0xBF => match addr.offset {
            0x0000..=0x1FFF => MemoryBlock::Ram(addr.offset as usize),
            0x2000..=0x7FFF => MemoryBlock::Register,
            0x8000..=0xFFFF => MemoryBlock::Rom(
                (addr.bank as usize - 0x80) * 0x8000 + (addr.offset as usize - 0x8000),
            ),
        },
        0xC0..=0xFF => match addr.offset {
            0x0000..=0x7FFF => MemoryBlock::Unmapped,
            0x8000..=0xFFFF => MemoryBlock::Rom(
                (addr.bank as usize - 0x80) * 0x8000 + (addr.offset as usize - 0x8000),
            ),
        },
    }
}

#[inline]
fn hirom_memory_map(addr: AddressU24) -> MemoryBlock {
    match addr.bank {
        0x00..=0x2F => match addr.offset {
            0x0000..=0x1FFF => MemoryBlock::Ram(addr.offset as usize),
            0x2000..=0x5FFF => MemoryBlock::Register,
            0x6000..=0x7FFF => MemoryBlock::Unmapped,
            0x8000..=0xFFFF => {
                MemoryBlock::Rom(addr.bank as usize * 0x10000 + (addr.offset as usize))
            }
        },
        0x30..=0x3F => match addr.offset {
            0x0000..=0x1FFF => MemoryBlock::Ram(addr.offset as usize),
            0x2000..=0x5FFF => MemoryBlock::Register,
            0x6000..=0x7FFF => MemoryBlock::Sram(
                (addr.bank as usize - 0x30) * 0x2000 + addr.offset as usize - 0x6000,
            ),
            0x8000..=0xFFFF => {
                MemoryBlock::Rom(addr.bank as usize * 0x10000 + (addr.offset as usize))
            }
        },
        0x40..=0x7D => MemoryBlock::Unmapped,
        0x7E..=0x7F => {
            MemoryBlock::Ram((addr.bank as usize - 0x7E) * 0x10000 + addr.offset as usize)
        }
        0x80..=0xBF => match addr.offset {
            0x0000..=0x1FFF => MemoryBlock::Ram(addr.offset as usize),
            0x2000..=0x5FFF => MemoryBlock::Register,
            0x6000..=0x7FFF => MemoryBlock::Unmapped,
            0x8000..=0xFFFF => {
                MemoryBlock::Rom((addr.bank as usize - 0x80) * 0x10000 + (addr.offset as usize))
            }
        },
        0xC0..=0xFF => {
            MemoryBlock::Rom((addr.bank as usize - 0xC0) * 0x10000 + (addr.offset as usize))
        }
    }
}

#[cfg(test)]
impl MainBus for crate::common::test_bus::TestBus<AddressU24> {
    fn consume_nmi_interrupt(&mut self) -> bool {
        false
    }

    fn consume_timer_interrupt(&mut self) -> bool {
        false
    }

    fn clock_info(&self) -> ClockInfo {
        ClockInfo::default()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use image::Rgba;
    use image::RgbaImage;

    use super::*;

    fn test_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/main_bus")
    }

    fn compare_to_golden(image: &RgbaImage, path_prefix: &Path) {
        let golden_path = path_prefix.with_extension("png");
        if golden_path.exists() {
            let golden: RgbaImage = image::open(&golden_path).unwrap().into_rgba8();
            if golden != *image {
                let actual_path = golden_path.with_extension("actual.png");
                image.save(&actual_path).unwrap();
                panic!("Image does not match golden. See {:?}", actual_path);
            }
        } else {
            image.save(golden_path).unwrap();
        }
    }

    pub fn gradient(color: [u8; 4], value: usize, max_value: usize) -> Rgba<u8> {
        let value = value.min(max_value);
        let factor = (value as f32 / max_value as f32) * 0.8 + 0.2;
        Rgba([
            (color[0] as f32 * factor) as u8,
            (color[1] as f32 * factor) as u8,
            (color[2] as f32 * factor) as u8,
            color[3],
        ])
    }

    const RED: [u8; 4] = [0xFF, 0x00, 0x00, 0xFF];
    const GREEN: [u8; 4] = [0x00, 0xFF, 0x00, 0xFF];
    const BLUE: [u8; 4] = [0x00, 0x00, 0xFF, 0xFF];
    const BLACK: [u8; 4] = [0x00, 0x00, 0x00, 0xFF];
    const GREY: [u8; 4] = [0x44, 0x44, 0x44, 0xFF];

    fn test_memory_map(memory_map: fn(AddressU24) -> MemoryBlock, path_prefix: &Path) {
        let mut image = RgbaImage::new(0xFF, 0xFF);
        for bank in 0..0xFF {
            for offset in 0..0xFF {
                let color = match memory_map(AddressU24::new(bank, offset * 0x100)) {
                    MemoryBlock::Ram(idx) => gradient(BLUE, idx, 0x20000),
                    MemoryBlock::Rom(idx) => gradient(GREEN, idx, 0x3E8000),
                    MemoryBlock::Sram(idx) => gradient(RED, idx, 0x78000),
                    MemoryBlock::Register => Rgba(GREY),
                    MemoryBlock::Unmapped => Rgba(BLACK),
                };
                image.put_pixel(bank as u32, offset as u32, color);
            }
        }
        compare_to_golden(&image, path_prefix);
    }

    #[test]
    pub fn test_lorom_memory_map_image() {
        test_memory_map(lorom_memory_map, &test_dir().join("lorom_memory_map"));
    }

    #[test]
    pub fn test_hirom_memory_map_image() {
        test_memory_map(hirom_memory_map, &test_dir().join("hirom_memory_map"));
    }

    #[test]
    pub fn test_hirom_rom_ranges() {
        // Check main ROM location
        assert_eq!(hirom_memory_map(0xC00000.into()), MemoryBlock::Rom(0x00));
        assert_eq!(
            hirom_memory_map(0xFFFFFF.into()),
            MemoryBlock::Rom(0x3FFFFF)
        );

        // Check ROM mirror at bank 0x80-0xBF
        assert_eq!(
            hirom_memory_map(0x808000.into()),
            MemoryBlock::Rom(0x008000)
        );
        assert_eq!(
            hirom_memory_map(0xBFFFFF.into()),
            MemoryBlock::Rom(0x3FFFFF)
        );

        // Check ROM mirror at bank 0x00-0x3F
        assert_eq!(
            hirom_memory_map(0x008000.into()),
            MemoryBlock::Rom(0x008000)
        );
        assert_eq!(
            hirom_memory_map(0x3FFFFF.into()),
            MemoryBlock::Rom(0x3FFFFF)
        );
    }
}
