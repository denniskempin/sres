//! SNES BRR sample decoding

use core::fmt;

use bilge::prelude::*;
use intbits::Bits;

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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct BrrBlock {
    header: BrrBlockHeader,
    samples: [BrrSamplePair; 8],
}

impl BrrBlock {
    fn from_bytes(bytes: &[u8; 9]) -> Self {
        Self {
            header: BrrBlockHeader::from(bytes[0]),
            samples: [
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

    fn decode(&self) -> impl Iterator<Item = i16> + '_ {
        self.samples.into_iter().flat_map(|pair| {
            let left_shift: u32 = u32::from(self.header.left_shift());
            [
                i4_to_i16(pair.a()).overflowing_shl(left_shift).0,
                i4_to_i16(pair.b()).overflowing_shl(left_shift).0,
            ]
        })
    }
}

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

#[cfg(test)]
mod tests {
    use hound;
    use std::path::PathBuf;

    use super::*;

    #[test]
    pub fn test_u4_to_i16() {
        assert_eq!(i4_to_i16(u4::new(0b0000)), 0);
        assert_eq!(i4_to_i16(u4::new(0b0111)), 7);
        assert_eq!(i4_to_i16(u4::new(0b1000)), -8);
        assert_eq!(i4_to_i16(u4::new(0b1111)), -1);
    }

    #[test]
    pub fn test_harp() {
        test_brr_decode("harp")
    }

    #[test]
    pub fn test_play_brr_sample() {
        test_brr_decode("play_brr_sample")
    }

    pub fn test_brr_decode(filename: &str) {
        let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let brr_path = root_dir.join(format!("src/spc700/brr/{filename}.brr"));
        let wav_path = root_dir.join(format!("src/spc700/brr/{filename}.wav"));
        let brr_data = std::fs::read(brr_path).unwrap();

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 32_000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(wav_path, spec).unwrap();
        for chunk in brr_data.chunks_exact(9) {
            let block = BrrBlock::from_bytes(chunk.try_into().unwrap());
            println!("{:?}", block);
            println!("{:?}", block.decode().collect::<Vec<_>>());
            for sample in block.decode() {
                writer.write_sample(sample).unwrap();
            }
        }
    }
}
