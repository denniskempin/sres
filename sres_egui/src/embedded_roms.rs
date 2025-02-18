use egui::ImageSource;
use egui::include_image;

pub struct CategoryInfo {
    pub name: &'static str,
    pub roms: &'static [RomFileInfo],
}

pub struct RomFileInfo {
    pub name: &'static str,
    pub image: ImageSource<'static>,
    pub attribution: Option<( &'static str, &'static str)>,
    pub path: &'static str,
}

#[allow(dead_code)]
const DEFAULT_IMAGE: ImageSource<'static> = include_image!("../roms/default.png");

// Include the generated ROM files
pub const EMBEDDED_ROMS: &[CategoryInfo] =
    include!(concat!(env!("OUT_DIR"), "/embedded_roms_generated.rs"));