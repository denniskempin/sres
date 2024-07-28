//! Implementation of the SPC700 CPU.
mod instructions;
mod opcode_table;
mod operands;
mod state;
mod status;
#[cfg(test)]
mod test;

use std::sync::atomic::Ordering;

use crate::common::address::Address;
use crate::common::address::AddressU16;
use crate::common::address::Wrap;
use crate::common::bus::Bus;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::common::debug_events::DEBUG_EVENTS_ENABLED;
use crate::common::system::InstructionMeta;
use crate::common::uint::UInt;

use self::opcode_table::InstructionDef;
use self::operands::AddressMode;
use self::operands::DecodedOperand;
pub use self::state::Spc700State;
use self::status::Spc700StatusFlags;

#[derive(Debug, Clone, PartialEq)]
pub enum Spc700Event {
    Step(Spc700State),
}

pub trait Spc700Bus: Bus<AddressU16> {
    fn master_cycle(&self) -> u64;
}

pub struct Spc700<BusT: Spc700Bus> {
    pub bus: BusT,
    debug_event_collector: DebugEventCollectorRef<Spc700Event>,
    opcode_table: [InstructionDef<BusT>; 256],
    pc: AddressU16,
    a: u8,
    y: u8,
    x: u8,
    sp: u8,
    status: Spc700StatusFlags,
}

impl<BusT: Spc700Bus> Spc700<BusT> {
    pub fn new(bus: BusT, debug_event_collector: DebugEventCollectorRef<Spc700Event>) -> Self {
        let mut cpu = Self {
            opcode_table: opcode_table::build_opcode_table(),
            bus,
            pc: AddressU16(0),
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            status: Spc700StatusFlags::default(),
            debug_event_collector,
        };
        cpu.reset();
        cpu
    }

    pub fn debug(&self) -> Spc700Debug<'_, BusT> {
        Spc700Debug(self)
    }

    pub fn reset(&mut self) {
        self.pc = AddressU16(0xFFC0);
        self.sp = 0xef;
        self.status.zero = true;
    }

    pub fn catch_up_to_master_clock(&mut self, master_cycles: u64) {
        while master_cycles > self.bus.master_cycle() {
            self.step();
        }
    }

    pub fn step(&mut self) {
        if DEBUG_EVENTS_ENABLED.load(Ordering::Relaxed) {
            self.debug_event_collector
                .collect_event(Spc700Event::Step(self.debug().state()));
        }

        let opcode = self.bus.cycle_read_u8(self.pc);
        let instruction = &self.opcode_table[opcode as usize];
        (instruction.execute)(self);
    }

    fn update_negative_zero_flags<T: UInt>(&mut self, value: T) {
        self.status.negative = value.bit(T::N_BITS - 1);
        self.status.zero = value.is_zero();
    }

    fn stack_push_u8(&mut self, value: u8) {
        self.bus
            .cycle_write_u8(AddressU16::new_direct_page(1, self.sp), value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.stack_push_u8(bytes[1]);
        self.stack_push_u8(bytes[0]);
    }

    fn stack_pop_u8(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.bus
            .cycle_read_u8(AddressU16::new_direct_page(1, self.sp))
    }

    fn stack_pop_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.stack_pop_u8(), self.stack_pop_u8()])
    }

    fn direct_page_addr(&self, offset: u8) -> AddressU16 {
        AddressU16::new_direct_page(if self.status.direct_page { 1 } else { 0 }, offset)
    }

    fn fetch_program_u8(&mut self) -> u8 {
        let value = self.bus.cycle_read_u8(self.pc);
        self.pc = self.pc.add(1_u8, Wrap::NoWrap);
        value
    }

    fn fetch_program_u16(&mut self) -> u16 {
        let value = self.bus.cycle_read_u16(self.pc, Wrap::NoWrap);
        self.pc = self.pc.add(2_u8, Wrap::NoWrap);
        value
    }
}

pub struct Spc700Debug<'a, BusT: Spc700Bus>(&'a Spc700<BusT>);

impl<'a, BusT: Spc700Bus> Spc700Debug<'a, BusT> {
    pub fn state(&self) -> Spc700State {
        Spc700State {
            instruction: self.disassembly(self.0.pc).0,
            a: self.0.a,
            x: self.0.x,
            y: self.0.y,
            sp: AddressU16(0x0100 + self.0.sp as u16),
            status: self.0.status.to_string(),
        }
    }

    pub fn disassembly(&self, addr: AddressU16) -> (InstructionMeta<AddressU16>, AddressU16) {
        let opcode = self.0.bus.peek_u8(addr).unwrap_or_default();
        let instruction = &self.0.opcode_table[opcode as usize];
        (instruction.disassembly)(self.0, addr)
    }
}
