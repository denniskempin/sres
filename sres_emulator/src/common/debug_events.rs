//! TODO Add documentation
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;

pub static DEBUG_EVENTS_ENABLED: AtomicBool = AtomicBool::new(false);

pub trait DebugErrorCollector {
    fn on_error(&mut self, message: String);
}

pub trait DebugEventCollector<EventT>: DebugErrorCollector {
    fn on_event(&mut self, event: EventT);
}

/// Wrapper to a dyn trait reference of a DebugEventCollector
///
/// This is used by emulator components to generate events, which can then be
/// collected by the debugger.
#[derive(Clone)]
pub struct DebugEventCollectorRef<EventT>(pub Rc<RefCell<dyn DebugEventCollector<EventT>>>);

impl<EventT> DebugEventCollectorRef<EventT> {
    #[cold]
    pub fn on_event(&self, event: EventT) {
        if DEBUG_EVENTS_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            self.0.deref().borrow_mut().on_event(event);
        }
    }

    #[cold]
    pub fn on_error(&self, message: String) {
        if DEBUG_EVENTS_ENABLED.load(std::sync::atomic::Ordering::Relaxed) {
            self.0.deref().borrow_mut().on_error(message);
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
