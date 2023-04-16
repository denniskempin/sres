use std::path::PathBuf;

use sres_emulator::bus::Bus;
use sres_emulator::bus::SresBus;
use sres_emulator::trace::Trace;

#[test]
pub fn test_cpuadc() {
    run_krom_test("CPUADC");
}

fn run_krom_test(test_name: &str) {
    let trace_path = PathBuf::from(format!("tests/cpu/{test_name}-trace.log"));
    let rom_path = PathBuf::from(format!("tests/cpu/{test_name}.sfc"));

    let mut bus = SresBus::new();
    bus.cartridge.load_sfc(&rom_path).unwrap();

    // TODO: Load vector table for initial PC.
    let first_instruction = bus.read(0x008000);
    assert_eq!(first_instruction, 0x78);

    for (_i, line) in Trace::from_file(&trace_path).unwrap().enumerate() {
        let line = line.unwrap();
        // Just format to string and back to verify Trace parsing for now.
        let parsed = format!("{}", line);
        assert_eq!(parsed.parse::<Trace>().unwrap(), line);
    }
}
