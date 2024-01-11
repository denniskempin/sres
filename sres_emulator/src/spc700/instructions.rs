use crate::bus::Address;
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
