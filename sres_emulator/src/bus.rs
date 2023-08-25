use std::path::Path;

use anyhow::Result;
use log::trace;

use crate::cartridge::Cartridge;
use crate::debugger::DebuggerRef;
use crate::dma::DmaController;
use crate::memory::Address;
use crate::memory::Wrap;
use crate::ppu::Ppu;
use crate::timer::PpuTimer;
use crate::uint::RegisterSize;
use crate::uint::U16Ext;
use crate::uint::UInt;

pub trait Bus {
    fn peek_u8(&self, addr: Address) -> Option<u8>;
    fn cycle_io(&mut self);
    fn cycle_read_u8(&mut self, addr: Address) -> u8;
    fn cycle_write_u8(&mut self, addr: Address, value: u8);
    fn reset(&mut self);

    fn cycle_read_u16(&mut self, addr: Address, wrap: Wrap) -> u16 {
        u16::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add(1_u16, wrap)),
        ])
    }

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

    fn peek_u16(&self, addr: Address, wrap: Wrap) -> Option<u16> {
        Some(u16::from_le_bytes([
            self.peek_u8(addr)?,
            self.peek_u8(addr.add(1_u16, wrap))?,
        ]))
    }
}

pub struct SresBus {
    pub memory: Vec<u8>,
    pub ppu_timer: PpuTimer,
    pub clock_speed: u64,
    pub dma_controller: DmaController,
    pub ppu: Ppu,
    pub debugger: DebuggerRef,
}

impl SresBus {
    pub fn new(debugger: DebuggerRef) -> Self {
        Self {
            memory: vec![0; 0x1000000],
            ppu_timer: PpuTimer::default(),
            clock_speed: 8,
            dma_controller: DmaController::default(),
            ppu: Ppu::new(),
            debugger,
        }
    }

    pub fn with_sfc(rom_path: &Path, debugger: DebuggerRef) -> Result<Self> {
        let mut bus = Self::new(debugger);
        // Load cartridge data into memory
        let mut cartridge = Cartridge::new();
        cartridge.load_sfc(rom_path)?;
        for (i, byte) in cartridge.rom.iter().enumerate() {
            bus.memory[0x8000 + i] = *byte;
        }
        Ok(bus)
    }

    pub fn with_sfc_data(rom: &[u8], debugger: DebuggerRef) -> Result<Self> {
        let mut bus = Self::new(debugger);
        // Load cartridge data into memory
        let mut cartridge = Cartridge::new();
        cartridge.load_sfc_data(rom)?;
        for (i, byte) in cartridge.rom.iter().enumerate() {
            bus.memory[0x8000 + i] = *byte;
        }
        Ok(bus)
    }

    pub fn with_program(program: &[u8], debugger: DebuggerRef) -> Self {
        let mut bus = Self::new(debugger);
        for (i, byte) in program.iter().enumerate() {
            bus.memory[i] = *byte;
        }
        bus
    }

    fn read_u8(&mut self, addr: Address) -> u8 {
        self.debugger
            .on_cpu_memory_access(crate::debugger::MemoryAccess::Read(addr));
        match u32::from(addr) {
            0x004210 => {
                let override_value = self.peek_u8(addr).unwrap_or(0);
                if override_value > 0 {
                    return override_value;
                }
                if self.ppu_timer.nmi_flag {
                    // Fake NMI hold, do not reset nmi flag for the first 2 cyles.
                    if !(self.ppu_timer.v == 225 && self.ppu_timer.h_counter <= 2) {
                        self.ppu_timer.nmi_flag = false;
                    }
                    0b1111_0010
                } else {
                    0b0111_0010
                }
            }
            0x002100..=0x00213F => self.ppu.read_ppu_register(addr.offset.low_byte()),
            _ => {
                if let Some(value) = self.peek_u8(addr) {
                    value
                } else {
                    self.debugger
                        .on_error(format!("Invalid read from {}", addr));
                    0
                }
            }
        }
    }

    #[allow(clippy::single_match)]
    fn write_u8(&mut self, addr: Address, val: u8) {
        self.debugger
            .on_cpu_memory_access(crate::debugger::MemoryAccess::Write(addr, val));
        match addr.bank {
            0x00..=0x1F => match addr.offset {
                0x2100..=0x213F => self.ppu.write_ppu_register(addr.offset.low_byte(), val),
                0x420B => self.dma_controller.write_420b_dma_enable(val),
                0x4300..=0x43FF => self
                    .dma_controller
                    .write_43xx_parameter(addr.offset.low_byte(), val),
                _ => self.memory[u32::from(addr) as usize] = val,
            },
            _ => {
                self.debugger.on_error(format!("Invalid write to {}", addr));
            }
        }
    }

    fn advance_master_clock(&mut self, cycles: u64) {
        self.ppu_timer.advance_master_clock(cycles);

        if let Some((transfers, duration)) = self
            .dma_controller
            .pending_transfers(self.ppu_timer.master_clock, self.clock_speed)
        {
            self.ppu_timer.advance_master_clock(duration);
            for (source, destination) in transfers {
                let value = self.read_u8(source);
                self.write_u8(destination, value);
            }
        }
        self.dma_controller.update_state();
    }
}

impl Bus for SresBus {
    fn peek_u8(&self, addr: Address) -> Option<u8> {
        Some(self.memory[u32::from(addr) as usize])
    }

    fn cycle_read_u8(&mut self, addr: Address) -> u8 {
        self.clock_speed = memory_access_speed(addr);
        trace!(target: "cycles", "cycle read {addr} ({} cycles)", self.clock_speed);
        self.ppu_timer.advance_master_clock(self.clock_speed - 6);
        let value = self.read_u8(addr);
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

        self.write_u8(addr, val);
    }

    fn cycle_io(&mut self) {
        trace!(target: "cycles", "cycle io (6 cycles)");
        self.clock_speed = 6;
        self.advance_master_clock(self.clock_speed);
    }

    fn reset(&mut self) {
        self.ppu_timer = PpuTimer::default();
        self.advance_master_clock(186);
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
