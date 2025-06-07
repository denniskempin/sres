use egui::Color32;
use egui::Context;
use egui::RichText;
use egui::ScrollArea;
use egui::TextStyle;
use egui::Ui;
use sres_emulator::apu::ApuBusEvent;
use sres_emulator::common::address::AddressU16;
use sres_emulator::common::address::AddressU24;
use sres_emulator::components::cpu::CpuEvent;
use sres_emulator::components::cpu::CpuState;
use sres_emulator::components::spc700::Spc700Event;
use sres_emulator::components::spc700::Spc700State;
use sres_emulator::debugger::DebugEvent;
use sres_emulator::main_bus::MainBusEvent;
use sres_emulator::System;

use super::event::event_filter_widget;
use crate::debug::cpu::ADDR_ANNOTATIONS;
use crate::debug::InternalLink;

pub struct LogViewer {
    is_open: bool,
}

impl LogViewer {
    pub fn new() -> Self {
        Self { is_open: false }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn show(&mut self, ctx: &Context, emulator: &System, selected: &mut InternalLink) {
        egui::Window::new("Log Viewer")
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                let mut debugger = emulator.debugger();
                event_filter_widget(ui, &mut debugger.log_points);

                ui.separator();

                let text_style = TextStyle::Monospace;
                let style = ui.style_mut();
                style.override_text_style = Some(text_style.clone());

                let num_rows = debugger.log.len();
                let row_height = ui.text_style_height(&text_style);

                ScrollArea::vertical()
                    .auto_shrink(false)
                    .stick_to_bottom(true)
                    .show_rows(ui, row_height, num_rows, |ui, row_range| {
                        for row in debugger
                            .log
                            .stack
                            .iter()
                            .rev()
                            .skip(row_range.start)
                            .take(row_range.end - row_range.start)
                        {
                            log_line(ui, row, selected);
                        }
                    });
            });
    }
}

/// Widget to display a single log line with syntax highlighting, hover info and the ability
/// to click addresses.
fn log_line(ui: &mut Ui, event: &DebugEvent, selected: &mut InternalLink) {
    use DebugEvent::*;
    ui.horizontal(|ui| {
        match event {
            MainBus(MainBusEvent::Read(addr, value)) => {
                label_cpu(ui);
                label_read(ui, "R");
                label_cpu_addr(ui, *addr, selected);
                label_normal(ui, format!("= {value:02X}"));
            }
            MainBus(MainBusEvent::Write(addr, value)) => {
                label_cpu(ui);
                label_write(ui, "W");
                label_cpu_addr(ui, *addr, selected);
                label_normal(ui, format!("= {value:02X}"));
            }
            Cpu(CpuEvent::Interrupt(interrupt)) => {
                label_cpu(ui);
                label_normal(ui, format!("IRQ: {interrupt:?}"));
            }
            Error(reason) => {
                label_error(ui, format!("Error: {reason:?}"));
            }
            Cpu(CpuEvent::Step(state)) => {
                label_cpu(ui);
                cpu_log_line(ui, state, selected);
            }
            Spc700(Spc700Event::Step(state)) => {
                spc700_log_line(ui, state, selected);
            }
            ApuBus(ApuBusEvent::Read(addr, value)) => {
                label_spc(ui);
                label_read(ui, "R");
                label_addr(ui, addr.to_string());
                label_normal(ui, format!("= {value:02X}"));
            }
            ApuBus(ApuBusEvent::Write(addr, value)) => {
                label_spc(ui);
                label_write(ui, "W");
                label_addr(ui, addr.to_string());
                label_normal(ui, format!("= {value:02X}"));
            }
        };
    });
}

fn cpu_log_line(ui: &mut Ui, state: &CpuState, selected: &mut InternalLink) {
    label_cpu_pc(ui, state.instruction.address, selected);
    label_strong(ui, state.instruction.operation.clone());
    if let Some(operand) = state.instruction.operand_str.clone() {
        let operand_label = label_addr(ui, format!("{operand:<12}"));
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

fn spc700_log_line(ui: &mut Ui, state: &Spc700State, selected: &mut InternalLink) {
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
    let addr_u32: u32 = addr.into();
    if ADDR_ANNOTATIONS.contains_key(&addr_u32) {
        ui.label(format!("[{}]", ADDR_ANNOTATIONS[&addr_u32]));
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
