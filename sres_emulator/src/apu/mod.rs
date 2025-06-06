//! Dummy implementation of the audio processing unit.
mod apu_bus;
mod test;

use std::collections::VecDeque;

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

// Buffer size for audio samples (about 1/4 second at 32kHz)
pub const AUDIO_BUFFER_SIZE: usize = 8192;
// Maximum number of buffers that can be queued (about 1 second of audio)
pub const MAX_BUFFER_QUEUE_SIZE: usize = 32;

/// A queue of audio sample buffers
pub struct BufferQueue<T> {
    buffers: VecDeque<T>,
    /// Whether the buffer queue has overflowed
    overflow: bool,
}

impl<T> Default for BufferQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BufferQueue<T> {
    pub fn new() -> Self {
        Self {
            buffers: VecDeque::new(),
            overflow: false,
        }
    }

    pub fn push(&mut self, buffer: T) -> bool {
        if self.buffers.len() < MAX_BUFFER_QUEUE_SIZE {
            self.buffers.push_back(buffer);
            self.overflow = false;
            true
        } else {
            if !self.overflow {
                self.overflow = true;
                error!("APU audio buffer queue overflow - dropping samples");
            }
            false
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.overflow = false;
        self.buffers.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.buffers.is_empty()
    }

    pub fn len(&self) -> usize {
        self.buffers.len()
    }

    pub fn clear(&mut self) {
        self.buffers.clear();
        self.overflow = false;
    }
}

pub struct Apu {
    pub spc700: Spc700<ApuBus>,
    /// Current buffer being filled with audio samples
    sample_buffer: Vec<i16>,
    /// Queue of completed audio sample buffers
    buffer_queue: BufferQueue<Vec<i16>>,
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
            sample_buffer: Vec::with_capacity(AUDIO_BUFFER_SIZE),
            buffer_queue: BufferQueue::new(),
            last_sample_cycle: 0,
        }
    }

    pub fn debug(&self) -> ApuDebug<'_> {
        ApuDebug(self)
    }

    /// Take the next available audio sample buffer from the queue
    pub fn take_pending_audio_buffer(&mut self) -> Option<Vec<i16>> {
        self.buffer_queue.pop()
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
            if self.sample_buffer.len() < AUDIO_BUFFER_SIZE {
                self.sample_buffer.push(sample);
            } else {
                // Buffer is full, create a new one and enqueue the full buffer
                let mut new_buffer = Vec::with_capacity(AUDIO_BUFFER_SIZE);
                std::mem::swap(&mut self.sample_buffer, &mut new_buffer);

                // Try to push the buffer to the queue
                self.buffer_queue.push(new_buffer);

                // Add the new sample to the fresh buffer
                self.sample_buffer.push(sample);
            }
        }
        self.spc700.catch_up_to_master_clock(new_clock.master_clock);
    }

    fn reset(&mut self) {
        self.last_sample_cycle = 0;
        self.sample_buffer = Vec::with_capacity(AUDIO_BUFFER_SIZE);
        self.buffer_queue.clear();
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
    pub fn dsp(self) -> SDspDebug<'a> {
        self.0.spc700.bus.dsp.debug()
    }

    pub fn ram(&self) -> &[u8] {
        &self.0.spc700.bus.ram
    }
}
