pub mod apu;
pub mod common;
pub mod components;
pub mod controller;
pub mod debugger;
pub mod main_bus;

use std::cell::RefCell;
use std::cell::RefMut;
use std::ops::Deref;
use std::rc::Rc;

use common::clock::ClockInfo;
use common::image::Image;
use controller::StandardController;
use debugger::DebuggerRef;

use crate::apu::ApuDebug;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::components::cartridge::Cartridge;
use crate::components::cpu::Cpu;
use crate::components::cpu::MainBus;
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
    debugger: DebuggerRef,
}

impl System {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::with_cartridge(&Cartridge::default())
    }

    pub fn with_cartridge(cartridge: &Cartridge) -> Self {
        let debugger = Rc::new(RefCell::new(Debugger::new()));
        Self {
            cpu: Cpu::new(
                MainBusImpl::new(cartridge, debugger.clone()),
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

    pub fn execute_for_duration(&mut self, _seconds: f64) -> ExecutionResult {
        // TODO: Implement frame skip/doubling if not running at 60fps
        self.execute_frames(1)
    }

    pub fn clock_info(&self) -> ClockInfo {
        self.cpu.bus.ppu.clock_info()
    }

    pub fn update_joypads(&mut self, joy1: StandardController, joy2: StandardController) {
        self.cpu.bus.joy1 = joy1.to_u16();
        self.cpu.bus.joy2 = joy2.to_u16();
    }

    pub fn get_rgba_framebuffer<ImageT: Image>(&self) -> ImageT {
        self.cpu.bus.ppu.get_rgba_framebuffer()
    }

    pub fn force_headless(&mut self) {
        self.cpu.bus.ppu.force_headless();
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

    /// Exposes debug information for investigating the system state.
    pub fn debug(&self) -> SystemDebug<'_> {
        SystemDebug(self)
    }

    /// Exposes an interactive debugger to set break and log points.
    pub fn debugger(&self) -> RefMut<'_, Debugger> {
        self.debugger.deref().borrow_mut()
    }
}

pub struct SystemDebug<'a>(&'a System);

impl<'a> SystemDebug<'a> {
    pub fn apu(self) -> ApuDebug<'a> {
        self.0.cpu.bus.apu.debug()
    }
}
