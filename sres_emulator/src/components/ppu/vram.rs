//! Implementation of VRAM containing tile and tilemap data.
use bitcode::Decode;
use bitcode::Encode;
use intbits::Bits;

use crate::common::address::AddressU15;
use crate::common::uint::U16Ext;

#[derive(Encode, Decode)]
pub struct Vram {
    memory: Vec<u16>,
    current_addr: AddressU15,
    read_latch: bool,
    increment_mode: bool,
    increment_amount: u16,
}

impl Vram {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            memory: vec![0; 0x20000],
            current_addr: AddressU15::from(0),
            read_latch: false,
            increment_mode: false,
            increment_amount: 1,
        }
    }

    /// Register 2115: VMAIN - Video port control
    /// 7  bit  0
    /// ---- ----
    /// M... RRII
    /// |    ||||
    /// |    ||++- Address increment amount:
    /// |    ||     0: Increment by 1 word
    /// |    ||     1: Increment by 32 words
    /// |    ||     2: Increment by 128 words
    /// |    ||     3: Increment by 128 words
    /// |    ++--- Address remapping: (VMADD -> Internal)
    /// |           0: None
    /// |           1: Remap rrrrrrrr YYYccccc -> rrrrrrrr cccccYYY (2bpp)
    /// |           2: Remap rrrrrrrY YYcccccP -> rrrrrrrc ccccPYYY (4bpp)
    /// |           3: Remap rrrrrrYY YcccccPP -> rrrrrrcc cccPPYYY (8bpp)
    /// +--------- Address increment mode:
    ///             0: Increment after writing $2118 or reading $2139
    ///             1: Increment after writing $2119 or reading $213A
    pub fn write_vmain(&mut self, value: u8) {
        self.increment_mode = value.bit(7);
        match value.bits(0..=1) {
            0 => self.increment_amount = 1,
            1 => self.increment_amount = 32,
            2 => self.increment_amount = 128,
            3 => self.increment_amount = 128,
            _ => unreachable!(),
        }
        match value.bits(2..=3) {
            0 => (),
            1 => log::error!("Address remapping: 2bpp"),
            2 => log::error!("Address remapping: 4bpp"),
            3 => log::error!("Address remapping: 8bpp"),
            _ => unreachable!(),
        }
    }

    /// Register 2116: VMADDL - VRAM word address low
    pub fn write_vmaddl(&mut self, value: u8) {
        self.current_addr.set_low_byte(value);
        self.read_latch = true;
    }

    /// Register 2117: VMADDH - VRAM word address high
    pub fn write_vmaddh(&mut self, value: u8) {
        self.current_addr.set_high_byte(value.bits(0..=6));
        self.read_latch = true;
    }

    /// Register 2118: VMDATAL - VRAM data write low
    pub fn write_vmdatal(&mut self, value: u8) {
        self.memory[usize::from(self.current_addr)].set_low_byte(value);
        if !self.increment_mode {
            self.current_addr = self.current_addr + self.increment_amount;
        }
    }

    /// Register 2119: VMDATAH - VRAM data write high
    pub fn write_vmdatah(&mut self, value: u8) {
        self.memory[usize::from(self.current_addr)].set_high_byte(value);
        if self.increment_mode {
            self.current_addr = self.current_addr + self.increment_amount;
        }
    }

    /// Register 2139: VMDATALREAD - VRAM data read low
    pub fn read_vmdatalread(&mut self) -> u8 {
        let value = self.peek_vmdatalread();
        if !self.increment_mode {
            if self.read_latch {
                self.read_latch = false;
            } else {
                self.current_addr = self.current_addr + self.increment_amount;
            }
        }
        value
    }

    pub fn peek_vmdatalread(&self) -> u8 {
        self.memory[usize::from(self.current_addr)].low_byte()
    }

    /// Register 213A: VMDATAHREAD - VRAM data read high
    pub fn read_vmdatahread(&mut self) -> u8 {
        let value = self.peek_vmdatahread();
        if self.increment_mode {
            if self.read_latch {
                self.read_latch = false;
            } else {
                self.current_addr = self.current_addr + self.increment_amount;
            }
        }
        value
    }

    pub fn peek_vmdatahread(&self) -> u8 {
        self.memory[usize::from(self.current_addr)].high_byte()
    }
}

impl std::ops::Index<AddressU15> for Vram {
    type Output = u16;

    fn index(&self, index: AddressU15) -> &Self::Output {
        &self.memory[usize::from(index)]
    }
}
