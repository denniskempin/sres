pub mod apu;
pub mod common;
pub mod components;
pub mod controller;
pub mod debugger;
pub mod main_bus;

use std::ops::Deref;
use std::sync::MutexGuard;

use common::util::EdgeDetector;
use components::ppu::Framebuffer;
use components::ppu::PpuDebug;

use crate::apu::Apu;
use crate::apu::ApuDebug;
use crate::apu::AudioBuffer;
use crate::common::clock::ClockInfo;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::components::cartridge::Cartridge;
use crate::components::cpu::Cpu;
use crate::components::cpu::MainBus;
use crate::components::ppu::Ppu;
use crate::debugger::BreakReason;
use crate::debugger::Debugger;
use crate::debugger::DebuggerRef;
use crate::debugger::EventFilter;
use crate::main_bus::devices::AsyncBusDeviceU24;
use crate::main_bus::devices::BatchedBusDeviceU24;
use crate::main_bus::devices::ManagedBusDeviceU24;
use crate::main_bus::devices::SyncBusDevice;
use crate::main_bus::MainBusImpl;

pub enum ExecutionResult {
    Normal,
    Halt,
    Break(BreakReason),
}

pub type CpuT = Cpu<MainBusImpl<BatchedBusDeviceU24<Ppu>, BatchedBusDeviceU24<Apu>>>;

/// System using batched PPU and APU updates for performance.
pub type BatchedSystem = SystemImpl<BatchedBusDeviceU24<Ppu>, BatchedBusDeviceU24<Apu>>;

/// System using synchronous PPU and APU updates on every cycle.
pub type SyncSystem = SystemImpl<SyncBusDevice<Ppu>, SyncBusDevice<Apu>>;

/// System running the APU on a separate thread.
pub type AsyncSystem = SystemImpl<AsyncBusDeviceU24<Ppu>, AsyncBusDeviceU24<Apu>>;

/// Default implementation used in UI
pub type System = BatchedSystem;

pub struct SystemImpl<PpuT: ManagedBusDeviceU24<Ppu>, ApuT: ManagedBusDeviceU24<Apu>> {
    pub cpu: Cpu<MainBusImpl<PpuT, ApuT>>,
    debugger: DebuggerRef,
    debugger_enabled: bool,
    vblank_detector: EdgeDetector,
    has_pending_video_frame: bool,
    pending_video_frame: Framebuffer,
}

impl<PpuT: ManagedBusDeviceU24<Ppu>, ApuT: ManagedBusDeviceU24<Apu>> SystemImpl<PpuT, ApuT> {
    pub fn with_cpu(cpu: Cpu<MainBusImpl<PpuT, ApuT>>, debugger: DebuggerRef) -> Self {
        let mut system = SystemImpl {
            cpu,
            debugger,
            debugger_enabled: false,
            vblank_detector: EdgeDetector::new(),
            has_pending_video_frame: false,
            pending_video_frame: Framebuffer::default(),
        };
        system.cpu.reset();
        system
    }
}

impl BatchedSystem {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::with_cartridge(&Cartridge::default())
    }

    pub fn with_cartridge(cartridge: &Cartridge) -> Self {
        let debugger = Debugger::new();
        Self::with_cpu(
            Cpu::new(
                MainBusImpl::new(
                    cartridge,
                    BatchedBusDeviceU24::new(Ppu::new()),
                    BatchedBusDeviceU24::new(Apu::new(debugger.clone())),
                    debugger.clone(),
                ),
                DebugEventCollectorRef(debugger.clone()),
            ),
            debugger,
        )
    }
}

impl SyncSystem {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::with_cartridge(&Cartridge::default())
    }

    pub fn with_cartridge(cartridge: &Cartridge) -> Self {
        let debugger = Debugger::new();
        Self::with_cpu(
            Cpu::new(
                MainBusImpl::new(
                    cartridge,
                    SyncBusDevice::new(Ppu::new()),
                    SyncBusDevice::new(Apu::new(debugger.clone())),
                    debugger.clone(),
                ),
                DebugEventCollectorRef(debugger.clone()),
            ),
            debugger,
        )
    }
}

impl AsyncSystem {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::with_cartridge(&Cartridge::default())
    }

    pub fn with_cartridge(cartridge: &Cartridge) -> Self {
        let debugger = Debugger::new();
        Self::with_cpu(
            Cpu::new(
                MainBusImpl::new(
                    cartridge,
                    AsyncBusDeviceU24::new(Ppu::new()),
                    AsyncBusDeviceU24::new(Apu::new(debugger.clone())),
                    debugger.clone(),
                ),
                DebugEventCollectorRef(debugger.clone()),
            ),
            debugger,
        )
    }
}

impl<PpuT: ManagedBusDeviceU24<Ppu>, ApuT: ManagedBusDeviceU24<Apu>> SystemImpl<PpuT, ApuT> {
    fn execute_until<F>(&mut self, should_break: F) -> ExecutionResult
    where
        F: Fn(&Cpu<MainBusImpl<PpuT, ApuT>>) -> bool,
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
        let target_sample_count = self.cpu.bus.apu.inner().sample_buffer_size() + count;
        self.execute_until(|cpu| cpu.bus.apu.inner().sample_buffer_size() >= target_sample_count)
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

    pub fn swap_video_frame(&mut self, buffer: &mut Framebuffer) -> bool {
        if self.has_pending_video_frame {
            std::mem::swap(&mut self.pending_video_frame, buffer);
            self.has_pending_video_frame = false;
            true
        } else {
            false
        }
    }

    pub fn swap_audio_buffer(&mut self, buffer: &mut AudioBuffer) {
        self.cpu.bus.apu.inner_mut().swap_audio_buffer(buffer)
    }

    pub fn force_headless(&mut self) {
        self.cpu.bus.ppu.inner_mut().force_headless();
    }

    pub fn save_ppu_state(&self) -> Vec<u8> {
        self.cpu.bus.ppu.inner().save_state()
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
            self.cpu
                .bus
                .ppu
                .inner_mut()
                .swap_framebuffer(&mut self.pending_video_frame);
            self.has_pending_video_frame = true;
        }
    }

    /// Exposes debug information for investigating the system state.
    pub fn debug(&self) -> SystemDebug<'_, PpuT, ApuT> {
        SystemDebug {
            ppu: self.cpu.bus.ppu.inner(),
            apu: self.cpu.bus.apu.inner(),
        }
    }

    /// Exposes an interactive debugger to set break and log points.
    pub fn debugger(&self) -> MutexGuard<'_, Debugger> {
        self.debugger.deref().lock().unwrap()
    }
}

pub struct SystemDebug<'a, PpuT: ManagedBusDeviceU24<Ppu> + 'a, ApuT: ManagedBusDeviceU24<Apu> + 'a>
{
    ppu: PpuT::InnerRef<'a>,
    apu: ApuT::InnerRef<'a>,
}

impl<'a, PpuT: ManagedBusDeviceU24<Ppu> + 'a, ApuT: ManagedBusDeviceU24<Apu> + 'a>
    SystemDebug<'a, PpuT, ApuT>
{
    pub fn ppu(&'a self) -> PpuDebug<'a> {
        self.ppu.debug()
    }

    pub fn apu(&'a self) -> ApuDebug<'a> {
        self.apu.debug()
    }
}
