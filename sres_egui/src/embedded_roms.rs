pub struct CategoryInfo {
    pub name: &'static str,
    pub roms: &'static [RomFileInfo],
}

pub struct RomFileInfo {
    pub name: &'static str,
    pub image: &'static [u8],
    pub attribution: Option<(&'static str, &'static str)>,
    pub path: &'static str,
    pub rom_data: &'static [u8],
}

#[allow(dead_code)]
const DEFAULT_IMAGE: &[u8] = include_bytes!("../roms/default.png");

// Include the generated ROM files
pub const EMBEDDED_ROMS: &[CategoryInfo] =
    include!(concat!(env!("OUT_DIR"), "/embedded_roms_generated.rs"));
