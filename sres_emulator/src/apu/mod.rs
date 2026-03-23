//! Dummy implementation of the audio processing unit.
mod apu_bus;
mod test;
mod timers;

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

/// A single APUIO port access (CPU read or write of $2140-$2143) captured with timing.
#[derive(Clone, Debug)]
pub struct ApuioAccess {
    /// Main CPU master clock at the time of the access.
    pub cpu_master_clock: u64,
    /// SPC700 internal master-cycle counter at the time of the access.
    pub spc700_master_cycle: u64,
    /// Port index 0-3 (maps to $2140-$2143 / $F4-$F7).
    pub port: u8,
    /// Value read or written.
    pub value: u8,
    /// `true` = CPU wrote this value into channel_in; `false` = CPU read channel_out.
    pub is_write: bool,
}

pub struct Apu {
    pub spc700: Spc700<ApuBus>,
    /// Audio sample buffer that grows as samples are generated
    sample_buffer: AudioBuffer,
    /// Last master clock cycle when a sample was generated
    last_sample_cycle: u64,
    /// Current CPU master clock, updated on every update_clock() call.
    current_master_clock: u64,
    /// Rolling log of recent APUIO accesses (up to MAX_APUIO_LOG_SIZE entries).
    pub apuio_log: Vec<ApuioAccess>,
}

/// Maximum number of APUIO accesses retained in the log.
pub const MAX_APUIO_LOG_SIZE: usize = 4096;

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
            current_master_clock: 0,
            apuio_log: Vec::new(),
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
        debug!(
            "APUIO[{:04X}] = {:02X}  (cpu_clk={} spc_cycle={})",
            addr.offset, value, self.current_master_clock, self.spc700.bus.master_cycle
        );
        self.push_apuio_access(channel_id as u8, value, true);
    }

    fn peek_apuio(&self, addr: AddressU24) -> u8 {
        let channel_id = (addr.offset - 0x2140) as usize % 4;
        self.spc700.bus.channel_out[channel_id]
    }

    fn read_apuio(&mut self, addr: AddressU24) -> u8 {
        let channel_id = (addr.offset - 0x2140) as usize % 4;
        let value = self.spc700.bus.channel_out[channel_id];
        debug!(
            "APUIO[{:04X}] reads {:02X}  (cpu_clk={} spc_cycle={})",
            addr.offset, value, self.current_master_clock, self.spc700.bus.master_cycle
        );
        self.push_apuio_access(channel_id as u8, value, false);
        value
    }

    fn push_apuio_access(&mut self, port: u8, value: u8, is_write: bool) {
        if self.apuio_log.len() >= MAX_APUIO_LOG_SIZE {
            self.apuio_log.remove(0);
        }
        self.apuio_log.push(ApuioAccess {
            cpu_master_clock: self.current_master_clock,
            spc700_master_cycle: self.spc700.bus.master_cycle,
            port,
            value,
            is_write,
        });
    }

    fn update_clock(&mut self, new_clock: ClockInfo) {
        self.current_master_clock = new_clock.master_clock;
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
        self.current_master_clock = 0;
        self.sample_buffer.clear();
        self.apuio_log.clear();
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

    /// Returns the rolling log of recent APUIO accesses with CPU and SPC700 timing.
    /// Each entry records whether the CPU read or wrote an APUIO port ($2140-$2143),
    /// the value transferred, and the master-clock timestamps for both processors.
    ///
    /// Use this to compare CPU↔APU handshake timing against a reference emulator
    /// (e.g. Mesen2). Look for the first access where your emulator's value differs.
    pub fn apuio_log(&self) -> &[ApuioAccess] {
        &self.0.apuio_log
    }

    /// Formats the APUIO log as a human-readable string, one access per line:
    /// `[cpu_clk=NNN spc_cycle=NNN] PORT N <R/W> value XX`
    pub fn format_apuio_log(&self) -> String {
        self.0
            .apuio_log
            .iter()
            .map(|a| {
                format!(
                    "[cpu_clk={:>10} spc_cycle={:>10}] PORT {} {} {:02X}",
                    a.cpu_master_clock,
                    a.spc700_master_cycle,
                    a.port,
                    if a.is_write { "W" } else { "R" },
                    a.value,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
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

    pub fn iter(&self) -> std::slice::Iter<'_, i16> {
        self.samples.iter()
    }
}

impl std::ops::Index<usize> for AudioBuffer {
    type Output = i16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.samples[index]
    }
}
