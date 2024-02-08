use std::fmt::LowerExp;
use std::ops::Div;

use intbits::Bits;

use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::DecodedU8Operand;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::util::uint::U16Ext;
use crate::util::uint::U8Ext;
use crate::util::uint::UInt;

use super::operands::BitOperand;
use super::operands::DecodedOperand;
use super::operands::Operand;
use super::operands::U16Operand;
use super::operands::U8Operand;
use super::AddressMode;
use super::Spc700StatusFlags;

////////////////////////////////////////////////////////////////////////////////
// Arithmetic instructions

pub fn adc(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    if right_op.is_address_mode(AddressMode::XIndirect)
        || right_op.is_address_mode(AddressMode::YIndirect)
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right_op.decode(cpu).load(cpu);
    let left_op = left_op.decode(cpu);
    let left = left_op.load(cpu);

    let (mut value, mut carry) = left.overflowing_add(right);
    if cpu.status.carry {
        let (value2, carry2) = value.overflowing_add(1);
        value = value2;
        carry = carry || carry2;
    }
    cpu.status.half_carry = ((right & 0x0F) + (left & 0x0F) + cpu.status.carry as u8) > 0x0F;
    cpu.status.carry = carry;
    cpu.status.overflow = ((right ^ value) & (left ^ value)).msb();
    cpu.update_negative_zero_flags(value);
    left_op.store(cpu, value);
}

pub fn sbc(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    if right_op.is_address_mode(AddressMode::XIndirect)
        || right_op.is_address_mode(AddressMode::YIndirect)
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right_op.decode(cpu).load(cpu);
    let left_op = left_op.decode(cpu);
    let left = left_op.load(cpu);

    let (mut value, mut carry) = left.overflowing_sub(right);
    if !cpu.status.carry {
        let (value2, carry2) = value.overflowing_sub(1);
        value = value2;
        carry = carry || carry2;
    }
    cpu.status.half_carry =
        !((left & 0x0F).wrapping_sub((right & 0x0F) + !cpu.status.carry as u8) & 0x10 == 0x10);
    cpu.status.carry = !carry;
    cpu.status.overflow = ((left ^ right) & (left ^ value)).msb();
    cpu.update_negative_zero_flags(value);
    left_op.store(cpu, value);
}

pub fn cmp(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    if right_op.is_address_mode(AddressMode::XIndirect)
        || right_op.is_address_mode(AddressMode::YIndirect)
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right_op.decode(cpu).load(cpu);
    let left = left_op.decode(cpu).load(cpu);
    let value = left.wrapping_sub(right);
    if left_op.is_in_memory() {
        cpu.bus.cycle_io();
    }
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = left >= right;
}

pub fn inc(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu).wrapping_add(1);
    if operand.is_alu_register() {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

pub fn dec(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu).wrapping_sub(1);
    if operand.is_alu_register() {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

pub fn mul(cpu: &mut Spc700<impl Spc700Bus>) {
    let a = cpu.a as u16;
    let x = cpu.x as u16;
    let result = a * x;
    cpu.update_negative_zero_flags((result >> 8) as u8);
    cpu.a = (result & 0xFF) as u8;
    cpu.x = (result >> 8) as u8;
}

pub fn div(cpu: &mut Spc700<impl Spc700Bus>) {
    // TODO: Naiive, and wrong implementation
    let y = cpu.y as u16;
    let a = cpu.a as u16;
    if y != 0 {
        let quotient = a / y;
        let remainder = a % y;
        cpu.update_negative_zero_flags(quotient as u8);
        cpu.y = remainder as u8;
        cpu.a = quotient as u8;
    } else {
        cpu.status.overflow = true;
    }
}

////////////////////////////////////////////////////////////////////////////////
// Shift instructions

pub fn rol(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    if operand.is_alu_register() {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = (value << 1) | cpu.status.carry as u8;
    cpu.status.carry = value.bit(7);
    cpu.update_negative_zero_flags(new_value);
    operand.store(cpu, new_value);
}

pub fn ror(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    if operand.is_alu_register() {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = (value >> 1) | (cpu.status.carry as u8) << 7;
    cpu.status.carry = value.bit(0);
    cpu.update_negative_zero_flags(new_value);
    operand.store(cpu, new_value);
}

pub fn asl(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    if operand.is_alu_register() {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = value << 1;
    cpu.status.carry = value.bit(7);
    cpu.update_negative_zero_flags(new_value);
    operand.store(cpu, new_value);
}

pub fn lsr(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    if operand.is_alu_register() {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = value >> 1;
    cpu.status.carry = value.bit(0);
    cpu.update_negative_zero_flags(new_value);
    operand.store(cpu, new_value);
}

////////////////////////////////////////////////////////////////////////////////
// Binary logic instructions

pub fn and(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    if right_op.is_address_mode(AddressMode::XIndirect)
        | right_op.is_address_mode(AddressMode::YIndirect)
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right_op.decode(cpu).load(cpu);
    let left_op = left_op.decode(cpu);
    let left = left_op.load(cpu);
    let value = left & right;
    cpu.update_negative_zero_flags(value);
    left_op.store(cpu, value);
}

pub fn or(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    if right_op.is_address_mode(AddressMode::XIndirect)
        | right_op.is_address_mode(AddressMode::YIndirect)
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right_op.decode(cpu).load(cpu);
    let left_op = left_op.decode(cpu);
    let left = left_op.load(cpu);
    let value = left | right;
    cpu.update_negative_zero_flags(value);
    left_op.store(cpu, value);
}

pub fn eor(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    if right_op.is_address_mode(AddressMode::XIndirect)
        | right_op.is_address_mode(AddressMode::YIndirect)
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right_op.decode(cpu).load(cpu);
    let left_op = left_op.decode(cpu);
    let left = left_op.load(cpu);
    let value = left ^ right;
    cpu.update_negative_zero_flags(value);
    left_op.store(cpu, value);
}

////////////////////////////////////////////////////////////////////////////////
// Status bit operations

pub fn clrp(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.direct_page = false;
}

pub fn setp(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.direct_page = true;
}

pub fn setc(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.carry = true;
}

pub fn clrc(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.carry = false;
}

pub fn ei(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.status.irq_enable = true;
}

pub fn di(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.status.irq_enable = false;
}

////////////////////////////////////////////////////////////////////////////////
// Stack instructions

pub fn push(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_read_u8(cpu.pc);
    let value = operand.load(cpu);
    cpu.stack_push_u8(value);
    cpu.bus.cycle_io();
}

pub fn pop(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    let value = cpu.stack_pop_u8();
    operand.store(cpu, value);
}

////////////////////////////////////////////////////////////////////////////////
// Call / Jump / Break instructions

pub fn jmp(cpu: &mut Spc700<impl Spc700Bus>, operand: U16Operand) {
    cpu.pc = AddressU16(operand.decode(cpu).load(cpu));
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

pub fn call(cpu: &mut Spc700<impl Spc700Bus>, operand: U16Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = AddressU16(operand.load(cpu));
}

pub fn tcall(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    let addr = AddressU16(0xFFDE).sub(operand.load(cpu) * 2, Wrap::NoWrap);
    cpu.pc = AddressU16(cpu.bus.cycle_read_u16(addr, Wrap::NoWrap));
}

pub fn pcall(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    cpu.pc = AddressU16(0xFF00).add(operand.load(cpu) as u16, Wrap::WrapPage);
}

pub fn ret(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    let pc = cpu.stack_pop_u16();
    cpu.pc = AddressU16(pc);
}

pub fn reti(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.status = Spc700StatusFlags::from(cpu.stack_pop_u8());
    let pc = cpu.stack_pop_u16();
    cpu.pc = AddressU16(pc);
}

////////////////////////////////////////////////////////////////////////////////
// Branch instructions

fn branch(cpu: &mut Spc700<impl Spc700Bus>, offset_op: U8Operand, condition: bool) {
    let offset = offset_op.decode(cpu).load(cpu) as i8;
    if condition {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn bra(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    branch(cpu, operand, true);
}

pub fn bpl(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    branch(cpu, operand, !cpu.status.negative);
}

pub fn bmi(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    branch(cpu, operand, cpu.status.negative);
}

pub fn bvc(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    branch(cpu, operand, !cpu.status.overflow);
}

pub fn bvs(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    branch(cpu, operand, cpu.status.overflow);
}

pub fn bcs(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    branch(cpu, operand, cpu.status.carry);
}

pub fn bcc(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    branch(cpu, operand, !cpu.status.carry);
}

pub fn bbs(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: BitOperand) {
    let value = right_op.decode(cpu).load(cpu);
    cpu.bus.cycle_io();
    branch(cpu, left_op, value);
}

pub fn bbc(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: BitOperand) {
    let value = right_op.decode(cpu).load(cpu);
    cpu.bus.cycle_io();
    branch(cpu, left_op, !value);
}

pub fn cbne(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    let value = right_op.decode(cpu).load(cpu);
    cpu.bus.cycle_io();
    branch(cpu, left_op, value != cpu.a);
}

pub fn dbnz(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    let left_op = left_op.decode(cpu);
    let value = left_op.load(cpu).wrapping_sub(1);
    left_op.store(cpu, value);
    branch(cpu, right_op, value != 0);
}

////////////////////////////////////////////////////////////////////////////////
// Single-bit instructions

pub fn set1(cpu: &mut Spc700<impl Spc700Bus>, operand: BitOperand) {
    operand.decode(cpu).store(cpu, true);
}

pub fn clr1(cpu: &mut Spc700<impl Spc700Bus>, operand: BitOperand) {
    operand.decode(cpu).store(cpu, false);
}

pub fn tset1(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    operand.load(cpu); // CPU will re-read the value for another cycle
    operand.store(cpu, value | cpu.a);
    cpu.update_negative_zero_flags(cpu.a.wrapping_sub(value));
}

pub fn tclr1(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    operand.load(cpu); // CPU will re-read the value for another cycle
    operand.store(cpu, value & !cpu.a);
    cpu.update_negative_zero_flags(cpu.a.wrapping_sub(value));
}

pub fn or1(cpu: &mut Spc700<impl Spc700Bus>, operand: BitOperand) {
    let bit = operand.decode(cpu).load(cpu);
    cpu.status.carry = cpu.status.carry || bit;
    cpu.bus.cycle_io()
}

pub fn and1(cpu: &mut Spc700<impl Spc700Bus>, operand: BitOperand) {
    let bit = operand.decode(cpu).load(cpu);
    cpu.status.carry = cpu.status.carry && bit;
}

pub fn eor1(cpu: &mut Spc700<impl Spc700Bus>, operand: BitOperand) {
    let bit = operand.decode(cpu).load(cpu);
    cpu.bus.cycle_io();
    cpu.status.carry = cpu.status.carry ^ bit;
}

pub fn mov1(cpu: &mut Spc700<impl Spc700Bus>, left_op: BitOperand, right_op: BitOperand) {
    let bit = right_op.decode(cpu).load(cpu);
    left_op.decode(cpu).store(cpu, bit);
    if right_op.is_carry() {
        cpu.bus.cycle_io();
    }
}

////////////////////////////////////////////////////////////////////////////////
// 16-bit (wide) instructions

pub fn addw(cpu: &mut Spc700<impl Spc700Bus>, left_op: U16Operand, right_op: U16Operand) {
    let right = right_op.decode(cpu).load(cpu);
    let left_op = left_op.decode(cpu);
    let left = left_op.load(cpu);
    if left_op.is_register() {
        cpu.bus.cycle_io();
    }

    let (value, carry) = left.overflowing_add(right);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = carry;
    cpu.status.overflow = ((right ^ value) & (left ^ value)).msb();
    cpu.status.half_carry = ((right & 0x0FFF) + (left & 0x0FFF)) > 0x0FFF;
    left_op.store(cpu, value)
}

pub fn subw(cpu: &mut Spc700<impl Spc700Bus>, left_op: U16Operand, right_op: U16Operand) {
    let right = right_op.decode(cpu).load(cpu);
    let left_op = left_op.decode(cpu);
    let left = left_op.load(cpu);
    if left_op.is_register() {
        cpu.bus.cycle_io();
    }

    let (value, carry) = left.overflowing_sub(right);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !carry;
    cpu.status.overflow = ((right ^ value) & (!left ^ value)).msb();
    cpu.status.half_carry = ((right & 0x0FFF) - (left & 0x0FFF)) > 0x0FFF;
    left_op.store(cpu, value);
}

pub fn cmpw(cpu: &mut Spc700<impl Spc700Bus>, left_op: U16Operand, right_op: U16Operand) {
    let right = right_op.decode(cpu).load(cpu);
    let left = left_op.decode(cpu).load(cpu);
    let result = left.wrapping_sub(right);
    cpu.update_negative_zero_flags(result);
    cpu.status.carry = left >= right;
}

pub fn incw(cpu: &mut Spc700<impl Spc700Bus>, operand: U16Operand) {
    let operand = operand.decode(cpu);
    let result = operand.load(cpu).wrapping_add(1);
    cpu.update_negative_zero_flags(result);
    operand.store(cpu, result);
}

pub fn decw(cpu: &mut Spc700<impl Spc700Bus>, operand: U16Operand) {
    let operand = operand.decode(cpu);
    let result = operand.load(cpu).wrapping_sub(1);
    cpu.update_negative_zero_flags(result);
    operand.store(cpu, result);
}

pub fn movw(cpu: &mut Spc700<impl Spc700Bus>, left_op: U16Operand, right_op: U16Operand) {
    let right = right_op.decode(cpu).load(cpu);
    let left_op = left_op.decode(cpu);
    cpu.bus.cycle_io();

    if left_op.is_register() {
        // Moves into registers will update the N and Z flags
        cpu.update_negative_zero_flags(right);
    } else {
    }
    left_op.store(cpu, right);
}

////////////////////////////////////////////////////////////////////////////////
// Misc instructions

pub fn mov(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    let right = right_op.decode(cpu).load(cpu);
    let left_op = left_op.decode(cpu);

    if left_op.is_alu_register() {
        // Moves into registers will update the N and Z flags
        cpu.update_negative_zero_flags(right);
        if right_op.is_alu_register() {
            cpu.bus.cycle_read_u8(cpu.pc);
        }
    } else {
        // Moves into memory locations will do an extra read from the target address.
        if right_op.is_address_mode(AddressMode::Dp) {
            // Somehow 0xFA (MOV from a direct page address into another) is an exception.
            cpu.bus.cycle_io();
        } else if left_op.is_address_mode(AddressMode::XIndirect) {
            cpu.bus.cycle_read_u8(cpu.pc);
            left_op.load(cpu);
        } else if left_op.is_address_mode(AddressMode::XIndirectAutoInc) {
            cpu.bus.cycle_read_u8(cpu.pc);
            cpu.bus.cycle_io();
        } else {
            left_op.load(cpu);
        }
    }
    left_op.store(cpu, right);
}

pub fn nop(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
}

pub fn xcn(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    let a = cpu.a;
    let a = (a >> 4) | (a << 4);
    cpu.update_negative_zero_flags(a);
    cpu.a = a;
}

pub fn daa(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    let mut a = cpu.a;
    let mut carry = cpu.status.carry;
    if cpu.status.half_carry || (!cpu.status.negative && (a & 0x0F) > 0x09) {
        a = a.wrapping_add(0x06);
    }
    if carry || (!cpu.status.negative && a > 0x9F) {
        a = a.wrapping_add(0x60);
        carry = true;
    }
    cpu.status.carry = carry;
    cpu.update_negative_zero_flags(a);
    cpu.a = a;
}

pub fn das(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    if !cpu.status.carry || cpu.a > 0x99 {
        cpu.a = cpu.a.wrapping_sub(0x60);
        cpu.status.carry = false;
    }
    if !cpu.status.half_carry || cpu.a.low_nibble() > 0x09 {
        cpu.a = cpu.a.wrapping_sub(0x06);
    }
    cpu.update_negative_zero_flags(cpu.a);
}
