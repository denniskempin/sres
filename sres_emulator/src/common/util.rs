//! General utility functions and types.
use std::collections::vec_deque::Iter;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct RingBuffer<T, const N: usize> {
    pub stack: VecDeque<T>,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn pop(&mut self) -> T {
        self.stack.pop_front().unwrap()
    }

    pub fn push(&mut self, data: T) {
        self.stack.push_front(data);
        self.stack.truncate(N);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.stack.iter()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self {
            stack: Default::default(),
        }
    }
}

/// A simple edge detector that can be used to detect rising and falling edges of a signal.
/// Used to simplify detection of start/end of vblank, timers, etc.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EdgeDetector {
    pub value: bool,
    pub rise_triggered: bool,
    pub fall_triggered: bool,
}

impl EdgeDetector {
    pub fn new() -> Self {
        Self {
            value: false,
            rise_triggered: false,
            fall_triggered: false,
        }
    }

    pub fn update_signal(&mut self, value: bool) {
        if value && !self.value {
            self.rise_triggered = true;
        }
        if !value && self.value {
            self.fall_triggered = true;
        }
        self.value = value;
    }

    pub fn consume_rise(&mut self) -> bool {
        let rise_triggered = self.rise_triggered;
        self.rise_triggered = false;
        rise_triggered
    }
    pub fn consume_fall(&mut self) -> bool {
        let fall_triggered = self.fall_triggered;
        self.fall_triggered = false;
        fall_triggered
    }
}

impl Default for EdgeDetector {
    fn default() -> Self {
        Self::new()
    }
}
