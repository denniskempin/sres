//! Dummy implementation of the audio processing unit.
mod apu_bus;
mod test;

use log::debug;

use self::apu_bus::ApuBus;
pub use self::apu_bus::ApuBusEvent;
use crate::common::address::AddressU24;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::components::s_dsp::SDspDebug;
use crate::components::spc700::Spc700;
use crate::debugger::DebuggerRef;

pub struct Apu {
    pub spc700: Spc700<ApuBus>,
}

impl Apu {
    #[allow(clippy::new_without_default)]
    pub fn new(debugger: DebuggerRef) -> Self {
        Self {
            spc700: Spc700::new(
                ApuBus::new(DebugEventCollectorRef(debugger.clone())),
                DebugEventCollectorRef(debugger.clone()),
            ),
        }
    }

    pub fn debug(&self) -> ApuDebug<'_> {
        ApuDebug(self)
    }

    pub fn generate_sample(&mut self) -> i16 {
        let memory = &self.spc700.bus.ram;
        self.spc700.bus.dsp.generate_sample(memory)
    }

    pub fn catch_up_to_master_clock(&mut self, master_clock: u64) {
        self.spc700.catch_up_to_master_clock(master_clock);
    }

    pub fn bus_read(&mut self, addr: AddressU24) -> u8 {
        self.read_apuio(addr)
    }

    pub fn bus_peek(&self, addr: AddressU24) -> Option<u8> {
        Some(self.peek_apuio(addr))
    }

    pub fn bus_write(&mut self, addr: AddressU24, value: u8) {
        self.write_apuio(addr, value)
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
