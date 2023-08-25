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

use std::path::Path;

use anyhow::Result;
use bus::SresBus;
use cpu::Cpu;
use debugger::BreakReason;
use debugger::DebuggerRef;

pub enum ExecutionResult {
    Normal,
    Halt,
    Break(BreakReason),
}

pub struct System {
    pub cpu: Cpu<SresBus>,
    pub debugger: DebuggerRef,
}

impl System {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let debugger = DebuggerRef::new();
        Self {
            cpu: Cpu::new(SresBus::new(debugger.clone()), debugger.clone()),
            debugger,
        }
    }

    pub fn with_sfc_bytes(sfc_data: &[u8]) -> Result<Self> {
        let debugger = DebuggerRef::new();
        Ok(Self {
            cpu: Cpu::new(
                SresBus::with_sfc_data(sfc_data, debugger.clone())?,
                debugger.clone(),
            ),
            debugger,
        })
    }

    pub fn with_sfc(sfc_path: &Path) -> Result<Self> {
        let debugger = DebuggerRef::new();
        Ok(Self {
            cpu: Cpu::new(
                SresBus::with_sfc(sfc_path, debugger.clone())?,
                debugger.clone(),
            ),
            debugger,
        })
    }

    pub fn is_debugger_enabled(&self) -> bool {
        self.debugger.enabled
    }

    pub fn enable_debugger(&mut self) {
        self.debugger.enabled = true;
        self.cpu.debugger.enabled = true;
        self.cpu.bus.debugger.enabled = true;
    }

    pub fn disable_debugger(&mut self) {
        self.debugger.enabled = false;
        self.cpu.debugger.enabled = false;
        self.cpu.bus.debugger.enabled = false;
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

            if let Some(break_reason) = self.debugger.take_break_reason() {
                return ExecutionResult::Break(break_reason);
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
