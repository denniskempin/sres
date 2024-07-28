//! TODO Add documentation
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;

pub static DEBUG_EVENTS_ENABLED: AtomicBool = AtomicBool::new(false);

pub trait DebugErrorCollector {
    #[cold]
    fn collect_error(&mut self, message: String);
}

pub trait DebugEventCollector<EventT>: DebugErrorCollector {
    #[cold]
    fn collect_event(&mut self, event: EventT);
}

/// Wrapper to a dyn trait reference of a DebugEventCollector
///
/// This is used by emulator components to generate events, which can then be
/// collected by the debugger.
#[derive(Clone)]
pub struct DebugEventCollectorRef<EventT>(pub Rc<RefCell<dyn DebugEventCollector<EventT>>>);

impl<EventT> DebugEventCollectorRef<EventT> {
    pub fn collect_event(&self, event: EventT) {
        if DEBUG_EVENTS_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            self.0.deref().borrow_mut().collect_event(event);
        }
    }

    pub fn collect_error(&self, message: String) {
        if DEBUG_EVENTS_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            self.0.deref().borrow_mut().collect_error(message);
        }
    }
}

#[cfg(test)]
pub fn dummy_collector<EventT>() -> DebugEventCollectorRef<EventT> {
    DebugEventCollectorRef(Rc::new(RefCell::new(DummyDebugEventCollector {})))
}

#[cfg(test)]
pub struct DummyDebugEventCollector {}

#[cfg(test)]
impl DebugErrorCollector for DummyDebugEventCollector {
    fn collect_error(&mut self, _message: String) {}
}

#[cfg(test)]
impl<EventT> DebugEventCollector<EventT> for DummyDebugEventCollector {
    fn collect_event(&mut self, _event: EventT) {}
}
