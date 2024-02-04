use std::fmt::LowerExp;
use std::ops::Div;

use intbits::Bits;

use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::Operand;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::util::uint::U16Ext;
use crate::util::uint::UInt;

use super::operands::OperandDef;
use super::AddressMode;
use super::Spc700StatusFlags;

pub fn nop(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
}

pub fn or1(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let bit = operand.load_u8(cpu).bit(operand.bit_idx());
    cpu.status.carry = cpu.status.carry || bit;
    cpu.bus.cycle_io()
}

pub fn asl(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
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

pub fn push(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_read_u8(cpu.pc);
    let value = operand.load_u8(cpu);
    cpu.stack_push_u8(value);
    cpu.bus.cycle_io();
}

pub fn set1(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let value = operand.load_u8(cpu).with_bit(operand.bit_idx(), true);
    operand.store_u8(cpu, value);
}

pub fn and(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    if let OperandDef::InMemory(AddressMode::XIndirect)
    | OperandDef::InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load_u8(cpu);
    let left = left.decode(cpu);
    let left_value = left.load_u8(cpu);
    let value = left_value & right_value;
    cpu.update_negative_zero_flags(value);
    left.store_u8(cpu, value);
}

pub fn or(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    if let OperandDef::InMemory(AddressMode::XIndirect)
    | OperandDef::InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load_u8(cpu);
    let left = left.decode(cpu);
    let left_value = left.load_u8(cpu);
    let value = left_value | right_value;
    cpu.update_negative_zero_flags(value);
    left.store_u8(cpu, value);
}

pub fn bpl(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let offset = operand.load_u8(cpu) as i8;
    if !cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn bbs(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    let right = right.decode(cpu);
    let value = right.load_u8(cpu).bit(right.bit_idx());
    cpu.bus.cycle_io();
    let left = left.decode(cpu);
    let offset = left.load_u8(cpu) as i8;
    if value {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn bbc(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    let right = right.decode(cpu);
    let value = right.load_u8(cpu).bit(right.bit_idx());
    cpu.bus.cycle_io();
    let left = left.decode(cpu);
    let offset = left.load_u8(cpu) as i8;
    if !value {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn clr1(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let value = operand.load_u8(cpu).with_bit(operand.bit_idx(), false);
    operand.store_u8(cpu, value);
}

pub fn tset1(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
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

pub fn tcall(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    let addr = AddressU16(0xFFDE).sub(operand.load_u8(cpu) * 2, Wrap::NoWrap);
    cpu.pc = AddressU16(cpu.bus.cycle_read_u16(addr, Wrap::NoWrap));
}

pub fn incw(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let value = operand.load_u16(cpu).wrapping_add(1);
    cpu.update_negative_zero_flags(value);
    operand.store_u16(cpu, value);
}

pub fn bmi(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let offset = operand.load_u8(cpu) as i8;
    if cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn inc(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let value = operand.load_u8(cpu).wrapping_add(1);
    if let Operand::Register(_) = operand {
        // ASL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    cpu.update_negative_zero_flags(value);
    operand.store_u8(cpu, value);
}

pub fn call(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = operand.effective_addr().unwrap();
}

pub fn decw(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let value = operand.load_u16(cpu).wrapping_sub(1);
    cpu.update_negative_zero_flags(value);
    operand.store_u16(cpu, value);
}

pub fn dec(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let value = operand.load_u8(cpu).wrapping_sub(1);
    if let Operand::Register(_) = operand {
        // ASL on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    cpu.update_negative_zero_flags(value);
    operand.store_u8(cpu, value);
}

pub fn cmp(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    if let OperandDef::InMemory(AddressMode::XIndirect)
    | OperandDef::InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load_u8(cpu);
    let left = left.decode(cpu);
    let left_value = left.load_u8(cpu);
    let value = left_value.wrapping_sub(right_value);
    if let Operand::InMemory(_, _, _) = left {
        cpu.bus.cycle_io();
    }
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = left_value >= right_value;
}

pub fn jmp(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    cpu.pc = operand.effective_addr().unwrap();
}

pub fn rol(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
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

pub fn cbne(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    let right = right.decode(cpu);
    let value = right.load_u8(cpu);
    cpu.bus.cycle_io();
    let left = left.decode(cpu);
    let offset = left.load_u8(cpu) as i8;
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

pub fn bra(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let offset = operand.load_u8(cpu) as i8;
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
}

pub fn setp(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.direct_page = true;
}

pub fn eor(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    if let OperandDef::InMemory(AddressMode::XIndirect)
    | OperandDef::InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load_u8(cpu);
    let left = left.decode(cpu);
    let left_value = left.load_u8(cpu);
    let value = left_value ^ right_value;
    cpu.update_negative_zero_flags(value);
    left.store_u8(cpu, value);
}

pub fn and1(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let bit = operand.load_u8(cpu).bit(operand.bit_idx());
    cpu.status.carry = cpu.status.carry && bit;
}

pub fn lsr(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let value = operand.load_u8(cpu);
    if let Operand::Register(_) = operand {
        // LSR on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = value >> 1;
    cpu.status.carry = value.bit(0);
    cpu.update_negative_zero_flags(new_value);
    operand.store_u8(cpu, new_value);
}

pub fn ror(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let value = operand.load_u8(cpu);
    if let Operand::Register(_) = operand {
        // ROR on registers has an extra unused read cycle
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = (value >> 1) | (cpu.status.carry as u8) << 7;
    cpu.status.carry = value.bit(0);
    cpu.update_negative_zero_flags(new_value);
    operand.store_u8(cpu, new_value);
}

pub fn clrc(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.carry = false;
}

pub fn dbnz(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    let left = left.decode(cpu);
    let value = left.load_u8(cpu);
    let new_value = value.wrapping_sub(1);
    left.store_u8(cpu, new_value);
    let right = right.decode(cpu);
    let offset = right.load_u8(cpu) as i8;
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

pub fn tclr1(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let value = operand.load_u8(cpu);
    operand.load_u8(cpu); // CPU will re-read the value for another cycle
    operand.store_u8(cpu, value & !cpu.a);
    cpu.update_negative_zero_flags(cpu.a.wrapping_sub(value));
}

pub fn pcall(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    cpu.pc = AddressU16(0xFF00).add(operand.load_u8(cpu) as u16, Wrap::WrapPage);
}

pub fn bvc(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let offset = operand.load_u8(cpu) as i8;
    if !cpu.status.overflow {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn cmpw(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    let right = right.decode(cpu);
    let right_value = right.load_u16(cpu);
    let left = left.decode(cpu);
    let left_value = left.load_u16(cpu);
    let value = left_value.wrapping_sub(right_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = left_value >= right_value;
}

pub fn mov(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    let right = right.decode(cpu);
    let value = right.load_u8(cpu);
    let left = left.decode(cpu);

    if let Operand::Register(_) = left {
        // Moves into registers will update the N and Z flags
        cpu.update_negative_zero_flags(value);
        if let Operand::Register(_) = right {
            cpu.bus.cycle_read_u8(cpu.pc);
        }
    } else {
        // Moves into memory locations will read from the target address
        // first.
        if let Operand::InMemory(_, AddressMode::Dp, _) = right {
            // Somehow 0xFA (MOV from a direct page address into another) is an exception.
            cpu.bus.cycle_io();
        } else {
            left.load_u8(cpu);
        }
    }
    left.store_u8(cpu, value);
}

pub fn bvs(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let offset = operand.load_u8(cpu) as i8;
    if cpu.status.overflow {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn addw(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    let right = right.decode(cpu);
    let right_value = right.load_u16(cpu);
    let left = left.decode(cpu);
    let left_value = left.load_u16(cpu);
    if let Operand::Register(_) = left {
        cpu.bus.cycle_io();
    }

    let (value, carry) = left_value.overflowing_add(right_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = carry;
    cpu.status.overflow = ((right_value ^ value) & (left_value ^ value)).msb();
    cpu.status.half_carry = ((right_value & 0x0FFF) + (left_value & 0x0FFF)) > 0x0FFF;
    left.store_u16(cpu, value)
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

pub fn adc(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    if let OperandDef::InMemory(AddressMode::XIndirect)
    | OperandDef::InMemory(AddressMode::YIndirect) = right
    {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let right = right.decode(cpu);
    let right_value = right.load_u8(cpu);
    let left = left.decode(cpu);
    let left_value = left.load_u8(cpu);
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
    left.store_u8(cpu, value);
}

pub fn eor1(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let bit = operand.load_u8(cpu).bit(operand.bit_idx());
    cpu.bus.cycle_io();
    cpu.status.carry = cpu.status.carry ^ bit;
}

pub fn pop(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    let value = cpu.stack_pop_u8();
    operand.store_u8(cpu, value);
}

pub fn bcc(cpu: &mut Spc700<impl Spc700Bus>, operand: OperandDef) {
    let operand = operand.decode(cpu);
    let offset = operand.load_u8(cpu) as i8;
    if !cpu.status.carry {
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();
        cpu.pc = cpu.pc.add_signed(offset.into(), Wrap::NoWrap);
    }
}

pub fn subw(cpu: &mut Spc700<impl Spc700Bus>, left: OperandDef, right: OperandDef) {
    let right = right.decode(cpu);
    let right_value = right.load_u16(cpu);
    let left = left.decode(cpu);
    let left_value = left.load_u16(cpu);
    if let Operand::Register(_) = left {
        cpu.bus.cycle_io();
    }

    let (value, carry) = left_value.overflowing_sub(right_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !carry;
    cpu.status.overflow = ((right_value ^ value) & (!left_value ^ value)).msb();
    cpu.status.half_carry = ((right_value & 0x0FFF) - (left_value & 0x0FFF)) > 0x0FFF;
    left.store_u16(cpu, value);
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
