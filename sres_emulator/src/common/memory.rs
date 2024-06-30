//! Utilities for formatting blocks of memory.
use std::io::BufWriter;
use std::io::Write;

pub fn format_memory(memory: &[u8]) -> String {
    let mut writer = BufWriter::new(Vec::new());
    for chunks in memory.chunks(16) {
        for chunk in chunks {
            write!(&mut writer, "{:02X} ", *chunk).unwrap();
        }
        writeln!(&mut writer).unwrap();
    }

    let bytes = writer.into_inner().unwrap();
    String::from_utf8(bytes).unwrap()
}

pub fn format_memory_u16(memory: &[u16]) -> String {
    let mut writer = BufWriter::new(Vec::new());
    for chunks in memory.chunks(16) {
        for chunk in chunks {
            write!(&mut writer, "{:04X} ", *chunk).unwrap();
        }
        writeln!(&mut writer).unwrap();
    }

    let bytes = writer.into_inner().unwrap();
    String::from_utf8(bytes).unwrap()
}
