//! SNES BRR sample decoding
#![allow(dead_code)]

use std::collections::VecDeque;

use bilge::prelude::*;
use intbits::Bits;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BrrDecoder {
    buffer: [i16; 2],
    end: bool,
    memory_index: usize,
    current_block: VecDeque<i16>,
    loop_addr: Option<usize>,
    last_block_header: Option<BrrBlockHeader>,
}

impl BrrDecoder {
    pub fn new(memory_index: usize) -> BrrDecoder {
        BrrDecoder {
            buffer: [0, 0],
            end: false,
            memory_index,
            current_block: VecDeque::with_capacity(16),
            loop_addr: None,
            last_block_header: None,
        }
    }

    pub fn reset(&mut self, memory_index: usize) {
        *self = BrrDecoder::new(memory_index);
    }

    pub fn set_loop_addr(&mut self, loop_addr: usize) {
        self.loop_addr = Some(loop_addr);
    }

    pub fn iter<'a>(&'a mut self, memory: &'a [u8]) -> BrrIterator<'a> {
        BrrIterator {
            decoder: self,
            memory,
        }
    }

    pub fn next_sample(&mut self, memory: &[u8]) -> Option<i16> {
        if self.current_block.is_empty() {
            if self.end {
                // Check if we should loop (only if last block had loop flag set)
                if let Some(header) = self.last_block_header {
                    if let (true, Some(loop_addr)) = (header.loop_flag(), self.loop_addr) {
                        // Reset decoder state for looping
                        self.buffer = [0, 0];
                        self.end = false;
                        self.memory_index = loop_addr;
                        self.current_block.clear();
                        self.last_block_header = None;
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            let new_index = self.memory_index + 9;
            let block =
                BrrBlock::from_bytes(memory[self.memory_index..new_index].try_into().unwrap());

            // Store header if this is an end block
            if block.header.end() {
                self.last_block_header = Some(block.header);
            }

            let samples = self.decode(&block);
            self.current_block.extend(samples.iter());
            self.memory_index = new_index;
        }
        self.current_block.pop_front()
    }

    fn decode_bytes(&mut self, raw_block: &[u8; 9]) -> [i16; 16] {
        self.decode(&BrrBlock::from_bytes(raw_block))
    }

    fn decode(&mut self, block: &BrrBlock) -> [i16; 16] {
        self.end = block.header.end();

        let left_shift = u32::from(block.header.left_shift());
        let coeff = match block.header.filter().value() {
            0 => [0.0, 0.0],
            1 => [15.0 / 16.0, 0.0],
            2 => [61.0 / 32.0, -15.0 / 16.0],
            3 => [115.0 / 64.0, -13.0 / 16.0],
            _ => unreachable!(),
        };

        block.raw_samples.map(|sample| {
            let shifted = i4_to_i16(sample)
                .overflowing_shl(left_shift)
                .0
                .overflowing_shr(1)
                .0;
            let output = (shifted as f64)
                + (self.buffer[0] as f64) * coeff[0]
                + (self.buffer[1] as f64) * coeff[1];
            self.buffer[1] = self.buffer[0];
            self.buffer[0] = output as i16;
            self.buffer[0]
        })
    }
}

pub struct BrrIterator<'a> {
    decoder: &'a mut BrrDecoder,
    memory: &'a [u8],
}

impl Iterator for BrrIterator<'_> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self.decoder.next_sample(self.memory)
    }
}

/// The BRR format is composed of blocks of 9 bytes with a header and 16 4-bit samples.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct BrrBlock {
    header: BrrBlockHeader,
    raw_samples: [u8; 16],
}

impl BrrBlock {
    /// Decode a BRR block from a 9 byte slice
    pub fn from_bytes(bytes: &[u8; 9]) -> Self {
        let mut raw_samples = [0; 16];
        for i in 0..16 {
            let odd = (i + 1) % 2;
            raw_samples[i] = bytes[1 + i / 2].bits((odd * 4)..((odd + 1) * 4));
        }
        Self {
            header: BrrBlockHeader::from(bytes[0]),
            raw_samples,
        }
    }
}

/// Converts a 4-bit signed integer to a 16-bit signed integer
fn i4_to_i16(sample: u8) -> i16 {
    let u16_sample = u16::from(sample);
    let value = u16_sample.bits(0..3);
    let sign = u16_sample.bit(3);
    if sign {
        -8 + value as i16
    } else {
        value as i16
    }
}

#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, Default, FromBits, PartialEq)]
struct BrrBlockHeader {
    end: bool,
    loop_flag: bool,
    filter: u2,
    left_shift: u4,
}

#[derive(Clone, Copy, Default, PartialEq)]
struct BrrSamplePair {
    a: u8,
    b: u8,
}

impl From<u8> for BrrSamplePair {
    fn from(value: u8) -> Self {
        Self {
            a: value.bits(0..3),
            b: value.bits(4..7),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::common::test_util::compare_wav_against_golden;

    #[test]
    pub fn test_u4_to_i16() {
        assert_eq!(i4_to_i16(0b0000), 0);
        assert_eq!(i4_to_i16(0b0111), 7);
        assert_eq!(i4_to_i16(0b1000), -8);
        assert_eq!(i4_to_i16(0b1111), -1);
    }

    /// Sample pairs of a single impulse
    fn impulse() -> [u8; 16] {
        let mut out = [0; 16];
        out[0] = 1;
        out
    }

    #[test]
    pub fn test_brr_block_from_bytes() {
        let bytes = [0x00, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        assert_eq!(
            BrrBlock::from_bytes(&bytes),
            BrrBlock {
                header: BrrBlockHeader::new(false, false, u2::new(0), u4::new(0)),
                raw_samples: [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                    0x0E, 0x0F, 0x00
                ]
            }
        )
    }

    #[test]
    pub fn test_decode_filter0() {
        let block = BrrBlock {
            header: BrrBlockHeader::new(true, false, u2::new(0), u4::new(6)),
            raw_samples: impulse(),
        };
        assert_eq!(
            BrrDecoder::default().decode(&block),
            [32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    }

    #[test]
    pub fn test_decode_filter1() {
        let block = BrrBlock {
            header: BrrBlockHeader::new(true, false, u2::new(1), u4::new(6)),
            raw_samples: impulse(),
        };
        assert_eq!(
            BrrDecoder::default().decode(&block),
            [32, 30, 28, 26, 24, 22, 20, 18, 16, 15, 14, 13, 12, 11, 10, 9]
        )
    }

    #[test]
    pub fn test_decode_filter2() {
        let block = BrrBlock {
            header: BrrBlockHeader::new(true, false, u2::new(2), u4::new(6)),
            raw_samples: impulse(),
        };
        assert_eq!(
            BrrDecoder::default().decode(&block),
            [32, 61, 86, 106, 121, 131, 136, 136, 131, 122, 109, 93, 75, 55, 34, 13]
        )
    }

    #[test]
    pub fn test_decode_filter3() {
        let block = BrrBlock {
            header: BrrBlockHeader::new(true, false, u2::new(3), u4::new(6)),
            raw_samples: impulse(),
        };
        assert_eq!(
            BrrDecoder::default().decode(&block),
            [32, 57, 76, 90, 99, 104, 106, 105, 102, 97, 91, 84, 77, 70, 63, 56]
        )
    }

    #[test]
    pub fn test_play_brr_sample() {
        test_brr_decode("play_brr_sample")
    }

    pub fn test_brr_decode(filename: &str) {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let prefix = root_dir.join(format!("src/components/s_dsp/brr/{filename}"));

        let brr_data = std::fs::read(prefix.with_extension("brr")).unwrap();
        let mut decoder = BrrDecoder::new(0);
        let decoded: Vec<i16> = decoder.iter(&brr_data).collect();

        compare_wav_against_golden(&decoded, &prefix);
    }
}
