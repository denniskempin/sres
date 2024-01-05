use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::Range;
use std::rc::Rc;

use super::cpu::InstructionMeta;
use crate::bus::MainBusImpl;
use crate::cpu::Cpu;
use crate::cpu::NativeVectorTable;
use crate::util::memory::AddressU24;
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
#[derive(Clone)]
pub enum BreakReason {
    ProgramCounter(AddressU24),
    CpuMemoryRead(AddressU24),
    CpuMemoryWrite(AddressU24),
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

    pub fn before_instruction(&mut self, pc: AddressU24) {
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
        log::error!("{}", msg);
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
    last_pcs: RingBuffer<AddressU24, 20>,
}

impl Debugger {
    /// Frontend facing API

    pub fn previous_instructions(&self, cpu: &Cpu<MainBusImpl>) -> Vec<InstructionMeta> {
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

    /// Internal API
    fn new() -> Self {
        Self {
            breakpoints: vec![],
            break_reason: None,
            last_pcs: RingBuffer::default(),
        }
    }

    fn before_instruction(&mut self, pc: AddressU24) {
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
