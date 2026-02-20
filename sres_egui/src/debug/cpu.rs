use egui::Button;
use egui::Color32;
use egui::RichText;
use egui::Ui;
use sres_emulator::System;

use super::syntax::cpu_disassembly_line;
use super::InternalLink;
use crate::debug::DebugCommand;

pub fn cpu_state_widget(ui: &mut Ui, emulator: &System) {
    let trace = &emulator.cpu.debug().state();

    ui.label(RichText::new("CPU").strong());
    ui.horizontal(|ui| {
        ui.label(format!("A {:04X}", trace.a));
        ui.separator();
        ui.label(format!("X {:04X}", trace.x));
        ui.separator();
        ui.label(format!("Y {:04X}", trace.y));
    });
    ui.horizontal(|ui| {
        ui.label("Status:");
        ui.label(trace.status.to_string());
    });
    ui.label(format!("Cycle: {}", emulator.clock_info().master_clock));
    ui.label(format!("PC: {:}", trace.instruction.address));
}

pub fn disassembly_widget(ui: &mut Ui, emulator: &System, selected: &mut InternalLink) {
    ui.label(RichText::new("Operations").strong());
    for trace_line in emulator.debugger().cpu_trace().skip(100) {
        cpu_disassembly_line(ui, trace_line.instruction.clone(), false, selected);
    }
    for (idx, meta) in emulator.cpu.debug().peek_next_operations(20).enumerate() {
        cpu_disassembly_line(ui, meta, idx == 0, selected);
    }
}

fn run_button_widget(ui: &mut Ui, paused: bool) -> egui::Response {
    if paused {
        ui.add(Button::new("Run").fill(Color32::DARK_GREEN))
    } else {
        ui.add(Button::new("Pause").fill(Color32::DARK_RED))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_button_paused() {
        crate::test_utils::widget_snapshot("cpu/run_button_paused", |ui| {
            run_button_widget(ui, true);
        });
    }

    #[test]
    fn run_button_running() {
        crate::test_utils::widget_snapshot("cpu/run_button_running", |ui| {
            run_button_widget(ui, false);
        });
    }

    #[test]
    fn debug_controls_paused() {
        crate::test_utils::widget_snapshot("cpu/debug_controls_paused", |ui| {
            debug_controls_widget(ui, DebugCommand::Pause);
        });
    }

    #[test]
    fn debug_controls_running() {
        crate::test_utils::widget_snapshot("cpu/debug_controls_running", |ui| {
            debug_controls_widget(ui, DebugCommand::Run);
        });
    }
}

pub fn debug_controls_widget(ui: &mut Ui, current_command: DebugCommand) -> Option<DebugCommand> {
    let mut new_command = None;

    ui.horizontal_wrapped(|ui| {
        let paused = matches!(current_command, DebugCommand::Pause);

        if run_button_widget(ui, paused).clicked() {
            if paused {
                new_command = Some(DebugCommand::Run);
            } else {
                new_command = Some(DebugCommand::Pause);
            }
        }
        if ui.add_enabled(paused, Button::new("Step")).clicked() {
            new_command = Some(DebugCommand::StepInstructions(1));
        }

        if ui.add_enabled(paused, Button::new("Step Frame")).clicked() {
            new_command = Some(DebugCommand::StepFrames(1));
        }
        if ui
            .add_enabled(paused, Button::new("Step Scanline"))
            .clicked()
        {
            new_command = Some(DebugCommand::StepScanlines(1));
        }
    });

    new_command
}
