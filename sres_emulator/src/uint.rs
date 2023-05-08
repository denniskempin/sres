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
    fn from_u32(target: u32) -> Self;
    fn from_u16(target: u16) -> Self;
    fn from_u8(target: u8) -> Self;
    fn peek_from_bus(bus: &mut impl Bus, addr: Address) -> Option<Self>;
    fn read_from_bus(bus: &mut impl Bus, addr: Address) -> Self;
    fn write_to_bus(&self, bus: &mut impl Bus, addr: Address);
    fn bit(&self, index: usize) -> bool;

    fn add_bcd(&self, rhs: Self, carry: bool) -> (Self, bool, bool);
}

impl UInt for u8 {
    fn store_in_u16(self, target: &mut u16) {
        target.set_bits(0..8, self as u16);
    }

    fn from_u32(target: u32) -> Self {
        target as u8
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

    fn add_bcd(&self, rhs: Self, carry: bool) -> (Self, bool, bool) {
        let lhs = *self as u16;
        let rhs = rhs as u16;

        let mut result = (lhs & 0x0f) + (rhs & 0x0f) + carry as u16;
        if result > 0x09 {
            result += 0x06;
        };
        let carry = result > 0x0f;

        result = (lhs & 0xf0) + (rhs & 0xf0) + ((carry as u16) << 4) + (result & 0x0f);
        let overflow = !(lhs ^ rhs) & (lhs ^ result) & 0x80 != 0;
        if result > 0x9f {
            result += 0x60;
        }
        let carry = result > 0xff;
        (result as u8, overflow, carry)
    }
}

impl UInt for u16 {
    fn store_in_u16(self, target: &mut u16) {
        *target = self
    }
    fn from_u32(target: u32) -> Self {
        target as u16
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

    fn add_bcd(&self, rhs: Self, carry: bool) -> (Self, bool, bool) {
        let lhs = *self as u32;
        let rhs = rhs as u32;

        let mut result = (lhs & 0x000f) + (rhs & 0x000f) + carry as u32;
        if result > 0x0009 {
            result += 0x0006;
        };
        let carry = result > 0x000f;

        result = (lhs & 0x00f0) + (rhs & 0x00f0) + ((carry as u32) << 4) + (result & 0x000f);
        if result > 0x009f {
            result += 0x0060;
        };
        let carry = result > 0x00ff;

        result = (lhs & 0x0f00) + (rhs & 0x0f00) + ((carry as u32) << 8) + (result & 0x00ff);
        if result > 0x09ff {
            result += 0x0600;
        };
        let carry = result > 0x0fff;

        result = (lhs & 0xf000) + (rhs & 0xf000) + ((carry as u32) << 12) + (result & 0x0fff);
        let overflow = !(lhs ^ rhs) & (lhs ^ result) & 0x8000 != 0;
        if result > 0x9fff {
            result += 0x6000;
        }
        let carry = result > 0xffff;
        (result as u16, overflow, carry)
    }
}

#[cfg(test)]
mod tests {
    use crate::uint::UInt;

    #[test]
    fn u8_bcd_add() {
        let (result, _, carry) = 0x01_u8.add_bcd(0x01, false);
        assert_eq!(result, 0x02);
        assert_eq!(carry, false);

        let (result, _, carry) = 0x01_u8.add_bcd(0x08, false);
        assert_eq!(result, 0x09);
        assert_eq!(carry, false);

        let (result, _, carry) = 0x01_u8.add_bcd(0x09, false);
        assert_eq!(result, 0x10);
        assert_eq!(carry, false);

        let (result, _, carry) = 0x10_u8.add_bcd(0x01, false);
        assert_eq!(result, 0x11);
        assert_eq!(carry, false);

        let (result, _, carry) = 0x19_u8.add_bcd(0x01, false);
        assert_eq!(result, 0x20);
        assert_eq!(carry, false);

        let (result, _, carry) = 0x99_u8.add_bcd(0x01, false);
        assert_eq!(result, 0x00);
        assert_eq!(carry, true);

        let (result, _, carry) = 0x90_u8.add_bcd(0x10, false);
        assert_eq!(result, 0x00);
        assert_eq!(carry, true);

        let (result, _, carry) = 0x90_u8.add_bcd(0x15, false);
        assert_eq!(result, 0x05);
        assert_eq!(carry, true);

        let (result, _, carry) = 0x99_u8.add_bcd(0x99, false);
        assert_eq!(result, 0x98);
        assert_eq!(carry, true);
    }

    #[test]
    fn u16_bcd_add() {
        let (result, _, carry) = 0x1234_u16.add_bcd(0x0001, false);
        assert_eq!(result, 0x1235);
        assert_eq!(carry, false);

        let (result, _, carry) = 0x0099_u16.add_bcd(0x0001, false);
        assert_eq!(result, 0x0100);
        assert_eq!(carry, false);

        let (result, _, carry) = 0x9999_u16.add_bcd(0x0001, false);
        assert_eq!(result, 0x0000);
        assert_eq!(carry, true);

        let (result, _, carry) = 0x9999_u16.add_bcd(0x1112, false);
        assert_eq!(result, 0x1111);
        assert_eq!(carry, true);

        let (result, _, carry) = 0x9999_u16.add_bcd(0x9999, false);
        assert_eq!(result, 0x9998);
        assert_eq!(carry, true);
    }
}
