pub mod apu;
pub mod bus;
pub mod cartridge;
pub mod controller;
pub mod cpu;
pub mod debugger;
pub mod main_bus;
pub mod ppu;
pub mod spc700;
pub mod trace;
pub mod util;

use std::cell::RefMut;
use std::ops::Deref;

use cartridge::Cartridge;
use cpu::Cpu;
use debugger::BreakReason;
use debugger::Debugger;
use debugger::DebuggerRef;
use main_bus::MainBusImpl;

pub enum ExecutionResult {
    Normal,
    Halt,
    Break(BreakReason),
}

pub struct System {
    pub cpu: Cpu<MainBusImpl>,
    debugger: DebuggerRef,
}

impl System {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::with_cartridge(&Cartridge::default())
    }

    pub fn debugger(&self) -> RefMut<'_, Debugger> {
        self.debugger.inner.deref().borrow_mut()
    }

    pub fn with_cartridge(cartridge: &Cartridge) -> Self {
        let debugger = DebuggerRef::new();
        Self {
            cpu: Cpu::new(
                MainBusImpl::new(cartridge, debugger.clone()),
                debugger.clone(),
            ),
            debugger,
        }
    }

    pub fn is_debugger_enabled(&self) -> bool {
        self.debugger.enabled
    }

    pub fn enable_debugger(&mut self) {
        self.debugger.enabled = true;
        self.cpu.debugger.enabled = true;
        self.cpu.bus.debugger.enabled = true;
        self.cpu.bus.dma_controller.debugger.enabled = true;
        self.cpu.bus.ppu.debugger.enabled = true;
        self.cpu.bus.apu.spc700.debugger.enabled = true;
    }

    pub fn disable_debugger(&mut self) {
        self.debugger.enabled = false;
        self.cpu.debugger.enabled = false;
        self.cpu.bus.debugger.enabled = false;
        self.cpu.bus.dma_controller.debugger.enabled = false;
        self.cpu.bus.ppu.debugger.enabled = false;
        self.cpu.bus.apu.spc700.debugger.enabled = false;
    }

    pub fn execute_until<F>(&mut self, should_break: F) -> ExecutionResult
    where
        F: Fn(&Cpu<MainBusImpl>) -> bool,
    {
        loop {
            if self.cpu.halt {
                return ExecutionResult::Halt;
            }

            self.cpu.step();
            self.cpu
                .bus
                .apu
                .catch_up_to_master_clock(self.cpu.bus.ppu.timer.master_clock);

            if let Some(break_reason) = self.debugger().take_break_reason() {
                return ExecutionResult::Break(break_reason);
            }

            if should_break(&self.cpu) {
                return ExecutionResult::Normal;
            }
        }
    }

    pub fn execute_one_instruction(&mut self) -> ExecutionResult {
        self.execute_until(|_| true)
    }

    pub fn execute_until_halt(&mut self) -> ExecutionResult {
        self.execute_until(|cpu| cpu.halt)
    }

    pub fn execute_frames(&mut self, count: u64) -> ExecutionResult {
        let target_frame = self.cpu.bus.ppu.timer.f + count;
        self.execute_until(|cpu| cpu.bus.ppu.timer.f >= target_frame)
    }

    pub fn execute_for_duration(&mut self, _seconds: f64) -> ExecutionResult {
        // TODO: Implement frame skip/doubling if not running at 60fps
        self.execute_frames(1)
    }
}
