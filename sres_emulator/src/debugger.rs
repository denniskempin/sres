use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Range;
use std::rc::Rc;

use super::cpu::InstructionMeta;
use crate::bus::SresBus;
use crate::cpu::Cpu;
use crate::memory::Address;
use crate::util::RingBuffer;

pub enum MemoryAccess {
    Read(u16),
    Write(u16, u8),
}

impl MemoryAccess {
    pub fn addr(&self) -> u16 {
        match self {
            MemoryAccess::Read(addr) => *addr,
            MemoryAccess::Write(addr, _) => *addr,
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
pub enum Trigger {
    CpuMemoryRead(Range<u16>),
    CpuMemoryWrite(Range<u16>),
    ExecutionError,
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
pub enum BreakReason {
    CpuMemoryRead(u16),
    CpuMemoryWrite(u16),
    ExecutionError(String),
}

impl Display for BreakReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BreakReason::CpuMemoryRead(addr) => {
                write!(f, "CPU memory read at address {:04X}", addr)
            }
            BreakReason::CpuMemoryWrite(addr) => {
                write!(f, "CPU memory write at address {:04X}", addr)
            }
            BreakReason::ExecutionError(e) => e.fmt(f),
        }
    }
}

#[derive(Clone)]
pub struct Debugger {
    pub breakpoints: Vec<Trigger>,
    pub break_reason: Option<BreakReason>,
    pub last_pcs: RingBuffer<Address, 32>,
}

impl Debugger {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            breakpoints: vec![Trigger::ExecutionError],
            break_reason: None,
            last_pcs: RingBuffer::default(),
        }))
    }

    pub fn previous_instructions<'a>(
        &'a self,
        cpu: &'a Cpu<SresBus>,
    ) -> impl Iterator<Item = InstructionMeta> + 'a {
        self.last_pcs
            .iter()
            .map(move |pc| cpu.load_instruction_meta(*pc).0)
            .rev()
    }

    pub fn before_instruction(&mut self, pc: Address) {
        self.last_pcs.push(pc);
    }

    pub fn take_break_reason(&mut self) -> Option<BreakReason> {
        self.break_reason.take()
    }

    pub fn on_error(&mut self, msg: String) {
        for trigger in self.breakpoints.iter() {
            if let Trigger::ExecutionError = trigger {
                self.break_reason = Some(BreakReason::ExecutionError(msg.clone()));
            }
        }
    }

    pub fn on_cpu_memory_access(&mut self, access: MemoryAccess) {
        for trigger in self.breakpoints.iter() {
            match trigger {
                Trigger::CpuMemoryRead(range) => {
                    if let MemoryAccess::Read(addr) = access {
                        if range.contains(&addr) {
                            self.break_reason = Some(BreakReason::CpuMemoryRead(addr));
                        }
                    }
                }
                Trigger::CpuMemoryWrite(range) => {
                    if let MemoryAccess::Write(addr, _) = access {
                        if range.contains(&addr) {
                            self.break_reason = Some(BreakReason::CpuMemoryWrite(addr));
                        }
                    }
                }
                _ => (),
            }
        }
    }
}
