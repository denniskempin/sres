use std::collections::vec_deque::Iter;
use std::collections::VecDeque;

use egui::ColorImage;
use egui::TextureHandle;
use egui::TextureOptions;
use image::RgbaImage;

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

pub trait SetFromRgbaImage {
    fn set_from_rgba_image(&mut self, image: &RgbaImage, options: TextureOptions);
}

impl SetFromRgbaImage for TextureHandle {
    fn set_from_rgba_image(&mut self, image: &RgbaImage, options: TextureOptions) {
        self.set(
            ColorImage::from_rgba_unmultiplied(
                [image.width() as usize, image.height() as usize],
                image.as_raw(),
            ),
            options,
        );
    }
}
