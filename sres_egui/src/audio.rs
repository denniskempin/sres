//! `AudioOutput` plays 32 kHz APU samples through a cpal stream at the device rate.
//! The callback linearly resamples from `AudioBufferQueue`; `update` always `swap_audio_buffer`s.

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::anyhow;
use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use cpal::SampleFormat;
use cpal::SizedSample;
use cpal::Stream;
use cpal::StreamConfig;
use log::error;
use log::info;
use sres_emulator::apu::AudioBuffer;
use sres_emulator::apu::APU_SAMPLE_RATE;
use sres_emulator::System;

const TARGET_BUFFER_SIZE: usize = 1024;

/// Audio output handler that manages playback of SNES APU audio samples
pub struct AudioOutput {
    stream: Option<Stream>,
    buffer_queue: Arc<Mutex<AudioBufferQueue>>,
    output_sample_rate: Option<u32>,
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioOutput {
    pub fn new() -> Self {
        Self {
            stream: None,
            buffer_queue: Arc::new(Mutex::new(AudioBufferQueue::default())),
            output_sample_rate: None,
        }
    }

    pub fn is_playing(&self) -> bool {
        self.stream.is_some()
    }

    pub fn start(&mut self) {
        if self.stream.is_some() {
            return;
        }

        let stream = match self.setup_audio_stream() {
            Ok(stream) => stream,
            Err(err) => {
                error!("Failed to setup audio stream: {err}");
                return;
            }
        };

        if let Err(err) = stream.play() {
            error!("Error playing audio stream: {err}");
            return;
        }
        info!(
            "Starting audio output at {} Hz",
            self.output_sample_rate
                .expect("setup_audio_stream sets output_sample_rate")
        );
        self.stream = Some(stream);
    }

    fn setup_audio_stream(&mut self) -> anyhow::Result<Stream> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("no default audio output device"))?;

        let supported_config = device.default_output_config()?;
        let sample_format = supported_config.sample_format();
        let config: StreamConfig = supported_config.into();
        let channels = config.channels as usize;
        let sample_rate = config.sample_rate;
        if sample_rate == 0 {
            return Err(anyhow!("output sample rate is 0"));
        }
        let step = f64::from(APU_SAMPLE_RATE) / f64::from(sample_rate);
        self.output_sample_rate = Some(sample_rate);

        match sample_format {
            SampleFormat::F32 => self.build_stream::<f32>(&device, config, channels, step),
            SampleFormat::I16 => self.build_stream::<i16>(&device, config, channels, step),
            SampleFormat::U16 => self.build_stream::<u16>(&device, config, channels, step),
            other => Err(anyhow!("unsupported audio sample format: {other:?}")),
        }
    }

    pub fn samples_needed_to_maintain_buffer(&self) -> usize {
        let buffer_size = self.buffer_queue.lock().unwrap().len();
        TARGET_BUFFER_SIZE.saturating_sub(buffer_size)
    }

    fn build_stream<T: SampleConverter>(
        &self,
        device: &cpal::Device,
        config: StreamConfig,
        channels: usize,
        step: f64,
    ) -> anyhow::Result<Stream> {
        let buffer_queue = self.buffer_queue.clone();
        Ok(device.build_output_stream(
            config,
            move |data: &mut [T::Output], _: &cpal::OutputCallbackInfo| {
                if let Ok(mut queue) = buffer_queue.lock() {
                    for frame in data.chunks_mut(channels) {
                        let sample = queue
                            .next_output_sample(step)
                            .map(T::convert)
                            .unwrap_or_else(T::silence);
                        for channel_sample in frame.iter_mut() {
                            *channel_sample = sample;
                        }
                    }
                }
            },
            |err| error!("Error in audio stream: {err}"),
            None,
        )?)
    }

    pub fn update(&mut self, emulator: &mut System) {
        if let Ok(mut queue) = self.buffer_queue.lock() {
            let mut buffer = queue.get_recycled_buffer();
            emulator.swap_audio_buffer(&mut buffer);
            if self.stream.is_some() {
                queue.push_buffer(buffer);
            } else {
                buffer.clear();
                queue.recycle_buffer(buffer);
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(stream) = self.stream.take() {
            info!("Stopping audio output");
            drop(stream);
        }
    }
}

/// A queue of audio sample buffers with a cursor tracking the current playback position
/// and a recycling pool to reduce allocations
#[derive(Default)]
struct AudioBufferQueue {
    buffers: VecDeque<AudioBuffer>,
    cursor: usize,
    recycled_buffers: Vec<AudioBuffer>,
    current: Option<i16>,
    phase: f64,
}

impl AudioBufferQueue {
    fn push_buffer(&mut self, buffer: AudioBuffer) {
        if !buffer.is_empty() {
            self.buffers.push_back(buffer);
        }
    }

    fn len(&self) -> usize {
        self.buffers
            .iter()
            .map(|buffer| buffer.len())
            .sum::<usize>()
            .saturating_sub(self.cursor)
    }

    fn peek_sample(&self) -> Option<i16> {
        let mut buffers = self.buffers.iter();
        let first = buffers.next()?;
        if self.cursor < first.len() {
            return Some(first[self.cursor]);
        }
        let second = buffers.next()?;
        if second.is_empty() {
            None
        } else {
            Some(second[0])
        }
    }

    fn next_sample(&mut self) -> Option<i16> {
        let buffer = self.buffers.front()?;
        if self.cursor >= buffer.len() {
            if let Some(mut consumed_buffer) = self.buffers.pop_front() {
                consumed_buffer.clear();
                self.recycle_buffer(consumed_buffer);
            }
            self.cursor = 0;
            return self.next_sample();
        }
        let sample = buffer[self.cursor];
        self.cursor += 1;
        Some(sample)
    }

    /// Linearly interpolate 32 kHz APU samples to the output rate.
    /// `step` is `APU_SAMPLE_RATE / output_sample_rate` in APU-sample units per output frame.
    /// On underrun returns `None` (silence) and does not advance `phase`.
    fn next_output_sample(&mut self, step: f64) -> Option<i16> {
        if self.current.is_none() {
            self.current = Some(self.next_sample()?);
            self.phase = 0.0;
        }
        let current = self.current?;

        let output = if self.phase <= 0.0 {
            current
        } else {
            lerp_i16(current, self.peek_sample()?, self.phase)
        };

        let new_phase = self.phase + step;
        let consume = new_phase.floor() as usize;
        if consume > self.len() {
            if self.len() == 0 {
                self.current = None;
                self.phase = 0.0;
            }
            return Some(output);
        }

        self.phase = new_phase;
        for _ in 0..consume {
            self.current = self.next_sample();
            self.phase -= 1.0;
        }
        if self.phase < 0.0 {
            self.phase = 0.0;
        }
        Some(output)
    }

    /// Add a buffer to the recycling pool for reuse
    /// Keeps a limited number of buffers to avoid unbounded memory growth
    fn recycle_buffer(&mut self, buffer: AudioBuffer) {
        const MAX_RECYCLED_BUFFERS: usize = 8;
        if self.recycled_buffers.len() < MAX_RECYCLED_BUFFERS {
            self.recycled_buffers.push(buffer);
        }
    }

    /// Get a recycled buffer if available, otherwise create a new one
    fn get_recycled_buffer(&mut self) -> AudioBuffer {
        self.recycled_buffers.pop().unwrap_or_default()
    }
}

fn lerp_i16(a: i16, b: i16, t: f64) -> i16 {
    let mixed = f64::from(a).mul_add(1.0 - t, f64::from(b) * t);
    mixed.round() as i16
}

/// Handles conversion between different sample formats
trait SampleConverter {
    type Output: SizedSample;
    fn convert(input: i16) -> Self::Output;
    fn silence() -> Self::Output;
}

impl SampleConverter for f32 {
    type Output = f32;
    fn convert(input: i16) -> Self::Output {
        input as f32 / 32768.0
    }
    fn silence() -> Self::Output {
        0.0
    }
}

impl SampleConverter for i16 {
    type Output = i16;
    fn convert(input: i16) -> Self::Output {
        input
    }
    fn silence() -> Self::Output {
        0
    }
}

impl SampleConverter for u16 {
    type Output = u16;
    fn convert(input: i16) -> Self::Output {
        ((input as i32 + 32768) as u32).min(65535) as u16
    }
    fn silence() -> Self::Output {
        32768
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn queue_from_samples(samples: &[i16]) -> AudioBufferQueue {
        let mut queue = AudioBufferQueue::default();
        let mut buffer = AudioBuffer::new();
        for &sample in samples {
            buffer.push_sample(sample);
        }
        queue.push_buffer(buffer);
        queue
    }

    #[test]
    fn resample_identity_at_ratio_one() {
        let mut queue = queue_from_samples(&[10, 20, 30, 40]);
        assert_eq!(queue.next_output_sample(1.0), Some(10));
        assert_eq!(queue.next_output_sample(1.0), Some(20));
        assert_eq!(queue.next_output_sample(1.0), Some(30));
        assert_eq!(queue.next_output_sample(1.0), Some(40));
        assert_eq!(queue.next_output_sample(1.0), None);
    }

    #[test]
    fn resample_constant_stays_constant() {
        let mut queue = queue_from_samples(&[1000; 16]);
        let step = 32_000.0 / 48_000.0;
        for _ in 0..20 {
            assert_eq!(queue.next_output_sample(step), Some(1000));
        }
    }

    #[test]
    fn resample_two_thirds_midpoint() {
        let mut queue = queue_from_samples(&[0, 30_000]);
        let step = 2.0 / 3.0;
        assert_eq!(queue.next_output_sample(step), Some(0));
        let mid = queue.next_output_sample(step).unwrap();
        assert!(
            (i32::from(mid) - 20_000).abs() <= 1,
            "expected ~20000, got {mid}"
        );
        assert_eq!(queue.next_output_sample(step), None);
        assert_eq!(queue.next_output_sample(step), None);
    }
}
