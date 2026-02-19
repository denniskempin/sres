#![allow(dead_code)]
use std::fmt::Display;

use bilge::prelude::*;
use intbits::Bits;

use super::brr::BrrDecoder;
use super::pitch::PitchGenerator;
use crate::common::uint::U16Ext;

pub const OUTX_BUFFER_SIZE: usize = 128;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum EnvelopeState {
    #[default]
    Attack,
    Decay,
    Sustain,
    Release,
}

/// DSP Envelope processor following SNES DSP envelope specifications
#[derive(Clone, Debug, PartialEq)]
pub struct DspEnvelope {
    /// Internal 16-bit envelope value (0-2047)
    value: u16,
    /// Current envelope state
    state: EnvelopeState,
    /// Counter for rate timing
    rate_counter: u8,
}

impl Default for DspEnvelope {
    fn default() -> Self {
        Self {
            value: 0,
            state: EnvelopeState::Attack,
            rate_counter: 0,
        }
    }
}

impl DspEnvelope {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the current envelope value (0-2047)
    pub fn value(&self) -> u16 {
        self.value
    }

    /// Get the 7-bit ENVX register value
    pub fn envx(&self) -> u8 {
        (self.value >> 4) as u8
    }

    /// Get current envelope state
    pub fn state(&self) -> EnvelopeState {
        self.state
    }

    /// Trigger key-on (start attack phase)
    pub fn key_on(&mut self) {
        self.value = 0;
        self.state = EnvelopeState::Attack;
        self.rate_counter = 0;
    }

    /// Trigger key-off (start release phase)
    pub fn key_off(&mut self) {
        self.state = EnvelopeState::Release;
    }

    /// Update envelope for one sample period
    pub fn update(&mut self, global_counter: u16, adsr1: Adsr1, adsr2: Adsr2, gain: Gain) {
        if adsr1.enable() {
            self.update_adsr(global_counter, adsr1, adsr2);
        } else {
            self.update_gain(global_counter, gain);
        }
    }

    fn update_adsr(&mut self, global_counter: u16, adsr1: Adsr1, adsr2: Adsr2) {
        match self.state {
            EnvelopeState::Attack => {
                let attack_rate = adsr1.attack_rate().value();
                let rate = if attack_rate == 15 {
                    31 // Special case: rate 31 for max attack
                } else {
                    (attack_rate * 2) + 1
                };

                if self.should_update_at_rate(global_counter, rate) {
                    if attack_rate == 15 {
                        // Linear increase +1024 at rate 31
                        self.value = self.value.saturating_add(1024);
                    } else {
                        // Linear increase +32
                        self.value = self.value.saturating_add(32);
                    }

                    if self.value >= 0x7E0 {
                        self.value = 0x7FF;
                        self.state = EnvelopeState::Decay;
                    }
                }
            }
            EnvelopeState::Decay => {
                let decay_rate = adsr1.decay_rate().value();
                let rate = (decay_rate * 2) + 16;

                if self.should_update_at_rate(global_counter, rate) {
                    // Exponential decrease: envelope -= 1, then envelope -= envelope >> 8
                    if self.value > 0 {
                        self.value -= 1;
                        self.value -= self.value >> 8;
                    }

                    // Check if we've reached sustain level
                    let sustain_level = (adsr2.sustain_level().value() as u16 + 1) << 8;
                    if self.value <= sustain_level {
                        self.state = EnvelopeState::Sustain;
                    }
                }
            }
            EnvelopeState::Sustain => {
                let sustain_rate = adsr2.release_rate().value();
                if sustain_rate > 0 && self.should_update_at_rate(global_counter, sustain_rate) {
                    // Exponential decrease: envelope -= 1, then envelope -= envelope >> 8
                    if self.value > 0 {
                        self.value -= 1;
                        self.value -= self.value >> 8;
                    }
                }
            }
            EnvelopeState::Release => {
                // Linear decrease at fixed rate of -8 every sample
                self.value = self.value.saturating_sub(8);
            }
        }
    }

    fn update_gain(&mut self, global_counter: u16, gain: Gain) {
        if self.state == EnvelopeState::Release {
            // Release always uses linear decrease -8 every sample
            self.value = self.value.saturating_sub(8);
            return;
        }

        match gain.mode() {
            GainMode::Fixed(value) => {
                // Fixed envelope: envelope = value << 4
                self.value = (value as u16) << 4;
            }
            GainMode::LinearDecay(rate) => {
                if self.should_update_at_rate(global_counter, rate) {
                    // Linear decrease: -32
                    self.value = self.value.saturating_sub(32);
                }
            }
            GainMode::ExponentialDecay(rate) => {
                if self.should_update_at_rate(global_counter, rate) {
                    // Exponential decrease: envelope -= 1, then envelope -= envelope >> 8
                    if self.value > 0 {
                        self.value -= 1;
                        self.value -= self.value >> 8;
                    }
                }
            }
            GainMode::LinearIncrease(rate) => {
                if self.should_update_at_rate(global_counter, rate) {
                    // Linear increase: +32
                    self.value = self.value.saturating_add(32);
                    if self.value > 0x7FF {
                        self.value = 0x7FF;
                    }
                }
            }
            GainMode::BentIncrease(rate) => {
                if self.should_update_at_rate(global_counter, rate) {
                    // Bent increase: +32 if below 0x600, +8 if above
                    let increase = if self.value < 0x600 { 32 } else { 8 };
                    self.value = self.value.saturating_add(increase);
                    if self.value > 0x7FF {
                        self.value = 0x7FF;
                    }
                }
            }
        }
    }

    fn should_update_at_rate(&self, global_counter: u16, rate: u8) -> bool {
        if rate == 0 || rate >= 32 {
            return rate == 31; // Special case for max attack rate
        }

        let period = DSP_PERIOD_TABLE[rate as usize];
        let offset = DSP_OFFSET_TABLE[rate as usize];

        if period == 0 {
            return false; // Infinite period
        }

        (global_counter.wrapping_add(offset)) % period == 0
    }
}

/// DSP Period Table - how many S-SMP clocks elapse per envelope operation
const DSP_PERIOD_TABLE: [u16; 32] = [
    0, // Rate 0: Infinite
    2048, 1536, 1280, 1024, 768, 640, 512, 384, 320, 256, 192, 160, 128, 96, 80, 64, 48, 40, 32,
    24, 20, 16, 12, 10, 8, 6, 5, 4, 3, 2, 1,
];

/// DSP Offset Table - delay offset applied to each rate
const DSP_OFFSET_TABLE: [u16; 32] = [
    0, // Rate 0: Never
    0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0, 1040,
    536, 0, 1040, 536, 0, 1040, 536, 0, 1040, 536, 0,
];

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Voice {
    /// VOL (L): $X0 - SVVV VVVV - Left channel volume, signed.argo c
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

    pub trigger_on: bool,
    pub trigger_off: bool,

    /// DSP envelope processor
    pub envelope: DspEnvelope,

    /// Buffer of recent samples / envelope values for debugging purposes
    pub envx_buffer: AudioRingBuffer<OUTX_BUFFER_SIZE>,
    pub outx_buffer: AudioRingBuffer<OUTX_BUFFER_SIZE>,

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

    pub fn generate_sample_with_noise(
        &mut self,
        memory: &[u8],
        dir: usize,
        use_noise: bool,
        noise_bits: u16,
        global_counter: u16,
    ) -> i16 {
        if self.trigger_on {
            let (start_addr, loop_addr) = self.dir_info(memory, dir);
            self.brr_decoder.reset(start_addr as usize);
            self.brr_decoder.set_loop_addr(loop_addr as usize);
            self.pitch_generator
                .init(&mut self.brr_decoder.iter(memory));
            self.envelope.key_on();
            self.trigger_on = false;
        }

        if self.trigger_off {
            self.envelope.key_off();
            self.trigger_off = false;
        }

        // Update envelope
        self.envelope
            .update(global_counter, self.adsr1, self.adsr2, self.gain);
        self.envx = self.envelope.envx();

        let sample = if use_noise {
            // Use bit 0 of noise_bits as the noise sample
            if noise_bits & 1 == 0 {
                -0x4000
            } else {
                0x4000
            }
        } else {
            self.pitch_generator
                .generate_sample(self.pitch, &mut self.brr_decoder.iter(memory))
        };

        // Apply envelope to sample
        let enveloped_sample = ((sample as i32) * (self.envelope.value() as i32)) >> 11;
        self.outx = (enveloped_sample >> 8) as i8;
        self.outx_buffer.push(sample);

        // Apply volume
        let left = (enveloped_sample * (self.vol_l as i32)) >> 7;
        let right = (enveloped_sample * (self.vol_r as i32)) >> 7;
        (left + right) as i16
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioRingBuffer<const N: usize> {
    samples: [i16; N],
    head: usize,
}

impl<const N: usize> AudioRingBuffer<N> {
    pub fn push(&mut self, sample: i16) {
        self.samples[self.head] = sample;
        self.head = (self.head + 1) % N;
    }

    pub fn iter(&self) -> impl Iterator<Item = &i16> {
        self.samples.iter()
    }
}

impl<const N: usize> Default for AudioRingBuffer<N> {
    fn default() -> Self {
        Self {
            samples: [0; N],
            head: 0,
        }
    }
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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Gain(pub u8);

impl Gain {
    pub fn mode(&self) -> GainMode {
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

pub enum GainMode {
    Fixed(u8),
    LinearDecay(u8),
    ExponentialDecay(u8),
    LinearIncrease(u8),
    BentIncrease(u8),
}

impl Display for GainMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GainMode::Fixed(value) => write!(f, "fixed,{value}"),
            GainMode::LinearDecay(value) => write!(f, "lin dec,{value}"),
            GainMode::ExponentialDecay(value) => write!(f, "exp dec,{value}"),
            GainMode::LinearIncrease(value) => write!(f, "lin inc,{value}"),
            GainMode::BentIncrease(value) => write!(f, "bent inc,{value}"),
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
        // 0x0300: Sample directory with a single entry pointing to 0x0400
        // 0x0400: Sample data
        let mut memory = [0_u8; 0x10000];
        memory[0x0300] = 0x00;
        memory[0x0301] = 0x04;
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
            trigger_on: true,
            trigger_off: false,
            envelope: DspEnvelope::new(),
            outx_buffer: AudioRingBuffer::default(),
            envx_buffer: AudioRingBuffer::default(),
            brr_decoder: BrrDecoder::default(),
            pitch_generator: PitchGenerator::default(),
        };

        const NUM_SAMPLES: usize = 7936; // Length of the play_brr_sample sample
        let output: Vec<i16> = (0..NUM_SAMPLES)
            .map(|i| voice.generate_sample_with_noise(&memory, 0x0300, false, 0, i as u16))
            .collect();
        compare_wav_against_golden(&output, &prefix)
    }
}
