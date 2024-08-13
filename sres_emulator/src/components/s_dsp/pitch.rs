#![allow(dead_code)]

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::{self};
use std::ops::Add;
use std::ops::Sub;

use intbits::Bits;

use crate::common::uint::U16Ext;
use crate::common::uint::U8Ext;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PitchGenerator {
    buffer: [i16; 12],
    counter: PitchCounter,
}

impl PitchGenerator {
    pub fn new(input: &mut impl Iterator<Item = i16>) -> Self {
        Self {
            buffer: [
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
                input.next().unwrap_or_default(),
            ],
            counter: PitchCounter::default(),
        }
    }

    pub fn generate_sample(&mut self, pitch: u16, input: &mut impl Iterator<Item = i16>) -> i16 {
        let fractional = self.counter.fractional();
        let coefficients = [
            GAUSSIAN_TABLE[0x0FF - fractional],
            GAUSSIAN_TABLE[0x1FF - fractional],
            GAUSSIAN_TABLE[0x100 + fractional],
            GAUSSIAN_TABLE[fractional],
        ];
        let samples = (0..4).map(|i| {
            let sample_index = (self.counter + 0x1000 * i).index();
            ((self.buffer[sample_index] as i32) * coefficients[i as usize]) >> 11
        });
        let sample = samples.sum::<i32>() as i16;

        let (new_counter, cross) = self.counter.add_detect_4byte_cross(pitch);
        if cross {
            let index = self.counter.index() / 4 * 4;
            self.buffer[index] = input.next().unwrap_or_default();
            self.buffer[index + 1] = input.next().unwrap_or_default();
            self.buffer[index + 2] = input.next().unwrap_or_default();
            self.buffer[index + 3] = input.next().unwrap_or_default();
        }
        self.counter = new_counter;
        sample
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct PitchCounter(u16);

impl PitchCounter {
    pub fn add_detect_4byte_cross(&self, count: u16) -> (PitchCounter, bool) {
        let new_counter = *self + count;
        let cross = self.0 / 0x4000 != new_counter.0 / 0x4000;
        (new_counter, cross)
    }

    pub fn index(&self) -> usize {
        self.0.high_byte().high_nibble() as usize
    }

    pub fn fractional(&self) -> usize {
        self.0.bits(4..12) as usize
    }
}

impl Display for PitchCounter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:X}.{:02X}0", self.index(), self.fractional())
    }
}

impl Add<u16> for PitchCounter {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        let mut new_counter = self.0 + rhs;
        if new_counter >= 0xC000 {
            new_counter -= 0xC000;
        }
        Self(new_counter)
    }
}

impl Sub<u16> for PitchCounter {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self::Output {
        let mut new_counter = self.0 - rhs;
        if new_counter >= 0xC000 {
            new_counter += 0xC000;
        }
        Self(new_counter)
    }
}

/// Gaussian lookup table from https://sneslab.net/wiki/S-DSP/Gaussian_Filter
const GAUSSIAN_TABLE: &[i32; 512] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2,
    2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10,
    10, 11, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15, 15, 16, 16, 17, 17, 18, 19, 19, 20, 20, 21, 21,
    22, 23, 23, 24, 24, 25, 26, 27, 27, 28, 29, 29, 30, 31, 32, 32, 33, 34, 35, 36, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 58, 59, 60, 61, 62, 64, 65,
    66, 67, 69, 70, 71, 73, 74, 76, 77, 78, 80, 81, 83, 84, 86, 87, 89, 90, 92, 94, 95, 97, 99,
    100, 102, 104, 106, 107, 109, 111, 113, 115, 117, 118, 120, 122, 124, 126, 128, 130, 132, 134,
    137, 139, 141, 143, 145, 147, 150, 152, 154, 156, 159, 161, 163, 166, 168, 171, 173, 175, 178,
    180, 183, 186, 188, 191, 193, 196, 199, 201, 204, 207, 210, 212, 215, 218, 221, 224, 227, 230,
    233, 236, 239, 242, 245, 248, 251, 254, 257, 260, 263, 267, 270, 273, 276, 280, 283, 286, 290,
    293, 297, 300, 304, 307, 311, 314, 318, 321, 325, 328, 332, 336, 339, 343, 347, 351, 354, 358,
    362, 366, 370, 374, 378, 381, 385, 389, 393, 397, 401, 405, 410, 414, 418, 422, 426, 430, 434,
    439, 443, 447, 451, 456, 460, 464, 469, 473, 477, 482, 486, 491, 495, 499, 504, 508, 513, 517,
    522, 527, 531, 536, 540, 545, 550, 554, 559, 563, 568, 573, 577, 582, 587, 592, 596, 601, 606,
    611, 615, 620, 625, 630, 635, 640, 644, 649, 654, 659, 664, 669, 674, 678, 683, 688, 693, 698,
    703, 708, 713, 718, 723, 728, 732, 737, 742, 747, 752, 757, 762, 767, 772, 777, 782, 787, 792,
    797, 802, 806, 811, 816, 821, 826, 831, 836, 841, 846, 851, 855, 860, 865, 870, 875, 880, 884,
    889, 894, 899, 904, 908, 913, 918, 923, 927, 932, 937, 941, 946, 951, 955, 960, 965, 969, 974,
    978, 983, 988, 992, 997, 1001, 1005, 1010, 1014, 1019, 1023, 1027, 1032, 1036, 1040, 1045,
    1049, 1053, 1057, 1061, 1066, 1070, 1074, 1078, 1082, 1086, 1090, 1094, 1098, 1102, 1106, 1109,
    1113, 1117, 1121, 1125, 1128, 1132, 1136, 1139, 1143, 1146, 1150, 1153, 1157, 1160, 1164, 1167,
    1170, 1174, 1177, 1180, 1183, 1186, 1190, 1193, 1196, 1199, 1202, 1205, 1207, 1210, 1213, 1216,
    1219, 1221, 1224, 1227, 1229, 1232, 1234, 1237, 1239, 1241, 1244, 1246, 1248, 1251, 1253, 1255,
    1257, 1259, 1261, 1263, 1265, 1267, 1269, 1270, 1272, 1274, 1275, 1277, 1279, 1280, 1282, 1283,
    1284, 1286, 1287, 1288, 1290, 1291, 1292, 1293, 1294, 1295, 1296, 1297, 1297, 1298, 1299, 1300,
    1300, 1301, 1302, 1302, 1303, 1303, 1303, 1304, 1304, 1304, 1304, 1304, 1305, 1305,
];

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use rasciigraph::Config;

    use super::PitchCounter;
    use super::PitchGenerator;

    #[test]
    pub fn test_4byte_cross() {
        assert_eq!(
            PitchCounter(0x0000).add_detect_4byte_cross(0x3FFF),
            (PitchCounter(0x3FFF), false)
        );
        assert_eq!(
            PitchCounter(0x3FFF).add_detect_4byte_cross(0x0001),
            (PitchCounter(0x4000), true)
        );
        assert_eq!(
            PitchCounter(0x7FFF).add_detect_4byte_cross(0x0001),
            (PitchCounter(0x8000), true)
        );
        assert_eq!(
            PitchCounter(0xBFFF).add_detect_4byte_cross(0x0001),
            (PitchCounter(0x0000), true)
        );
    }

    #[test]
    pub fn test_pitch_counter_increment() {
        let mut input = [0_i16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
            .iter()
            .copied();
        // Loads the first 12 samples. 4 remaining.
        let mut generator = PitchGenerator::new(&mut input);
        assert_eq!(&generator.buffer, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(input.len(), 4);

        // Generate a sample at 0x3FFF, should not load more samples.
        generator.generate_sample(0x3FFF, &mut input);
        assert_eq!(&generator.buffer, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(input.len(), 4);

        // Generate a sample at 0x4000, this should fill the buffer with 4 more samples.
        generator.generate_sample(0x0001, &mut input);
        assert_eq!(
            &generator.buffer,
            &[12, 13, 14, 15, 4, 5, 6, 7, 8, 9, 10, 11]
        );
        assert_eq!(input.len(), 0);
    }

    const RECT: [i16; 12] = [0, 0, 0, 0, 100, 100, 100, 100, 0, 0, 0, 0];

    #[test]
    pub fn test_normal_pitch() {
        test_interpolation(
            0x1000,
            RECT.to_vec(),
            vec![0, 0, 18, 81, 99, 99, 81, 18, 0, 0, 0, 0],
        )
    }

    #[test]
    pub fn test_half_pitch() {
        test_interpolation(
            0x0800,
            RECT.to_vec(),
            vec![
                0, 0, 0, 2, 18, 49, 81, 96, 99, 98, 99, 96, 81, 49, 18, 2, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
        )
    }

    #[test]
    pub fn test_double_pitch() {
        test_interpolation(0x2000, RECT.to_vec(), vec![0, 18, 99, 81, 0, 0])
    }

    fn test_interpolation(pitch: u16, input_vec: Vec<i16>, expected: Vec<i16>) {
        let mut input = input_vec.iter().copied();
        let mut generator = PitchGenerator::new(&mut input);
        let actual = (0..(expected.len()))
            .map(|_| generator.generate_sample(pitch, &mut input))
            .collect_vec();

        if expected != actual {
            println!();
            println!("Input: {:?}", input_vec);
            debug_plot(input_vec);
            println!();
            println!("Expected: {:?}", expected);
            debug_plot(expected);
            println!();
            println!("Actual: {:?}", actual);
            debug_plot(actual);
            panic!("Result does not match!")
        }
    }

    fn debug_plot(data: impl IntoIterator<Item = i16>) {
        println!(
            "{}",
            rasciigraph::plot(
                data.into_iter().map(|v| (v as f64) / 1024.0).collect_vec(),
                Config::default()
                    .with_offset(4)
                    .with_height(10)
                    .with_width(120),
            )
        );
    }
}
