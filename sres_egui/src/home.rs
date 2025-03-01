use std::path::PathBuf;

use egui::Context;
use egui::FontId;
use egui::ImageSource;
use egui::OpenUrl;
use egui::RichText;
use egui::Sense;
use egui::Ui;
use sres_emulator::components::cartridge::Cartridge;

use crate::embedded_roms::RomFileInfo;
use crate::embedded_roms::EMBEDDED_ROMS;

pub fn home_screen<F>(ctx: &Context, on_load_cartridge: F)
where
    F: FnMut(Cartridge),
{
    let mut callback = on_load_cartridge;
    egui::CentralPanel::default().show(ctx, |ui| {
        for category in EMBEDDED_ROMS {
            ui.vertical(|ui| {
                ui.label(RichText::new(category.name).font(FontId::proportional(32.0)));
                ui.horizontal(|ui| {
                    for rom_info in category.roms {
                        cartridge_card(ctx, ui, rom_info, &mut callback);
                    }
                });
            });
        }
    });
}

fn cartridge_card<F>(ctx: &Context, ui: &mut Ui, rom_info: &RomFileInfo, on_load_cartridge: &mut F)
where
    F: FnMut(Cartridge),
{
    egui::Frame::window(ui.style()).show(ui, |ui| {
        ui.vertical(|ui| {
            if ui
                .add(
                    egui::Image::new(ImageSource::Bytes {
                        uri: rom_info.path.into(),
                        bytes: egui::load::Bytes::Static(rom_info.image),
                    })
                    .fit_to_exact_size(egui::Vec2::splat(256.0))
                    .sense(Sense::click()),
                )
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                on_cartridge_click(rom_info, on_load_cartridge);
            }
            if ui
                .label(RichText::new(rom_info.name).heading())
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                on_cartridge_click(rom_info, on_load_cartridge);
            }
            if let Some((author, url)) = rom_info.attribution {
                ui.horizontal(|ui| {
                    ui.label("By:");
                    if ui.link(author).clicked() {
                        ctx.open_url(OpenUrl {
                            url: url.to_string(),
                            new_tab: true,
                        });
                    }
                });
            } else {
                ui.label("");
            }
        });
    });
}

fn on_cartridge_click<F>(rom_info: &RomFileInfo, on_load_cartridge: &mut F)
where
    F: FnMut(Cartridge),
{
    let path = PathBuf::from("roms").join(rom_info.path);
    println!("Loading ROM: {:?}", path);
    let cartridge = Cartridge::with_sfc_data(rom_info.rom_data, None).unwrap();
    on_load_cartridge(cartridge);
}
