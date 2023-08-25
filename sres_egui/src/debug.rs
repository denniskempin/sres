use std::fmt::Debug;

use eframe::CreationContext;
use egui::Ui;
use sres_emulator::ExecutionResult;
use sres_emulator::System;

mod cpu;

pub struct DebugUi {
    command: Option<DebugCommand>,
    alert: Alert,
}

impl DebugUi {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        DebugUi {
            command: None,
            alert: Alert::default(),
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

    pub fn right_debug_panel(&mut self, ui: &mut Ui, emulator: &System) {
        self.alert.render(ui);

        cpu::debug_controls_widget(ui, self.command, |command| self.command = command);
        ui.separator();
        cpu::cpu_state_widget(ui, emulator);
        ui.separator();
        cpu::disassembly_widget(ui, emulator);
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
    pub fn render(&mut self, ui: &mut Ui) {
        egui::Window::new("Error")
            .open(&mut self.is_open)
            .show(ui.ctx(), |ui| {
                ui.label(self.text.clone());
            });
    }

    pub fn show(&mut self, text: &str) {
        self.text = text.to_string();
        self.is_open = true;
    }
}
