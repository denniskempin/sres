use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
#[cfg(target_arch = "wasm32")]
use std::convert::TryInto;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::sync::Arc;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use egui::Color32;
use egui::ColorImage;
use egui::ImageData;
use sres_emulator::util::image::Image;
use sres_emulator::util::image::Rgba32;

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

    pub fn len(&self) -> usize {
        self.stack.len()
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

    fn set_pixel(&mut self, index: (u32, u32), value: Rgba32) {
        self.inner[(index.0 as usize, index.1 as usize)] =
            Color32::from_rgba_premultiplied(value.0[0], value.0[1], value.0[2], value.0[3]);
    }
}

impl From<EguiImageImpl> for ImageData {
    fn from(value: EguiImageImpl) -> Self {
        ImageData::Color(Arc::new(value.inner))
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(std::time::Instant);
#[cfg(not(target_arch = "wasm32"))]
impl Instant {
    pub fn now() -> Self {
        Self(std::time::Instant::now())
    }
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        self.0.duration_since(earlier.0)
    }
    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        self.0.checked_add(duration).map(Self)
    }
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        self.0.checked_sub(duration).map(Self)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Date, js_name = now)]
    fn date_now() -> f64;
}
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant(u64);
#[cfg(target_arch = "wasm32")]
impl Instant {
    pub fn now() -> Self {
        Self(date_now() as u64)
    }
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_millis(self.0 - earlier.0)
    }
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        match duration.as_millis().try_into() {
            Ok(duration) => self.0.checked_add(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }
    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        match duration.as_millis().try_into() {
            Ok(duration) => self.0.checked_sub(duration).map(|i| Self(i)),
            Err(_) => None,
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, other: Duration) -> Instant {
        self.checked_add(other).unwrap()
    }
}
impl Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, other: Duration) -> Instant {
        self.checked_sub(other).unwrap()
    }
}
impl Sub<Instant> for Instant {
    type Output = Duration;
    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}
impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}
impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}
