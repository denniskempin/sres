use egui::Color32;
use egui::RichText;
use egui::Ui;
use sres_emulator::common::address::AddressU16;
use sres_emulator::common::address::AddressU24;
use sres_emulator::common::debug_events::ApuEvent;
use sres_emulator::common::debug_events::CpuEvent;
use sres_emulator::common::debug_events::DebugEvent;

use super::InternalLink;

/// Widget to display a single log line with syntax highlighting, hover info and the ability
/// to click addresses.
pub fn log_line(ui: &mut Ui, event: &DebugEvent, selected: &mut InternalLink) {
    use DebugEvent::*;
    ui.horizontal(|ui| {
        match event {
            Cpu(CpuEvent::Read(addr, value)) => {
                label_cpu(ui);
                label_read(ui, "R");
                label_cpu_addr(ui, *addr, selected);
                label_normal(ui, format!("= {:02X}", value));
            }
            Cpu(CpuEvent::Write(addr, value)) => {
                label_cpu(ui);
                label_write(ui, "W");
                label_cpu_addr(ui, *addr, selected);
                label_normal(ui, format!("= {:02X}", value));
            }
            Cpu(CpuEvent::Interrupt(interrupt)) => {
                label_cpu(ui);
                label_normal(ui, format!("IRQ: {:?}", interrupt));
            }
            Error(reason) => {
                label_error(ui, format!("Error: {:?}", reason));
            }
            Cpu(CpuEvent::Step(state)) => {
                label_cpu(ui);
                cpu_log_line(ui, state, selected);
            }
            Apu(ApuEvent::Step(state)) => {
                spc700_log_line(ui, state, selected);
            }
            Apu(ApuEvent::Read(addr, value)) => {
                label_spc(ui);
                label_read(ui, "R");
                label_addr(ui, addr.to_string());
                label_normal(ui, format!("= {:02X}", value));
            }
            Apu(ApuEvent::Write(addr, value)) => {
                label_spc(ui);
                label_write(ui, "W");
                label_addr(ui, addr.to_string());
                label_normal(ui, format!("= {:02X}", value));
            }
        };
    });
}

fn cpu_log_line(
    ui: &mut Ui,
    state: &sres_emulator::common::trace::CpuTraceLine,
    selected: &mut InternalLink,
) {
    label_cpu_pc(ui, state.instruction.address, selected);
    label_strong(ui, state.instruction.operation.clone());
    if let Some(operand) = state.instruction.operand_str.clone() {
        let operand_label = label_addr(ui, format!("{:<12}", operand));
        // If there is an effective address, show on hover and link on click.
        if let Some(addr) = state.instruction.effective_addr {
            if operand_label.on_hover_text(addr.to_string()).clicked() {
                *selected = InternalLink::CpuMemory(addr);
            }
        }
    } else {
        label_normal(ui, "            ");
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
    label_strong(ui, state.status.to_string());
}

fn spc700_log_line(
    ui: &mut Ui,
    state: &sres_emulator::common::trace::Spc700TraceLine,
    selected: &mut InternalLink,
) {
    label_spc(ui);
    label_spc700_pc(ui, state.instruction.address, selected);
    label_normal(ui, " ");
    label_strong(ui, format!("{:<5}", state.instruction.operation));
    label_strong(
        ui,
        format!(
            "{:<11}",
            state.instruction.operand_str.as_deref().unwrap_or("")
        ),
    );
    label_strong(ui, "A");
    label_normal(ui, format!("{:02X}", state.a));
    label_strong(ui, "X");
    label_normal(ui, format!("{:02X}", state.x));
    label_strong(ui, "Y");
    label_normal(ui, format!("{:02X}", state.y));
    label_strong(ui, "SP");
    label_normal(ui, format!("{:02X}", state.sp.0));
    label_strong(ui, "P");
    label_normal(ui, state.status.to_string());
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

fn label_spc700_pc(ui: &mut Ui, addr: AddressU16, selected: &mut InternalLink) {
    if label_addr(ui, addr.to_string()).clicked() {
        *selected = InternalLink::Spc700ProgramCounter(addr);
    }
}

fn label_cpu(ui: &mut Ui) -> egui::Response {
    ui.label(RichText::new("CPU:").color(Color32::LIGHT_GREEN))
}

fn label_spc(ui: &mut Ui) -> egui::Response {
    ui.label(RichText::new("SPC:").color(Color32::LIGHT_YELLOW))
}

fn label_read(ui: &mut Ui, text: impl Into<String>) -> egui::Response {
    ui.label(RichText::new(text.into()).color(Color32::LIGHT_GRAY))
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
