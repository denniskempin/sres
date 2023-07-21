use std::fmt::Display;
use std::fmt::Formatter;

use crate::uint::U16Ext;
use crate::uint::U32Ext;
use crate::uint::UIntTruncate;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
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

    pub fn add<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapBank => Address {
                bank: self.bank,
                offset: self.offset.wrapping_add(rhs.to_u16()),
            },
            Wrap::NoWrap => (u32::from(*self).wrapping_add(rhs.to_u32())).into(),
        }
    }

    pub fn sub<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapBank => Address {
                bank: self.bank,
                offset: self.offset.wrapping_sub(rhs.to_u16()),
            },
            Wrap::NoWrap => (u32::from(*self).wrapping_sub(rhs.to_u32())).into(),
        }
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
