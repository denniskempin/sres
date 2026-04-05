use egui::Context;
use egui::ScrollArea;
use egui::TextStyle;
use sres_emulator::System;

use super::event::event_filter_widget;
use super::syntax::log_line;
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
