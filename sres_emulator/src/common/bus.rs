//! Generic Bus trait that can be used with both U16 and U24 addresses.

use std::ops::RangeInclusive;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::common::address::Address;
use crate::common::address::AddressU24;
use crate::common::address::Wrap;
use crate::common::clock::ClockInfo;
use crate::common::uint::UInt;
use crate::common::uint::UIntSize;

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
            UIntSize::U8 => T::from_u8(self.cycle_read_u8(addr)),
            UIntSize::U16 => T::from_u16(self.cycle_read_u16(addr, wrap)),
        }
    }

    #[inline]
    fn cycle_write_generic<T: UInt>(&mut self, addr: AddressT, value: T, wrap: Wrap) {
        match T::SIZE {
            UIntSize::U8 => self.cycle_write_u8(addr, value.to_u8()),
            UIntSize::U16 => self.cycle_write_u16(addr, value.to_u16(), wrap),
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
    const NAME: &'static str;
    fn peek(&self, addr: AddressU24) -> Option<u8>;
    fn read(&mut self, addr: AddressU24) -> u8;
    fn write(&mut self, addr: AddressU24, value: u8);
    fn update_clock(&mut self, new_clock: ClockInfo);
    fn reset(&mut self);
}

const CACHE_SIZE: usize = 32 * 1024;

enum BusAction {
    Clock(ClockInfo),
    Write(ClockInfo, AddressU24, u8),
}

pub struct BatchedBusDeviceU24<DeviceT: BusDeviceU24> {
    pub inner: DeviceT,
    cache: Vec<BusAction>,
    inner_clock: ClockInfo,
    current_clock: ClockInfo,
}

impl<DeviceT: BusDeviceU24> BusDeviceU24 for BatchedBusDeviceU24<DeviceT> {
    const NAME: &'static str = DeviceT::NAME;

    fn peek(&self, addr: AddressU24) -> Option<u8> {
        self.inner.peek(addr)
    }

    fn read(&mut self, addr: AddressU24) -> u8 {
        self.sync();
        self.inner.read(addr)
    }

    fn write(&mut self, addr: AddressU24, value: u8) {
        if self.cache.len() >= CACHE_SIZE {
            self.sync();
        }
        self.cache
            .push(BusAction::Write(self.current_clock, addr, value))
    }

    fn update_clock(&mut self, new_clock: ClockInfo) {
        if new_clock.master_clock - self.inner_clock.master_clock > 1024 {
            self.cache.push(BusAction::Clock(new_clock));
            self.inner_clock = new_clock;
        }
        self.current_clock = new_clock;
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
            current_clock: ClockInfo::default(),
            inner_clock: ClockInfo::default(),
        }
    }

    pub fn sync(&mut self) {
        let cache_size = format!("{} {}", DeviceT::NAME, self.cache.len());
        puffin::profile_function!(&cache_size);
        for action in self.cache.drain(..) {
            match action {
                BusAction::Clock(clock) => {
                    self.inner.update_clock(clock);
                }
                BusAction::Write(clock, addr, value) => {
                    self.inner.update_clock(clock);
                    self.inner.write(addr, value);
                }
            }
        }
        self.inner.update_clock(self.current_clock);
    }
}

pub struct AsyncBusDeviceU24<DeviceT: BusDeviceU24 + Send + 'static> {
    pub inner: Arc<Mutex<DeviceT>>,

    sender: SyncSender<BusAction>,
    inner_clock: ClockInfo,
    current_clock: ClockInfo,
}

impl<DeviceT: BusDeviceU24 + Send + 'static> BusDeviceU24 for AsyncBusDeviceU24<DeviceT> {
    const NAME: &'static str = DeviceT::NAME;

    fn peek(&self, addr: AddressU24) -> Option<u8> {
        self.inner.lock().unwrap().peek(addr)
    }

    fn read(&mut self, addr: AddressU24) -> u8 {
        puffin::profile_function!(&DeviceT::NAME);
        self.sync();
        self.inner.lock().unwrap().read(addr)
    }

    fn write(&mut self, addr: AddressU24, value: u8) {
        self.sender
            .send(BusAction::Write(self.current_clock, addr, value))
            .unwrap()
    }

    fn update_clock(&mut self, new_clock: ClockInfo) {
        if new_clock.master_clock - self.inner_clock.master_clock > 1024 {
            self.sender.send(BusAction::Clock(new_clock)).unwrap();
            self.inner_clock = new_clock;
        }
        self.current_clock = new_clock;
    }

    fn reset(&mut self) {
        self.inner.lock().unwrap().reset()
    }
}

impl<DeviceT: BusDeviceU24 + Send + 'static> AsyncBusDeviceU24<DeviceT> {
    pub fn new(raw_inner: DeviceT) -> Self {
        let (sender, receiver) = sync_channel::<BusAction>(1024);
        let inner = Arc::new(Mutex::new(raw_inner));
        let inner_clone = inner.clone();
        thread::spawn(move || {
            while let Ok(action) = receiver.recv() {
                let mut inner = inner.lock().unwrap();
                {
                    puffin::profile_scope!("processing events", &DeviceT::NAME);
                    Self::process_action(&mut inner, action);
                    while let Ok(action) = receiver.try_recv() {
                        Self::process_action(&mut inner, action);
                    }
                }
            }
        });

        Self {
            inner: inner_clone,
            sender,
            current_clock: ClockInfo::default(),
            inner_clock: ClockInfo::default(),
        }
    }

    pub fn sync(&mut self) {
        puffin::profile_function!(&DeviceT::NAME);
        // Wait for lock to free after all actions have been processed
        // (I guess there could be a race condition if the thread has not yet started processing)
        drop(self.inner.lock().unwrap());
    }

    fn process_action(inner: &mut DeviceT, action: BusAction) {
        match action {
            BusAction::Clock(clock) => {
                inner.update_clock(clock);
            }
            BusAction::Write(clock, addr, value) => {
                inner.update_clock(clock);
                inner.write(addr, value);
            }
        }
    }
}
