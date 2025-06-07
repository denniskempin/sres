use egui::Context;
use egui::FontId;
use egui::Label;
use egui::RichText;
use egui::ScrollArea;
use egui::TextStyle;
use itertools::Itertools;
use sres_emulator::common::address::Address;
use sres_emulator::common::address::AddressU24;
use sres_emulator::common::address::Wrap;

pub struct MemoryViewer {
    title: String,
    is_open: bool,
    scroll_to_location: Option<AddressU24>,
}

impl MemoryViewer {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            is_open: false,
            scroll_to_location: None,
        }
    }

    pub fn open_at(&mut self, location: AddressU24) {
        self.scroll_to_location = Some(location);
        self.is_open = true;
    }

    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn show<F>(&mut self, ctx: &Context, peek: F)
    where
        F: Fn(AddressU24) -> Option<u8>,
    {
        egui::Window::new(&self.title)
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                ui.style_mut().override_font_id = Some(FontId::monospace(10.0));
                ui.add(
                    Label::new(RichText::new(
                        "      00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F",
                    ))
                    .wrap_mode(egui::TextWrapMode::Extend),
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
                        let addr = AddressU24::from(row as u32 * bytes_per_line);
                        let bytes =
                            (0..bytes_per_line).map(|offset| peek(addr.add(offset, Wrap::NoWrap)));
                        let bytes_str = bytes
                            .map(|b| {
                                b.map(|b| format!("{b:02X}"))
                                    .unwrap_or_else(|| "XX".to_string())
                            })
                            .join(" ");
                        ui.add(
                            Label::new(RichText::new(format!("{addr}: {bytes_str}")))
                                .wrap_mode(egui::TextWrapMode::Extend),
                        );
                    }
                });
            });
    }
}
