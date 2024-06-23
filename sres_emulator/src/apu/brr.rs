//! SNES BRR sample decoding

use core::fmt;
use std::collections::HashMap;

use anyhow::bail;
use bilge::prelude::*;
use intbits::Bits;

pub struct BRRDecoder {
    cache: HashMap<u8, Vec<i16>>,
}

impl BRRDecoder {
    pub fn decode<'a>(&'a mut self, addr: u8, ram: &[u8]) -> &'a Vec<i16> {
        if !self.cache.contains_key(&addr) {
            self.cache
                .insert(addr, decode_brr(&ram[(addr as usize)..]).unwrap());
        }
        &self.cache[&addr]
    }
}

/// Decodes BRR encoded audio into signed 16-bit PCM samples.
///
/// The decoding will continue until a BRR block with end=true is found. If the data ends before
/// that, an error is returned.
pub fn decode_brr(data: &[u8]) -> anyhow::Result<Vec<i16>> {
    let mut samples: Vec<i16> = Vec::new();
    for chunk in data.chunks_exact(9) {
        let history = if samples.len() > 2 {
            [samples[samples.len() - 1], samples[samples.len() - 2]]
        } else {
            [0, 0]
        };
        let block = BrrBlock::from_bytes(chunk.try_into().unwrap());
        samples.extend(block.samples(history));
        if block.header.end() {
            return Ok(samples);
        }
    }
    bail!("End of data before final BRR block");
}

/// The BRR format is composed of blocks of 9 bytes with a header and 16 4-bit samples.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct BrrBlock {
    header: BrrBlockHeader,
    sample_pairs: [BrrSamplePair; 8],
}

impl BrrBlock {
    /// Decode a BRR block from a 9 byte slice
    pub fn from_bytes(bytes: &[u8; 9]) -> Self {
        Self {
            header: BrrBlockHeader::from(bytes[0]),
            sample_pairs: [
                BrrSamplePair::from(bytes[1]),
                BrrSamplePair::from(bytes[2]),
                BrrSamplePair::from(bytes[3]),
                BrrSamplePair::from(bytes[4]),
                BrrSamplePair::from(bytes[5]),
                BrrSamplePair::from(bytes[6]),
                BrrSamplePair::from(bytes[7]),
                BrrSamplePair::from(bytes[8]),
            ],
        }
    }

    /// Unfiltered samples of this block, shifted by header.left_shift
    fn raw_samples(&self) -> impl Iterator<Item = i16> + '_ {
        self.sample_pairs.iter().flat_map(|pair| {
            let left_shift: u32 = u32::from(self.header.left_shift());
            [
                i4_to_i16(pair.a())
                    .overflowing_shl(left_shift)
                    .0
                    .overflowing_shr(1)
                    .0,
                i4_to_i16(pair.b())
                    .overflowing_shl(left_shift)
                    .0
                    .overflowing_shr(1)
                    .0,
            ]
        })
    }

    /// Decoded samples for this block
    ///
    /// Requires a history of the last two samples as they are used in the decoding filter
    pub fn samples(&self, mut history: [i16; 2]) -> impl Iterator<Item = i16> + '_ {
        let coeff = match self.header.filter().value() {
            0 => [0.0, 0.0],
            1 => [15.0 / 16.0, 0.0],
            2 => [61.0 / 32.0, -15.0 / 16.0],
            3 => [115.0 / 64.0, -13.0 / 16.0],
            _ => unreachable!(),
        };
        self.raw_samples().map(move |sample| {
            let output =
                (sample as f64) + (history[0] as f64) * coeff[0] + (history[1] as f64) * coeff[1];
            history[1] = history[0];
            history[0] = output as i16;
            history[0]
        })
    }
}

/// Converts a 4-bit signed integer to a 16-bit signed integer
fn i4_to_i16(sample: u4) -> i16 {
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

#[bitsize(8)]
#[derive(Clone, Copy, Default, FromBits, PartialEq)]
struct BrrSamplePair {
    a: u4,
    b: u4,
}

impl fmt::Debug for BrrSamplePair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", i4_to_i16(self.a()), i4_to_i16(self.b()))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use hound;
    use itertools::Itertools;

    use super::*;

    #[test]
    pub fn test_u4_to_i16() {
        assert_eq!(i4_to_i16(u4::new(0b0000)), 0);
        assert_eq!(i4_to_i16(u4::new(0b0111)), 7);
        assert_eq!(i4_to_i16(u4::new(0b1000)), -8);
        assert_eq!(i4_to_i16(u4::new(0b1111)), -1);
    }

    /// Sample pairs of a single impulse
    fn impulse() -> [BrrSamplePair; 8] {
        [
            BrrSamplePair::new(u4::new(1), u4::new(0)),
            BrrSamplePair::new(u4::new(0), u4::new(0)),
            BrrSamplePair::new(u4::new(0), u4::new(0)),
            BrrSamplePair::new(u4::new(0), u4::new(0)),
            BrrSamplePair::new(u4::new(0), u4::new(0)),
            BrrSamplePair::new(u4::new(0), u4::new(0)),
            BrrSamplePair::new(u4::new(0), u4::new(0)),
            BrrSamplePair::new(u4::new(0), u4::new(0)),
        ]
    }

    #[test]
    pub fn test_decode_filter0() {
        let block = BrrBlock {
            header: BrrBlockHeader::new(true, false, u2::new(0), u4::new(6)),
            sample_pairs: impulse(),
        };
        assert_eq!(
            block.samples([0, 0]).collect_vec(),
            vec![32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    }

    #[test]
    pub fn test_decode_filter1() {
        let block = BrrBlock {
            header: BrrBlockHeader::new(true, false, u2::new(1), u4::new(6)),
            sample_pairs: impulse(),
        };
        assert_eq!(
            block.samples([0, 0]).collect_vec(),
            vec![32, 30, 28, 26, 24, 22, 20, 18, 16, 15, 14, 13, 12, 11, 10, 9]
        )
    }

    #[test]
    pub fn test_decode_filter2() {
        let block = BrrBlock {
            header: BrrBlockHeader::new(true, false, u2::new(2), u4::new(6)),
            sample_pairs: impulse(),
        };
        assert_eq!(
            block.samples([0, 0]).collect_vec(),
            vec![32, 61, 86, 106, 121, 131, 136, 136, 131, 122, 109, 93, 75, 55, 34, 13]
        )
    }

    #[test]
    pub fn test_decode_filter3() {
        let block = BrrBlock {
            header: BrrBlockHeader::new(true, false, u2::new(3), u4::new(6)),
            sample_pairs: impulse(),
        };
        assert_eq!(
            block.samples([0, 0]).collect_vec(),
            vec![32, 57, 76, 90, 99, 104, 106, 105, 102, 97, 91, 84, 77, 70, 63, 56]
        )
    }

    #[test]
    pub fn test_play_brr_sample() {
        test_brr_decode("play_brr_sample")
    }

    pub fn test_brr_decode(filename: &str) {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let brr_path = root_dir.join(format!("src/apu/brr/{filename}.brr"));
        let wav_path = root_dir.join(format!("src/apu/brr/{filename}.wav"));
        let brr_data = std::fs::read(brr_path).unwrap();

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 32_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let decoded = decode_brr(&brr_data).unwrap();
        let mut writer = hound::WavWriter::create(wav_path, spec).unwrap();
        for sample in decoded {
            writer.write_sample(sample).unwrap();
        }
    }
}
