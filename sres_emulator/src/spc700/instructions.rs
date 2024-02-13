use intbits::Bits;

use super::operands::BitOperand;
use super::operands::DecodedOperand;
use super::operands::Operand;
use super::operands::U8Operand;
use super::AddressMode;
use super::Spc700StatusFlags;
use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::util::uint::U16Ext;
use crate::util::uint::U8Ext;
use crate::util::uint::UInt;

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

    let (mut result, mut carry) = left.overflowing_sub(right);
    if !cpu.status.carry {
        let (value2, carry2) = result.overflowing_sub(1);
        result = value2;
        carry = carry || carry2;
    }
    cpu.status.half_carry =
        (left & 0x0F).wrapping_sub((right & 0x0F) + !cpu.status.carry as u8) & 0x10 != 0x10;
    cpu.status.carry = !carry;
    cpu.status.overflow = ((left ^ right) & (left ^ result)).msb();
    cpu.update_negative_zero_flags(result);
    left_op.store(cpu, result);
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
    cpu.bus.cycle_read_u8(cpu.pc);
    for _ in 0..7 {
        cpu.bus.cycle_io();
    }
    let result = cpu.y as u16 * cpu.a as u16;
    cpu.a = result.low_byte();
    cpu.y = result.high_byte();
    cpu.update_negative_zero_flags(cpu.y);
}

pub fn div(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    for _ in 0..10 {
        cpu.bus.cycle_io();
    }

    let ya = u16::from_be_bytes([cpu.y, cpu.a]);
    let y = cpu.y as u16;
    let x = cpu.x as u16;

    cpu.status.half_carry = (y & 15) >= (x & 15);
    cpu.status.overflow = y >= x;

    // The logic for overflowing div's is weird, helpful explanation at:
    // https://helmet.kafuka.org/bboard/thread.php?id=228
    if y < (x << 1) {
        cpu.a = (ya / x) as u8;
        cpu.y = (ya % x) as u8;
    } else {
        cpu.a = 255_u16.wrapping_sub(ya.wrapping_sub(x << 9) / (256 - x)) as u8;
        cpu.y = (x + ((ya.wrapping_sub(x << 9)) % (256 - x))) as u8;
    }
    cpu.update_negative_zero_flags(cpu.a);
}

////////////////////////////////////////////////////////////////////////////////
// Shift instructions

enum ShiftType {
    Rol,
    Ror,
    Asl,
    Lsr,
}

#[inline]
fn shift(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand, shift_type: ShiftType) {
    let operand = operand.decode(cpu);
    let value = operand.load(cpu);
    if operand.is_alu_register() {
        cpu.bus.cycle_read_u8(cpu.pc);
    }
    let new_value = match shift_type {
        ShiftType::Rol => (value << 1).with_bit(0, cpu.status.carry),
        ShiftType::Ror => (value >> 1).with_bit(7, cpu.status.carry),
        ShiftType::Asl => value << 1,
        ShiftType::Lsr => value >> 1,
    };
    cpu.status.carry = match shift_type {
        ShiftType::Rol | ShiftType::Asl => value.bit(7),
        ShiftType::Ror | ShiftType::Lsr => value.bit(0),
    };
    cpu.update_negative_zero_flags(new_value);
    operand.store(cpu, new_value);
}

pub fn rol(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    shift(cpu, operand, ShiftType::Rol);
}

pub fn ror(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    shift(cpu, operand, ShiftType::Ror);
}

pub fn asl(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    shift(cpu, operand, ShiftType::Asl);
}

pub fn lsr(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    shift(cpu, operand, ShiftType::Lsr);
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

pub fn clrv(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.status.overflow = false;
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

pub fn jmp(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    cpu.pc = AddressU16(operand.decode(cpu).load_u16(cpu));
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

pub fn call(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.pc.0);
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = AddressU16(operand.load_u16(cpu));
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

#[inline]
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

pub fn beq(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    branch(cpu, operand, cpu.status.zero);
}

pub fn bne(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    branch(cpu, operand, !cpu.status.zero);
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
    if left_op.is_alu_register() {
        cpu.bus.cycle_read_u8(cpu.pc);
        cpu.bus.cycle_io();
    }
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

pub fn not1(cpu: &mut Spc700<impl Spc700Bus>, operand: BitOperand) {
    let operand = operand.decode(cpu);
    if operand.is_carry() {
        cpu.bus.cycle_read_u8(cpu.pc);
        cpu.bus.cycle_io();
    }
    // TODO: this stinks!
    let bit = operand.peek(cpu);
    operand.store(cpu, !bit);
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
    cpu.status.carry ^= bit;
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

pub fn addw(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let low = operand.load(cpu);
    cpu.bus.cycle_io();
    let high = operand.load_high(cpu);

    let right = u16::from_le_bytes([low, high]);
    let left = u16::from_le_bytes([cpu.a, cpu.y]);
    let (result, carry) = left.overflowing_add(right);
    cpu.update_negative_zero_flags(result);
    cpu.status.carry = carry;
    cpu.status.overflow = ((right ^ result) & (left ^ result)).msb();
    cpu.status.half_carry = ((right & 0x0FFF) + (left & 0x0FFF)) > 0x0FFF;
    cpu.a = result.low_byte();
    cpu.y = result.high_byte();
}

pub fn subw(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    let low = operand.load(cpu);
    cpu.bus.cycle_io();
    let high = operand.load_high(cpu);

    let right = u16::from_le_bytes([low, high]);
    let left = u16::from_le_bytes([cpu.a, cpu.y]);
    let (result, carry) = left.overflowing_sub(right);
    cpu.update_negative_zero_flags(result);
    cpu.status.carry = !carry;
    cpu.status.half_carry =
        (left & 0x0FFF).wrapping_sub((right & 0x0FFF) + !cpu.status.carry as u16) & 0x1000
            != 0x1000;
    cpu.status.overflow = ((left ^ right) & (left ^ result)).msb();
    cpu.a = result.low_byte();
    cpu.y = result.high_byte();
}

pub fn cmpw(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    let right = right_op.decode(cpu).load_u16(cpu);
    let left = left_op.decode(cpu).load_u16(cpu);
    let result = left.wrapping_sub(right);
    cpu.update_negative_zero_flags(result);
    cpu.status.carry = left >= right;
}

pub fn incw(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    // decrement low and high byte separately to match hardware read/write cycle order
    let (low, overflow) = operand.load(cpu).overflowing_add(1);
    operand.store(cpu, low);
    let high = operand.load_high(cpu).wrapping_add(overflow as u8);
    operand.store_high(cpu, high);
    cpu.update_negative_zero_flags(u16::from_le_bytes([low, high]));
}

pub fn decw(cpu: &mut Spc700<impl Spc700Bus>, operand: U8Operand) {
    let operand = operand.decode(cpu);
    // decrement low and high byte separately to match hardware read/write cycle order
    let (low, overflow) = operand.load(cpu).overflowing_sub(1);
    operand.store(cpu, low);
    let high = operand.load_high(cpu).wrapping_sub(overflow as u8);
    operand.store_high(cpu, high);
    cpu.update_negative_zero_flags(u16::from_le_bytes([low, high]));
}

pub fn movw(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    let right_op = right_op.decode(cpu);
    let left_op = left_op.decode(cpu);

    if left_op.is_alu_register() {
        // MOVW YA, dp
        // Loading the dp has an extra idle cycle between bytes.
        // Smell: Leaky abstraction
        let low = right_op.load(cpu);
        cpu.bus.cycle_io();
        let high = right_op.load_high(cpu);
        let right = u16::from_le_bytes([low, high]);
        cpu.update_negative_zero_flags(right);
        left_op.store_u16(cpu, right);
    } else {
        // MOVW dp, YA
        // Storing into the dp address will do an extra read from the target
        // address.
        // Smell: Leaky abstraction
        let right = right_op.load_u16(cpu);
        left_op.load(cpu);
        left_op.store_u16(cpu, right);
    };
}

////////////////////////////////////////////////////////////////////////////////
// Misc instructions

pub fn mov(cpu: &mut Spc700<impl Spc700Bus>, left_op: U8Operand, right_op: U8Operand) {
    let right = if right_op.is_address_mode(AddressMode::XIndirect) {
        // TODO: this stinks!
        let right_op = right_op.decode(cpu);
        cpu.bus.cycle_read_u8(cpu.pc);
        right_op.load(cpu)
    } else {
        right_op.decode(cpu).load(cpu)
    };
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
        } else if left_op.is_address_mode(AddressMode::DpIndirectYIdx) {
            left_op.load(cpu);
        } else if left_op.is_address_mode(AddressMode::XIndirect) {
            cpu.bus.cycle_read_u8(cpu.pc);
            left_op.load(cpu);
        } else if left_op.is_address_mode(AddressMode::XIndirectAutoInc) {
            // TODO: this stinks!
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
    cpu.bus.cycle_io();
    if cpu.status.carry || cpu.a > 0x99 {
        cpu.a = cpu.a.wrapping_add(0x60);
        cpu.status.carry = true;
    }
    if cpu.status.half_carry || cpu.a.low_nibble() > 0x09 {
        cpu.a = cpu.a.wrapping_add(0x06);
    }
    cpu.update_negative_zero_flags(cpu.a);
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

pub fn sleep(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
}

pub fn stop(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
    cpu.bus.cycle_read_u8(cpu.pc);
    cpu.bus.cycle_io();
}
