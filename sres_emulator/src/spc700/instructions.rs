use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;

use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;

pub fn nop(cpu: &mut Spc700<impl Spc700Bus>) {
    cpu.bus.cycle_read_u8(cpu.pc.add(1_u8, Wrap::NoWrap));
}
