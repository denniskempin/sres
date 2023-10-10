use std::fmt::Display;
use std::fmt::Formatter;

use intbits::Bits;

use crate::util::uint::U16Ext;

pub struct Vram {
    pub memory: Vec<u16>,
    pub current_addr: VramAddr,
    pub read_latch: bool,
    pub increment_mode: bool,
}

impl Vram {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            memory: vec![0; 0x20000],
            current_addr: VramAddr::from(0),
            read_latch: false,
            increment_mode: false,
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
        self.increment_mode = value.bit(7)
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
            self.current_addr.increment();
        }
    }

    /// Register 2119: VMDATAH - VRAM data write high
    pub fn write_vmdatah(&mut self, value: u8) {
        self.memory[usize::from(self.current_addr)].set_high_byte(value);
        if self.increment_mode {
            self.current_addr.increment();
        }
    }

    /// Register 2139: VMDATALREAD - VRAM data read low
    pub fn read_vmdatalread(&mut self) -> u8 {
        let value = self.peek_vmdatalread();
        if !self.increment_mode {
            if self.read_latch {
                self.read_latch = false;
            } else {
                self.current_addr.increment();
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
                self.current_addr.increment();
            }
        }
        value
    }

    pub fn peek_vmdatahread(&self) -> u8 {
        self.memory[usize::from(self.current_addr)].high_byte()
    }
}

impl std::ops::Index<VramAddr> for Vram {
    type Output = u16;

    fn index(&self, index: VramAddr) -> &Self::Output {
        &self.memory[usize::from(index)]
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct VramAddr(pub u16);

impl VramAddr {
    pub fn set_low_byte(&mut self, value: u8) {
        self.0.set_low_byte(value);
    }

    pub fn set_high_byte(&mut self, value: u8) {
        self.0.set_high_byte(value.bits(0..=6) & 0x7F);
    }

    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1) & 0x7FFF;
    }
}

impl std::ops::Add<u16> for VramAddr {
    type Output = Self;

    fn add(self, rhs: u16) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Self(self.0.wrapping_add(rhs) & 0x7FFF)
    }
}

impl std::ops::Add<u32> for VramAddr {
    type Output = Self;

    fn add(self, rhs: u32) -> Self {
        self + rhs as u16
    }
}

impl std::ops::Sub<u16> for VramAddr {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Self(self.0.wrapping_sub(rhs) & 0x7FFF)
    }
}

impl From<u16> for VramAddr {
    fn from(value: u16) -> Self {
        Self(value & 0x7FFF)
    }
}

impl From<VramAddr> for usize {
    fn from(value: VramAddr) -> Self {
        value.0 as usize
    }
}

impl Display for VramAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:04X}", self.0)
    }
}
