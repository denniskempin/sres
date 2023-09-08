use intbits::Bits;

#[derive(Default, Clone, Copy, Debug)]
pub struct ColorIdx(pub u8);

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rgba(pub [u8; 4]);

const CONVERSION_FACTOR: f32 = 8.225806;

impl Rgba {
    pub fn from_rgb_u15(value: u16) -> Self {
        let r = (value.bits(0..=4) as f32 * CONVERSION_FACTOR) as u8;
        let g = (value.bits(5..=9) as f32 * CONVERSION_FACTOR) as u8;
        let b = (value.bits(10..=14) as f32 * CONVERSION_FACTOR) as u8;
        Self([r, g, b, 255])
    }

    pub fn to_rgb_u15(self) -> u16 {
        0_u16
            .with_bits(0..=4, (self.0[0] as f32 / CONVERSION_FACTOR) as u16)
            .with_bits(5..=9, (self.0[1] as f32 / CONVERSION_FACTOR) as u16)
            .with_bits(10..=14, (self.0[2] as f32 / CONVERSION_FACTOR) as u16)
    }
}

pub struct Framebuffer {
    pub data: Vec<ColorIdx>,
}

impl Framebuffer {}

impl Default for Framebuffer {
    fn default() -> Self {
        Self {
            data: vec![ColorIdx::default(); 240 * 160],
        }
    }
}
