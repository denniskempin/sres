use std::fmt::LowerExp;

use intbits::Bits;

use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::Operand;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::util::uint::UInt;

use super::AddressMode;

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
    if let Operand::Register(_) = operand {
        // ASL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
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

pub fn and(cpu: &mut Spc700<impl Spc700Bus>, left: Operand, right: Operand) {
    if let Operand::InMemory(_, AddressMode::XIndirect, _)
    | Operand::InMemory(_, AddressMode::YIndirect, _) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let left_value = left.load_u8(cpu);
    let right_value = right.load_u8(cpu);
    let value = left_value & right_value;
    cpu.update_negative_zero_flags(value);
    left.store_u8(cpu, value);
}

pub fn or(cpu: &mut Spc700<impl Spc700Bus>, left: Operand, right: Operand) {
    if let Operand::InMemory(_, AddressMode::XIndirect, _)
    | Operand::InMemory(_, AddressMode::YIndirect, _) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let left_value = left.load_u8(cpu);
    let right_value = right.load_u8(cpu);
    let value = left_value | right_value;
    cpu.update_negative_zero_flags(value);
    left.store_u8(cpu, value);
}

pub fn bpl(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let offset = operand.load_u8(cpu) as i8;
    if !cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn bbs(cpu: &mut Spc700<impl Spc700Bus>, left: Operand, right: Operand) {
    let value = right.load_u8(cpu).bit(right.bit_idx());
    cpu.bus.cycle_io();
    let offset = left.load_u8(cpu) as i8;
    if value {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn bbc(cpu: &mut Spc700<impl Spc700Bus>, left: Operand, right: Operand) {
    let value = right.load_u8(cpu).bit(right.bit_idx());
    cpu.bus.cycle_io();
    let offset = left.load_u8(cpu) as i8;
    if !value {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn clr1(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u8(cpu).with_bit(operand.bit_idx(), false);
    operand.store_u8(cpu, value);
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

pub fn incw(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u16(cpu).wrapping_add(1);
    cpu.update_negative_zero_flags(value);
    operand.store_u16(cpu, value);
}

pub fn bmi(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let offset = operand.load_u8(cpu) as i8;
    if cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn inc(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u8(cpu).wrapping_add(1);
    if let Operand::Register(_) = operand {
        // ASL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    cpu.update_negative_zero_flags(value);
    operand.store_u8(cpu, value);
}

pub fn call(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = operand.effective_addr().unwrap();
}

pub fn decw(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u16(cpu).wrapping_sub(1);
    cpu.update_negative_zero_flags(value);
    operand.store_u16(cpu, value);
}

pub fn dec(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u8(cpu).wrapping_sub(1);
    if let Operand::Register(_) = operand {
        // ASL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    cpu.update_negative_zero_flags(value);
    operand.store_u8(cpu, value);
}

pub fn cmp(cpu: &mut Spc700<impl Spc700Bus>, left: Operand, right: Operand) {
    let left_value = left.load_u8(cpu);
    let right_value = right.load_u8(cpu);
    let value = left_value.wrapping_sub(right_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = left_value >= right_value;
}

pub fn jmp(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    cpu.pc = operand.effective_addr().unwrap();
}

pub fn rol(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load_u8(cpu);
    if let Operand::Register(_) = operand {
        // ROL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = (value << 1) | cpu.status.carry as u8;
    cpu.status.carry = value.bit(7);
    cpu.update_negative_zero_flags(new_value);
    operand.store_u8(cpu, new_value);
}

pub fn cbne(cpu: &mut Spc700<impl Spc700Bus>, left: Operand, right: Operand) {
    let value = right.load_u8(cpu);
    let offset = left.load_u8(cpu) as i8;
    cpu.bus.cycle_io();
    if value != cpu.a {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn clrp(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.direct_page = false;
}

pub fn bra(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let offset = operand.load_u8(cpu) as i8;
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
}
