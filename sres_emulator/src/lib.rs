pub mod apu;
pub mod common;
pub mod components;
pub mod controller;
pub mod debugger;
pub mod main_bus;

use std::cell::RefMut;
use std::ops::Deref;

use common::util::EdgeDetector;
use components::ppu::Framebuffer;
use components::ppu::PpuDebug;

use crate::apu::Apu;
use crate::apu::ApuDebug;
use crate::common::bus::AsyncBusDeviceU24;
use crate::common::clock::ClockInfo;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::common::image::Image;
use crate::components::cartridge::Cartridge;
use crate::components::cpu::Cpu;
use crate::components::cpu::MainBus;
use crate::components::ppu::Ppu;
use crate::debugger::BreakReason;
use crate::debugger::Debugger;
use crate::debugger::DebuggerRef;
use crate::debugger::EventFilter;
use crate::main_bus::MainBusImpl;

pub enum ExecutionResult {
    Normal,
    Halt,
    Break(BreakReason),
}
pub type CpuT = Cpu<MainBusImpl<AsyncBusDeviceU24<Ppu>, AsyncBusDeviceU24<Apu>>>;

pub struct System {
    pub cpu: CpuT,
    debugger: DebuggerRef,
    debugger_enabled: bool,
    vblank_detector: EdgeDetector,
    pending_video_frame: Option<Framebuffer>,
}

impl System {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::with_cartridge(&Cartridge::default())
    }

    pub fn with_cartridge(cartridge: &Cartridge) -> Self {
        let debugger = Debugger::new();
        Self {
            cpu: Cpu::new(
                MainBusImpl::new(
                    cartridge,
                    AsyncBusDeviceU24::new(Ppu::new()),
                    AsyncBusDeviceU24::new(Apu::new(debugger.clone())),
                    debugger.clone(),
                ),
                DebugEventCollectorRef(debugger.clone()),
            ),
            debugger,
            debugger_enabled: false,
            vblank_detector: EdgeDetector::new(),
            pending_video_frame: None,
        }
    }

    fn execute_until<F>(&mut self, should_break: F) -> ExecutionResult
    where
        F: Fn(&CpuT) -> bool,
    {
        loop {
            if self.cpu.halted() {
                return ExecutionResult::Halt;
            }

            self.step();

            if let Some(break_reason) = self.debugger().take_break_reason() {
                return ExecutionResult::Break(break_reason);
            }

            if should_break(&self.cpu) {
                self.cpu.bus.ppu.sync();
                self.cpu.bus.apu.sync();
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

    pub fn execute_scanlines(&mut self, count: u64) -> ExecutionResult {
        let target_scanline = self.cpu.bus.clock_info().v + count;
        self.execute_until(|cpu| cpu.bus.clock_info().v >= target_scanline)
    }

    pub fn execute_for_duration(&mut self, _seconds: f64) -> ExecutionResult {
        // TODO: Implement frame skip/doubling if not running at 60fps
        self.execute_frames(1)
    }

    pub fn clock_info(&self) -> ClockInfo {
        self.cpu.bus.clock_info()
    }

    pub fn update_joypads(&mut self, joy1: u16, joy2: u16) {
        self.cpu.bus.update_joypads(joy1, joy2);
    }

    pub fn pending_rgba_video_frame<ImageT: Image>(&self) -> Option<ImageT> {
        self.pending_video_frame.as_ref().map(|fb| fb.to_rgba())
    }

    pub fn ppu(&mut self) -> MutexGuard<'_, Ppu> {
        self.cpu.bus.ppu.inner.lock().unwrap()
    }

    pub fn apu(&mut self) -> MutexGuard<'_, Apu> {
        self.cpu.bus.apu.inner.lock().unwrap()
    }

    pub fn debug_until(&mut self, event: EventFilter) -> ExecutionResult {
        self.debugger_enabled = true;
        self.debugger().enable();
        self.debugger().add_break_point(event.clone());
        let result = loop {
            if self.cpu.halted() {
                break ExecutionResult::Halt;
            }

            self.step();

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

    fn step(&mut self) {
        self.cpu.step();
        if self.debugger_enabled {
            self.cpu.bus.ppu.sync();
            self.cpu.bus.apu.sync();
        }

        self.vblank_detector
            .update_signal(self.cpu.bus.clock_info().vblank());
        if self.vblank_detector.consume_rise() {
            self.cpu.bus.ppu.sync();
            self.cpu.bus.apu.sync();
            // TODO: Unnecessary clone, should use re-usable buffers or double buffering
            self.pending_video_frame = Some(self.cpu.bus.ppu.inner.lock().unwrap().framebuffer().clone());
        }
    }

    /// Exposes debug information for investigating the system state.
    pub fn debug(&self) -> SystemDebug<'_> {
        SystemDebug {
            ppu: self.cpu.bus.ppu.inner.lock().unwrap(),
            apu: self.cpu.bus.apu.inner.lock().unwrap(),
        }
    }

    /// Exposes an interactive debugger to set break and log points.
    pub fn debugger(&self) -> RefMut<'_, Debugger> {
        self.debugger.deref().borrow_mut()
    }
}

pub struct SystemDebug<'a>{
    ppu: MutexGuard<'a, Ppu>,
    apu: MutexGuard<'a, Apu>,
}

impl<'a> SystemDebug<'a> {
    pub fn ppu(&'a self) -> PpuDebug<'a> {
        self.ppu.debug()
    }

    pub fn apu(&'a self) -> ApuDebug<'a> {
        self.apu.debug()
    }
}
