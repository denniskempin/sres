use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;

use super::cpu::InstructionMeta;
use crate::bus::SresBus;
use crate::cpu::Cpu;
use crate::cpu::NativeVectorTable;
use crate::util::memory::Address;
use crate::util::time::Instant;
use crate::util::RingBuffer;

pub enum MemoryAccess {
    Read(Address),
    Write(Address, u8),
}

impl MemoryAccess {
    pub fn addr(&self) -> Address {
        match self {
            MemoryAccess::Read(addr) => *addr,
            MemoryAccess::Write(addr, _) => *addr,
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq)]
pub enum Trigger {
    ProgramCounter(Range<u32>),
    CpuMemoryRead(Range<u32>),
    CpuMemoryWrite(Range<u32>),
    ExecutionError,
    Interrupt(NativeVectorTable),
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
pub enum BreakReason {
    ProgramCounter(Address),
    CpuMemoryRead(Address),
    CpuMemoryWrite(Address),
    ExecutionError(String),
    Interrupt(NativeVectorTable),
    Custom(String),
}

impl Display for BreakReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BreakReason::ProgramCounter(addr) => {
                write!(f, "Program counter reached address {}", addr)
            }
            BreakReason::CpuMemoryRead(addr) => {
                write!(f, "CPU memory read at address {}", addr)
            }
            BreakReason::CpuMemoryWrite(addr) => {
                write!(f, "CPU memory write at address {}", addr)
            }
            BreakReason::Custom(msg) => msg.fmt(f),
            BreakReason::ExecutionError(msg) => msg.fmt(f),
            BreakReason::Interrupt(handler) => write!(f, "{} Interrupt", handler),
        }
    }
}

/// A wrapper around Rc<RefCell<Debugger>> to access a shared instance of the debugger.
///
/// Each instance has it's own `enabled` flag to enable/disable access to the debugger
/// and default to sensible no-op behavior. This prevents frequent access to the
/// Rc<RefCell<>> when the debugger is disabled.
#[derive(Clone)]
pub struct DebuggerRef {
    pub enabled: bool,
    pub inner: Rc<RefCell<Debugger>>,
}

impl DebuggerRef {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Debugger::new())),
            enabled: false,
        }
    }

    pub fn end_frame(&mut self, frame_time: Duration) {
        if self.enabled {
            self.inner.deref().borrow_mut().end_frame(frame_time);
        }
    }

    pub fn add_perf_counter(&mut self, counter: PerfCounter, duration: Duration) {
        if self.enabled {
            self.inner
                .deref()
                .borrow_mut()
                .add_perf_counter(counter, duration);
        }
    }

    pub fn scoped_perf_counter(&mut self, counter: PerfCounter) -> Option<ScopedPerfCounter> {
        if self.enabled {
            Some(ScopedPerfCounter {
                debugger: self.clone(),
                counter,
                start: Instant::now(),
            })
        } else {
            None
        }
    }

    pub fn before_instruction(&mut self, pc: Address) {
        if self.enabled {
            self.inner.deref().borrow_mut().before_instruction(pc);
        }
    }

    pub fn on_interrupt(&mut self, handler: NativeVectorTable) {
        if self.enabled {
            self.inner.deref().borrow_mut().on_interrupt(handler);
        }
    }

    pub fn on_error(&mut self, msg: String) {
        if self.enabled {
            self.inner.deref().borrow_mut().on_error(msg);
        }
    }

    pub fn trigger_custom(&mut self, msg: String) {
        if self.enabled {
            self.inner.deref().borrow_mut().trigger_custom(msg);
        }
    }

    pub fn on_cpu_memory_access(&mut self, access: MemoryAccess) {
        if self.enabled {
            self.inner.deref().borrow_mut().on_cpu_memory_access(access);
        }
    }
}

impl Default for DebuggerRef {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Debugger {
    breakpoints: Vec<Trigger>,
    break_reason: Option<BreakReason>,
    last_pcs: RingBuffer<Address, 20>,

    perf_counters: PerfCounters,
    perf_counters_history: RingBuffer<PerfCounters, 30>,
    frame_time_history: RingBuffer<Duration, 30>,
}

impl Debugger {
    /// Frontend facing API

    pub fn previous_instructions(&self, cpu: &Cpu<SresBus>) -> Vec<InstructionMeta> {
        self.last_pcs
            .iter()
            .map(move |pc| cpu.load_instruction_meta(*pc).0)
            .rev()
            .collect()
    }

    pub fn take_break_reason(&mut self) -> Option<BreakReason> {
        self.break_reason.take()
    }

    pub fn has_breakpoint(&self, trigger: &Trigger) -> bool {
        self.breakpoints.iter().any(|t| t == trigger)
    }

    pub fn add_breakpoint(&mut self, trigger: Trigger) {
        if !self.has_breakpoint(&trigger) {
            self.breakpoints.push(trigger);
        }
    }

    pub fn remove_breakpoint(&mut self, trigger: &Trigger) {
        self.breakpoints.retain(|t| t != trigger)
    }

    pub fn toggle_breakpoint(&mut self, trigger: Trigger) {
        if self.has_breakpoint(&trigger) {
            self.remove_breakpoint(&trigger)
        } else {
            self.add_breakpoint(trigger)
        }
    }

    pub fn get_avg_frame_time_ms(&self) -> f32 {
        let mut total = Duration::from_secs(0);
        for frame_time in self.frame_time_history.iter() {
            total += *frame_time;
        }
        total.as_micros() as f32 / self.frame_time_history.len() as f32 / 1000.0
    }

    pub fn get_avg_perf_counter_ms(&self, counter: PerfCounter) -> f32 {
        let mut total = Duration::from_secs(0);
        for perf_counters in self.perf_counters_history.iter() {
            total += perf_counters.get(counter);
        }
        total.as_micros() as f32 / self.frame_time_history.len() as f32 / 1000.0
    }

    /// Internal API
    fn new() -> Self {
        Self {
            breakpoints: vec![],
            break_reason: None,
            last_pcs: RingBuffer::default(),
            perf_counters: PerfCounters::default(),
            perf_counters_history: RingBuffer::default(),
            frame_time_history: RingBuffer::default(),
        }
    }

    fn end_frame(&mut self, frame_time: Duration) {
        self.perf_counters_history.push(self.perf_counters);
        self.frame_time_history.push(frame_time);
        self.perf_counters = PerfCounters::default();
    }

    fn add_perf_counter(&mut self, component: PerfCounter, duration: Duration) {
        self.perf_counters.add(component, duration);
    }

    fn before_instruction(&mut self, pc: Address) {
        self.last_pcs.push(pc);
        for trigger in self.breakpoints.iter() {
            if let Trigger::ProgramCounter(range) = trigger {
                if range.contains(&u32::from(pc)) {
                    self.break_reason = Some(BreakReason::ProgramCounter(pc));
                }
            }
        }
    }

    fn on_interrupt(&mut self, interrupt: NativeVectorTable) {
        for trigger in &self.breakpoints {
            if let Trigger::Interrupt(handler) = trigger {
                if *handler == interrupt {
                    self.break_reason = Some(BreakReason::Interrupt(interrupt));
                }
            }
        }
    }
    fn on_error(&mut self, msg: String) {
        //error!("{}", msg);
        for trigger in self.breakpoints.iter() {
            if let Trigger::ExecutionError = trigger {
                self.break_reason = Some(BreakReason::ExecutionError(msg.clone()));
            }
        }
    }

    /// Can be added to code during development to trigger the debugger.
    #[allow(dead_code)]
    fn trigger_custom(&mut self, msg: String) {
        self.break_reason = Some(BreakReason::Custom(msg));
    }

    fn on_cpu_memory_access(&mut self, access: MemoryAccess) {
        for trigger in self.breakpoints.iter() {
            match trigger {
                Trigger::CpuMemoryRead(range) => {
                    if let MemoryAccess::Read(addr) = access {
                        if range.contains(&u32::from(addr)) {
                            self.break_reason = Some(BreakReason::CpuMemoryRead(addr));
                        }
                    }
                }
                Trigger::CpuMemoryWrite(range) => {
                    if let MemoryAccess::Write(addr, _) = access {
                        if range.contains(&u32::from(addr)) {
                            self.break_reason = Some(BreakReason::CpuMemoryWrite(addr));
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
pub struct ScopedPerfCounter {
    debugger: DebuggerRef,
    counter: PerfCounter,
    start: Instant,
}

impl Drop for ScopedPerfCounter {
    fn drop(&mut self) {
        self.debugger
            .inner
            .deref()
            .borrow_mut()
            .add_perf_counter(self.counter, self.start.elapsed());
    }
}

#[derive(Clone, Copy)]
pub enum PerfCounter {
    Ppu = 0,
    Timers = 1,
    Cpu = 2,
    Dma = 3,
}

impl Display for PerfCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerfCounter::Ppu => write!(f, "PPU"),
            PerfCounter::Timers => write!(f, "Timers"),
            PerfCounter::Cpu => write!(f, "CPU"),
            PerfCounter::Dma => write!(f, "DMA"),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct PerfCounters {
    pub counters: [Duration; 4],
}

impl PerfCounters {
    pub fn get(&self, counter: PerfCounter) -> Duration {
        self.counters[counter as usize]
    }

    pub fn add(&mut self, counter: PerfCounter, duration: Duration) {
        self.counters[counter as usize] += duration;
    }

    pub fn add_all(&mut self, other: &Self) {
        for (counter, duration) in other.counters.iter().enumerate() {
            self.counters[counter] += *duration;
        }
    }
}
