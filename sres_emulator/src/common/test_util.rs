//! 32 kHz mono i16 WAV helpers. `compare_wav_against_golden` writes the golden if missing.
use std::path::Path;

const SAMPLE_RATE: u32 = 32_000;
const CHANNELS: u16 = 1;
const BITS_PER_SAMPLE: u16 = 16;
const PCM_FORMAT: u16 = 1;

pub fn compare_wav_against_golden(data: &[i16], path_prefix: &Path) {
    let golden_path = path_prefix.with_extension("wav");
    if golden_path.exists() {
        let golden = read_snes_wav(&golden_path);
        if data != golden {
            let actual_path = path_prefix.with_extension("actual.wav");
            write_snes_wav(data, &actual_path);
            panic!("Actual result does not match golden. See {actual_path:?}");
        }
    } else {
        write_snes_wav(data, &golden_path);
    }
}

pub fn write_snes_wav(data: &[i16], filename: &Path) {
    std::fs::write(filename, encode_snes_wav(data)).unwrap();
}

pub fn read_snes_wav(filename: &Path) -> Vec<i16> {
    let bytes = std::fs::read(filename).unwrap();
    decode_snes_wav(&bytes)
}

fn encode_snes_wav(data: &[i16]) -> Vec<u8> {
    let block_align = CHANNELS * (BITS_PER_SAMPLE / 8);
    let byte_rate = SAMPLE_RATE * u32::from(block_align);
    let data_bytes = u32::try_from(data.len() * 2).expect("WAV data exceeds 4 GiB");
    let riff_size = 36_u32
        .checked_add(data_bytes)
        .expect("WAV RIFF size exceeds 4 GiB");

    let mut out = Vec::with_capacity(44 + data_bytes as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&riff_size.to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16_u32.to_le_bytes());
    out.extend_from_slice(&PCM_FORMAT.to_le_bytes());
    out.extend_from_slice(&CHANNELS.to_le_bytes());
    out.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&BITS_PER_SAMPLE.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_bytes.to_le_bytes());
    for sample in data {
        out.extend_from_slice(&sample.to_le_bytes());
    }
    out
}

fn decode_snes_wav(bytes: &[u8]) -> Vec<i16> {
    assert!(bytes.len() >= 12, "WAV file too short");
    assert_eq!(&bytes[0..4], b"RIFF", "Missing RIFF header");
    assert_eq!(&bytes[8..12], b"WAVE", "Missing WAVE header");

    let mut found_fmt = false;
    let mut samples = None;
    let mut offset: usize = 12;

    while offset.saturating_add(8) <= bytes.len() {
        let chunk_id = &bytes[offset..offset + 4];
        let chunk_len = u32::from_le_bytes([
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
        ]) as usize;
        let payload_start = offset + 8;
        let payload_end = payload_start
            .checked_add(chunk_len)
            .expect("WAV chunk length overflow");
        assert!(payload_end <= bytes.len(), "WAV chunk truncated");
        let payload = &bytes[payload_start..payload_end];

        if chunk_id == b"fmt " {
            assert!(payload.len() >= 16, "WAV fmt chunk too short");
            let audio_format = u16::from_le_bytes([payload[0], payload[1]]);
            let channels = u16::from_le_bytes([payload[2], payload[3]]);
            let sample_rate = u32::from_le_bytes([payload[4], payload[5], payload[6], payload[7]]);
            let bits_per_sample = u16::from_le_bytes([payload[14], payload[15]]);
            assert_eq!(
                (audio_format, channels, sample_rate, bits_per_sample),
                (PCM_FORMAT, CHANNELS, SAMPLE_RATE, BITS_PER_SAMPLE),
                "Unexpected WAV format"
            );
            found_fmt = true;
        } else if chunk_id == b"data" {
            let (chunks, remainder) = payload.as_chunks::<2>();
            assert!(remainder.is_empty(), "WAV data chunk has odd byte count");
            samples = Some(chunks.iter().copied().map(i16::from_le_bytes).collect());
        }

        offset = payload_end + (chunk_len & 1);
    }

    assert!(found_fmt, "WAV missing fmt chunk");
    samples.expect("WAV missing data chunk")
}

/// Formats a line-by-line diff between `actual` and `expected` for test failure output.
pub fn format_line_diff(actual: &str, expected: &str) -> String {
    let mut out = String::new();
    let actual_lines: Vec<&str> = actual.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();
    for i in 0..actual_lines.len().max(expected_lines.len()) {
        let a = actual_lines.get(i).copied();
        let e = expected_lines.get(i).copied();
        if a == e {
            if let Some(line) = a {
                out.push_str("  ");
                out.push_str(line);
                out.push('\n');
            }
        } else {
            if let Some(line) = e {
                out.push_str("- ");
                out.push_str(line);
                out.push('\n');
            }
            if let Some(line) = a {
                out.push_str("+ ");
                out.push_str(line);
                out.push('\n');
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wav_roundtrip_samples() {
        let samples = vec![i16::MIN, -12345, -1, 0, 1, 12345, i16::MAX];
        let encoded = encode_snes_wav(&samples);
        assert_eq!(encoded.len(), 44 + samples.len() * 2);
        assert_eq!(&encoded[0..4], b"RIFF");
        assert_eq!(&encoded[8..12], b"WAVE");
        assert_eq!(decode_snes_wav(&encoded), samples);
    }

    #[test]
    fn wav_file_roundtrip_and_odd_chunk_padding() {
        let path = std::env::temp_dir().join(format!("sres_wav_test_{}.wav", std::process::id()));
        let samples = vec![0, 100, -100, i16::MAX, i16::MIN];
        write_snes_wav(&samples, &path);
        let read_back = read_snes_wav(&path);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(read_back, samples);

        // Insert an odd-length metadata chunk before `data` to verify RIFF word alignment.
        let raw = encode_snes_wav(&samples);
        let data_pos = 36;
        let mut with_extra = Vec::new();
        with_extra.extend_from_slice(&raw[..data_pos]);
        with_extra.extend_from_slice(b"JUNK");
        with_extra.extend_from_slice(&3_u32.to_le_bytes());
        with_extra.extend_from_slice(b"abc\0");
        with_extra.extend_from_slice(&raw[data_pos..]);
        let riff_size = u32::try_from(with_extra.len() - 8).unwrap();
        with_extra[4..8].copy_from_slice(&riff_size.to_le_bytes());
        assert_eq!(decode_snes_wav(&with_extra), samples);
    }
}
