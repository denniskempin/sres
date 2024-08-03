mod brr;
mod test;
mod voice;

use self::voice::Voice;
use crate::common::uint::U8Ext;

pub struct SDsp {
    raw: [u8; 128],
    voices: [Voice; 8],
}

impl SDsp {
    pub fn read_register(&self, reg: u8) -> u8 {
        match reg.low_nibble() {
            0x0..=0x9 => self.voices[reg.high_nibble() as usize].read_register(reg.low_nibble()),
            _ => self.raw[reg as usize],
        }
    }

    pub fn write_register(&mut self, reg: u8, value: u8) {
        match reg.low_nibble() {
            0x0..=0x9 => {
                self.voices[reg.high_nibble() as usize].write_register(reg.low_nibble(), value)
            }
            _ => self.raw[reg as usize] = value,
        }
    }

    pub fn debug(&self) -> SDspDebug<'_> {
        SDspDebug(self)
    }
}

impl Default for SDsp {
    fn default() -> Self {
        Self {
            raw: [0; 128],
            voices: [Voice::default(); 8],
        }
    }
}

pub struct SDspDebug<'a>(&'a SDsp);

impl SDspDebug<'_> {
    pub fn voice(&self, voice: usize) -> String {
        self.0.voices[voice].to_string()
    }
}
