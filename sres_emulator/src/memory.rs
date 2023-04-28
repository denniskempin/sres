use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;

use intbits::Bits;

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
        let offset = self.offset + rhs as u16;
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
    fn peek<Addr: ToAddress>(&self, addr: Addr) -> Option<u8>;
    fn read<Addr: ToAddress>(&mut self, addr: Addr) -> u8;
    fn write<Addr: ToAddress>(&mut self, addr: Addr, value: u8);

    fn peek_u16<Addr: ToAddress>(&self, addr: Addr) -> Option<u16> {
        let addr = addr.to_address();
        Some(u16::from_le_bytes([self.peek(addr)?, self.peek(addr + 1)?]))
    }

    fn read_u16<Addr: ToAddress>(&mut self, addr: Addr) -> u16 {
        let addr = addr.to_address();
        u16::from_le_bytes([self.read(addr), self.read(addr + 1)])
    }

    fn write_u16(&mut self, addr: Address, value: u16) {
        let bytes = value.to_le_bytes();
        self.write(addr, bytes[0]);
        self.write(addr + 1, bytes[1]);
    }
}
