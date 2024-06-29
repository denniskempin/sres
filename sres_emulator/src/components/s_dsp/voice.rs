use std::fmt::Display;

use bilge::prelude::*;
use intbits::Bits;

use crate::util::uint::U16Ext;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Voice {
    /// VOL (L): $X0 - SVVV VVVV - Left channel volume, signed.
    vol_l: i8,
    /// VOL (R): $X1 - SVVV VVVV - Right channel volume, signed.
    vol_r: i8,
    /// P (L)   $X2 - LLLL LLLL - Low 8 bits of sample pitch.
    /// P (H) - $X3 - --HH HHHH - High 6 bits of sample pitch.
    pitch: u16,
    /// SCRN: $X4 SSSS SSSS Selects a sample source entry from the directory
    sample_source: u8,
    /// ADSR (1): $X5 - EDDD AAAA - ADSR enable (E), decay rate (D), attack rate (A).
    adsr1: Adsr1,
    /// ADSR (2): $X6 - SSSR RRRR - Sustain level (S), release rate (R).
    adsr2: Adsr2,
    /// GAIN: $X7 - 0VVV VVVV or 1MMV VVVV - Mode (M), value (V).
    gain: Gain,
    /// ENVX: $X8 - 0VVV VVVV - Reads current 7-bit value of ADSR/GAIN envelope.
    envx: u8,
    /// OUTX - $X9 - SVVV VVVV - Reads signed 8-bit value of current sample wave multiplied by ENVX, before applying VOL.
    outx: i8,
}

impl Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vol:{}/{} pitch:{}", self.vol_l, self.vol_r, self.pitch,)?;
        if self.adsr1.enable() {
            write!(
                f,
                " adsr:({},{},{},{})",
                self.adsr1.attack_rate(),
                self.adsr1.decay_rate(),
                self.adsr2.sustain_level(),
                self.adsr2.release_rate(),
            )?;
        } else {
            write!(f, " gain:({})", self.gain.mode())?;
        }
        write!(
            f,
            " src:${:02x} env:{} out:{}",
            self.sample_source, self.envx, self.outx
        )?;
        Ok(())
    }
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
            0x7 => self.gain.0,
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
            0x7 => self.gain.0 = value,
            0x8 => self.envx = value,
            0x9 => self.outx = value as i8,
            _ => {}
        }
    }
}

#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, Default, FromBits, PartialEq)]
struct Adsr1 {
    attack_rate: u4,
    decay_rate: u3,
    enable: bool,
}

#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, Default, FromBits, PartialEq)]
struct Adsr2 {
    release_rate: u5,
    sustain_level: u3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Gain(u8);

impl Gain {
    fn mode(&self) -> GainMode {
        if self.0.bit(0) {
            let rate = self.0.bits(0..5);
            match self.0.bits(5..7) {
                0 => GainMode::LinearDecay(rate),
                1 => GainMode::ExponentialDecay(rate),
                2 => GainMode::LinearIncrease(rate),
                3 => GainMode::BentIncrease(rate),
                _ => unreachable!(),
            }
        } else {
            GainMode::Fixed(self.0.bits(0..7))
        }
    }
}

enum GainMode {
    Fixed(u8),
    LinearDecay(u8),
    ExponentialDecay(u8),
    LinearIncrease(u8),
    BentIncrease(u8),
}

impl Display for GainMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GainMode::Fixed(value) => write!(f, "fixed,{}", value),
            GainMode::LinearDecay(value) => write!(f, "lin dec,{}", value),
            GainMode::ExponentialDecay(value) => write!(f, "exp dec,{}", value),
            GainMode::LinearIncrease(value) => write!(f, "lin inc,{}", value),
            GainMode::BentIncrease(value) => write!(f, "bent inc,{}", value),
        }
    }
}
