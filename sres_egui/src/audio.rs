use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;
use cpal::traits::StreamTrait;
use cpal::BuildStreamError;
use cpal::SampleFormat;
use cpal::SizedSample;
use cpal::Stream;
use cpal::StreamConfig;
use log::error;
use log::info;
use sres_emulator::System;

/// Audio output handler that manages playback of SNES APU audio samples
pub struct AudioOutput {
    stream: Option<Stream>,
    buffer_queue: Arc<Mutex<AudioBufferQueue>>,
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
        }
    }

    pub fn start(&mut self) {
        if self.stream.is_some() {
            return;
        }

        info!("Starting audio output");
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
        self.stream = Some(stream);
    }

    fn setup_audio_stream(&self) -> Result<Stream, BuildStreamError> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(BuildStreamError::DeviceNotAvailable)?;

        let supported_config = device
            .default_output_config()
            .map_err(|_| BuildStreamError::StreamConfigNotSupported)?;

        let config = StreamConfig {
            channels: 2,                          // Stereo output
            sample_rate: cpal::SampleRate(32000), // SNES APU sample rate
            buffer_size: cpal::BufferSize::Default,
        };

        match supported_config.sample_format() {
            SampleFormat::F32 => self.build_stream::<f32>(&device, &config),
            SampleFormat::I16 => self.build_stream::<i16>(&device, &config),
            SampleFormat::U16 => self.build_stream::<u16>(&device, &config),
            _ => Err(BuildStreamError::StreamConfigNotSupported),
        }
    }

    fn build_stream<T: SampleConverter>(
        &self,
        device: &cpal::Device,
        config: &StreamConfig,
    ) -> Result<Stream, BuildStreamError> {
        let buffer_queue = self.buffer_queue.clone();
        device.build_output_stream(
            config,
            move |data: &mut [T::Output], _: &cpal::OutputCallbackInfo| {
                if let Ok(mut queue) = buffer_queue.lock() {
                    // Process two samples at a time for stereo
                    for chunk in data.chunks_exact_mut(2) {
                        let sample = queue
                            .next_sample()
                            .map(T::convert)
                            .unwrap_or_else(T::silence);
                        chunk[0] = sample; // Left channel
                        chunk[1] = sample; // Right channel
                    }
                }
            },
            |err| error!("Error in audio stream: {err}"),
            None,
        )
    }

    pub fn update(&mut self, emulator: &mut System) {
        if self.stream.is_none() {
            return;
        }

        if let Ok(mut queue) = self.buffer_queue.lock() {
            queue.push_buffer(emulator.take_audio_samples());
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
#[derive(Default)]
struct AudioBufferQueue {
    buffers: VecDeque<Vec<i16>>,
    cursor: usize,
}

impl AudioBufferQueue {
    fn push_buffer(&mut self, buffer: Vec<i16>) {
        if !buffer.is_empty() {
            self.buffers.push_back(buffer);
        }
    }

    fn next_sample(&mut self) -> Option<i16> {
        let buffer = self.buffers.front()?;
        if self.cursor >= buffer.len() {
            self.buffers.pop_front();
            self.cursor = 0;
            return self.next_sample();
        }
        let sample = buffer[self.cursor];
        self.cursor += 1;
        Some(sample)
    }
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
