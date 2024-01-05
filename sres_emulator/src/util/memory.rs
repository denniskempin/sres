use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::io::BufWriter;
use std::io::Write;

use itertools::Itertools;

use super::uint::RegisterSize;
use super::uint::UInt;
use crate::util::uint::U16Ext;
use crate::util::uint::U32Ext;
use crate::util::uint::UIntTruncate;

pub trait Address: Eq + Hash + Display + Ord + Copy + Clone {
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
        (addr.bank as u32) << 16 | (addr.offset as u32)
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

/// Implements a sparse memory HashMap with a readable display format.
#[derive(Default, PartialEq)]
pub struct SparseMemory<AddressT: Address> {
    pub memory: HashMap<AddressT, u8>,
}

impl<AddressT: Address> SparseMemory<AddressT> {
    pub fn get(&self, addr: AddressT) -> Option<u8> {
        self.memory.get(&addr).copied()
    }

    pub fn set(&mut self, addr: AddressT, value: u8) {
        self.memory.insert(addr, value);
    }
}

impl<AddressT: Address> Display for SparseMemory<AddressT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (addr, value) in self.memory.iter().sorted() {
            writeln!(f, "{}: {:02X}", addr, value)?;
        }
        Ok(())
    }
}

pub fn format_memory(memory: &[u8]) -> String {
    let mut writer = BufWriter::new(Vec::new());
    for chunks in memory.chunks(16) {
        for chunk in chunks {
            write!(&mut writer, "{:02X} ", *chunk).unwrap();
        }
        writeln!(&mut writer).unwrap();
    }

    let bytes = writer.into_inner().unwrap();
    String::from_utf8(bytes).unwrap()
}

pub fn format_memory_u16(memory: &[u16]) -> String {
    let mut writer = BufWriter::new(Vec::new());
    for chunks in memory.chunks(16) {
        for chunk in chunks {
            write!(&mut writer, "{:04X} ", *chunk).unwrap();
        }
        writeln!(&mut writer).unwrap();
    }

    let bytes = writer.into_inner().unwrap();
    String::from_utf8(bytes).unwrap()
}

/// Generic trait shared by all bus implementations.
pub trait Bus<AddressT: Address> {
    fn peek_u8(&self, addr: AddressT) -> Option<u8>;
    fn cycle_io(&mut self);
    fn cycle_read_u8(&mut self, addr: AddressT) -> u8;
    fn cycle_write_u8(&mut self, addr: AddressT, value: u8);
    fn reset(&mut self);

    #[inline]
    fn cycle_read_u16(&mut self, addr: AddressT, wrap: Wrap) -> u16 {
        u16::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add(1_u16, wrap)),
        ])
    }

    #[inline]
    fn cycle_write_u16(&mut self, addr: AddressT, value: u16, wrap: Wrap) {
        let bytes = value.to_le_bytes();
        self.cycle_write_u8(addr, bytes[0]);
        self.cycle_write_u8(addr.add(1_u16, wrap), bytes[1]);
    }

    #[inline]
    fn peek_u16(&self, addr: AddressT, wrap: Wrap) -> Option<u16> {
        Some(u16::from_le_bytes([
            self.peek_u8(addr)?,
            self.peek_u8(addr.add(1_u16, wrap))?,
        ]))
    }

    #[inline]
    fn cycle_read_generic<T: UInt>(&mut self, addr: AddressT, wrap: Wrap) -> T {
        match T::SIZE {
            RegisterSize::U8 => T::from_u8(self.cycle_read_u8(addr)),
            RegisterSize::U16 => T::from_u16(self.cycle_read_u16(addr, wrap)),
        }
    }

    #[inline]
    fn cycle_write_generic<T: UInt>(&mut self, addr: AddressT, value: T, wrap: Wrap) {
        match T::SIZE {
            RegisterSize::U8 => self.cycle_write_u8(addr, value.to_u8()),
            RegisterSize::U16 => self.cycle_write_u16(addr, value.to_u16(), wrap),
        }
    }
}
