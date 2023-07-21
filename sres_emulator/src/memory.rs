use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;

use crate::uint::U16Ext;
use crate::uint::U32Ext;
use crate::uint::UInt;

pub enum Wrap {
    WrapBank,
    NoWrap,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Copy)]
pub struct Address {
    pub bank: u8,
    pub offset: u16,
}

impl Address {
    pub fn new(bank: u8, offset: u16) -> Self {
        Address { bank, offset }
    }

    pub fn new_direct_page(bank: u8, page: u8, offset: u8) -> Self {
        Address {
            bank,
            offset: ((page as u16) << 8) | (offset as u16),
        }
    }

    pub fn add2<T: UInt>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapBank => Address {
                bank: self.bank,
                offset: self.offset.wrapping_add(rhs.to_u16()),
            },
            Wrap::NoWrap => (u32::from(*self).wrapping_add(rhs.to_u32())).into(),
        }
    }

    pub fn sub2<T: UInt>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapBank => Address {
                bank: self.bank,
                offset: self.offset.wrapping_sub(rhs.to_u16()),
            },
            Wrap::NoWrap => (u32::from(*self).wrapping_sub(rhs.to_u32())).into(),
        }
    }
}

impl Add<usize> for Address {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        let bank = self.bank;
        let offset = self.offset.wrapping_add(rhs as u16);
        Address { bank, offset }
    }
}

impl Add<u16> for Address {
    type Output = Self;

    #[inline]
    fn add(self, rhs: u16) -> Self::Output {
        let bank = self.bank;
        let offset = self.offset.wrapping_add(rhs);
        Address { bank, offset }
    }
}

impl Add<u32> for Address {
    type Output = Self;

    #[inline]
    fn add(self, rhs: u32) -> Self::Output {
        let bank = self.bank;
        let offset = self.offset.wrapping_add(rhs as u16);
        Address { bank, offset }
    }
}

impl Add<i32> for Address {
    type Output = Self;

    #[inline]
    fn add(self, rhs: i32) -> Self::Output {
        let bank = self.bank;
        let offset = self.offset.wrapping_add(rhs as u16);
        Address { bank, offset }
    }
}

impl From<Address> for u32 {
    #[inline]
    fn from(addr: Address) -> Self {
        (addr.bank as u32) << 16 | (addr.offset as u32)
    }
}

impl From<u32> for Address {
    #[inline]
    fn from(addr: u32) -> Self {
        Address {
            bank: addr.high_word().low_byte(),
            offset: addr.low_word(),
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:02X}{:04X}", self.bank, self.offset)
    }
}
