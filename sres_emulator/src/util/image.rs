use intbits::Bits;
use serde::Deserialize;
use serde::Serialize;

/// Conversion factor from u5 to u8
const U5_TO_U8_CONVERSION: f32 = 8.225806;

/// Index into Color Palette
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorIdx(pub u8);

/// RGB format used by SNES, 5 bits per channel
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgb15(pub u16);

/// 32-bit RGBA format used on modern machines for interop with egui and image-rs
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rgba32(pub [u8; 4]);

impl From<Rgb15> for Rgba32 {
    fn from(value: Rgb15) -> Self {
        let r = (value.0.bits(0..=4) as f32 * U5_TO_U8_CONVERSION) as u8;
        let g = (value.0.bits(5..=9) as f32 * U5_TO_U8_CONVERSION) as u8;
        let b = (value.0.bits(10..=14) as f32 * U5_TO_U8_CONVERSION) as u8;
        Self([r, g, b, 255])
    }
}

impl From<Rgba32> for Rgb15 {
    fn from(value: Rgba32) -> Self {
        Self(
            0_u16
                .with_bits(0..=4, (value.0[0] as f32 / U5_TO_U8_CONVERSION) as u16)
                .with_bits(5..=9, (value.0[1] as f32 / U5_TO_U8_CONVERSION) as u16)
                .with_bits(10..=14, (value.0[2] as f32 / U5_TO_U8_CONVERSION) as u16),
        )
    }
}

/// Abstract interface for image::RgbaImage (used in tests) or egui::ColorImage (used in sres_egui).
pub trait Image {
    fn new(width: u32, height: u32) -> Self;
    fn set_pixel(&mut self, index: (u32, u32), value: Rgba32);
}
