//! Unimplemented-behavior pane: hit counts, `EventFilter::Unimplemented` break checkbox, and reset.
//! Inspection only; does not execute.

use egui::Context;
use egui::ScrollArea;
use egui::TextStyle;
use egui::Ui;
use sres_emulator::common::unimplemented::UnimplementedBehavior;
use sres_emulator::debugger::EventFilter;
use sres_emulator::System;

pub struct UnimplementedViewer {
    is_open: bool,
}

impl UnimplementedViewer {
    pub fn new() -> Self {
        Self { is_open: false }
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn show(&mut self, ctx: &Context, emulator: &System) {
        let Self { is_open } = self;
        egui::Window::new("Unimplemented")
            .open(is_open)
            .show(ctx, |ui| {
                let mut debugger = emulator.debugger();
                let hits = debugger.unimplemented_hits();
                let mut break_on = debugger.has_break_point(&EventFilter::Unimplemented);
                let mut reset = false;
                ScrollArea::vertical().show(ui, |ui| {
                    unimplemented_widget(ui, &hits, &mut break_on, &mut reset);
                });
                if break_on {
                    debugger.add_break_point(EventFilter::Unimplemented);
                } else {
                    debugger.remove_break_point(&EventFilter::Unimplemented);
                }
                if reset {
                    debugger.clear_unimplemented_counts();
                }
            });
    }
}

pub fn unimplemented_widget(
    ui: &mut Ui,
    hits: &[(UnimplementedBehavior, u64)],
    break_on: &mut bool,
    reset: &mut bool,
) {
    ui.checkbox(break_on, "Break when reached");
    if ui.button("Reset counts").clicked() {
        *reset = true;
    }
    ui.separator();

    if hits.is_empty() {
        ui.label("No unimplemented behaviors reached.");
        return;
    }

    ui.style_mut().override_text_style = Some(TextStyle::Monospace);
    for (behavior, count) in hits {
        ui.horizontal(|ui| {
            ui.label(format!("{count:>8}"));
            ui.label(behavior.to_string());
        });
    }
}

#[cfg(test)]
mod tests {
    use sres_emulator::common::unimplemented::UnimplementedBehavior;

    use super::*;

    #[test]
    fn unimplemented_widget_snapshot() {
        let hits = [
            (UnimplementedBehavior::SerialJoypadRead, 42),
            (UnimplementedBehavior::Wrio, 7),
            (UnimplementedBehavior::PpuStat77Read, 1),
        ];
        let mut break_on = true;
        crate::test_utils::widget_snapshot("unimplemented/unimplemented_widget", |ui| {
            let mut reset = false;
            unimplemented_widget(ui, &hits, &mut break_on, &mut reset);
        });
    }
}
