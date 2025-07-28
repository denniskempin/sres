//! Dummy implementation of the audio processing unit.
mod apu_bus;
mod test;

use log::debug;
use log::error;

use self::apu_bus::ApuBus;
pub use self::apu_bus::ApuBusEvent;
use crate::common::address::AddressU24;
use crate::common::bus::BusDeviceU24;
use crate::common::clock::ClockInfo;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::components::s_dsp::SDspDebug;
use crate::components::spc700::Spc700;
use crate::debugger::DebuggerRef;

// SNES APU sample rate is 32kHz
pub const APU_SAMPLE_RATE: u32 = 32000;
// Master clock frequency is ~21.47 MHz
pub const MASTER_CLOCK_FREQUENCY: u64 = 21477272;
// Number of master clock cycles per audio sample
pub const CYCLES_PER_SAMPLE: u64 = MASTER_CLOCK_FREQUENCY / APU_SAMPLE_RATE as u64;

// Maximum audio buffer size (about 1 second at 32kHz)
pub const MAX_AUDIO_BUFFER_SIZE: usize = 32000;

// Roughly 2 frames worth of audio samples, so we should rarely exceed this.
pub const AUDIO_BUFFER_CAPACITY: usize = 1024;

pub struct Apu {
    pub spc700: Spc700<ApuBus>,
    /// Audio sample buffer that grows as samples are generated
    sample_buffer: AudioBuffer,
    /// Last master clock cycle when a sample was generated
    last_sample_cycle: u64,
}

impl Apu {
    #[allow(clippy::new_without_default)]
    pub fn new(debugger: DebuggerRef) -> Self {
        Self {
            spc700: Spc700::new(
                ApuBus::new(DebugEventCollectorRef(debugger.clone())),
                DebugEventCollectorRef(debugger.clone()),
            ),
            sample_buffer: AudioBuffer::new(),
            last_sample_cycle: 0,
        }
    }

    pub fn debug(&self) -> ApuDebug<'_> {
        ApuDebug(self)
    }

    /// Swap the current audio sample buffer with a provided buffer
    /// This avoids copying samples by exchanging buffers directly
    pub fn swap_audio_buffer(&mut self, buffer: &mut AudioBuffer) {
        self.sample_buffer.swap(buffer);
    }

    pub fn sample_buffer_size(&self) -> usize {
        self.sample_buffer.len()
    }

    // Generate a single audio sample
    pub fn generate_sample(&mut self) -> i16 {
        let memory = &self.spc700.bus.ram;
        self.spc700.bus.dsp.generate_sample(memory)
    }

    /// Register 2140..2144: APUION - APU IO Channels
    fn write_apuio(&mut self, addr: AddressU24, value: u8) {
        let channel_id = (addr.offset - 0x2140) as usize % 4;
        self.spc700.bus.channel_in[channel_id] = value;
        debug!("APUIO[{:04X}] = {:02X}", addr.offset, value);
    }

    fn peek_apuio(&self, addr: AddressU24) -> u8 {
        let channel_id = (addr.offset - 0x2140) as usize % 4;
        self.spc700.bus.channel_out[channel_id]
    }

    fn read_apuio(&mut self, addr: AddressU24) -> u8 {
        let channel_id = (addr.offset - 0x2140) as usize % 4;
        let value = self.spc700.bus.channel_out[channel_id];
        debug!("APUIO[{:04X}] reads {:02X}", addr.offset, value);
        value
    }

    fn update_clock(&mut self, new_clock: ClockInfo) {
        while new_clock.master_clock - self.last_sample_cycle >= CYCLES_PER_SAMPLE {
            self.last_sample_cycle += CYCLES_PER_SAMPLE;
            self.spc700.catch_up_to_master_clock(new_clock.master_clock);

            let sample = self.generate_sample();

            // Add sample to buffer, dropping oldest samples if buffer gets too large
            if self.sample_buffer.len() >= MAX_AUDIO_BUFFER_SIZE {
                error!("APU audio buffer overflow - dropping samples");
                self.sample_buffer.clear();
            }
            self.sample_buffer.push_sample(sample);
        }
        self.spc700.catch_up_to_master_clock(new_clock.master_clock);
    }

    fn reset(&mut self) {
        self.last_sample_cycle = 0;
        self.sample_buffer.clear();
    }
}

impl BusDeviceU24 for Apu {
    const NAME: &'static str = "APU";

    fn read(&mut self, addr: AddressU24) -> u8 {
        self.read_apuio(addr)
    }

    fn peek(&self, addr: AddressU24) -> Option<u8> {
        Some(self.peek_apuio(addr))
    }

    fn write(&mut self, addr: AddressU24, value: u8) {
        self.write_apuio(addr, value)
    }

    fn update_clock(&mut self, new_clock: ClockInfo) {
        self.update_clock(new_clock)
    }

    fn reset(&mut self) {
        self.reset()
    }
}

pub struct ApuDebug<'a>(&'a Apu);

impl<'a> ApuDebug<'a> {
    pub fn dsp(&'a self) -> SDspDebug<'a> {
        self.0.spc700.bus.dsp.debug()
    }

    pub fn ram(&self) -> &[u8] {
        &self.0.spc700.bus.ram
    }
}

/// A typed wrapper around Vec<i16> for audio samples with proper capacity management
#[derive(Default)]
pub struct AudioBuffer {
    samples: Vec<i16>,
}

impl AudioBuffer {
    /// Create a new AudioBuffer with the default capacity
    pub fn new() -> Self {
        Self {
            samples: Vec::with_capacity(AUDIO_BUFFER_CAPACITY),
        }
    }

    /// Add a single audio sample to the buffer
    pub fn push_sample(&mut self, sample: i16) {
        self.samples.push(sample);
    }

    /// Get the number of samples in the buffer
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// Clear all samples from the buffer, keeping capacity
    pub fn clear(&mut self) {
        self.samples.clear();
    }

    /// Swap contents with another AudioBuffer
    pub fn swap(&mut self, other: &mut AudioBuffer) {
        std::mem::swap(&mut self.samples, &mut other.samples);
    }

    pub fn into_vec(self) -> Vec<i16> {
        self.samples
    }
}

impl std::ops::Index<usize> for AudioBuffer {
    type Output = i16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.samples[index]
    }
}
