use std::ops::DerefMut;

use egui::TextStyle;
use egui::Ui;
use egui_hooks::UseHookExt;
use sres_emulator::debugger::EventFilter;

pub fn event_filter_widget(ui: &mut Ui, event_filters: &mut Vec<EventFilter>) {
    ui.vertical(|ui| {
        event_filter_input_widget(ui, event_filters);
        event_filter_list_widget(ui, event_filters);
    });
}

fn event_filter_input_widget(ui: &mut Ui, event_filters: &mut Vec<EventFilter>) {
    let mut breakpoint_text = ui.use_state(String::default, ()).into_var();
    let error_message = ui.use_state(Option::<String>::default, ());
    let mut show_help = ui.use_state(|| false, ()).into_var();

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
                        event_filters.push(filter);
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
    ui.collapsing("Events", |ui| {
        event_filter_quick_add(ui, event_filters);
    });
    event_filter_help_window(ui, show_help.deref_mut());

    if let Some(ref error_msg) = *error_message {
        ui.colored_label(egui::Color32::RED, error_msg);
    }
}

fn event_filter_quick_add(ui: &mut Ui, event_filters: &mut Vec<EventFilter>) {
    let mut log_point_button = |ui: &mut Ui, label: &str, filter: EventFilter| {
        if ui
            .add(egui::Button::new(label).selected(event_filters.contains(&filter)))
            .clicked()
        {
            if !event_filters.contains(&filter) {
                event_filters.push(filter);
            } else {
                event_filters.retain(|t| t != &filter)
            }
        }
    };
    let text_style = TextStyle::Monospace;
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
