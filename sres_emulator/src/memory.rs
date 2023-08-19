use std::fmt::Display;
use std::fmt::Formatter;
use std::io::BufWriter;
use std::io::Write;

use crate::uint::U16Ext;
use crate::uint::U32Ext;
use crate::uint::UIntTruncate;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Wrap {
    WrapPage,
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

    pub fn add_signed(&self, rhs: i32, wrap: Wrap) -> Self {
        if rhs > 0 {
            self.add(rhs.unsigned_abs(), wrap)
        } else {
            self.sub(rhs.unsigned_abs(), wrap)
        }
    }

    pub fn add<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapPage => Address {
                bank: self.bank,
                offset: (self.offset & 0xFF00)
                    + (self.offset as u8).wrapping_add(rhs.to_u8()) as u16,
            },
            Wrap::WrapBank => Address {
                bank: self.bank,
                offset: self.offset.wrapping_add(rhs.to_u16()),
            },
            Wrap::NoWrap => (u32::from(*self).wrapping_add(rhs.to_u32())).into(),
        }
    }

    pub fn add_detect_page_cross<T: UIntTruncate + Copy>(
        &self,
        rhs: T,
        wrap: Wrap,
    ) -> (Self, bool) {
        let page_cross = rhs.to_u8() as u16 + self.offset.to_u8() as u16 > 0xFF;
        (self.add(rhs, wrap), page_cross)
    }

    pub fn sub<T: UIntTruncate>(&self, rhs: T, wrap: Wrap) -> Self {
        match wrap {
            Wrap::WrapPage => Address {
                bank: self.bank,
                offset: (self.offset & 0xFF00)
                    + (self.offset as u8).wrapping_sub(rhs.to_u8()) as u16,
            },
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
