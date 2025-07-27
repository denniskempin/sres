pub mod apu;
pub mod common;
pub mod components;
pub mod controller;
pub mod debugger;
pub mod main_bus;

use std::cell::RefMut;
use std::ops::Deref;

use common::bus::BatchedBusDeviceU24;
use common::util::EdgeDetector;
use components::ppu::Framebuffer;
use components::ppu::PpuDebug;

use crate::apu::Apu;
use crate::apu::ApuDebug;
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
pub type CpuT = Cpu<MainBusImpl<BatchedBusDeviceU24<Ppu>, BatchedBusDeviceU24<Apu>>>;

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
                    BatchedBusDeviceU24::new(Ppu::new()),
                    BatchedBusDeviceU24::new(Apu::new(debugger.clone())),
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

    pub fn execute_cycles(&mut self, count: u64) -> ExecutionResult {
        let target_master_clock = self.cpu.bus.clock_info().master_clock + count;
        self.execute_until(|cpu| cpu.bus.clock_info().master_clock >= target_master_clock)
    }

    pub fn execute_for_audio_samples(&mut self, count: usize) -> ExecutionResult {
        self.execute_until(|cpu| cpu.bus.apu.inner.sample_buffer_size() >= count)
    }

    pub fn execute_for_duration(&mut self, seconds: f64) -> ExecutionResult {
        use crate::apu::MASTER_CLOCK_FREQUENCY;
        let target_cycles = (seconds * MASTER_CLOCK_FREQUENCY as f64) as u64;
        self.execute_cycles(target_cycles)
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

    pub fn take_audio_samples(&mut self) -> Vec<i16> {
        self.cpu.bus.apu.inner.take_audio_samples()
    }

    pub fn ppu(&mut self) -> &mut Ppu {
        &mut self.cpu.bus.ppu.inner
    }

    pub fn apu(&mut self) -> &mut Apu {
        &mut self.cpu.bus.apu.inner
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
            self.pending_video_frame = Some(self.cpu.bus.ppu.inner.framebuffer().clone());
        }
    }

    /// Exposes debug information for investigating the system state.
    pub fn debug(&self) -> SystemDebug<'_> {
        SystemDebug {
            ppu: &self.cpu.bus.ppu.inner,
            apu: &self.cpu.bus.apu.inner,
        }
    }

    /// Exposes an interactive debugger to set break and log points.
    pub fn debugger(&self) -> RefMut<'_, Debugger> {
        self.debugger.deref().borrow_mut()
    }
}

pub struct SystemDebug<'a> {
    ppu: &'a Ppu,
    apu: &'a Apu,
}

impl<'a> SystemDebug<'a> {
    pub fn ppu(&'a self) -> PpuDebug<'a> {
        self.ppu.debug()
    }

    pub fn apu(&'a self) -> ApuDebug<'a> {
        self.apu.debug()
    }
}
