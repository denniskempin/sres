use intbits::Bits;

use crate::common::address::AddressU16;
use crate::common::bus::Bus;
use crate::debugger::DebuggerRef;
use crate::debugger::Event;

use super::s_dsp::SDsp;
use super::spc700::Spc700Bus;

pub struct ApuBus {
    pub debugger: DebuggerRef,
    pub master_cycle: u64,
    pub ram: [u8; 0x10000],
    pub channel_in: [u8; 4],
    pub channel_out: [u8; 4],
    pub timer: [u8; 3],
    pub dsp_register_select: u8,
    pub dsp_register_readonly: bool,
    pub dsp: SDsp,
}

struct ApuDebug<'a> {
    bus: &'a ApuBus,
}

impl<'a> ApuDebug<'a> {
    fn new(bus: &'a ApuBus) -> Self {
        Self { bus }
    }

    fn voice(&self, idx: usize) -> String {
        "".to_string()
    }
}

impl ApuBus {
    #[allow(clippy::new_without_default)]
    pub fn new(debugger: DebuggerRef) -> Self {
        Self {
            debugger,
            master_cycle: 0,
            ram: [0; 0x10000],
            channel_in: [0; 4],
            channel_out: [0; 4],
            timer: [0x0F; 3],
            dsp_register_readonly: false,
            dsp_register_select: 0,
            dsp: Default::default(),
        }
    }
}

impl Bus<AddressU16> for ApuBus {
    fn peek_u8(&self, addr: AddressU16) -> Option<u8> {
        match addr.0 {
            0x00F2 => Some(self.dsp_register_select.bits(0..=6)),
            0x00F3 => Some(self.dsp.read_register(self.dsp_register_select)),
            0x00F4..=0x00F7 => Some(self.channel_in[addr.0 as usize - 0x00F4]),
            0x00FD..=0x00FF => Some(self.timer[addr.0 as usize - 0x00FD]),
            0xFFC0..=0xFFFF => Some(IPL_BOOT_ROM[(addr.0 - 0xFFC0) as usize]),
            _ => Some(self.ram[addr.0 as usize]),
        }
    }

    fn cycle_io(&mut self) {
        self.master_cycle += 21;
    }

    fn cycle_read_u8(&mut self, addr: AddressU16) -> u8 {
        self.master_cycle += 21;
        let value = self.peek_u8(addr).unwrap_or_default();
        self.debugger.on_event(Event::Spc700MemoryRead(addr, value));
        value
    }

    fn cycle_write_u8(&mut self, addr: AddressU16, value: u8) {
        self.debugger
            .on_event(Event::Spc700MemoryWrite(addr, value));

        self.master_cycle += 21;
        #[allow(clippy::single_match)]
        match addr.0 {
            0x00F2 => {
                self.dsp_register_readonly = value.bit(7);
                self.dsp_register_select = value.bits(0..=6);
            }
            0x00F3 => {
                if self.dsp_register_readonly {
                    return;
                }
                self.dsp.write_register(self.dsp_register_select, value);
            }
            0x00F4..=0x00F7 => self.channel_out[addr.0 as usize - 0x00F4] = value,
            _ => self.ram[addr.0 as usize] = value,
        }
    }

    fn reset(&mut self) {}
}

impl Spc700Bus for ApuBus {
    fn master_cycle(&self) -> u64 {
        self.master_cycle
    }
}

/// See https://github.com/gilligan/snesdev/blob/master/docs/spc700.txt
const IPL_BOOT_ROM: [u8; 64] = [
    0xCD, 0xEF, 0xBD, 0xE8, 0x00, 0xC6, 0x1D, 0xD0, 0xFC, 0x8F, 0xAA, 0xF4, 0x8F, 0xBB, 0xF5, 0x78,
    0xCC, 0xF4, 0xD0, 0xFB, 0x2F, 0x19, 0xEB, 0xF4, 0xD0, 0xFC, 0x7E, 0xF4, 0xD0, 0x0B, 0xE4, 0xF5,
    0xCB, 0xF4, 0xD7, 0x00, 0xFC, 0xD0, 0xF3, 0xAB, 0x01, 0x10, 0xEF, 0x7E, 0xF4, 0x10, 0xEB, 0xBA,
    0xF6, 0xDA, 0x00, 0xBA, 0xF4, 0xC4, 0xF4, 0xDD, 0x5D, 0xD0, 0xDB, 0x1F, 0x00, 0x00, 0xC0, 0xFF,
];
