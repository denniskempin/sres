mod cpu;
mod ppu;

use std::fmt::Debug;

use eframe::CreationContext;
use egui::Context;
use egui::Ui;
use ppu::PpuDebugWindow;
use sres_emulator::ExecutionResult;
use sres_emulator::System;

pub struct DebugUi {
    command: Option<DebugCommand>,
    alert: Alert,
    ppu_debug: PpuDebugWindow,
}

impl DebugUi {
    pub fn new(cc: &CreationContext) -> Self {
        DebugUi {
            command: None,
            alert: Alert::default(),
            ppu_debug: PpuDebugWindow::new(cc),
        }
    }

    fn run_command(
        &mut self,
        emulator: &mut System,
        command: DebugCommand,
        delta_t: f64,
    ) -> ExecutionResult {
        match command {
            DebugCommand::Run => emulator.execute_for_duration(delta_t),
            DebugCommand::StepFrames(n) => {
                self.command = if n > 1 {
                    Some(DebugCommand::StepFrames(n - 1))
                } else {
                    None
                };
                emulator.execute_one_frame()
            }
            DebugCommand::StepScanlines(n) => {
                self.command = if n > 1 {
                    Some(DebugCommand::StepScanlines(n - 1))
                } else {
                    None
                };

                let current_scanline = emulator.cpu.bus.ppu_timer.v;
                emulator.execute_until(|cpu| cpu.bus.ppu_timer.v > current_scanline)
            }
            DebugCommand::StepInstructions(n) => {
                self.command = if n > 1 {
                    Some(DebugCommand::StepInstructions(n - 1))
                } else {
                    None
                };
                emulator.execute_until(|_| true)
            }
        }
    }

    pub fn run_emulator(&mut self, emulator: &mut System, delta_t: f64) {
        if let Some(command) = self.command {
            match self.run_command(emulator, command, delta_t) {
                ExecutionResult::Normal => (),
                ExecutionResult::Halt => {
                    self.alert.show("CPU Halted");
                    self.command = None;
                }
                ExecutionResult::Break(reason) => {
                    self.alert.show(&reason.to_string());
                    self.command = None;
                }
            }
        }
    }

    pub fn modals(&mut self, ctx: &Context, emulator: &mut System) {
        self.alert.render(ctx);
        self.ppu_debug.show(ctx, emulator);
    }

    pub fn right_debug_panel(&mut self, ui: &mut Ui, emulator: &System) {
        cpu::debug_controls_widget(ui, self.command, |command| self.command = command);
        ui.separator();
        cpu::cpu_state_widget(ui, emulator);
        ui.separator();
        cpu::disassembly_widget(ui, emulator);
    }

    pub fn bottom_debug_panel(&mut self, ui: &mut Ui, _emulator: &System) {
        ui.horizontal(|ui| {
            if ui.button("PPU").clicked() {
                self.ppu_debug.open = !self.ppu_debug.open;
            }
            ui.button("APU").clicked();
            if ui.button("Memory").clicked() {}
        });
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DebugCommand {
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
