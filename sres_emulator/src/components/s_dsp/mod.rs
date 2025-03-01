#![allow(clippy::single_match)]

mod brr;
mod pitch;
mod test;
mod voice;

use intbits::Bits;

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
            0xC => match reg.high_nibble() {
                0x4 => {
                    for (idx, voice) in self.voices.iter_mut().enumerate() {
                        voice.trigger_on = value.bit(idx);
                    }
                }
                _ => {}
            },
            _ => self.raw[reg as usize] = value,
        }
    }

    pub fn generate_sample(&mut self, memory: &[u8]) -> i16 {
        let dir = 0; // TODO
        self.voices
            .iter_mut()
            .map(|v| v.generate_sample(memory, dir))
            .fold(0i16, |acc, x| acc.saturating_add(x))
    }

    pub fn debug(&self) -> SDspDebug<'_> {
        SDspDebug(self)
    }
}

impl Default for SDsp {
    fn default() -> Self {
        Self {
            raw: [0; 128],
            voices: [
                Voice::default(),
                Voice::default(),
                Voice::default(),
                Voice::default(),
                Voice::default(),
                Voice::default(),
                Voice::default(),
                Voice::default(),
            ],
        }
    }
}

pub struct SDspDebug<'a>(&'a SDsp);

impl SDspDebug<'_> {
    pub fn voice(&self, voice: usize) -> String {
        self.0.voices[voice].to_string()
    }
}
