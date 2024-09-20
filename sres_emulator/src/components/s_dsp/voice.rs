#![allow(dead_code)]
use std::fmt::Display;

use bilge::prelude::*;
use intbits::Bits;

use super::brr::BrrDecoder;
use super::pitch::PitchGenerator;
use crate::common::uint::U16Ext;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Voice {
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
    pub gain: Gain,
    /// ENVX: $X8 - 0VVV VVVV - Reads current 7-bit value of ADSR/GAIN envelope.
    pub envx: u8,
    /// OUTX - $X9 - SVVV VVVV - Reads signed 8-bit value of current sample wave multiplied by ENVX, before applying VOL.
    pub outx: i8,

    brr_decoder: BrrDecoder,
    pitch_generator: PitchGenerator,
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

    pub fn dir_info(&self, memory: &[u8], dir: usize) -> (u16, u16) {
        let source_addr = dir + self.sample_source as usize * 4;
        let start_addr = u16::from_le_bytes([memory[source_addr], memory[source_addr + 1]]);
        let loop_addr = u16::from_le_bytes([memory[source_addr + 2], memory[source_addr + 3]]);
        (start_addr, loop_addr)
    }

    pub fn on(&mut self, memory: &[u8], dir: usize) {
        let (start_addr, _) = self.dir_info(memory, dir);
        self.brr_decoder.reset(start_addr as usize);
        self.pitch_generator
            .init(&mut self.brr_decoder.iter(memory));
    }

    pub fn generate_sample(&mut self, memory: &[u8], _dir: usize) -> i16 {
        // TODO: loop addr is not handled
        self.pitch_generator
            .generate_sample(self.pitch, &mut self.brr_decoder.iter(memory))
    }
}

#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, Default, FromBits, PartialEq)]
pub(crate) struct Adsr1 {
    attack_rate: u4,
    decay_rate: u3,
    enable: bool,
}

#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, Default, FromBits, PartialEq)]
pub(crate) struct Adsr2 {
    release_rate: u5,
    sustain_level: u3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Gain(pub u8);

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

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use bilge::prelude::*;

    use super::*;
    use crate::common::test_util::compare_wav_against_golden;

    #[test]
    fn play_brr_sample_test() {
        let filename = "voice_brr_sample";
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let prefix = root_dir.join(format!("src/components/s_dsp/voice/{filename}"));
        let brr_data = std::fs::read(prefix.with_extension("brr")).unwrap();

        // APU memory layout see `tests/apu_tests/play_brr_sample.spc.asm`
        // 0x0300: Sample directory with a single entry pointing to (0x0400, 0x0877)
        // 0x0400: Sample data
        let mut memory = [0_u8; 0x10000];
        memory[0x0300] = 0x00;
        memory[0x0301] = 0x04;
        memory[0x0302] = 0x77;
        memory[0x0303] = 0x08;
        memory[0x0400..0x0400 + brr_data.len()].copy_from_slice(&brr_data);

        let mut voice = Voice {
            vol_l: 127,
            vol_r: 127,
            pitch: 4096,
            sample_source: 0,
            adsr1: Adsr1::new(u4::new(10), u3::new(7), true),
            adsr2: Adsr2::new(u5::new(0), u3::new(7)),
            gain: Gain(0),
            envx: 0,
            outx: 0,
            brr_decoder: BrrDecoder::default(),
            pitch_generator: PitchGenerator::default(),
        };
        voice.on(&memory, 0x0300);

        const NUM_SAMPLES: usize = 7936; // Length of the play_brr_sample sample
        let output: Vec<i16> = (0..NUM_SAMPLES)
            .map(|_| voice.generate_sample(&memory, 0x0300))
            .collect();
        compare_wav_against_golden(&output, &prefix)
    }
}
