use std::path::PathBuf;

use sres_emulator::cartridge::SnesHeader;
use sres_emulator::trace::Trace;

#[test]
pub fn test_cpuadc() {
    run_krom_test("CPUADC");
}

fn run_krom_test(test_name: &str) {
    let trace_path = PathBuf::from(format!("tests/cpu/{test_name}-trace.log"));
    let rom_path = PathBuf::from(format!("tests/cpu/{test_name}.sfc"));

    let rom = std::fs::read(rom_path).unwrap();
    let header = SnesHeader::find_header_in_rom(&rom).unwrap();
    assert_eq!(header.rom_size, rom.len());
    println!("{:?}", header);

    for (_i, line) in Trace::from_file(&trace_path).unwrap().enumerate() {
        let line = line.unwrap();
        // Just format to string and back to verify Trace parsing for now.
        let parsed = format!("{}", line);
        assert_eq!(parsed.parse::<Trace>().unwrap(), line);
    }
}
