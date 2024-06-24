use bilge::prelude::*;

use crate::util::uint::U16Ext;
use crate::util::uint::U8Ext;

pub struct SDsp {
    pub raw: [u8; 128],
    pub voices: [Voice; 8],
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
}

impl Default for SDsp {
    fn default() -> Self {
        Self {
            raw: [0; 128],
            voices: [Voice::default(); 8],
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Voice {
    /// VOL (L): $X0 - SVVV VVVV - Left channel volume, signed.
    pub vol_l: i8,
    /// VOL (R): $X1 - SVVV VVVV - Right channel volume, signed.
    pub vol_r: i8,
    /// P (L)   $X2 - LLLL LLLL - Low 8 bits of sample pitch.
    /// P (H) - $X3 - --HH HHHH - High 6 bits of sample pitch.
    pub pitch: u16,
    /// SCRN: $X4 SSSS SSSS Selects a sample source entry from the directory
    pub sample_source: u8,
    /// ADSR (1): $X5 - EDDD AAAA - ADSR enable (E), decay rate (D), attack rate (A).
    pub adsr1: Adsr1,
    /// ADSR (2): $X6 - SSSR RRRR - Sustain level (S), release rate (R).
    pub adsr2: Adsr2,
    /// GAIN: $X7 - 0VVV VVVV or 1MMV VVVV - Mode (M), value (V).
    pub gain: u8,
    /// ENVX: $X8 - 0VVV VVVV - Reads current 7-bit value of ADSR/GAIN envelope.
    pub envx: u8,
    /// OUTX - $X9 - SVVV VVVV - Reads signed 8-bit value of current sample wave multiplied by ENVX, before applying VOL.
    pub outx: i8,
}

#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, Default, FromBits, PartialEq)]
pub struct Adsr1 {
    pub attack_rate: u4,
    pub decay_rate: u3,
    pub enable: bool,
}

#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, Default, FromBits, PartialEq)]
pub struct Adsr2 {
    pub release_rate: u5,
    pub sustain_level: u3,
}

impl Voice {
    pub fn read_register(&self, reg: u8) -> u8 {
        match reg {
            0x0 => self.vol_l as u8,
            0x1 => self.vol_r as u8,
            0x2 => self.pitch.low_byte(),
            0x3 => self.pitch.high_byte(),
            0x4 => self.sample_source,
            0x5 => self.adsr1.value,
            0x6 => self.adsr2.value,
            0x7 => self.gain,
            0x8 => self.envx,
            0x9 => self.outx as u8,
            _ => 0,
        }
    }

    pub fn write_register(&mut self, reg: u8, value: u8) {
        match reg {
            0x0 => self.vol_l = value as i8,
            0x1 => self.vol_r = value as i8,
            0x2 => self.pitch = self.pitch.with_low_byte(value),
            0x3 => self.pitch = self.pitch.with_high_byte(value),
            0x4 => self.sample_source = value,
            0x5 => self.adsr1 = value.into(),
            0x6 => self.adsr2 = value.into(),
            0x7 => self.gain = value,
            0x8 => self.envx = value,
            0x9 => self.outx = value as i8,
            _ => {}
        }
    }
}
