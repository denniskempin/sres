use std::ops::DerefMut;

use egui::Ui;
use egui_hooks::UseHookExt;
use sres_emulator::debugger::EventFilter;

pub fn event_filter_widget(ui: &mut Ui, event_filters: &mut Vec<EventFilter>) {
    ui.vertical(|ui| {
        if let Some(breakpoint) = event_filter_input_widget(ui) {
            event_filters.push(breakpoint);
        }

        event_filter_list_widget(ui, event_filters);
    });
}

fn event_filter_input_widget(ui: &mut Ui) -> Option<EventFilter> {
    let mut breakpoint_text = ui.use_state(String::default, ()).into_var();
    let error_message = ui.use_state(Option::<String>::default, ());
    let mut show_help = ui.use_state(|| false, ()).into_var();

    let mut breakpoint_to_add = None;
    ui.horizontal(|ui| {
        let response = ui.add(
            egui::TextEdit::singleline(breakpoint_text.deref_mut())
                .hint_text("e.g. pc 8000, r 2100:2140, irq nmi, LDA"),
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

        if ui.button("?").clicked() {
            *show_help = true;
        }
    });

    event_filter_help_window(ui, show_help.deref_mut());

    // Display error message if any
    if let Some(ref error_msg) = *error_message {
        ui.colored_label(egui::Color32::RED, error_msg);
    }

    breakpoint_to_add
}

fn event_filter_help_window(ui: &mut Ui, show: &mut bool) {
    egui::Window::new("Event Filter Help")
        .open(show)
        .resizable(true)
        .default_width(500.0)
        .show(ui.ctx(), |ui| {
            let text_style = egui::TextStyle::Monospace;
            let style = ui.style_mut();
            style.override_text_style = Some(text_style.clone());

            ui.heading("Supported Event Filter Formats");
            ui.separator();

            ui.label("Event Types:");
            ui.label("  pc <address/range>  - CPU program counter");
            ui.label("  r <address/range>   - CPU memory read");
            ui.label("  w <address/range>   - CPU memory write");
            ui.label("  <instruction>       - CPU instruction (e.g. LDA, JMP)");
            ui.label("  irq [type]          - Interrupt (optional type: nmi, etc.)");
            ui.label("  s-pc <address/range> - SPC700 program counter");
            ui.separator();

            ui.label("Address formats:");
            ui.label("  Addresses are in hexadecimal (no 0x prefix needed)");
            ui.label("  Single address: 8000");
            ui.label("  Range: 8000:9000 (inclusive start, exclusive end)");
            ui.label("  Open range: 8000: or :9000");
            ui.separator();

            ui.label("Examples:");
            ui.label("  pc 8000             - Trigger when PC reaches $8000");
            ui.label("  pc 8000:9000        - Trigger when PC is in range $8000-$8FFF");
            ui.label("  r 2100:2140        - Trigger on reads to PPU registers");
            ui.label("  w 2100              - Trigger on writes to INIDISP register");
            ui.label("  LDA                 - Trigger on any LDA instruction");
            ui.label("  irq nmi             - Trigger on NMI interrupt");
            ui.label("  s-pc 200:300       - Trigger when SPC700 PC is in range $200-$2FF");
        });
}

fn event_filter_list_widget(ui: &mut Ui, event_filters: &mut Vec<EventFilter>) {
    let mut to_remove = Vec::new();
    for (i, event_filter) in event_filters.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.label(format!("{event_filter}"));
            if ui.button("x").clicked() {
                to_remove.push(i);
            }
        });
    }
    for &i in to_remove.iter().rev() {
        event_filters.remove(i);
    }
}
