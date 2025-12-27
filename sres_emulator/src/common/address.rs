//! Types for both U16 and U24 addresses used by the different CPUs
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;

use bitcode::Decode;
use bitcode::Encode;
use intbits::Bits;

use crate::common::uint::U16Ext;
use crate::common::uint::U32Ext;
use crate::common::uint::UIntTruncate;
pub use crate::common::uint::VariableLengthUInt;

/// Address types enforce that the wrapping behavior for each calculation is explicitly specified.
pub trait Address: Eq + Hash + Display + Ord + Copy + Clone + From<u32> + Into<u32> {
    fn add_signed(&self, rhs: i32, wrap: Wrap) -> Self;
    fn add<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self;
    fn add_detect_page_cross<T: UIntTruncate + Copy>(&self, rhs: T, wrap: Wrap) -> (bool, Self);
    fn sub<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self;
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Wrap {
    WrapPage,
    WrapBank,
    NoWrap,
}

/// Address type used by the main bus.
#[derive(Clone, Debug, Default, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct AddressU24 {
    pub bank: u8,
    pub offset: u16,
}

impl AddressU24 {
    pub fn new(bank: u8, offset: u16) -> Self {
        AddressU24 { bank, offset }
    }

    pub fn new_direct_page(bank: u8, page: u8, offset: u8) -> Self {
        AddressU24 {
            bank,
            offset: ((page as u16) << 8) | (offset as u16),
        }
    }
}

impl Address for AddressU24 {
    fn add_signed(&self, rhs: i32, wrap: Wrap) -> Self {
        if rhs > 0 {
            self.add(rhs.unsigned_abs(), wrap)
        } else {
            self.sub(rhs.unsigned_abs(), wrap)
        }
    }

    fn add<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapPage => AddressU24 {
                bank: self.bank,
                offset: (self.offset & 0xFF00)
                    + (self.offset as u8).wrapping_add(rhs.to_u8()) as u16,
            },
            Wrap::WrapBank => AddressU24 {
                bank: self.bank,
                offset: self.offset.wrapping_add(rhs.to_u16()),
            },
            Wrap::NoWrap => (u32::from(*self).wrapping_add(rhs.to_u32())).into(),
        }
    }

    fn add_detect_page_cross<T: UIntTruncate + Copy>(&self, rhs: T, wrap: Wrap) -> (bool, Self) {
        let page_cross = rhs.to_u8() as u16 + self.offset.to_u8() as u16 > 0xFF;
        (page_cross, self.add(rhs, wrap))
    }

    fn sub<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapPage => AddressU24 {
                bank: self.bank,
                offset: (self.offset & 0xFF00)
                    + (self.offset as u8).wrapping_sub(rhs.to_u8()) as u16,
            },
            Wrap::WrapBank => AddressU24 {
                bank: self.bank,
                offset: self.offset.wrapping_sub(rhs.to_u16()),
            },
            Wrap::NoWrap => (u32::from(*self).wrapping_sub(rhs.to_u32())).into(),
        }
    }
}

impl From<AddressU24> for u32 {
    #[inline]
    fn from(addr: AddressU24) -> Self {
        ((addr.bank as u32) << 16) | (addr.offset as u32)
    }
}

impl From<u32> for AddressU24 {
    #[inline]
    fn from(addr: u32) -> Self {
        AddressU24 {
            bank: addr.high_word().low_byte(),
            offset: addr.low_word(),
        }
    }
}

impl Display for AddressU24 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:02X}{:04X}", self.bank, self.offset)
    }
}

/// Address type used by the SPC700.
#[derive(Clone, Debug, Default, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct AddressU16(pub u16);

impl AddressU16 {
    pub fn new_direct_page(page: u8, offset: u8) -> Self {
        AddressU16(((page as u16) << 8) | (offset as u16))
    }
}

impl Address for AddressU16 {
    fn add_signed(&self, rhs: i32, wrap: Wrap) -> Self {
        if rhs > 0 {
            self.add(rhs.unsigned_abs(), wrap)
        } else {
            self.sub(rhs.unsigned_abs(), wrap)
        }
    }

    fn add<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapPage => {
                AddressU16((self.0 & 0xFF00) + (self.0 as u8).wrapping_add(rhs.to_u8()) as u16)
            }
            Wrap::NoWrap => Self(self.0.wrapping_add(rhs.to_u16())),
            Wrap::WrapBank => unimplemented!(),
        }
    }

    fn add_detect_page_cross<T: UIntTruncate + Copy>(&self, rhs: T, wrap: Wrap) -> (bool, Self) {
        let page_cross = rhs.to_u8() as u16 + self.0.to_u8() as u16 > 0xFF;
        (page_cross, self.add(rhs, wrap))
    }

    fn sub<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapPage => {
                AddressU16((self.0 & 0xFF00) + (self.0 as u8).wrapping_sub(rhs.to_u8()) as u16)
            }
            Wrap::NoWrap => Self(self.0.wrapping_sub(rhs.to_u16())),
            Wrap::WrapBank => unimplemented!(),
        }
    }
}

impl Display for AddressU16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:04X}", self.0)
    }
}

impl From<u32> for AddressU16 {
    #[inline]
    fn from(addr: u32) -> Self {
        AddressU16(addr as u16)
    }
}

impl From<AddressU16> for u32 {
    #[inline]
    fn from(addr: AddressU16) -> Self {
        addr.0 as u32
    }
}

#[derive(Copy, Clone, Debug, Default, Encode, Decode)]
pub struct AddressU15(pub u16);

impl AddressU15 {
    pub fn set_low_byte(&mut self, value: u8) {
        self.0.set_low_byte(value);
    }

    pub fn set_high_byte(&mut self, value: u8) {
        self.0.set_high_byte(value.bits(0..=6) & 0x7F);
    }
}

impl std::ops::Add<u16> for AddressU15 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Self(self.0.wrapping_add(rhs) & 0x7FFF)
    }
}

impl std::ops::Add<u32> for AddressU15 {
    type Output = Self;

    fn add(self, rhs: u32) -> Self {
        self + rhs as u16
    }
}

impl std::ops::Sub<u16> for AddressU15 {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Self(self.0.wrapping_sub(rhs) & 0x7FFF)
    }
}

impl From<u16> for AddressU15 {
    fn from(value: u16) -> Self {
        Self(value & 0x7FFF)
    }
}

impl From<u32> for AddressU15 {
    #[inline]
    fn from(addr: u32) -> Self {
        Self((addr as u16) & 0x7FFF)
    }
}

impl From<AddressU15> for usize {
    fn from(value: AddressU15) -> Self {
        value.0 as usize
    }
}

impl Display for AddressU15 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:04X}", self.0)
    }
}

/// Metadata about a decoded instruction at an address for disassembly.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct InstructionMeta<AddressT: Address> {
    pub address: AddressT,
    pub operation: String,
    pub operand_str: Option<String>,
    pub effective_addr_and_value: Option<(AddressT, VariableLengthUInt)>,
}
