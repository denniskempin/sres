use log::debug;

use crate::uint::U16Ext;
use crate::uint::UInt;

pub struct Vram {
    pub memory: Vec<u16>,
    pub current_addr: u16,
    pub read_latch: bool,
    pub increment_mode: bool,
}

impl Vram {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            memory: vec![0; 0x20000],
            current_addr: 0,
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
        self.current_addr.set_high_byte(value);
        self.read_latch = true;
    }

    /// Register 2118: VMDATAL - VRAM data write low
    pub fn write_vmdatal(&mut self, value: u8) {
        debug!("VRAM[{:04X}].low = {}", self.current_addr, value);
        self.memory[self.current_addr as usize].set_low_byte(value);
        if !self.increment_mode {
            self.current_addr = self.current_addr.wrapping_add(1);
        }
    }

    /// Register 2119: VMDATAH - VRAM data write high
    pub fn write_vmdatah(&mut self, value: u8) {
        debug!("VRAM[{:04X}].high = {}", self.current_addr, value);
        self.memory[self.current_addr as usize].set_high_byte(value);
        if self.increment_mode {
            self.current_addr = self.current_addr.wrapping_add(1);
        }
    }

    /// Register 2139: VMDATALREAD - VRAM data read low
    pub fn read_vmdatalread(&mut self) -> u8 {
        let value = self.peek_vmdatalread();
        if !self.increment_mode {
            if self.read_latch {
                self.read_latch = false;
            } else {
                self.current_addr = self.current_addr.wrapping_add(1);
            }
        }
        value
    }

    pub fn peek_vmdatalread(&self) -> u8 {
        self.memory[self.current_addr as usize].low_byte()
    }

    /// Register 213A: VMDATAHREAD - VRAM data read high
    pub fn read_vmdatahread(&mut self) -> u8 {
        let value = self.peek_vmdatahread();
        if self.increment_mode {
            if self.read_latch {
                self.read_latch = false;
            } else {
                self.current_addr = self.current_addr.wrapping_add(1);
            }
        }
        value
    }

    pub fn peek_vmdatahread(&self) -> u8 {
        self.memory[self.current_addr as usize].high_byte()
    }
}
