use std::path::PathBuf;

use pretty_assertions::assert_eq;
use sres_emulator::bus::SresBus;
use sres_emulator::cpu::Cpu;
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
    let mut cpu = Cpu::new(bus);
    cpu.reset();

    for (i, expected_line) in Trace::from_file(&trace_path).unwrap().enumerate() {
        // Exit test after unimplemented part
        if i == 3 {
            break;
        }

        let expected_line = expected_line.unwrap();
        let actual_line = cpu.trace();
        assert_eq!(actual_line, expected_line);
        cpu.step();
    }
}
