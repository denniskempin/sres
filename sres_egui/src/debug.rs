mod cpu;
mod ppu;

use std::fmt::Debug;

use eframe::CreationContext;
use egui::Context;
use egui::FontId;
use egui::Label;
use egui::RichText;
use egui::ScrollArea;
use egui::TextStyle;
use egui::Ui;
use itertools::Itertools;
use ppu::PpuDebugWindow;
use sres_emulator::bus::Bus;
use sres_emulator::util::memory::Address;
use sres_emulator::util::memory::Wrap;
use sres_emulator::ExecutionResult;
use sres_emulator::System;

struct MemoryViewer {
    title: String,
    is_open: bool,
    scroll_to_location: Option<Address>,
}

impl MemoryViewer {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            is_open: false,
            scroll_to_location: None,
        }
    }

    pub fn show<F>(&mut self, ctx: &Context, peek: F)
    where
        F: Fn(Address) -> Option<u8>,
    {
        egui::Window::new(&self.title)
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                ui.style_mut().override_font_id = Some(FontId::monospace(10.0));
                ui.add(
                    Label::new(RichText::new(
                        "      00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F",
                    ))
                    .wrap(false),
                );

                let text_style = TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                let bytes_per_line: u32 = 16;
                let num_rows = 0xFFFFFF / bytes_per_line;

                let mut scroll = ScrollArea::vertical();
                if let Some(location) = self.scroll_to_location {
                    let coarse_location = (u32::from(location) / bytes_per_line) as f32;
                    self.scroll_to_location = None;
                    scroll = scroll.vertical_scroll_offset(
                        coarse_location * (row_height + ui.spacing().item_spacing.y),
                    );
                }
                scroll.show_rows(ui, row_height, num_rows as usize, |ui, row_range| {
                    for row in row_range {
                        let addr = Address::from(row as u32 * bytes_per_line);
                        let bytes =
                            (0..bytes_per_line).map(|offset| peek(addr.add(offset, Wrap::NoWrap)));
                        let bytes_str = bytes
                            .map(|b| {
                                b.map(|b| format!("{:02X}", b))
                                    .unwrap_or_else(|| "XX".to_string())
                            })
                            .join(" ");
                        ui.add(
                            Label::new(RichText::new(format!("{}: {}", addr, bytes_str)))
                                .wrap(false),
                        );
                    }
                });
            });
    }
}

pub struct DebugUi {
    command: Option<DebugCommand>,
    alert: Alert,
    ppu_debug: PpuDebugWindow,
    memory_viewer: MemoryViewer,
}

impl DebugUi {
    pub fn new(cc: &CreationContext) -> Self {
        DebugUi {
            command: None,
            alert: Alert::default(),
            ppu_debug: PpuDebugWindow::new(cc),
            memory_viewer: MemoryViewer::new("CPU Bus"),
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
                emulator.execute_frames(1)
            }
            DebugCommand::StepScanlines(n) => {
                self.command = if n > 1 {
                    Some(DebugCommand::StepScanlines(n - 1))
                } else {
                    None
                };

                let current_scanline = emulator.cpu.bus.ppu.timer.v;
                emulator.execute_until(|cpu| cpu.bus.ppu.timer.v > current_scanline)
            }
            DebugCommand::StepInstructions(n) => {
                self.command = if n > 1 {
                    Some(DebugCommand::StepInstructions(n - 1))
                } else {
                    None
                };
                emulator.execute_until(|_| true)
            }
            DebugCommand::RunToNmi => {
                self.command = None;
                emulator.execute_until(|cpu| cpu.bus.nmi_interrupt && !cpu.status.irq_disable)
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
        self.memory_viewer
            .show(ctx, |addr| emulator.cpu.bus.peek_u8(addr));
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
            if ui.button("Memory").clicked() {
                self.memory_viewer.is_open = !self.memory_viewer.is_open;
            }
        });
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DebugCommand {
    Run,
    StepFrames(u32),
    StepInstructions(u32),
    StepScanlines(u32),
    RunToNmi,
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
