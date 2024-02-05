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
use crate::util::uint::UInt;

use super::operands::BitOperand;
use super::operands::DecodedOperand;
use super::operands::DecodedU16Operand;
use super::operands::Operand;
use super::operands::U16Operand;
use super::operands::U8Operand;
use super::AddressMode;
use super::Spc700StatusFlags;

pub fn nop(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
}

pub fn asl(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    if let DecodedU8Operand::Register(_) = operand {
        // ASL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = value << 1;
    cpu.status.carry = value.bit(7);
    cpu.update_negative_zero_flags(new_value);
    operand.store(cpu, new_value);
}

pub fn push(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_read_u8(cpu.pc);
    let value = operand.load(cpu);
    cpu.stack_push_u8(value);
    cpu.bus.cycle_io();
}

pub fn and(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: U8Operand) {
    if let U8Operand::U8InMemory(AddressMode::XIndirect)
    | U8Operand::U8InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load(cpu);
    let left = left.decode(cpu);
    let left_value = left.load(cpu);
    let value = left_value & right_value;
    cpu.update_negative_zero_flags(value);
    left.store(cpu, value);
}

pub fn or(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: U8Operand) {
    if let U8Operand::U8InMemory(AddressMode::XIndirect)
    | U8Operand::U8InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load(cpu);
    let left = left.decode(cpu);
    let left_value = left.load(cpu);
    let value = left_value | right_value;
    cpu.update_negative_zero_flags(value);
    left.store(cpu, value);
}

pub fn bpl(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let offset = operand.load(cpu) as i8;
    if !cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn bbs(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: BitOperand) {
    let right = right.decode(cpu);
    let value = right.load(cpu);
    cpu.bus.cycle_io();
    let left = left.decode(cpu);
    let offset = left.load(cpu) as i8;
    if value {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn bbc(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: BitOperand) {
    let right = right.decode(cpu);
    let value = right.load(cpu);
    cpu.bus.cycle_io();
    let left = left.decode(cpu);
    let offset = left.load(cpu) as i8;
    if !value {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
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

pub fn tcall(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    let addr = AddressU16(0xFFDE).sub(operand.load(cpu) * 2, Wrap::NoWrap);
    cpu.pc = AddressU16(cpu.bus.cycle_read_u16(addr, Wrap::NoWrap));
}

pub fn bmi(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let offset = operand.load(cpu) as i8;
    if cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn inc(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu).wrapping_add(1);
    if let DecodedU8Operand::Register(_) = operand {
        // ASL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

pub fn call(cpu: &mut Spc700<impl Spc700Bus>, operand: U16Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = AddressU16(operand.load(cpu));
}

pub fn dec(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu).wrapping_sub(1);
    if let DecodedU8Operand::Register(_) = operand {
        // ASL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

pub fn cmp(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: U8Operand) {
    if let U8Operand::U8InMemory(AddressMode::XIndirect)
    | U8Operand::U8InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load(cpu);
    let left = left.decode(cpu);
    let left_value = left.load(cpu);
    let value = left_value.wrapping_sub(right_value);
    if let DecodedU8Operand::InMemory(_, _) = left {
        cpu.bus.cycle_io();
    }
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = left_value >= right_value;
}

pub fn jmp(cpu: &mut Spc700<impl Spc700Bus>, operand: U16Operand) {
    cpu.pc = AddressU16(operand.decode(cpu).load(cpu));
}

pub fn rol(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    if let DecodedU8Operand::Register(_) = operand {
        // ROL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = (value << 1) | cpu.status.carry as u8;
    cpu.status.carry = value.bit(7);
    cpu.update_negative_zero_flags(new_value);
    operand.store(cpu, new_value);
}

pub fn cbne(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: U8Operand) {
    let right = right.decode(cpu);
    let value = right.load(cpu);
    cpu.bus.cycle_io();
    let left = left.decode(cpu);
    let offset = left.load(cpu) as i8;
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

pub fn bra(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let offset = operand.load(cpu) as i8;
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
}

pub fn setp(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.direct_page = true;
}

pub fn eor(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: U8Operand) {
    if let U8Operand::U8InMemory(AddressMode::XIndirect)
    | U8Operand::U8InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load(cpu);
    let left = left.decode(cpu);
    let left_value = left.load(cpu);
    let value = left_value ^ right_value;
    cpu.update_negative_zero_flags(value);
    left.store(cpu, value);
}

pub fn lsr(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    if let DecodedU8Operand::Register(_) = operand {
        // LSR on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = value >> 1;
    cpu.status.carry = value.bit(0);
    cpu.update_negative_zero_flags(new_value);
    operand.store(cpu, new_value);
}

pub fn ror(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    if let DecodedU8Operand::Register(_) = operand {
        // ROR on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = (value >> 1) | (cpu.status.carry as u8) << 7;
    cpu.status.carry = value.bit(0);
    cpu.update_negative_zero_flags(new_value);
    operand.store(cpu, new_value);
}

pub fn clrc(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.carry = false;
}

pub fn dbnz(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: U8Operand) {
    let left = left.decode(cpu);
    let value = left.load(cpu);
    let new_value = value.wrapping_sub(1);
    left.store(cpu, new_value);
    let right = right.decode(cpu);
    let offset = right.load(cpu) as i8;
    if new_value != 0 {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn ret(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    let pcl = cpu.stack_pop_u8();
    let pch = cpu.stack_pop_u8();
    cpu.pc.0.set_low_byte(pcl);
    cpu.pc.0.set_high_byte(pch);
}

pub fn pcall(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    cpu.pc = AddressU16(0xFF00).add(operand.load(cpu) as u16, Wrap::WrapPage);
}

pub fn bvc(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let offset = operand.load(cpu) as i8;
    if !cpu.status.overflow {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn mov(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: U8Operand) {
    let right = right.decode(cpu);
    let value = right.load(cpu);
    let left = left.decode(cpu);

    if let DecodedU8Operand::Register(_) = left {
        // Moves into registers will update the N and Z flags
        cpu.update_negative_zero_flags(value);
        if let DecodedU8Operand::Register(_) = right {
            cpu.bus.cycle_read_u8(cpu.pc);
        }
    } else {
        // Moves into memory locations will read from the target address
        // first.
        if let DecodedU8Operand::InMemory(AddressMode::Dp, _) = right {
            // Somehow 0xFA (MOV from a direct page address into another) is an exception.
            cpu.bus.cycle_io();
        } else {
            left.load(cpu);
        }
    }
    left.store(cpu, value);
}

pub fn bvs(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let offset = operand.load(cpu) as i8;
    if cpu.status.overflow {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn reti(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.status = Spc700StatusFlags::from(cpu.stack_pop_u8());
    let pcl = cpu.stack_pop_u8();
    let pch = cpu.stack_pop_u8();
    cpu.pc.0.set_low_byte(pcl);
    cpu.pc.0.set_high_byte(pch);
}

pub fn setc(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.carry = true;
}

pub fn adc(cpu: &mut Spc700<impl Spc700Bus>, left: U8Operand, right: U8Operand) {
    if let U8Operand::U8InMemory(AddressMode::XIndirect)
    | U8Operand::U8InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load(cpu);
    let left = left.decode(cpu);
    let left_value = left.load(cpu);
    let (mut value, mut carry) = left_value.overflowing_add(right_value);
    if cpu.status.carry {
        let (value2, carry2) = value.overflowing_add(1);
        value = value2;
        carry = carry || carry2;
    }
    cpu.status.half_carry =
        ((right_value & 0x0F) + (left_value & 0x0F) + cpu.status.carry as u8) > 0x0F;
    cpu.status.carry = carry;
    cpu.status.overflow = ((right_value ^ value) & (left_value ^ value)).msb();
    cpu.update_negative_zero_flags(value);
    left.store(cpu, value);
}

pub fn pop(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    let value = cpu.stack_pop_u8();
    operand.store(cpu, value);
}

pub fn bcc(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let offset = operand.load(cpu) as i8;
    if !cpu.status.carry {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn div(cpu: &mut Spc700<impl Spc700Bus>) {
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

////////////////////////////////////////////////////////////////////////////////
/// Bit-wise instructions

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

////////////////////////////////////////////////////////////////////////////////
/// 16-bit (wide) instructions

pub fn addw(cpu: &mut Spc700<impl Spc700Bus>, left: U16Operand, right: U16Operand) {
    let right_value = right.decode(cpu).load(cpu);
    let left = left.decode(cpu);
    let left_value = left.load(cpu);
    if let DecodedU16Operand::RegisterYA = left {
        cpu.bus.cycle_io();
    }

    let (value, carry) = left_value.overflowing_add(right_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = carry;
    cpu.status.overflow = ((right_value ^ value) & (left_value ^ value)).msb();
    cpu.status.half_carry = ((right_value & 0x0FFF) + (left_value & 0x0FFF)) > 0x0FFF;
    left.store(cpu, value)
}

pub fn subw(cpu: &mut Spc700<impl Spc700Bus>, left: U16Operand, right: U16Operand) {
    let right_value = right.decode(cpu).load(cpu);
    let left = left.decode(cpu);
    let left_value = left.load(cpu);
    if let DecodedU16Operand::RegisterYA = left {
        cpu.bus.cycle_io();
    }

    let (value, carry) = left_value.overflowing_sub(right_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !carry;
    cpu.status.overflow = ((right_value ^ value) & (!left_value ^ value)).msb();
    cpu.status.half_carry = ((right_value & 0x0FFF) - (left_value & 0x0FFF)) > 0x0FFF;
    left.store(cpu, value);
}

pub fn cmpw(cpu: &mut Spc700<impl Spc700Bus>, left: U16Operand, right: U16Operand) {
    let right_value = right.decode(cpu).load(cpu);
    let left_value = left.decode(cpu).load(cpu);
    let value = left_value.wrapping_sub(right_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = left_value >= right_value;
}

pub fn incw(cpu: &mut Spc700<impl Spc700Bus>, operand: U16Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu).wrapping_add(1);
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

pub fn decw(cpu: &mut Spc700<impl Spc700Bus>, operand: U16Operand) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu).wrapping_sub(1);
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}
