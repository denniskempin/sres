//! Implementation of the CGRAM containing color palette data.
use bitcode::Decode;
use bitcode::Encode;

use crate::common::image::Rgb15;
use crate::common::uint::U16Ext;

#[derive(Encode, Decode)]
pub struct CgRam {
    /// Contains the contents of CGRAM translated into RGBA values for more efficient rendering.
    pub memory: Vec<Rgb15>,
    /// Contains the currently selected CGRAM address set via the CGADD register.
    current_addr: u8,
    /// Represents the write latch. Contains the previous written value or None if the latch is
    /// not set.
    latch: Option<u8>,
}

impl CgRam {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            memory: vec![Rgb15::default(); 0x100],
            current_addr: 0,
            latch: None,
        }
    }
    /// Register 2121: CGADD - CGRAM address
    /// 7  bit  0
    /// ---- ----
    /// AAAA AAAA
    /// |||| ||||
    /// ++++-++++- CGRAM word address
    ///
    /// On write: cgram_byte = 0
    pub fn write_cgadd(&mut self, value: u8) {
        self.current_addr = value;
        self.latch = None;
    }

    /// Register 2122: CGDATA - CGRAM data write
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  .BBB BBGG   GGGR RRRR
    ///   ||| ||||   |||| ||||
    ///   ||| ||||   |||+-++++- Red component
    ///   ||| ||++---+++------- Green component
    ///   +++-++--------------- Blue component
    ///
    /// On write: If cgram_byte == 0, cgram_latch = value
    ///           If cgram_byte == 1, CGDATA = (value << 8) | cgram_latch
    ///           cgram_byte = ~cgram_byte
    ///
    pub fn write_cgdata(&mut self, value: u8) {
        match self.latch {
            None => {
                self.latch = Some(value);
            }
            Some(low_byte) => {
                self.memory[self.current_addr as usize] =
                    Rgb15(u16::from_le_bytes([low_byte, value]));
                self.latch = None;
                self.current_addr = self.current_addr.wrapping_add(1);
            }
        }
    }

    /// Register 213B - CGDATAREAD - CGRAM data read
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  xBBB BBGG   GGGR RRRR
    ///  |||| ||||   |||| ||||
    ///  |||| ||||   |||+-++++- Red component
    ///  |||| ||++---+++------- Green component
    ///  |+++-++-------------- Blue component
    ///  +-------------------- PPU2 open bus
    ///
    /// On read: If cgram_byte == 0, value = CGDATA.low
    ///          If cgram_byte == 1, value = CGDATA.high
    ///          cgram_byte = ~cgram_byte
    pub fn read_cgdataread(&mut self) -> u8 {
        match self.latch {
            None => {
                let value = self.memory[self.current_addr as usize].0;
                self.latch = Some(value.high_byte());
                value.low_byte()
            }
            Some(high_byte) => {
                self.latch = None;
                self.current_addr = self.current_addr.wrapping_add(1);
                high_byte
            }
        }
    }

    pub fn peek_cgdataread(&self) -> u8 {
        match self.latch {
            None => self.memory[self.current_addr as usize].0.low_byte(),
            Some(high_byte) => high_byte,
        }
    }
}

impl std::ops::Index<u8> for CgRam {
    type Output = Rgb15;

    fn index(&self, index: u8) -> &Rgb15 {
        &self.memory[index as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_cgdata() {
        let mut cgram = CgRam::new();
        cgram.write_cgadd(0x42);
        cgram.write_cgdata(0x03);
        cgram.write_cgdata(0xE0);
        assert_eq!(cgram.memory[0x42], Rgb15(0xE003));
    }

    #[test]
    fn test_read_cgdataread() {
        let mut cgram = CgRam::new();
        cgram.memory[0x42] = Rgb15(0xE003);
        cgram.write_cgadd(0x42);
        assert_eq!(cgram.read_cgdataread(), 0x03);
        assert_eq!(cgram.read_cgdataread(), 0xE0);
    }
}
