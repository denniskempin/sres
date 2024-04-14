use egui::Color32;
use egui::RichText;
use egui::Ui;
use sres_emulator::bus::AddressU24;
use sres_emulator::debugger::Event;

use super::InternalLink;

/// Widget to display a single log line with syntax highlighting, hover info and the ability
/// to click addresses.
pub fn log_line(ui: &mut Ui, event: &Event, selected: &mut InternalLink) {
    ui.horizontal(|ui| {
        match event {
            Event::CpuMemoryRead(addr) => {
                label_read(ui, "R");
                label_cpu_addr(ui, *addr, selected);
            }
            Event::CpuMemoryWrite(addr, value) => {
                label_write(ui, "W");
                label_cpu_addr(ui, *addr, selected);
                label_normal(ui, format!("= {:02X}", value));
            }
            Event::CpuInterrupt(interrupt) => {
                label_normal(ui, format!("IRQ: {:?}", interrupt));
            }
            Event::ExecutionError(reason) => {
                label_error(ui, format!("Error: {:?}", reason));
            }
            Event::CpuStep(state) => {
                cpu_log_line(ui, state, selected);
            }
        };
    });
}

fn cpu_log_line(
    ui: &mut Ui,
    state: &sres_emulator::trace::CpuTraceLine,
    selected: &mut InternalLink,
) {
    label_cpu_pc(ui, state.instruction.address, selected);
    label_strong(ui, state.instruction.operation.clone());
    if let Some(operand) = state.instruction.operand_str.clone() {
        let operand_label = label_addr(ui, format!("{:<10}", operand));
        // If there is an effective address, show on hover and link on click.
        if let Some(addr) = state.instruction.effective_addr {
            if operand_label.on_hover_text(addr.to_string()).clicked() {
                *selected = InternalLink::CpuMemory(addr);
            }
        }
    } else {
        label_normal(ui, "          ");
    }

    label_strong(ui, "A");
    label_normal(ui, format!("{:04X}", state.a));
    label_strong(ui, "X");
    label_normal(ui, format!("{:04X}", state.x));
    label_strong(ui, "Y");
    label_normal(ui, format!("{:04X}", state.y));
    label_strong(ui, "S");
    label_normal(ui, format!("{:04X}", state.s));
    label_strong(ui, "D");
    label_normal(ui, format!("{:04X}", state.d));
    label_strong(ui, "DB");
    label_normal(ui, format!("{:02X}", state.db));
    label_strong(ui, String::from(state.status)).on_hover_ui(|ui| {
        ui.vertical(|ui| {
            if state.status.negative {
                label_strong(ui, "Negative");
            } else {
                label_normal(ui, "Negative");
            }
            if state.status.overflow {
                label_strong(ui, "Overflow");
            } else {
                label_normal(ui, "Overflow");
            }
            if state.status.accumulator_register_size {
                label_strong(ui, "A 8-bit");
            } else {
                label_normal(ui, "A 16-bit");
            }
            if state.status.index_register_size_or_break {
                label_strong(ui, "XY 8-bit");
            } else {
                label_normal(ui, "XY 16-bit");
            }
            if state.status.decimal {
                label_strong(ui, "Decimal");
            } else {
                label_normal(ui, "Decimal");
            }
            if state.status.irq_disable {
                label_strong(ui, "IRQ Disable");
            } else {
                label_normal(ui, "IRQ Enable");
            }
            if state.status.zero {
                label_strong(ui, "Zero");
            } else {
                label_normal(ui, "Zero");
            }
            if state.status.carry {
                label_strong(ui, "Carry");
            } else {
                label_normal(ui, "Carry");
            }
        });
    });
}
fn label_addr(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::LIGHT_BLUE))
}

fn label_cpu_addr(ui: &mut Ui, addr: AddressU24, selected: &mut InternalLink) {
    if label_addr(ui, addr.to_string()).clicked() {
        *selected = InternalLink::CpuMemory(addr);
    }
}

fn label_cpu_pc(ui: &mut Ui, addr: AddressU24, selected: &mut InternalLink) {
    if label_addr(ui, addr.to_string()).clicked() {
        *selected = InternalLink::CpuProgramCounter(addr);
    }
}

fn label_read(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::LIGHT_GREEN))
}

fn label_write(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::LIGHT_RED))
}

fn label_error(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::RED))
}

fn label_normal(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()))
}

fn label_strong(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::WHITE).strong())
}
