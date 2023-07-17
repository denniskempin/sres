use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;

use crate::uint::RegisterSize;
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
        let offset = self.offset.wrapping_add(rhs as u16);
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

pub trait Memory {
    fn peek_u8(&self, addr: Address) -> Option<u8>;
    fn read_u8(&mut self, addr: Address) -> u8;
    fn write_u8(&mut self, addr: Address, value: u8);

    fn peek_u16(&self, addr: Address) -> Option<u16> {
        Some(u16::from_le_bytes([
            self.peek_u8(addr)?,
            self.peek_u8(addr.add2(1_u16, Wrap::NoWrap))?,
        ]))
    }
    fn read_u16(&mut self, addr: Address) -> u16 {
        u16::from_le_bytes([
            self.read_u8(addr),
            self.read_u8(addr.add2(1_u16, Wrap::NoWrap)),
        ])
    }

    fn read_u24(&mut self, addr: Address) -> u32 {
        u32::from_le_bytes([
            self.read_u8(addr),
            self.read_u8(addr.add2(1_u16, Wrap::NoWrap)),
            self.read_u8(addr.add2(2_u16, Wrap::NoWrap)),
            0,
        ])
    }

    fn write_u16(&mut self, addr: Address, value: u16) {
        let bytes = value.to_le_bytes();
        self.write_u8(addr.add2(1_u16, Wrap::NoWrap), bytes[1]);
        self.write_u8(addr, bytes[0]);
    }

    fn peek_u24(&self, addr: Address) -> Option<u32> {
        Some(u32::from_le_bytes([
            self.peek_u8(addr)?,
            self.peek_u8(addr.add2(1_u16, Wrap::NoWrap))?,
            self.peek_u8(addr.add2(2_u16, Wrap::NoWrap))?,
            0,
        ]))
    }

    #[inline]
    fn peek<T: UInt>(&self, addr: Address) -> Option<T> {
        match T::SIZE {
            RegisterSize::U8 => self.peek_u8(addr).map(T::from_u8),
            RegisterSize::U16 => self.peek_u16(addr).map(T::from_u16),
        }
    }

    #[inline]
    fn read_generic<T: UInt>(&mut self, addr: Address) -> T {
        match T::SIZE {
            RegisterSize::U8 => T::from_u8(self.read_u8(addr)),
            RegisterSize::U16 => T::from_u16(self.read_u16(addr)),
        }
    }

    #[inline]
    fn write_generic<T: UInt>(&mut self, addr: Address, value: T) {
        match T::SIZE {
            RegisterSize::U8 => self.write_u8(addr, value.to_u8()),
            RegisterSize::U16 => self.write_u16(addr, value.to_u16()),
        }
    }
}
