//! General utility functions and types.
use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::io::BufWriter;
use std::io::Write;

use bitcode::Decode;
use bitcode::Encode;

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

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.stack.drain(..)
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
#[derive(Clone, Copy, PartialEq, Eq, Debug, Encode, Decode)]
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

pub fn format_memory(memory: &[u8]) -> String {
    let mut writer = BufWriter::new(Vec::new());
    for chunks in memory.chunks(16) {
        for chunk in chunks {
            write!(&mut writer, "{:02X} ", *chunk).unwrap();
        }
        writeln!(&mut writer).unwrap();
    }

    let bytes = writer.into_inner().unwrap();
    String::from_utf8(bytes).unwrap()
}

pub fn format_memory_u16(memory: &[u16]) -> String {
    let mut writer = BufWriter::new(Vec::new());
    for chunks in memory.chunks(16) {
        for chunk in chunks {
            write!(&mut writer, "{:04X} ", *chunk).unwrap();
        }
        writeln!(&mut writer).unwrap();
    }

    let bytes = writer.into_inner().unwrap();
    String::from_utf8(bytes).unwrap()
}
