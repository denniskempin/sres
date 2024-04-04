//! Debugger functionality
//!
//! Allows internal components to notify of events during emulation (e.g. memory access),
//! and allows the front end to set and detect breakpoints on those events.
use std::cell::RefCell;
use std::ops::Deref;
use std::ops::Range;
use std::rc::Rc;

use log::log_enabled;
use log::Level;

use crate::bus::AddressU24;
use crate::cpu::Cpu;
use crate::cpu::NativeVectorTable;
use crate::main_bus::MainBus;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::trace::CpuTraceLine;
use crate::trace::Spc700TraceLine;
use crate::trace::TraceLine;
use crate::util::RingBuffer;

pub enum MemoryAccess {
    Read(AddressU24),
    Write(AddressU24, u8),
}

impl MemoryAccess {
    pub fn addr(&self) -> AddressU24 {
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
#[derive(Clone, strum::Display)]
pub enum BreakReason {
    ProgramCounter(AddressU24),
    CpuMemoryRead(AddressU24),
    CpuMemoryWrite(AddressU24),
    ExecutionError(String),
    Interrupt(NativeVectorTable),
    Custom(String),
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

    pub fn before_cpu_instruction(&self, cpu: &Cpu<impl MainBus>) {
        if log_enabled!(target: "cpu_state", Level::Trace) {
            log::trace!(target: "cpu_state", "{}", CpuTraceLine::from_cpu(cpu));
        }
        if self.enabled {
            self.inner.deref().borrow_mut().before_cpu_instruction(cpu);
        }
    }

    pub fn before_spc700_instruction(&self, spc700: &Spc700<impl Spc700Bus>) {
        if log_enabled!(target: "apu_state", Level::Trace) {
            log::trace!(target: "apu_state", "{}", Spc700TraceLine::from_spc700(spc700));
        }
        if self.enabled {
            self.inner
                .deref()
                .borrow_mut()
                .before_spc700_instruction(spc700);
        }
    }

    pub fn on_interrupt(&self, handler: NativeVectorTable) {
        if self.enabled {
            self.inner.deref().borrow_mut().on_interrupt(handler);
        }
    }

    pub fn on_error(&self, msg: String) {
        log::error!("{}", msg);
        if self.enabled {
            self.inner.deref().borrow_mut().on_error(msg);
        }
    }

    pub fn trigger_custom(&self, msg: String) {
        if self.enabled {
            self.inner.deref().borrow_mut().trigger_custom(msg);
        }
    }

    pub fn on_cpu_memory_access(&self, access: MemoryAccess) {
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
    trace: RingBuffer<TraceLine, 128>,
}

impl Debugger {
    /// Frontend facing API

    pub fn consume_trace(&mut self) -> Vec<TraceLine> {
        self.trace.stack.drain(..).rev().collect()
    }

    pub fn cpu_trace(&self) -> impl Iterator<Item = &CpuTraceLine> {
        self.trace.iter().filter_map(|line| match line {
            TraceLine::Cpu(cpu) => Some(cpu),
            _ => None,
        })
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

    /// Internal API
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            breakpoints: vec![],
            break_reason: None,
            trace: RingBuffer::default(),
        }
    }

    fn before_spc700_instruction(&mut self, spc700: &Spc700<impl Spc700Bus>) {
        if spc700.pc.0 == 0xFFFB {
            self.break_reason = Some(BreakReason::Custom("SPC".to_string()));
        }
        self.trace
            .push(TraceLine::Spc700(Spc700TraceLine::from_spc700(spc700)));
    }

    fn before_cpu_instruction(&mut self, cpu: &Cpu<impl MainBus>) {
        self.trace.push(TraceLine::Cpu(CpuTraceLine::from_cpu(cpu)));
        for trigger in self.breakpoints.iter() {
            if let Trigger::ProgramCounter(range) = trigger {
                if range.contains(&u32::from(cpu.pc)) {
                    self.break_reason = Some(BreakReason::ProgramCounter(cpu.pc));
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
