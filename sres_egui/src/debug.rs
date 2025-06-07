mod apu;
mod cpu;
mod event;
mod log_viewer;
mod memory;
mod ppu;
mod syntax;

use std::fmt::Debug;
use std::time::Duration;

use apu::ApuDebugWindow;
use eframe::CreationContext;
use egui::Color32;
use egui::Context;
use egui::RichText;
use egui::Ui;
use log_viewer::LogViewer;
use memory::MemoryViewer;
use ppu::PpuDebugWindow;
use sres_emulator::common::address::AddressU16;
use sres_emulator::common::address::AddressU24;
use sres_emulator::common::bus::Bus;
use sres_emulator::common::util::RingBuffer;
use sres_emulator::debugger::BreakReason;
use sres_emulator::ExecutionResult;
use sres_emulator::System;

use crate::util::Instant;

pub struct DebugUi {
    command: DebugCommand,
    alert: Alert,
    ppu_debug: PpuDebugWindow,
    apu_debug: ApuDebugWindow,
    memory_viewer: MemoryViewer,
    past_emulation_times: RingBuffer<Duration, 60>,
    log_viewer: LogViewer,
    selected_memory_location: InternalLink,
    pub show_profiler: bool,
    break_reason: Option<BreakReason>,
}

impl DebugUi {
    pub fn new(cc: &CreationContext) -> Self {
        DebugUi {
            command: DebugCommand::Pause,
            alert: Alert::default(),
            ppu_debug: PpuDebugWindow::new(cc),
            apu_debug: ApuDebugWindow::new(),
            memory_viewer: MemoryViewer::new("CPU Bus"),
            show_profiler: false,
            log_viewer: LogViewer::new(),
            past_emulation_times: RingBuffer::default(),
            selected_memory_location: InternalLink::None,
            break_reason: None,
        }
    }

    fn run_command(
        &mut self,
        emulator: &mut System,
        command: DebugCommand,
        _delta_t: f64,
    ) -> ExecutionResult {
        match command {
            DebugCommand::Pause => ExecutionResult::Normal,
            DebugCommand::Run => {
                let start = Instant::now();
                let result = emulator.execute_frames(1);
                if let ExecutionResult::Normal = result {
                    self.past_emulation_times.push(start.elapsed());
                }
                result
            }
            DebugCommand::StepFrames(n) => {
                self.command = if n > 1 {
                    DebugCommand::StepFrames(n - 1)
                } else {
                    DebugCommand::Pause
                };
                emulator.execute_frames(1)
            }
            DebugCommand::StepScanlines(n) => {
                self.command = if n > 1 {
                    DebugCommand::StepScanlines(n - 1)
                } else {
                    DebugCommand::Pause
                };
                emulator.execute_scanlines(1)
            }
            DebugCommand::StepInstructions(n) => {
                self.command = if n > 1 {
                    DebugCommand::StepInstructions(n - 1)
                } else {
                    DebugCommand::Pause
                };
                emulator.execute_one_instruction()
            }
        }
    }

    pub fn run_emulator(&mut self, emulator: &mut System, delta_t: f64) {
        puffin::profile_function!();
        if !matches!(self.command, DebugCommand::Pause) {
            match self.run_command(emulator, self.command, delta_t) {
                ExecutionResult::Normal => (),
                ExecutionResult::Halt => {
                    self.alert.show("CPU Halted");
                    self.command = DebugCommand::Pause;
                }
                ExecutionResult::Break(reason) => {
                    self.break_reason = Some(reason);
                    self.command = DebugCommand::Pause;
                }
            }
        }
    }

    pub fn modals(&mut self, ctx: &Context, emulator: &mut System) {
        self.alert.render(ctx);
        self.ppu_debug.show(ctx, emulator);
        self.apu_debug.show(ctx, emulator);
        /* if self.show_profiler && !puffin_egui::profiler_window(ctx) {
            self.show_profiler = false;
        } */
        self.log_viewer
            .show(ctx, emulator, &mut self.selected_memory_location);

        match self.selected_memory_location {
            InternalLink::None => (),
            InternalLink::CpuMemory(addr) => {
                self.memory_viewer.open_at(addr);
            }
            InternalLink::CpuProgramCounter(addr) => {
                self.memory_viewer.open_at(addr);
            }
            InternalLink::Spc700ProgramCounter(_) => (),
        }
        self.selected_memory_location = InternalLink::None;

        self.memory_viewer
            .show(ctx, |addr| emulator.cpu.bus.peek_u8(addr));
    }

    pub fn right_debug_panel(&mut self, ui: &mut Ui, emulator: &System) {
        self.perf_widget(ui);
        ui.separator();
        if let Some(command) = cpu::debug_controls_widget(ui, self.command) {
            self.command = command;
            self.break_reason = None;
        }

        if let Some(ref reason) = self.break_reason {
            ui.label(
                RichText::new(format!("Breakpoint: {}", reason.trigger))
                    .strong()
                    .color(Color32::RED),
            );
        }

        ui.separator();
        event::event_filter_widget(ui, &mut emulator.debugger().break_points);
        ui.separator();
        cpu::cpu_state_widget(ui, emulator);
        ui.separator();
        cpu::disassembly_widget(ui, emulator, &mut self.selected_memory_location);
    }

    pub fn bottom_debug_panel(&mut self, ui: &mut Ui, _emulator: &System) {
        ui.horizontal(|ui| {
            if ui.button("PPU").clicked() {
                self.ppu_debug.toggle();
            }
            if ui.button("APU").clicked() {
                self.apu_debug.toggle();
            }
            if ui.button("Memory").clicked() {
                self.memory_viewer.toggle();
            }
            if ui.button("Profiler").clicked() {
                self.show_profiler = !self.show_profiler;
            }
            if ui.button("Log Viewer").clicked() {
                self.log_viewer.toggle();
            }
        });
    }

    fn perf_widget(&self, ui: &mut Ui) {
        let avg_duration = self
            .past_emulation_times
            .iter()
            .map(|d| d.as_secs_f64())
            .sum::<f64>()
            / self.past_emulation_times.len() as f64;
        ui.label(format!("Emulation: {:.2}ms", avg_duration * 1000.0));
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DebugCommand {
    Pause,
    Run,
    StepFrames(u32),
    StepInstructions(u32),
    StepScanlines(u32),
}

#[derive(Default)]
pub struct Alert {
    text: String,
    is_open: bool,
}

impl Alert {
    pub fn render(&mut self, ctx: &Context) {
        egui::Window::new("Error")
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                ui.label(self.text.clone());
            });
    }

    pub fn show(&mut self, text: &str) {
        self.text = text.to_string();
        self.is_open = true;
    }
}

#[allow(dead_code)]
pub enum InternalLink {
    None,
    CpuMemory(AddressU24),
    CpuProgramCounter(AddressU24),
    Spc700ProgramCounter(AddressU16),
}
