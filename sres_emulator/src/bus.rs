use std::path::Path;

use anyhow::Result;

use crate::cartridge::Cartridge;
use crate::memory::Memory;
use crate::memory::ToAddress;

pub trait Bus: Memory {}

pub struct TestBus {
    pub memory: Vec<u8>,
}

impl TestBus {
    pub fn with_sfc(rom_path: &Path) -> Result<Self> {
        let mut bus = Self::default();
        // Load cartridge data into memory
        let mut cartridge = Cartridge::new();
        cartridge.load_sfc(rom_path)?;
        for (i, byte) in cartridge.rom.iter().enumerate() {
            bus.memory[0x8000 + i] = *byte;
        }
        Ok(bus)
    }
    
    pub fn with_sfc_data(rom: &[u8]) -> Result<Self> {
        let mut bus = Self::default();
        // Load cartridge data into memory
        let mut cartridge = Cartridge::new();
        cartridge.load_sfc_data(rom)?;
        for (i, byte) in cartridge.rom.iter().enumerate() {
            bus.memory[0x8000 + i] = *byte;
        }
        Ok(bus)
    }
    pub fn with_program(program: &[u8]) -> Self {
        let mut bus = Self::default();
        for (i, byte) in program.iter().enumerate() {
            bus.memory[i] = *byte;
        }
        bus
    }
}

impl Default for TestBus {
    fn default() -> Self {
        Self {
            memory: vec![0; 0x1000000],
        }
    }
}

impl Memory for TestBus {
    fn peek_u8(&self, addr: impl ToAddress) -> Option<u8> {
        let addr = addr.to_address();
        Some(self.memory[u32::from(addr) as usize])
    }

    fn read_u8(&mut self, addr: impl ToAddress) -> u8 {
        self.peek_u8(addr).unwrap_or(0)
    }

    fn write_u8(&mut self, addr: impl ToAddress, val: u8) {
        let addr = addr.to_address();
        self.memory[u32::from(addr) as usize] = val;
    }
}

impl Bus for TestBus {}
