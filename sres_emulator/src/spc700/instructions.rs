use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::Operand;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::util::uint::UInt;

pub fn nop(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc.add(1_u8, Wrap::NoWrap));
}

pub fn or1(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let bit = operand.load_bit(cpu);
    cpu.status.carry = cpu.status.carry || bit;
    cpu.bus.cycle_io()
}

pub fn asl(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u8(cpu);
    let new_value = value << 1;
    cpu.status.carry = value.bit(7);
    cpu.update_negative_zero_flags(new_value);
    operand.store_u8(cpu, new_value);
}

pub fn push(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    cpu.bus.cycle_read_u8(cpu.pc.add(1_u8, Wrap::NoWrap));
    let value = operand.load_u8(cpu);
    cpu.stack_push_u8(value);
    cpu.bus.cycle_io();
}

pub fn tset1(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u8(cpu);
    operand.load_u8(cpu); // CPU will re-read the value for another cycle
    operand.store_u8(cpu, value | cpu.a);
    cpu.update_negative_zero_flags(cpu.a.wrapping_sub(value));
}

pub fn brk(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc.add(1_u8, Wrap::NoWrap));
    cpu.stack_push_u16(cpu.pc.add(1_u16, Wrap::NoWrap).0);
    cpu.stack_push_u8(cpu.status.into());
    cpu.status.irq_enable = false;
    cpu.status.break_command = true;
    cpu.bus.cycle_io();
    cpu.pc = AddressU16(cpu.bus.cycle_read_u16(AddressU16(0xffde), Wrap::NoWrap))
        .sub(1_u16, Wrap::NoWrap);
}

pub fn tcall<const N: u16>(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc.add(1_u8, Wrap::NoWrap));
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.add(1_u16, Wrap::NoWrap).0);
    cpu.bus.cycle_io();
    let addr = AddressU16(0xFFDE).sub(N * 2, Wrap::NoWrap);
    cpu.pc = AddressU16(cpu.bus.cycle_read_u16(addr, Wrap::NoWrap));
}
