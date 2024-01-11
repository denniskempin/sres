//! Implementation of the SPC700 CPU.
mod instructions;
mod opcode_table;
mod operands;
mod status;

use std::fmt::Display;

use crate::bus::AddressU16;
use crate::bus::Bus;
use crate::debugger::DebuggerRef;
use crate::spc700::opcode_table::InstructionDef;
pub use crate::spc700::status::Spc700StatusFlags;

pub trait Spc700Bus: Bus<AddressU16> {}

pub struct Spc700<BusT: Spc700Bus> {
    pub opcode_table: [InstructionDef<BusT>; 256],
    pub bus: BusT,
    pub pc: AddressU16,
    pub a: u8,
    pub y: u8,
    pub x: u8,
    pub sp: u8,
    pub dsw: u8,
    pub status: Spc700StatusFlags,
    pub master_cycle: u64,
    pub debugger: DebuggerRef,
}

impl<BusT: Spc700Bus> Spc700<BusT> {
    pub fn new(bus: BusT, debugger: DebuggerRef) -> Self {
        let mut cpu = Self {
            opcode_table: opcode_table::build_opcode_table(),
            bus,
            pc: AddressU16(0),
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            dsw: 0,
            status: Spc700StatusFlags::default(),
            master_cycle: 0,
            debugger,
        };
        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        self.pc = AddressU16(0);
    }

    pub fn step(&mut self) {
        let opcode = self.bus.cycle_read_u8(self.pc);
        let instruction = &self.opcode_table[opcode as usize];
        (instruction.execute)(self);
    }
}

impl<BusT: Spc700Bus> Display for Spc700<BusT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} A:{:02X} X:{:02X} Y:{:02X} SP:{:02X} DSW:{:02X} {}",
            self.pc, self.a, self.x, self.y, self.sp, self.dsw, self.status,
        )
    }
}
