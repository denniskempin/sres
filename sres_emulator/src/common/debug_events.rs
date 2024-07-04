//! TODO Add documentation
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;

use crate::common::address::AddressU24;
use crate::common::trace::CpuTraceLine;
use crate::common::trace::Spc700TraceLine;

use super::address::AddressU16;
use super::constants::NativeVectorTable;

pub static DEBUG_EVENTS_ENABLED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Debug, PartialEq)]
pub enum DebugEvent {
    Cpu(CpuEvent),
    Apu(ApuEvent),
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CpuEvent {
    Step(CpuTraceLine),
    Interrupt(NativeVectorTable),
    Read(AddressU24, u8),
    Write(AddressU24, u8),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApuEvent {
    Step(Spc700TraceLine),
    Read(AddressU16, u8),
    Write(AddressU16, u8),
}

pub trait DebugEventCollector {
    #[cold]
    fn collect_event(&mut self, event: DebugEvent);
}

/// Wrapper to a dyn trait reference of a DebugEventCollector
///
/// This is used by emulator components to generate events, which can then be
/// collected by the debugger.
#[derive(Clone)]
pub struct DebugEventCollectorRef(pub Rc<RefCell<dyn DebugEventCollector>>);

impl DebugEventCollectorRef {
    pub fn collect_event(&self, event: DebugEvent) {
        if DEBUG_EVENTS_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            self.0.deref().borrow_mut().collect_event(event);
        }
    }
    pub fn collect_cpu_event(&self, event: CpuEvent) {
        self.collect_event(DebugEvent::Cpu(event));
    }

    pub fn collect_apu_event(&self, event: ApuEvent) {
        self.collect_event(DebugEvent::Apu(event));
    }
}

#[cfg(test)]
pub fn dummy_collector() -> DebugEventCollectorRef {
    DebugEventCollectorRef(Rc::new(RefCell::new(DummyDebugEventCollector {})))
}

#[cfg(test)]
pub struct DummyDebugEventCollector {}

#[cfg(test)]
impl DebugEventCollector for DummyDebugEventCollector {
    fn collect_event(&mut self, _event: DebugEvent) {}
}
