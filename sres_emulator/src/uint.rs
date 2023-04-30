use std::ops::BitXor;

use intbits::Bits;
use num_traits::ops::overflowing::OverflowingAdd;
use num_traits::ops::overflowing::OverflowingSub;
use num_traits::PrimInt;
use num_traits::WrappingAdd;
use num_traits::WrappingSub;

use crate::bus::Bus;
use crate::memory::Address;

pub trait UInt:
    PrimInt + OverflowingSub + OverflowingAdd + BitXor + WrappingAdd + WrappingSub
{
    const N_BITS: usize;
    const N_BYTES: usize;

    fn store_in_u16(self, target: &mut u16);
    fn from_u16(target: u16) -> Self;
    fn from_u8(target: u8) -> Self;
    fn peek_from_bus(bus: &mut impl Bus, addr: Address) -> Option<Self>;
    fn read_from_bus(bus: &mut impl Bus, addr: Address) -> Self;
    fn write_to_bus(&self, bus: &mut impl Bus, addr: Address);
    fn bit(&self, index: usize) -> bool;
}

impl UInt for u8 {
    fn store_in_u16(self, target: &mut u16) {
        target.set_bits(0..8, self as u16);
    }

    fn from_u16(target: u16) -> Self {
        target as u8
    }

    fn from_u8(target: u8) -> Self {
        target
    }

    fn read_from_bus(bus: &mut impl Bus, addr: Address) -> Self {
        bus.read(addr)
    }

    fn peek_from_bus(bus: &mut impl Bus, addr: Address) -> Option<Self> {
        bus.peek(addr)
    }

    const N_BITS: usize = 8;
    const N_BYTES: usize = 1;

    fn bit(&self, index: usize) -> bool {
        Bits::bit(*self, index)
    }

    fn write_to_bus(&self, bus: &mut impl Bus, addr: Address) {
        bus.write(addr, *self);
    }
}

impl UInt for u16 {
    fn store_in_u16(self, target: &mut u16) {
        *target = self
    }

    fn from_u16(target: u16) -> Self {
        target
    }

    fn from_u8(target: u8) -> Self {
        target as u16
    }

    fn peek_from_bus(bus: &mut impl Bus, addr: Address) -> Option<Self> {
        bus.peek_u16(addr)
    }

    fn read_from_bus(bus: &mut impl Bus, addr: Address) -> Self {
        bus.read_u16(addr)
    }

    const N_BITS: usize = 16;
    const N_BYTES: usize = 2;

    fn bit(&self, index: usize) -> bool {
        Bits::bit(*self, index)
    }

    fn write_to_bus(&self, bus: &mut impl Bus, addr: Address) {
        bus.write_u16(addr, *self);
    }
}
