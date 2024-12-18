//! Generic Bus trait that can be used with both U16 and U24 addresses.

use std::ops::RangeInclusive;

use crate::common::address::Address;
use crate::common::address::Wrap;
use crate::common::uint::RegisterSize;
use crate::common::uint::UInt;

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

    fn peek_range(&self, range: RangeInclusive<u32>) -> Vec<u8> {
        range
            .into_iter()
            .map(|idx| self.peek_u8(AddressT::from(idx)).unwrap_or_default())
            .collect()
    }
}
