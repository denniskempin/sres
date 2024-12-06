//! Generic Bus trait that can be used with both U16 and U24 addresses.

use std::ops::RangeInclusive;

use crate::common::address::Address;
use crate::common::address::AddressU24;
use crate::common::address::Wrap;
use crate::common::clock::ClockInfo;
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

pub trait BusDeviceU24 {
    fn peek(&self, addr: AddressU24) -> Option<u8>;
    fn read(&mut self, addr: AddressU24) -> u8;
    fn write(&mut self, addr: AddressU24, value: u8);
    fn update_clock(&mut self, new_clock: ClockInfo);
    fn reset(&mut self);
}

const CACHE_SIZE: usize = 32 * 1024;

enum BusAction {
    Clock(ClockInfo),
    Write(AddressU24, u8),
}

pub struct BatchedBusDeviceU24<DeviceT: BusDeviceU24> {
    pub inner: DeviceT,
    cache: Vec<BusAction>,
}

impl<DeviceT: BusDeviceU24> BusDeviceU24 for BatchedBusDeviceU24<DeviceT> {
    fn peek(&self, addr: AddressU24) -> Option<u8> {
        self.inner.peek(addr)
    }

    fn read(&mut self, addr: AddressU24) -> u8 {
        self.drain_cache();
        self.inner.read(addr)
    }

    fn write(&mut self, addr: AddressU24, value: u8) {
        if self.cache.len() >= CACHE_SIZE {
            self.drain_cache();
        }
        self.cache.push(BusAction::Write(addr, value))
    }

    fn update_clock(&mut self, new_clock: ClockInfo) {
        if self.cache.len() >= CACHE_SIZE {
            self.drain_cache();
        }
        self.cache.push(BusAction::Clock(new_clock));
    }

    fn reset(&mut self) {
        self.cache.clear();
        self.inner.reset()
    }
}

impl<DeviceT: BusDeviceU24> BatchedBusDeviceU24<DeviceT> {
    pub fn new(inner: DeviceT) -> Self {
        Self {
            inner,
            cache: Vec::with_capacity(CACHE_SIZE),
        }
    }

    pub fn drain_cache(&mut self) {
        for action in self.cache.drain(..) {
            match action {
                BusAction::Clock(new_clock) => self.inner.update_clock(new_clock),
                BusAction::Write(addr, value) => self.inner.write(addr, value),
            }
        }
    }
}
