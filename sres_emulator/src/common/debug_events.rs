//! `DebugEventCollectorRef` for components to emit debugger events.
//! `on_event`/`on_error`/`on_unimplemented` no-op unless `DEBUG_EVENTS_ENABLED` (zero-cost path: root).
//! `on_event_with` builds the event only when collection is on.
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use crate::common::unimplemented::UnimplementedBehavior;

pub static DEBUG_EVENTS_ENABLED: AtomicBool = AtomicBool::new(false);

pub trait DebugErrorCollector {
    fn on_error(&mut self, message: String);
    fn on_unimplemented(&mut self, behavior: UnimplementedBehavior);
}

pub trait DebugEventCollector<EventT>: DebugErrorCollector {
    fn on_event(&mut self, event: EventT);
}

/// Wrapper to a dyn trait reference of a DebugEventCollector
///
/// This is used by emulator components to generate events, which can then be
/// collected by the debugger.
#[derive(Clone)]
pub struct DebugEventCollectorRef<EventT>(pub Arc<Mutex<dyn DebugEventCollector<EventT> + Send>>);

impl<EventT> DebugEventCollectorRef<EventT> {
    #[inline(always)]
    pub fn on_event(&self, event: EventT) {
        if DEBUG_EVENTS_ENABLED.load(Ordering::Relaxed) {
            self.dispatch_event(event);
        }
    }

    #[inline(always)]
    pub fn on_event_with(&self, make: impl FnOnce() -> EventT) {
        if DEBUG_EVENTS_ENABLED.load(Ordering::Relaxed) {
            self.dispatch_event(make());
        }
    }

    #[inline(always)]
    pub fn on_error(&self, message: String) {
        if DEBUG_EVENTS_ENABLED.load(Ordering::Relaxed) {
            self.dispatch_error(message);
        }
    }

    #[inline(always)]
    pub fn on_unimplemented(&self, behavior: UnimplementedBehavior) {
        if DEBUG_EVENTS_ENABLED.load(Ordering::Relaxed) {
            self.dispatch_unimplemented(behavior);
        }
    }

    #[cold]
    fn dispatch_event(&self, event: EventT) {
        self.0.deref().lock().unwrap().on_event(event);
    }

    #[cold]
    fn dispatch_error(&self, message: String) {
        self.0.deref().lock().unwrap().on_error(message);
    }

    #[cold]
    fn dispatch_unimplemented(&self, behavior: UnimplementedBehavior) {
        self.0.deref().lock().unwrap().on_unimplemented(behavior);
    }
}

/// Collector that drops events, errors, and unimplemented reports.
pub fn noop_collector<EventT>() -> DebugEventCollectorRef<EventT> {
    DebugEventCollectorRef(Arc::new(Mutex::new(NoopDebugEventCollector {})))
}

struct NoopDebugEventCollector {}

impl DebugErrorCollector for NoopDebugEventCollector {
    fn on_error(&mut self, _message: String) {}
    fn on_unimplemented(&mut self, _behavior: UnimplementedBehavior) {}
}

impl<EventT> DebugEventCollector<EventT> for NoopDebugEventCollector {
    fn on_event(&mut self, _event: EventT) {}
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn mock_collector<EventT>() -> DebugEventCollectorRef<EventT> {
        noop_collector()
    }
}
