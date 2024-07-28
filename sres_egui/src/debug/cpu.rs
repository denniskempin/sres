use egui::Button;
use egui::Color32;
use egui::RichText;
use egui::Ui;
use sres_emulator::common::address::Address;
use sres_emulator::common::address::InstructionMeta;
use sres_emulator::System;

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
    ui.label(format!(
        "Cycle: {}",
        emulator.cpu.bus.ppu.clock_info().master_clock
    ));
    ui.label(format!("PC: {:}", trace.instruction.address));
}

pub fn disassembly_widget(ui: &mut Ui, emulator: &System) {
    ui.label(RichText::new("Operations").strong());

    for trace_line in emulator.debugger().cpu_trace().skip(100) {
        disassembly_line(ui, trace_line.instruction.clone(), false);
    }
    for (idx, meta) in emulator.cpu.debug().peek_next_operations(20).enumerate() {
        disassembly_line(ui, meta, idx == 0);
    }
}

fn disassembly_line<AddressT: Address>(
    ui: &mut Ui,
    meta: InstructionMeta<AddressT>,
    current: bool,
) {
    ui.horizontal(|ui| {
        let addr_str = if current {
            format!("> {:}", meta.address)
        } else {
            format!("  {:}", meta.address)
        };
        ui.label(RichText::new(addr_str));
        ui.label(RichText::new(meta.operation).strong());
        if let Some(operand_str) = meta.operand_str {
            let mut text = RichText::new(operand_str.clone()).strong();
            if operand_str.starts_with('$')
                || operand_str.starts_with('[')
                || operand_str.starts_with('(')
            {
                text = text.color(Color32::LIGHT_YELLOW);
            } else if operand_str.starts_with('#') {
                text = text.color(Color32::LIGHT_GREEN);
            } else if operand_str.starts_with('+') | operand_str.starts_with('-') {
                text = text.color(Color32::LIGHT_RED);
            }
            ui.label(text);
        }
        if let Some(effective_addr) = meta.effective_addr {
            ui.label(
                RichText::new(format!("[{:}]", effective_addr))
                    .strong()
                    .color(Color32::LIGHT_BLUE),
            );
        }
    });
}

pub fn debug_controls_widget(
    ui: &mut Ui,
    current_command: Option<DebugCommand>,
    mut on_new_command: impl FnMut(Option<DebugCommand>),
) {
    ui.horizontal_wrapped(|ui| {
        let paused = current_command.is_none();

        if ui.button(if paused { "Run" } else { "Pause" }).clicked() {
            if paused {
                on_new_command(Some(DebugCommand::Run));
            } else {
                on_new_command(None);
            }
        }
        if ui.add_enabled(paused, Button::new("Step")).clicked() {
            on_new_command(Some(DebugCommand::StepInstructions(1)));
        }

        if ui.add_enabled(paused, Button::new("Step Frame")).clicked() {
            on_new_command(Some(DebugCommand::StepFrames(1)));
        }
        if ui
            .add_enabled(paused, Button::new("Step Scanline"))
            .clicked()
        {
            on_new_command(Some(DebugCommand::StepScanlines(1)));
        }
    });
}
