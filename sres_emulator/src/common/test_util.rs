#![cfg(test)]

use std::path::Path;

use hound::WavReader;
use hound::WavWriter;

pub fn compare_wav_against_golden(data: &[i16], path_prefix: &Path) {
    let golden_path = path_prefix.with_extension("wav");
    if golden_path.exists() {
        let golden = read_snes_wav(&golden_path);
        if data != golden {
            let actual_path = path_prefix.with_extension("actual.wav");
            write_snes_wav(data, &actual_path);
            panic!("Actual result does not match golden. See {:?}", actual_path);
        }
    } else {
        write_snes_wav(data, &golden_path);
    }
}

pub fn write_snes_wav(data: &[i16], filename: &Path) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 32_000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(filename, spec).unwrap();
    let mut i16_writer = writer.get_i16_writer(data.len() as u32);
    for sample in data {
        i16_writer.write_sample(*sample);
    }
    i16_writer.flush().unwrap()
}

pub fn read_snes_wav(filename: &Path) -> Vec<i16> {
    let mut reader = WavReader::open(filename).unwrap();
    reader.samples().map(|s| s.unwrap()).collect()
}
