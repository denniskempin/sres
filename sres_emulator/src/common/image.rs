//! SNES `Rgb15` (5-bit/channel), host `Rgba32`, palette `ColorIdx`, and `Image`.
//! `Image` is implemented by egui (`sres_egui`) and by `image` in PPU tests.
//! `Rgb15::color_math` adds or subtracts, optional `/2`, then clamps each channel to `0..=31`.
use bitcode::Decode;
use bitcode::Encode;
use intbits::Bits;

/// Conversion factor from u5 to u8
const U5_TO_U8_CONVERSION: f32 = 8.225806;

/// Index into Color Palette
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorIdx(pub u8);

/// RGB format used by SNES, 5 bits per channel
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Encode, Decode)]
pub struct Rgb15(pub u16);

impl Rgb15 {
    pub fn set_r(&mut self, value: u8) {
        self.0 = self.0.with_bits(0..=4, value as u16);
    }

    pub fn set_g(&mut self, value: u8) {
        self.0 = self.0.with_bits(5..=9, value as u16);
    }

    pub fn set_b(&mut self, value: u8) {
        self.0 = self.0.with_bits(10..=14, value as u16);
    }

    pub fn r(&self) -> u8 {
        self.0.bits(0..=4) as u8
    }

    pub fn g(&self) -> u8 {
        self.0.bits(5..=9) as u8
    }

    pub fn b(&self) -> u8 {
        self.0.bits(10..=14) as u8
    }

    /// Color math: add or subtract `sub` from `self`, optional `/2`, then clamp each channel to `0..=31`.
    pub fn color_math(self, sub: Rgb15, subtract: bool, half: bool) -> Rgb15 {
        fn channel(main: u8, sub: u8, subtract: bool, half: bool) -> u8 {
            let mut x = if subtract {
                main as i16 - sub as i16
            } else {
                main as i16 + sub as i16
            };
            if half {
                x /= 2;
            }
            x.clamp(0, 31) as u8
        }
        let mut out = Rgb15(0);
        out.set_r(channel(self.r(), sub.r(), subtract, half));
        out.set_g(channel(self.g(), sub.g(), subtract, half));
        out.set_b(channel(self.b(), sub.b(), subtract, half));
        out
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb(r: u8, g: u8, b: u8) -> Rgb15 {
        let mut color = Rgb15(0);
        color.set_r(r);
        color.set_g(g);
        color.set_b(b);
        color
    }

    #[test]
    fn color_math_add_clamps_to_31() {
        assert_eq!(
            rgb(16, 16, 16).color_math(rgb(16, 16, 16), false, false),
            rgb(31, 31, 31)
        );
    }

    #[test]
    fn color_math_add_half_divides_before_clamp() {
        assert_eq!(
            rgb(16, 16, 16).color_math(rgb(16, 16, 16), false, true),
            rgb(16, 16, 16)
        );
        assert_eq!(
            rgb(31, 31, 31).color_math(rgb(31, 31, 31), false, true),
            rgb(31, 31, 31)
        );
        assert_eq!(
            rgb(31, 31, 31).color_math(rgb(0, 0, 0), false, true),
            rgb(15, 15, 15)
        );
    }

    #[test]
    fn color_math_subtract_clamps_to_0() {
        assert_eq!(
            rgb(5, 5, 5).color_math(rgb(10, 10, 10), true, false),
            rgb(0, 0, 0)
        );
    }
}
