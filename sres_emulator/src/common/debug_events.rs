//! TODO Add documentation
use core::marker::PhantomData;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;

use super::util::RingBuffer;

pub static DEBUG_EVENTS_ENABLED: AtomicBool = AtomicBool::new(false);

pub trait DebugErrorCollector {
    #[cold]
    fn on_error(&mut self, message: String);
}

pub trait DebugEventCollector<EventT>: DebugErrorCollector {
    #[cold]
    fn on_event(&mut self, event: EventT);
}

/// Wrapper to a dyn trait reference of a DebugEventCollector
///
/// This is used by emulator components to generate events, which can then be
/// collected by the debugger.
#[derive(Clone)]
pub struct DebugEventCollectorRef<EventT>(pub Rc<RefCell<dyn DebugEventCollector<EventT>>>);

impl<EventT> DebugEventCollectorRef<EventT> {
    pub fn on_event(&self, event: EventT) {
        if DEBUG_EVENTS_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            self.0.deref().borrow_mut().on_event(event);
        }
    }

    pub fn on_error(&self, message: String) {
        if DEBUG_EVENTS_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            self.0.deref().borrow_mut().on_error(message);
        }
    }
}
pub trait EventFilter<EventT> {
    fn matches(&self, event: &EventT) -> bool;
}

pub struct DebuggerConfig<EventT, EventFilterT: EventFilter<EventT>> {
    pub enabled: bool,
    pub event_filter: Vec<EventFilterT>,
    pub phantom: PhantomData<EventT>,
}

impl<EventT, EventFilterT: EventFilter<EventT>> DebuggerConfig<EventT, EventFilterT> {
 pub fn new(enabled: bool, event_filter: Vec<EventFilterT>) -> Self {
    Self {
        enabled,
        event_filter,
        phantom: PhantomData,
    }
 }
}

impl<EventT, EventFilterT: EventFilter<EventT>> Default for DebuggerConfig<EventT, EventFilterT> {
    fn default() -> Self {
        Self {
            enabled: false,
            event_filter: Vec::new(),
            phantom: PhantomData,
        }
    }
}

pub struct DebugEventLogger<EventT, EventFilterT: EventFilter<EventT>> {
    pub config: DebuggerConfig<EventT, EventFilterT>,
    pub log: RingBuffer<EventT, 1024>,
}

impl<EventT, EventFilterT: EventFilter<EventT>> DebugEventLogger<EventT, EventFilterT> {
    pub fn new() -> Self {
        Self {
            config: DebuggerConfig::default(),
            log: RingBuffer::default(),
        }
    }

    pub fn collect_event(&mut self, event: EventT) {
        if !self.config.enabled {
            return;
        }
        if self
            .config
            .event_filter
            .iter()
            .any(|filter| filter.matches(&event))
        {
            self.log.push(event);
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn mock_collector<EventT>() -> DebugEventCollectorRef<EventT> {
        DebugEventCollectorRef(Rc::new(RefCell::new(MockDebugEventCollector {})))
    }

    struct MockDebugEventCollector {}

    impl DebugErrorCollector for MockDebugEventCollector {
        fn on_error(&mut self, _message: String) {}
    }

    impl<EventT> DebugEventCollector<EventT> for MockDebugEventCollector {
        fn on_event(&mut self, _event: EventT) {}
    }
}
