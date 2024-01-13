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
    let value = operand.load::<u8>(cpu);
    let new_value = value << 1;
    cpu.status.carry = value.bit(7);
    cpu.update_negative_zero_flags(new_value);
    operand.store::<u8>(cpu, new_value);
}

fn push(cpu: &mut Spc700<impl Spc700Bus>, value: u8) {
    cpu.bus.cycle_read_u8(cpu.pc.add(1_u8, Wrap::NoWrap));
    cpu.stack_push_u8(value);
    cpu.bus.cycle_io();
}

pub fn push_psw(cpu: &mut Spc700<impl Spc700Bus>) {
    push(cpu, cpu.status.into());
}

pub fn tset1(cpu: &mut Spc700<impl Spc700Bus>, operand: Operand) {
    let value = operand.load::<u8>(cpu);
    operand.load::<u8>(cpu); // CPU will re-read the value for another cycle
    operand.store::<u8>(cpu, value | cpu.a);
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
