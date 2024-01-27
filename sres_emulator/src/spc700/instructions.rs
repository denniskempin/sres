use std::fmt::LowerExp;

use intbits::Bits;

use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::Operand;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::util::uint::UInt;

pub fn nop(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
}

pub fn or1(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let bit = operand.load_u8(cpu).bit(operand.bit_idx());
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
    cpu.bus.cycle_read_u8(cpu.pc);
    let value = operand.load_u8(cpu);
    cpu.stack_push_u8(value);
    cpu.bus.cycle_io();
}

pub fn set1(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u8(cpu).with_bit(operand.bit_idx(), true);
    operand.store_u8(cpu, value);
}

pub fn or(cpu: &mut Spc700<impl Spc700Bus>, left: Operand, right: Operand) {
    let left_value = left.load_u8(cpu);
    let right_value = right.load_u8(cpu);
    let value = left_value | right_value;
    cpu.update_negative_zero_flags(value);
    left.store_u8(cpu, value);
}

pub fn bbs(cpu: &mut Spc700<impl Spc700Bus>, left: Operand, right: Operand) {
    let value = right.load_u8(cpu).bit(right.bit_idx());
    cpu.bus.cycle_io();
    let offset = left.load_u8(cpu) as i8;
    if value {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    } else {
    }
}

pub fn tset1(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u8(cpu);
    operand.load_u8(cpu); // CPU will re-read the value for another cycle
    operand.store_u8(cpu, value | cpu.a);
    cpu.update_negative_zero_flags(cpu.a.wrapping_sub(value));
}

pub fn brk(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.stack_push_u16(cpu.pc.0);
    cpu.stack_push_u8(cpu.status.into());
    cpu.status.irq_enable = false;
    cpu.status.break_command = true;
    cpu.bus.cycle_io();
    cpu.pc = AddressU16(cpu.bus.cycle_read_u16(AddressU16(0xffde), Wrap::NoWrap));
}

pub fn tcall(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    let addr = AddressU16(0xFFDE).sub(operand.load_u8(cpu) * 2, Wrap::NoWrap);
    cpu.pc = AddressU16(cpu.bus.cycle_read_u16(addr, Wrap::NoWrap));
}
