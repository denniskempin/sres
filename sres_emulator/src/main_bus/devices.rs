use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;

use crate::common::address::AddressU24;
use crate::common::bus::BusDeviceU24;
use crate::common::clock::ClockInfo;

/// A wrapper around a `BusDeviceU24` that manages when the inner device is updated.
/// Implementations include batched, async, and synchronous (no-op) variants.
pub trait ManagedBusDeviceU24<InnerT: BusDeviceU24>: BusDeviceU24 {
    type InnerRef<'a>: Deref<Target = InnerT>
    where
        Self: 'a;
    type InnerRefMut<'a>: DerefMut<Target = InnerT>
    where
        Self: 'a;

    fn inner(&self) -> Self::InnerRef<'_>;
    fn inner_mut(&mut self) -> Self::InnerRefMut<'_>;
    fn sync(&mut self);
}

/// A no-op wrapper that passes all bus operations directly to the inner device.
pub struct SyncBusDevice<DeviceT: BusDeviceU24> {
    pub inner: DeviceT,
}

impl<DeviceT: BusDeviceU24> SyncBusDevice<DeviceT> {
    pub fn new(inner: DeviceT) -> Self {
        Self { inner }
    }
}

impl<DeviceT: BusDeviceU24> BusDeviceU24 for SyncBusDevice<DeviceT> {
    const NAME: &'static str = DeviceT::NAME;

    fn peek(&self, addr: AddressU24) -> Option<u8> {
        self.inner.peek(addr)
    }

    fn read(&mut self, addr: AddressU24) -> u8 {
        self.inner.read(addr)
    }

    fn write(&mut self, addr: AddressU24, value: u8) {
        self.inner.write(addr, value)
    }

    fn update_clock(&mut self, new_clock: ClockInfo) {
        self.inner.update_clock(new_clock)
    }

    fn reset(&mut self) {
        self.inner.reset()
    }
}

impl<DeviceT: BusDeviceU24> ManagedBusDeviceU24<DeviceT> for SyncBusDevice<DeviceT> {
    type InnerRef<'a>
        = &'a DeviceT
    where
        Self: 'a;
    type InnerRefMut<'a>
        = &'a mut DeviceT
    where
        Self: 'a;

    fn inner(&self) -> &DeviceT {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut DeviceT {
        &mut self.inner
    }

    fn sync(&mut self) {}
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
        self.flush();
        self.inner.read(addr)
    }

    fn write(&mut self, addr: AddressU24, value: u8) {
        if self.cache.len() >= CACHE_SIZE {
            self.flush();
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

    pub fn flush(&mut self) {
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

impl<DeviceT: BusDeviceU24> ManagedBusDeviceU24<DeviceT> for BatchedBusDeviceU24<DeviceT> {
    type InnerRef<'a>
        = &'a DeviceT
    where
        Self: 'a;
    type InnerRefMut<'a>
        = &'a mut DeviceT
    where
        Self: 'a;

    fn inner(&self) -> &DeviceT {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut DeviceT {
        &mut self.inner
    }

    fn sync(&mut self) {
        self.flush();
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
        self.flush();
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

    pub fn flush(&mut self) {
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

impl<DeviceT: BusDeviceU24 + Send + 'static> ManagedBusDeviceU24<DeviceT>
    for AsyncBusDeviceU24<DeviceT>
{
    type InnerRef<'a>
        = MutexGuard<'a, DeviceT>
    where
        Self: 'a;
    type InnerRefMut<'a>
        = MutexGuard<'a, DeviceT>
    where
        Self: 'a;

    fn inner(&self) -> MutexGuard<'_, DeviceT> {
        self.inner.lock().unwrap()
    }

    fn inner_mut(&mut self) -> MutexGuard<'_, DeviceT> {
        self.inner.lock().unwrap()
    }

    fn sync(&mut self) {
        self.flush();
    }
}
