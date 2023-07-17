use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;

use intbits::Bits;

use crate::uint::RegisterSize;
use crate::uint::UInt;

#[derive(Clone, Debug, Default, PartialEq, Eq, Copy)]
pub struct Address {
    pub bank: u8,
    pub offset: u16,
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

pub trait ToAddress {
    fn to_address(self) -> Address;
}

impl ToAddress for Address {
    #[inline]
    fn to_address(self) -> Address {
        self
    }
}

impl ToAddress for u32 {
    #[inline]
    fn to_address(self) -> Address {
        let bank = self.bits(16..24) as u8;
        let offset = self.bits(0..16) as u16;
        Address { bank, offset }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:02X}{:04X}", self.bank, self.offset)
    }
}

pub trait Memory {
    fn peek_u8(&self, addr: impl ToAddress) -> Option<u8>;
    fn read_u8(&mut self, addr: impl ToAddress) -> u8;
    fn write_u8(&mut self, addr: impl ToAddress, value: u8);

    fn peek_u16(&self, addr: impl ToAddress) -> Option<u16> {
        let addr = addr.to_address();
        Some(u16::from_le_bytes([
            self.peek_u8(addr)?,
            self.peek_u8(u32::from(addr) + 1)?,
        ]))
    }
    fn read_u16(&mut self, addr: impl ToAddress) -> u16 {
        let addr = addr.to_address();
        u16::from_le_bytes([self.read_u8(addr), self.read_u8(u32::from(addr) + 1)])
    }

    fn read_u24(&mut self, addr: impl ToAddress) -> u32 {
        let addr = addr.to_address();
        u32::from_le_bytes([
            self.read_u8(addr),
            self.read_u8(u32::from(addr) + 1),
            self.read_u8(u32::from(addr) + 2),
            0,
        ])
    }

    fn write_u16(&mut self, addr: impl ToAddress, value: u16) {
        let addr = addr.to_address();
        let bytes = value.to_le_bytes();
        self.write_u8(u32::from(addr) + 1, bytes[1]);
        self.write_u8(addr, bytes[0]);
    }

    fn peek_u24(&self, addr: impl ToAddress) -> Option<u32> {
        let addr = addr.to_address();
        Some(u32::from_le_bytes([
            self.peek_u8(addr)?,
            self.peek_u8(u32::from(addr) + 1)?,
            self.peek_u8(u32::from(addr) + 2)?,
            0,
        ]))
    }

    #[inline]
    fn peek<T: UInt>(&self, addr: impl ToAddress) -> Option<T> {
        match T::SIZE {
            RegisterSize::U8 => self.peek_u8(addr).map(T::from_u8),
            RegisterSize::U16 => self.peek_u16(addr).map(T::from_u16),
        }
    }

    #[inline]
    fn read_generic<T: UInt>(&mut self, addr: impl ToAddress) -> T {
        match T::SIZE {
            RegisterSize::U8 => T::from_u8(self.read_u8(addr)),
            RegisterSize::U16 => T::from_u16(self.read_u16(addr)),
        }
    }

    #[inline]
    fn write_generic<T: UInt>(&mut self, addr: impl ToAddress, value: T) {
        match T::SIZE {
            RegisterSize::U8 => self.write_u8(addr, value.to_u8()),
            RegisterSize::U16 => self.write_u16(addr, value.to_u16()),
        }
    }
}
