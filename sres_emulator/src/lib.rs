pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod debugger;
pub mod dma;
pub mod logging;
pub mod memory;
pub mod ppu;
pub mod timer;
pub mod trace;
pub mod uint;
pub mod util;

use std::{cell::RefCell, path::Path, rc::Rc};

use anyhow::Result;

use bus::SresBus;
use cpu::Cpu;
use debugger::{BreakReason, Debugger};

pub enum ExecutionResult {
    Normal,
    Halt,
    Break(BreakReason),
}

pub struct System {
    pub cpu: Cpu<SresBus>,
    pub debugger: Option<Rc<RefCell<Debugger>>>,
}

impl System {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::with_bus(SresBus::new())
    }

    pub fn with_sfc_bytes(sfc_data: &[u8]) -> Result<Self> {
        Ok(Self::with_bus(SresBus::with_sfc_data(sfc_data)?))
    }

    pub fn with_sfc(sfc_path: &Path) -> Result<Self> {
        Ok(Self::with_bus(SresBus::with_sfc(sfc_path)?))
    }

    fn with_bus(bus: SresBus) -> Self {
        System {
            cpu: Cpu::new(bus),
            debugger: None,
        }
    }

    pub fn enable_debugger(&mut self) {
        self.debugger = Some(Debugger::new());
        self.cpu.debugger = self.debugger.clone();
        self.cpu.bus.debugger = self.debugger.clone();
    }

    pub fn disable_debugger(&mut self) {
        self.debugger = None;
        self.cpu.debugger = None;
        self.cpu.bus.debugger = None;
    }

    pub fn execute_until<F>(&mut self, should_break: F) -> ExecutionResult
    where
        F: Fn(&Cpu<SresBus>) -> bool,
    {
        loop {
            if self.cpu.halt {
                return ExecutionResult::Halt;
            }

            self.cpu.step();

            if let Some(debugger) = &self.debugger {
                if let Some(break_reason) = debugger.borrow_mut().break_reason.take() {
                    return ExecutionResult::Break(break_reason);
                }
            }

            if should_break(&self.cpu) {
                return ExecutionResult::Normal;
            }
        }
    }

    pub fn execute_until_halt(&mut self) -> ExecutionResult {
        self.execute_until(|cpu| cpu.halt)
    }

    pub fn execute_one_frame(&mut self) -> ExecutionResult {
        let current_frame = self.cpu.bus.ppu_timer.f;
        self.execute_until(|cpu| cpu.bus.ppu_timer.f > current_frame)
    }

    pub fn execute_for_duration(&mut self, _seconds: f64) -> ExecutionResult {
        // TODO: Implement frame skip/doubling if not running at 60fps
        self.execute_one_frame()
    }
}
