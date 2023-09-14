use std::collections::vec_deque::Iter;
use std::collections::VecDeque;

use egui::Color32;
use egui::ColorImage;
use egui::ImageData;
use sres_emulator::image::Image;
use sres_emulator::image::Rgba;

#[derive(Clone)]
pub struct RingBuffer<T, const N: usize> {
    pub stack: VecDeque<T>,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn pop(&mut self) -> T {
        self.stack.pop_front().unwrap()
    }

    pub fn push(&mut self, data: T) {
        self.stack.push_front(data);
        self.stack.truncate(N);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.stack.iter()
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self {
            stack: Default::default(),
        }
    }
}
pub struct EguiImageImpl {
    inner: ColorImage,
}

impl Image for EguiImageImpl {
    fn new(width: u32, height: u32) -> Self {
        EguiImageImpl {
            inner: ColorImage::new([width as usize, height as usize], Color32::TRANSPARENT),
        }
    }

    fn set_pixel(&mut self, index: (u32, u32), value: Rgba) {
        self.inner[(index.0 as usize, index.1 as usize)] =
            Color32::from_rgba_premultiplied(value.0[0], value.0[1], value.0[2], value.0[3]);
    }
}

impl From<EguiImageImpl> for ImageData {
    fn from(value: EguiImageImpl) -> Self {
        ImageData::Color(value.inner)
    }
}
