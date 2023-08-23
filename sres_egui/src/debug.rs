use std::fmt::Debug;

use eframe::CreationContext;
use egui::Ui;
use sres_emulator::System;

mod cpu;

pub struct DebugUi {
    command: Option<DebugCommand>,
}

impl DebugUi {
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        DebugUi {
            command: Some(DebugCommand::Run),
        }
    }

    fn run_command(&mut self, emulator: &mut System, command: DebugCommand, delta_t: f64) {
        match command {
            DebugCommand::Run => {
                emulator.execute_for_duration(delta_t);
            }
            DebugCommand::StepFrames(n) => {
                emulator.execute_one_frame();
                self.command = if n > 1 {
                    Some(DebugCommand::StepFrames(n - 1))
                } else {
                    None
                };
            }
            DebugCommand::StepScanlines(n) => {
                let current_scanline = emulator.cpu.bus.ppu_timer.v;
                emulator.execute_until(|cpu| cpu.bus.ppu_timer.v > current_scanline);
                self.command = if n > 1 {
                    Some(DebugCommand::StepScanlines(n - 1))
                } else {
                    None
                };
            }
            DebugCommand::StepInstructions(n) => {
                emulator.execute_until(|_| true);
                self.command = if n > 1 {
                    Some(DebugCommand::StepInstructions(n - 1))
                } else {
                    None
                };
            }
        }
    }

    pub fn run_emulator(&mut self, emulator: &mut System, delta_t: f64) {
        if let Some(command) = self.command {
            self.run_command(emulator, command, delta_t);
        }
    }

    pub fn right_debug_panel(&mut self, ui: &mut Ui, emulator: &System) {
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
