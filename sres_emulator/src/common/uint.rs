//! Utility extensions for unsigned integers and generic traits to allow abstraction over
//! multiple types of unsigned integers.
use std::fmt::UpperHex;
use std::ops::BitXor;
use std::ops::Shl;

use intbits::Bits;
use num_traits::ops::overflowing::OverflowingAdd;
use num_traits::ops::overflowing::OverflowingSub;
use num_traits::PrimInt;
use num_traits::WrappingAdd;
use num_traits::WrappingSub;

pub enum RegisterSize {
    U8,
    U16,
}

pub trait UIntTruncate {
    // TODO: Rename to clarify these will truncate values
    fn to_u32(self) -> u32;
    fn to_u16(self) -> u16;
    fn to_u8(self) -> u8;

    fn from_u32(target: u32) -> Self;
    fn from_u16(target: u16) -> Self;
    fn from_u8(target: u8) -> Self;
}

pub trait UInt:
    PrimInt
    + OverflowingSub
    + OverflowingAdd
    + BitXor
    + WrappingAdd
    + WrappingSub
    + Shl
    + UIntTruncate
    + UpperHex
{
    const N_BITS: usize;
    const N_BYTES: usize;
    const SIZE: RegisterSize;

    fn bit(&self, index: usize) -> bool;
    fn set_bit(&mut self, index: usize, value: bool);

    fn msb(&self) -> bool {
        self.bit(Self::N_BITS - 1)
    }
    fn lsb(&self) -> bool {
        self.bit(0)
    }

    fn add_bcd(&self, rhs: Self, carry: bool) -> (Self, bool, bool);
    fn sub_bcd(&self, rhs: Self, carry: bool) -> (Self, bool, bool);
}

impl UIntTruncate for u8 {
    #[inline]
    fn from_u32(target: u32) -> Self {
        target as u8
    }

    #[inline]
    fn from_u16(target: u16) -> Self {
        target as u8
    }

    #[inline]
    fn from_u8(target: u8) -> Self {
        target
    }

    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }

    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }

    #[inline]
    fn to_u8(self) -> u8 {
        self
    }
}

impl UInt for u8 {
    const N_BITS: usize = 8;
    const N_BYTES: usize = 1;
    const SIZE: RegisterSize = RegisterSize::U8;

    fn bit(&self, index: usize) -> bool {
        Bits::bit(*self, index)
    }

    fn set_bit(&mut self, index: usize, value: bool) {
        Bits::set_bit(self, index, value);
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

    fn sub_bcd(&self, rhs: Self, carry: bool) -> (Self, bool, bool) {
        let lhs = *self as i16;
        let rhs = rhs as i16 ^ 0xFF;

        let mut result = (lhs & 0x0f) + (rhs & 0x0f) + carry as i16;
        if result <= 0x0f {
            result -= 0x06
        };
        let carry = result > 0x0f;

        result = (lhs & 0xf0) + (rhs & 0xf0) + ((carry as i16) << 4) + (result & 0x0f);
        let overflow = !(lhs ^ rhs) & (lhs ^ result) & 0x80 != 0;
        if result <= 0xff {
            result -= 0x60
        }
        let carry = result > 0xff;
        (result as u8, overflow, carry)
    }
}

impl UIntTruncate for u16 {
    #[inline]
    fn from_u32(target: u32) -> Self {
        target as u16
    }

    #[inline]
    fn from_u16(target: u16) -> Self {
        target
    }

    #[inline]
    fn from_u8(target: u8) -> Self {
        target as u16
    }

    #[inline]
    fn to_u32(self) -> u32 {
        self as u32
    }

    #[inline]
    fn to_u16(self) -> u16 {
        self
    }

    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}

impl UInt for u16 {
    const N_BITS: usize = 16;
    const N_BYTES: usize = 2;
    const SIZE: RegisterSize = RegisterSize::U16;

    fn bit(&self, index: usize) -> bool {
        Bits::bit(*self, index)
    }

    fn set_bit(&mut self, index: usize, value: bool) {
        Bits::set_bit(self, index, value);
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

    fn sub_bcd(&self, rhs: Self, carry: bool) -> (Self, bool, bool) {
        let lhs = *self as i32;
        let rhs = rhs as i32 ^ 0xffff;

        let mut result = (lhs & 0x000f) + (rhs & 0x000f) + carry as i32;
        if result <= 0x000f {
            result -= 0x0006;
        };
        let carry = result > 0x000f;

        result = (lhs & 0x00f0) + (rhs & 0x00f0) + ((carry as i32) << 4) + (result & 0x000f);
        if result <= 0x00ff {
            result -= 0x0060;
        };
        let carry = result > 0x00ff;

        result = (lhs & 0x0f00) + (rhs & 0x0f00) + ((carry as i32) << 8) + (result & 0x00ff);
        if result <= 0x0fff {
            result -= 0x0600;
        };
        let carry = result > 0x0fff;

        result = (lhs & 0xf000) + (rhs & 0xf000) + ((carry as i32) << 12) + (result & 0x0fff);
        let overflow = !(lhs ^ rhs) & (lhs ^ result) & 0x8000 != 0;
        if result <= 0xffff {
            result -= 0x6000;
        }
        let carry = result > 0xffff;
        (result as u16, overflow, carry)
    }
}

impl UIntTruncate for u32 {
    #[inline]
    fn from_u32(target: u32) -> Self {
        target
    }

    #[inline]
    fn from_u16(target: u16) -> Self {
        target as u32
    }

    #[inline]
    fn from_u8(target: u8) -> Self {
        target as u32
    }

    #[inline]
    fn to_u32(self) -> u32 {
        self
    }

    #[inline]
    fn to_u16(self) -> u16 {
        self as u16
    }

    #[inline]
    fn to_u8(self) -> u8 {
        self as u8
    }
}

pub trait U32Ext {
    fn low_word(self) -> u16;
    fn high_word(self) -> u16;
}

impl U32Ext for u32 {
    #[inline]
    fn low_word(self) -> u16 {
        self as u16
    }

    #[inline]
    fn high_word(self) -> u16 {
        (self >> 16) as u16
    }
}

pub trait U16Ext {
    fn low_byte(self) -> u8;
    fn high_byte(self) -> u8;
    fn set_low_byte(&mut self, low: u8);
    fn set_high_byte(&mut self, high: u8);
    fn with_low_byte(self, low: u8) -> u16;
    fn with_high_byte(self, high: u8) -> u16;
}

impl U16Ext for u16 {
    #[inline]
    fn low_byte(self) -> u8 {
        self as u8
    }

    #[inline]
    fn high_byte(self) -> u8 {
        (self >> 8) as u8
    }

    #[inline]
    fn set_low_byte(&mut self, low: u8) {
        *self = (*self & 0xFF00) | (low as u16);
    }

    #[inline]
    fn set_high_byte(&mut self, high: u8) {
        *self = (*self & 0x00FF) | ((high as u16) << 8);
    }

    #[inline]
    fn with_low_byte(self, low: u8) -> u16 {
        (self & 0xFF00) | (low as u16)
    }

    #[inline]
    fn with_high_byte(self, high: u8) -> u16 {
        (self & 0x00FF) | ((high as u16) << 8)
    }
}

pub trait U8Ext {
    fn low_nibble(self) -> u8;
    fn high_nibble(self) -> u8;
    fn set_low_nibble(&mut self, low: u8);
    fn set_high_nibble(&mut self, high: u8);
    fn with_low_nibble(self, low: u8) -> u8;
    fn with_high_nibble(self, high: u8) -> u8;
}

impl U8Ext for u8 {
    #[inline]
    fn low_nibble(self) -> u8 {
        self & 0x0f
    }

    #[inline]
    fn high_nibble(self) -> u8 {
        (self >> 4) & 0x0f
    }

    #[inline]
    fn set_low_nibble(&mut self, low: u8) {
        *self = *self & 0xf0 | low & 0x0f;
    }

    #[inline]
    fn set_high_nibble(&mut self, high: u8) {
        *self = (*self & 0x0f) | ((high & 0x0f) << 4);
    }

    #[inline]
    fn with_low_nibble(self, low: u8) -> u8 {
        (self & 0xf0) | (low & 0x0f)
    }

    #[inline]
    fn with_high_nibble(self, high: u8) -> u8 {
        ((high & 0x0f) << 4) | (self & 0x0f)
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use super::*;

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
