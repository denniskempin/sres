//! Implementation of the SPC700 CPU.
mod instructions;
mod opcode_table;
mod operands;
mod spc700_bus;
mod status;

use std::fmt::Display;

use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::debugger::DebuggerRef;
use crate::spc700::opcode_table::InstructionDef;
pub use crate::spc700::operands::AddressMode;
pub use crate::spc700::operands::DecodedOperand;
pub use crate::spc700::spc700_bus::Spc700Bus;
pub use crate::spc700::spc700_bus::Spc700BusImpl;
pub use crate::spc700::status::Spc700StatusFlags;
use crate::util::uint::UInt;

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
        self.pc = AddressU16(0xFFC0);
        self.sp = 0xef;
        self.status.zero = true;
    }

    pub fn disassembly(&self, addr: AddressU16) -> (String, AddressU16) {
        let opcode = self.bus.peek_u8(addr).unwrap_or_default();
        let instruction = &self.opcode_table[opcode as usize];
        (instruction.disassembly)(self, addr.add(1_u8, Wrap::NoWrap))
    }

    pub fn step(&mut self) {
        let opcode = self.bus.cycle_read_u8(self.pc);
        self.pc = self.pc.add(1_u8, Wrap::NoWrap);
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

    pub fn direct_page_addr(&self, offset: u8) -> AddressU16 {
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

impl<BusT: Spc700Bus> Display for Spc700<BusT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (disassembly, _) = self.disassembly(self.pc);
        write!(
            f,
            "{} {} A:{:02X} X:{:02X} Y:{:02X} SP:{:02X} DSW:{:02X} {}",
            self.pc, disassembly, self.a, self.x, self.y, self.sp, self.dsw, self.status,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::trace::Spc700TraceLine;

    fn assert_state(spc700: &Spc700<impl Spc700Bus>, expected_state: &str) {
        let actual = Spc700TraceLine::from_spc700(spc700).to_string();
        println!("{}", actual);
        assert_eq!(actual, expected_state);
    }

    fn assert_states(spc700: &mut Spc700<impl Spc700Bus>, expected_states: &[&str]) {
        for expected_state in expected_states {
            assert_state(spc700, expected_state);
            spc700.step();
        }
    }

    #[test]
    fn boot_rom_test() {
        let mut spc700 = Spc700::new(Spc700BusImpl::new(), Default::default());
        assert_states(
            &mut spc700,
            &[
                "..ffc0 mov   x, #$ef           A:00 X:00 Y:00 SP:01ef YA:0000 ......Z.",
                "..ffc2 mov   sp, x             A:00 X:ef Y:00 SP:01ef YA:0000 N.......",
                "..ffc3 mov   a, #$00           A:00 X:ef Y:00 SP:01ef YA:0000 N.......",
                "..ffc5 mov   (x), a            A:00 X:ef Y:00 SP:01ef YA:0000 ......Z.",
                "..ffc6 dec   x                 A:00 X:ef Y:00 SP:01ef YA:0000 ......Z.",
                "..ffc7 bne   $ffc5             A:00 X:ee Y:00 SP:01ef YA:0000 N.......",
                "..ffc5 mov   (x), a            A:00 X:ee Y:00 SP:01ef YA:0000 N.......",
            ],
        );
    }
}
