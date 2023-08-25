use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::Range;
use std::rc::Rc;

use log::error;

use super::cpu::InstructionMeta;
use crate::bus::SresBus;
use crate::cpu::Cpu;
use crate::memory::Address;
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
#[derive(Clone)]
pub enum Trigger {
    CpuMemoryRead(Range<u32>),
    CpuMemoryWrite(Range<u32>),
    ExecutionError,
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
pub enum BreakReason {
    CpuMemoryRead(Address),
    CpuMemoryWrite(Address),
    ExecutionError(String),
}

impl Display for BreakReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BreakReason::CpuMemoryRead(addr) => {
                write!(f, "CPU memory read at address {}", addr)
            }
            BreakReason::CpuMemoryWrite(addr) => {
                write!(f, "CPU memory write at address {}", addr)
            }
            BreakReason::ExecutionError(e) => e.fmt(f),
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
    inner: Rc<RefCell<Debugger>>,
}

impl DebuggerRef {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(Debugger::new())),
            enabled: false,
        }
    }

    pub fn previous_instructions(&self, cpu: &Cpu<SresBus>) -> Vec<InstructionMeta> {
        if self.enabled {
            self.inner.borrow().previous_instructions(cpu)
        } else {
            vec![]
        }
    }

    pub fn before_instruction(&mut self, pc: Address) {
        if self.enabled {
            self.inner.deref().borrow_mut().before_instruction(pc);
        }
    }

    pub fn take_break_reason(&mut self) -> Option<BreakReason> {
        if self.enabled {
            self.inner.deref().borrow_mut().take_break_reason()
        } else {
            None
        }
    }

    pub fn on_error(&mut self, msg: String) {
        if self.enabled {
            self.inner.deref().borrow_mut().on_error(msg);
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

struct Debugger {
    pub breakpoints: Vec<Trigger>,
    pub break_reason: Option<BreakReason>,
    pub last_pcs: RingBuffer<Address, 32>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            breakpoints: vec![Trigger::ExecutionError],
            break_reason: None,
            last_pcs: RingBuffer::default(),
        }
    }

    pub fn previous_instructions(&self, cpu: &Cpu<SresBus>) -> Vec<InstructionMeta> {
        self.last_pcs
            .iter()
            .map(move |pc| cpu.load_instruction_meta(*pc).0)
            .rev()
            .collect()
    }

    pub fn before_instruction(&mut self, pc: Address) {
        self.last_pcs.push(pc);
    }

    pub fn take_break_reason(&mut self) -> Option<BreakReason> {
        self.break_reason.take()
    }

    pub fn on_error(&mut self, msg: String) {
        error!("{}", msg);
        self.break_reason = Some(BreakReason::ExecutionError(msg));
    }

    pub fn on_cpu_memory_access(&mut self, access: MemoryAccess) {
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
