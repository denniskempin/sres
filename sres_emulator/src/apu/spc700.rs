use std::fmt::Display;

use intbits::Bits;

use crate::debugger::DebuggerRef;
use crate::util::memory::AddressU16;
use crate::util::memory::Wrap;

// TODO: Consider sharing a generic bus trait with cpu-specific extensions in a separate trait
pub trait Spc700Bus {
    fn peek_u8(&self, addr: AddressU16) -> Option<u8>;
    fn cycle_io(&mut self);
    fn cycle_read_u8(&mut self, addr: AddressU16) -> u8;
    fn cycle_write_u8(&mut self, addr: AddressU16, value: u8);
    fn reset(&mut self);

    #[inline]
    fn cycle_read_u16(&mut self, addr: AddressU16, wrap: Wrap) -> u16 {
        u16::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add(1_u16, wrap)),
        ])
    }

    #[inline]
    fn cycle_write_u16(&mut self, addr: AddressU16, value: u16, wrap: Wrap) {
        let bytes = value.to_le_bytes();
        self.cycle_write_u8(addr, bytes[0]);
        self.cycle_write_u8(addr.add(1_u16, wrap), bytes[1]);
    }

    #[inline]
    fn peek_u16(&self, addr: AddressU16, wrap: Wrap) -> Option<u16> {
        Some(u16::from_le_bytes([
            self.peek_u8(addr)?,
            self.peek_u8(addr.add(1_u16, wrap))?,
        ]))
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Spc700StatusFlags {
    pub carry: bool,
    pub zero: bool,
    pub irq_enable: bool,
    pub half_carry: bool,
    pub break_command: bool,
    pub direct_page: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl From<u8> for Spc700StatusFlags {
    fn from(value: u8) -> Self {
        Self {
            carry: value.bit(0),
            zero: value.bit(1),
            irq_enable: value.bit(2),
            half_carry: value.bit(3),
            break_command: value.bit(4),
            direct_page: value.bit(5),
            overflow: value.bit(6),
            negative: value.bit(7),
        }
    }
}

impl From<Spc700StatusFlags> for u8 {
    fn from(value: Spc700StatusFlags) -> Self {
        0_u8.with_bit(0, value.carry)
            .with_bit(1, value.zero)
            .with_bit(2, value.irq_enable)
            .with_bit(3, value.half_carry)
            .with_bit(4, value.break_command)
            .with_bit(5, value.direct_page)
            .with_bit(6, value.overflow)
            .with_bit(7, value.negative)
    }
}

impl Display for Spc700StatusFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            if self.carry { "C" } else { "." },
            if self.zero { "Z" } else { "." },
            if self.irq_enable { "I" } else { "." },
            if self.half_carry { "H" } else { "." },
            if self.break_command { "B" } else { "." },
            if self.direct_page { "D" } else { "." },
            if self.overflow { "V" } else { "." },
            if self.negative { "N" } else { "." },
        )
    }
}

pub struct Spc700<BusT: Spc700Bus> {
    pub bus: BusT,
    pub pc: AddressU16,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub dsw: u8,
    pub status: Spc700StatusFlags,
    pub master_cycle: u64,
    pub debugger: DebuggerRef,
}

impl<BusT: Spc700Bus> Spc700<BusT> {
    pub fn new(bus: BusT, debugger: DebuggerRef) -> Self {
        let mut cpu = Self {
            bus,
            pc: AddressU16(0),
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            dsw: 0,
            status: Spc700StatusFlags::default(),
            master_cycle: 0,
            debugger,
        };
        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        self.pc = AddressU16(0);
    }

    pub fn step(&mut self) {}
}

impl<BusT: Spc700Bus> Display for Spc700<BusT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} A:{:02X} X:{:02X} Y:{:02X} SP:{:02X} DSW:{:02X} {}",
            self.pc, self.a, self.x, self.y, self.sp, self.dsw, self.status,
        )
    }
}
