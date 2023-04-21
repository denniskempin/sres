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

    fn add(self, rhs: usize) -> Self::Output {
        let bank = self.bank;
        let offset = self.offset + rhs as u16;
        Address { bank, offset }
    }
}

impl From<Address> for u32 {
    fn from(addr: Address) -> Self {
        (addr.bank as u32) << 16 | (addr.offset as u32)
    }
}

pub trait ToAddress {
    fn to_address(self) -> Address;
}

impl ToAddress for Address {
    fn to_address(self) -> Address {
        self
    }
}

impl ToAddress for u32 {
    fn to_address(self) -> Address {
        let bank = self.bits(16..24) as u8;
        let offset = self.bits(0..16) as u16;
        Address { bank, offset }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:02X}:{:04X}", self.bank, self.offset)
    }
}

pub trait Memory {
    fn peek<Addr: ToAddress>(&self, addr: Addr) -> Option<u8>;
    fn read<Addr: ToAddress>(&mut self, addr: Addr) -> u8;
    fn write<Addr: ToAddress>(&mut self, addr: Addr, value: u8);
}
