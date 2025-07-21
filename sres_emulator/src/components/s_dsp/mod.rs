#![allow(clippy::single_match)]

mod brr;
mod pitch;
mod test;
pub mod voice;

use bilge::prelude::*;
use intbits::Bits;

use self::voice::Voice;
use crate::common::uint::U8Ext;

pub struct SDsp {
    raw: [u8; 128],
    voices: [Voice; 8],
    dir: u8,
    flg: Flg,
    noise_generator: NoiseGenerator,
}

impl SDsp {
    pub fn read_register(&self, reg: u8) -> u8 {
        match reg {
            0x5D => self.dir,
            0x6C => self.flg.value,
            reg => match reg.low_nibble() {
                0x0..=0x9 => {
                    self.voices[reg.high_nibble() as usize].read_register(reg.low_nibble())
                }
                _ => self.raw[reg as usize],
            },
        }
    }

    pub fn write_register(&mut self, reg: u8, value: u8) {
        match reg {
            0x5D => self.dir = value,
            0x6C => self.flg = value.into(),
            reg => match reg.low_nibble() {
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
            },
        }
    }

    pub fn generate_sample(&mut self, memory: &[u8]) -> i16 {
        // Get the current noise bits from the noise generator
        let noise_bits = self.noise_generator.generate(self.flg.noise_frequency());

        let directory_offset = (self.dir as usize) * 0x100;
        let noise_on = self.raw[0x3D]; // NON register
        self.voices
            .iter_mut()
            .enumerate()
            .map(|(i, v)| {
                let use_noise = noise_on.bit(i);
                v.generate_sample_with_noise(memory, directory_offset, use_noise, noise_bits)
            })
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
            dir: 0,
            flg: Flg::default(),
            noise_generator: NoiseGenerator::new(),
        }
    }
}

pub struct SDspDebug<'a>(&'a SDsp);

impl SDspDebug<'_> {
    pub fn voice(&self, voice: usize) -> String {
        self.0.voices[voice].to_string()
    }

    pub fn voices(&self) -> &[Voice; 8] {
        &self.0.voices
    }

    pub fn sample_directory(&self) -> u8 {
        self.0.dir
    }

    pub fn flags(&self) -> Flg {
        self.0.flg
    }

    pub fn noise_enable(&self) -> u8 {
        self.0.raw[0x3D]
    }

    pub fn key_on(&self) -> u8 {
        self.0.raw[0x4C]
    }

    pub fn key_off(&self) -> u8 {
        self.0.raw[0x5C]
    }
}

// Flg register
// 7  bit  0
// ---- ----
// RMEN NNNN
// |||| ||||
// |||+-++++- Noise frequency (N)
// ||+------- Echo disable (E)
// |+-------- Mute all (M)
// +--------- Soft reset (R)
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, Default, FromBits, PartialEq)]
pub struct Flg {
    /// Bit 7: Soft reset (R)
    pub reset: bool,
    /// Bit 6: Mute all voices (M)
    pub mute: bool,
    /// Bit 5: Echo disable (E)
    pub echo_disable: bool,
    /// Bits 0-4: Noise frequency (N)
    pub noise_frequency: u5,
}

/// Handles the SNES DSP white noise generation
struct NoiseGenerator {
    /// Current state of the noise generator shift register
    bits: u16,
    /// Counter for noise rate timing
    counter: u32,
}

impl NoiseGenerator {
    /// Creates a new noise generator with default state
    fn new() -> Self {
        Self {
            bits: 0x4000,
            counter: 0,
        }
    }

    /// Updates the noise generator state based on the given frequency setting
    /// Returns the current noise bits
    fn generate(&mut self, noise_frequency: u5) -> u16 {
        self.counter = self.counter.wrapping_add(1);
        let noise_rate_divider = NOISE_RATE_DIVIDERS[noise_frequency.value() as usize];
        // Only update noise when counter reaches the divider
        if noise_rate_divider > 0 && self.counter >= noise_rate_divider {
            let noise_bit = self.bits.bit(14) ^ self.bits.bit(13);
            self.bits = (self.bits << 1).with_bit(0, noise_bit);
            if self.bits == 0 {
                self.bits = 1;
            }
        }
        self.bits
    }
}

/// Noise frequency divider values for 32kHz sample rate
/// Calculated as 32000/frequency, rounded to nearest integer
const NOISE_RATE_DIVIDERS: [u32; 32] = [
    0,    // $00: 0 Hz (Disabled)
    2000, // $01: 16 Hz (32000/16)
    1524, // $02: 21 Hz
    1280, // $03: 25 Hz
    1032, // $04: 31 Hz
    762,  // $05: 42 Hz
    640,  // $06: 50 Hz
    508,  // $07: 63 Hz
    386,  // $08: 83 Hz
    320,  // $09: 100 Hz
    256,  // $0A: 125 Hz
    192,  // $0B: 167 Hz
    160,  // $0C: 200 Hz
    128,  // $0D: 250 Hz
    96,   // $0E: 333 Hz
    80,   // $0F: 400 Hz
    64,   // $10: 500 Hz (32000/500)
    48,   // $11: 667 Hz
    40,   // $12: 800 Hz
    32,   // $13: 1.0 kHz
    25,   // $14: 1.3 kHz
    20,   // $15: 1.6 kHz
    16,   // $16: 2.0 kHz
    12,   // $17: 2.7 kHz
    10,   // $18: 3.2 kHz
    8,    // $19: 4.0 kHz
    6,    // $1A: 5.3 kHz
    5,    // $1B: 6.4 kHz
    4,    // $1C: 8.0 kHz
    3,    // $1D: 10.7 kHz
    2,    // $1E: 16.0 kHz
    1,    // $1F: 32.0 kHz
];
