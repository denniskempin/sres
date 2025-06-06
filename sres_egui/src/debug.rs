mod apu;
mod cpu;
mod memory;
mod ppu;
mod syntax;

use std::cell::RefMut;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::time::Duration;

use apu::ApuDebugWindow;
use eframe::CreationContext;
use egui::Color32;
use egui::Context;
use egui::RichText;
use egui::ScrollArea;
use egui::Ui;
use egui_hooks::UseHookExt;
use memory::MemoryViewer;
use ppu::PpuDebugWindow;
use sres_emulator::common::address::AddressU16;
use sres_emulator::common::address::AddressU24;
use sres_emulator::common::bus::Bus;
use sres_emulator::common::util::RingBuffer;
use sres_emulator::debugger::BreakReason;
use sres_emulator::debugger::Debugger;
use sres_emulator::debugger::EventFilter;
use sres_emulator::ExecutionResult;
use sres_emulator::System;

use self::syntax::log_line;
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
        self.memory_viewer
            .show(ctx, |addr| emulator.cpu.bus.peek_u8(addr));
        /* if self.show_profiler && !puffin_egui::profiler_window(ctx) {
            self.show_profiler = false;
        } */
        self.log_viewer
            .show(ctx, emulator, &mut self.selected_memory_location);
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
                RichText::new(format!("Breakpoint: {}", reason.trigger.to_string()))
                    .strong()
                    .color(Color32::RED),
            );
        }

        ui.separator();
        breakpoints_widget(ui, emulator.debugger());
        ui.separator();
        cpu::cpu_state_widget(ui, emulator);
        ui.separator();
        cpu::disassembly_widget(ui, emulator);
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

pub fn breakpoints_widget(ui: &mut Ui, mut debugger: RefMut<'_, Debugger>) {
    ui.vertical(|ui| {
        if let Some(breakpoint) = breakpoint_input_widget(ui) {
            debugger.add_break_point(breakpoint);
        }

        active_breakpoints_widget(ui, &mut debugger.break_points);
    });
}

fn breakpoint_input_widget(ui: &mut Ui) -> Option<EventFilter> {
    let mut breakpoint_text = ui.use_state(String::default, ()).into_var();
    let error_message = ui.use_state(Option::<String>::default, ());

    let mut breakpoint_to_add = None;
    ui.horizontal(|ui| {
        let response = ui.add(
            egui::TextEdit::singleline(breakpoint_text.deref_mut())
                .hint_text("e.g. pc 0x8000, r 0x2100..0x2140, irq nmi, LDA"),
        );

        let enter_pressed = response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));

        if enter_pressed || ui.button("Add").clicked() {
            let input = breakpoint_text.trim();
            if !input.is_empty() {
                match input.parse::<EventFilter>() {
                    Ok(filter) => {
                        breakpoint_to_add = Some(filter);
                        breakpoint_text.clear();
                        error_message.set_next(None);
                    }
                    Err(e) => {
                        error_message.set_next(Some(format!("Error: {e}")));
                    }
                }
            }
        }
    });

    // Display error message if any
    if let Some(ref error_msg) = *error_message {
        ui.colored_label(egui::Color32::RED, error_msg);
    }

    breakpoint_to_add
}

fn active_breakpoints_widget(ui: &mut Ui, breakpoints: &mut Vec<EventFilter>) {
    let mut to_remove = Vec::new();
    for (i, breakpoint) in breakpoints.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.label(format!("{breakpoint}"));
            if ui.button("x").clicked() {
                to_remove.push(i);
            }
        });
    }
    for &i in to_remove.iter().rev() {
        breakpoints.remove(i);
    }
}

#[allow(dead_code)]
pub enum InternalLink {
    None,
    CpuMemory(AddressU24),
    CpuProgramCounter(AddressU24),
    Spc700ProgramCounter(AddressU16),
}

struct LogViewer {
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
                let mut log_point_button = |ui: &mut Ui, label: &str, filter: EventFilter| {
                    if ui
                        .add(
                            egui::Button::new(label)
                                .selected(debugger.log_points.contains(&filter)),
                        )
                        .clicked()
                    {
                        debugger.toggle_log_point(filter);
                    }
                };

                let text_style = egui::TextStyle::Monospace;
                let style = ui.style_mut();
                style.override_text_style = Some(text_style.clone());
                ui.horizontal(|ui| {
                    ui.label("CPU:     ");
                    log_point_button(ui, "Step", EventFilter::CpuProgramCounter(0..u32::MAX));
                    log_point_button(ui, "Irq", EventFilter::Interrupt(None));
                    log_point_button(ui, "Err", EventFilter::ExecutionError);
                    ui.label("Bus");
                    log_point_button(ui, "R", EventFilter::CpuMemoryRead(0..u32::MAX));
                    log_point_button(ui, "W", EventFilter::CpuMemoryWrite(0..u32::MAX));
                });

                ui.horizontal(|ui| {
                    ui.label("CPU MMIO:");
                    ui.label("PPU");
                    log_point_button(ui, "R", EventFilter::CpuMemoryRead(0x2100..0x2140));
                    log_point_button(ui, "W", EventFilter::CpuMemoryWrite(0x2100..0x2140));
                    ui.label("APU");
                    log_point_button(ui, "R", EventFilter::CpuMemoryRead(0x2140..0x2144));
                    log_point_button(ui, "W", EventFilter::CpuMemoryWrite(0x2140..0x2144));
                    ui.label("WRAM");
                    log_point_button(ui, "R", EventFilter::CpuMemoryRead(0x2180..0x2184));
                    log_point_button(ui, "W", EventFilter::CpuMemoryWrite(0x2180..0x2184));
                    ui.label("Other");
                    log_point_button(ui, "R", EventFilter::CpuMemoryRead(0x4016..0x4400));
                    log_point_button(ui, "W", EventFilter::CpuMemoryWrite(0x4016..0x4400));
                });

                ui.horizontal(|ui| {
                    ui.label("SPC:     ");
                    log_point_button(ui, "Step", EventFilter::Spc700ProgramCounter(0..u16::MAX));
                    ui.label("Bus");
                    log_point_button(ui, "R", EventFilter::Spc700MemoryRead(0..u16::MAX));
                    log_point_button(ui, "W", EventFilter::Spc700MemoryWrite(0..u16::MAX));
                });

                ui.horizontal(|ui| {
                    ui.label("SPC MMIO:");
                    ui.label("Ctrl");
                    log_point_button(ui, "R", EventFilter::Spc700MemoryRead(0x00f1..0x00f2));
                    log_point_button(ui, "W", EventFilter::Spc700MemoryWrite(0x00f1..0x00f2));
                    ui.label("CPU");
                    log_point_button(ui, "R", EventFilter::Spc700MemoryRead(0x00f4..0x00f8));
                    log_point_button(ui, "W", EventFilter::Spc700MemoryWrite(0x00f4..0x00f8));
                    ui.label("DSP");
                    log_point_button(ui, "R", EventFilter::Spc700MemoryRead(0x00f2..0x00f4));
                    log_point_button(ui, "W", EventFilter::Spc700MemoryWrite(0x00f2..0x00f4));
                    ui.label("Timer");
                    log_point_button(ui, "R", EventFilter::Spc700MemoryRead(0x00fa..0x0100));
                    log_point_button(ui, "W", EventFilter::Spc700MemoryWrite(0x00fa..0x0100));
                });

                ui.separator();

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
