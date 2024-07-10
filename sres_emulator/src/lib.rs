pub mod apu;
pub mod cartridge;
pub mod common;
pub mod components;
pub mod controller;
pub mod debugger;
pub mod main_bus;

use std::cell::RefCell;
use std::cell::RefMut;
use std::ops::Deref;
use std::rc::Rc;

use common::debug_events::DebugEventCollectorRef;
use components::cpu::MainBus;

use crate::apu::ApuDebug;
use crate::cartridge::Cartridge;
use crate::components::cpu::Cpu;
use crate::debugger::BreakReason;
use crate::debugger::Debugger;
use crate::debugger::EventFilter;
use crate::main_bus::MainBusImpl;

pub enum ExecutionResult {
    Normal,
    Halt,
    Break(BreakReason),
}

pub struct System {
    pub cpu: Cpu<MainBusImpl>,
    debugger: Rc<RefCell<Debugger>>,
}

impl System {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::with_cartridge(&Cartridge::default())
    }

    pub fn debug(&self) -> SystemDebug<'_> {
        SystemDebug(self)
    }

    pub fn debugger(&self) -> RefMut<'_, Debugger> {
        self.debugger.deref().borrow_mut()
    }

    pub fn with_cartridge(cartridge: &Cartridge) -> Self {
        let debugger = Rc::new(RefCell::new(Debugger::new()));
        Self {
            cpu: Cpu::new(
                MainBusImpl::new(cartridge, DebugEventCollectorRef(debugger.clone())),
                DebugEventCollectorRef(debugger.clone()),
            ),
            debugger,
        }
    }

    pub fn execute_until<F>(&mut self, should_break: F) -> ExecutionResult
    where
        F: Fn(&Cpu<MainBusImpl>) -> bool,
    {
        loop {
            if self.cpu.halted() {
                return ExecutionResult::Halt;
            }

            self.cpu.step();
            self.cpu
                .bus
                .apu
                .catch_up_to_master_clock(self.cpu.bus.clock_info().master_clock);

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
        self.execute_until(|cpu| cpu.halted())
    }

    pub fn execute_frames(&mut self, count: u64) -> ExecutionResult {
        let target_frame = self.cpu.bus.clock_info().f + count;
        self.execute_until(|cpu| cpu.bus.clock_info().f >= target_frame)
    }

    pub fn debug_until(&mut self, event: EventFilter) -> ExecutionResult {
        self.debugger().enable();
        self.debugger().add_break_point(event.clone());
        let result = loop {
            if self.cpu.halted() {
                break ExecutionResult::Halt;
            }

            self.cpu.step();
            self.cpu
                .bus
                .apu
                .catch_up_to_master_clock(self.cpu.bus.clock_info().master_clock);

            if let Some(break_reason) = self.debugger().take_break_reason() {
                if break_reason.trigger == event {
                    break ExecutionResult::Normal;
                } else {
                    break ExecutionResult::Break(break_reason);
                }
            }
        };
        self.debugger().remove_break_point(&event);

        result
    }

    pub fn execute_for_duration(&mut self, _seconds: f64) -> ExecutionResult {
        // TODO: Implement frame skip/doubling if not running at 60fps
        self.execute_frames(1)
    }
}

pub struct SystemDebug<'a>(&'a System);

impl<'a> SystemDebug<'a> {
    pub fn apu(self) -> ApuDebug<'a> {
        self.0.cpu.bus.apu.debug()
    }
}
