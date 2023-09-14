use std::path::Path;

use anyhow::Result;
use log::trace;
use log::warn;

use crate::apu::Apu;
use crate::cartridge::Cartridge;
use crate::debugger::DebuggerRef;
use crate::dma::DmaController;
use crate::memory::Address;
use crate::memory::Wrap;
use crate::ppu::Ppu;
use crate::uint::RegisterSize;
use crate::uint::UInt;

pub trait Bus {
    fn peek_u8(&self, addr: Address) -> Option<u8>;
    fn cycle_io(&mut self);
    fn cycle_read_u8(&mut self, addr: Address) -> u8;
    fn cycle_write_u8(&mut self, addr: Address, value: u8);
    fn reset(&mut self);
    fn check_nmi_interrupt(&mut self) -> bool;

    #[inline]
    fn cycle_read_u16(&mut self, addr: Address, wrap: Wrap) -> u16 {
        u16::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add(1_u16, wrap)),
        ])
    }

    #[inline]
    fn cycle_read_u24(&mut self, addr: Address, wrap: Wrap) -> u32 {
        u32::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add(1_u16, wrap)),
            self.cycle_read_u8(addr.add(2_u16, wrap)),
            0,
        ])
    }

    #[inline]
    fn cycle_read_generic<T: UInt>(&mut self, addr: Address, wrap: Wrap) -> T {
        match T::SIZE {
            RegisterSize::U8 => T::from_u8(self.cycle_read_u8(addr)),
            RegisterSize::U16 => T::from_u16(self.cycle_read_u16(addr, wrap)),
        }
    }

    #[inline]
    fn cycle_write_u16(&mut self, addr: Address, value: u16, wrap: Wrap) {
        let bytes = value.to_le_bytes();
        self.cycle_write_u8(addr.add(1_u16, wrap), bytes[1]);
        self.cycle_write_u8(addr, bytes[0]);
    }

    #[inline]
    fn cycle_write_generic<T: UInt>(&mut self, addr: Address, value: T, wrap: Wrap) {
        match T::SIZE {
            RegisterSize::U8 => self.cycle_write_u8(addr, value.to_u8()),
            RegisterSize::U16 => self.cycle_write_u16(addr, value.to_u16(), wrap),
        }
    }

    #[inline]
    fn peek_u16(&self, addr: Address, wrap: Wrap) -> Option<u16> {
        Some(u16::from_le_bytes([
            self.peek_u8(addr)?,
            self.peek_u8(addr.add(1_u16, wrap))?,
        ]))
    }
}

enum MemoryBlock {
    Ram(usize),
    Rom(usize),
    Register,
    Unmapped,
}

pub struct SresBus {
    pub wram: Vec<u8>,
    pub rom: Vec<u8>,
    pub clock_speed: u64,
    pub dma_controller: DmaController,
    pub ppu: Ppu,
    pub apu: Apu,
    pub debugger: DebuggerRef,
    pub nmi_enable: bool,
    pub nmi_interrupt: bool,
    pub nmi_signaled: bool,
}

impl SresBus {
    pub fn new(debugger: DebuggerRef) -> Self {
        Self {
            wram: vec![0; 0x4000000],
            rom: vec![0; 0x4000000],
            clock_speed: 8,
            dma_controller: DmaController::new(debugger.clone()),
            ppu: Ppu::new(),
            apu: Apu::new(),
            debugger,
            nmi_enable: false,
            nmi_interrupt: false,
            nmi_signaled: false,
        }
    }

    pub fn with_sfc(rom_path: &Path, debugger: DebuggerRef) -> Result<Self> {
        let mut bus = Self::new(debugger);
        // Load cartridge data into memory
        let mut cartridge = Cartridge::new();
        cartridge.load_sfc(rom_path)?;
        for (i, byte) in cartridge.rom.iter().enumerate() {
            bus.rom[i] = *byte;
        }
        Ok(bus)
    }

    pub fn with_sfc_data(rom: &[u8], debugger: DebuggerRef) -> Result<Self> {
        let mut bus = Self::new(debugger);
        // Load cartridge data into memory
        let mut cartridge = Cartridge::new();
        cartridge.load_sfc_data(rom)?;
        for (i, byte) in cartridge.rom.iter().enumerate() {
            bus.rom[i] = *byte;
        }
        Ok(bus)
    }

    pub fn with_program(program: &[u8], debugger: DebuggerRef) -> Self {
        let mut bus = Self::new(debugger);
        for (i, byte) in program.iter().enumerate() {
            bus.wram[i] = *byte;
        }
        bus
    }

    pub fn bus_peek(&self, addr: Address) -> Option<u8> {
        match self.map_memory(addr) {
            MemoryBlock::Ram(offset) => Some(self.wram[offset]),
            MemoryBlock::Rom(offset) => Some(self.rom[offset]),
            MemoryBlock::Register => match addr.offset {
                0x2100..=0x213F => self.ppu.bus_peek(addr),
                0x2140..=0x217F => self.apu.bus_peek(addr),
                0x4210 => Some(self.peek_rdnmi()),
                0x420B | 0x420C | 0x4300..=0x43FF => self.dma_controller.bus_peek(addr),
                _ => None,
            },
            MemoryBlock::Unmapped => None,
        }
    }

    pub fn bus_read(&mut self, addr: Address) -> u8 {
        self.debugger
            .on_cpu_memory_access(crate::debugger::MemoryAccess::Read(addr));
        match self.map_memory(addr) {
            MemoryBlock::Ram(offset) => self.wram[offset],
            MemoryBlock::Rom(offset) => self.rom[offset],
            MemoryBlock::Register => match addr.offset {
                0x2100..=0x213F => self.ppu.bus_read(addr),
                0x2140..=0x217F => self.apu.bus_read(addr),
                0x4210 => self.read_rdnmi(),
                0x420B | 0x420C | 0x4300..=0x43FF => self.dma_controller.bus_read(addr),
                _ => {
                    self.debugger
                        .on_error(format!("Invalid read from register {}", addr));
                    0
                }
            },
            MemoryBlock::Unmapped => {
                self.debugger
                    .on_error(format!("Invalid read from {}", addr));
                0
            }
        }
    }

    #[allow(clippy::single_match)]
    fn bus_write(&mut self, addr: Address, value: u8) {
        self.debugger
            .on_cpu_memory_access(crate::debugger::MemoryAccess::Write(addr, value));

        match self.map_memory(addr) {
            MemoryBlock::Ram(offset) => self.wram[offset] = value,
            MemoryBlock::Rom(offset) => self.rom[offset] = value,
            MemoryBlock::Register => match addr.offset {
                0x2100..=0x213F => self.ppu.bus_write(addr, value),
                0x2140..=0x217F => self.apu.bus_write(addr, value),
                0x4200 => self.write_nmitimen(value),
                0x420B | 0x420C | 0x4300..=0x43FF => self.dma_controller.bus_write(addr, value),
                _ => {
                    self.debugger
                        .on_error(format!("Invalid write to register {}", addr));
                }
            },
            MemoryBlock::Unmapped => {
                self.debugger.on_error(format!("Invalid write to {}", addr));
            }
        }
    }

    fn map_memory(&self, addr: Address) -> MemoryBlock {
        match addr.bank {
            0x00..=0x3F => match addr.offset {
                0x0000..=0x1FFF => MemoryBlock::Ram(addr.offset as usize),
                0x2000..=0x7FFF => MemoryBlock::Register,
                0x8000..=0xFFFF => {
                    MemoryBlock::Rom(addr.bank as usize * 0x8000 + (addr.offset as usize - 0x8000))
                }
            },
            0x40..=0x7D => MemoryBlock::Rom(
                0x200000 + (addr.bank as usize - 0x40) * 0x10000 + addr.offset as usize,
            ),
            0x7E..=0x7F => {
                MemoryBlock::Ram((addr.bank as usize - 0x7E) * 0x10000 + addr.offset as usize)
            }
            _ => MemoryBlock::Unmapped,
        }
    }

    /// Register $4200: NMITIMEN - NMI, Timer and IRQ Enable/Flag
    /// 7  bit  0
    /// ---- ----
    /// N.VH ...J
    /// | ||    |
    /// | ||    +- Joypad auto-read enable
    /// | ++------ H/V timer IRQ:
    /// |           00 = Disable timer
    /// |           01 = IRQ when H counter == HTIME
    /// |           10 = IRQ when V counter == VTIME and H counter == 0
    /// |           11 = IRQ when V counter == VTIME and H counter == HTIME
    /// +--------- Vblank NMI enable
    fn write_nmitimen(&mut self, value: u8) {
        self.nmi_enable = value.bit(7);
        warn!("NMITINEN = {:02X} Not fully implemented", value);
    }

    /// Register $4210: RDNMI - Read NMI Flag
    /// 7  bit  0
    /// ---- ----
    /// Nxxx VVVV
    /// |||| ||||
    /// |||| ++++- CPU version
    /// |+++------ (Open bus)
    /// +--------- Vblank flag
    fn read_rdnmi(&mut self) -> u8 {
        let value = self.peek_rdnmi();
        if self.ppu.timer.nmi_flag {
            // Fake NMI hold, do not reset nmi flag for the first 2 cyles.
            if !(self.ppu.timer.v == 225 && self.ppu.timer.h_counter <= 2) {
                self.ppu.timer.nmi_flag = false;
            }
        }
        value
    }

    fn peek_rdnmi(&self) -> u8 {
        if self.ppu.timer.nmi_flag {
            0b1111_0010
        } else {
            0b0111_0010
        }
    }

    fn advance_master_clock(&mut self, cycles: u64) {
        self.ppu.advance_master_clock(cycles);
        if self.ppu.timer.nmi_flag {
            if !self.nmi_signaled {
                self.nmi_interrupt = true;
                self.nmi_signaled = true;
            }
        } else {
            self.nmi_signaled = false;
        }

        if let Some((transfers, duration)) = self
            .dma_controller
            .pending_transfers(self.ppu.timer.master_clock, self.clock_speed)
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

impl Bus for SresBus {
    fn peek_u8(&self, addr: Address) -> Option<u8> {
        self.bus_peek(addr)
    }

    fn cycle_read_u8(&mut self, addr: Address) -> u8 {
        self.clock_speed = memory_access_speed(addr);
        trace!(target: "cycles", "cycle read {addr} ({} cycles)", self.clock_speed);
        self.ppu.advance_master_clock(self.clock_speed - 6);
        let value = self.bus_read(addr);
        self.advance_master_clock(6);
        value
    }

    #[allow(clippy::single_match)]
    fn cycle_write_u8(&mut self, addr: Address, val: u8) {
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

    fn check_nmi_interrupt(&mut self) -> bool {
        let value = self.nmi_interrupt;
        self.nmi_interrupt = false;
        value
    }
}

/// Memory access speed as per memory map. See:
/// https://wiki.superfamicom.org/memory-mapping#memory-map-67
fn memory_access_speed(addr: Address) -> u64 {
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
