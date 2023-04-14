use std::path::PathBuf;

use anyhow::Context;
use sres_emulator::trace::Trace;

#[test]
pub fn test_cpuadc() {
    run_krom_test("CPUADC");
}

fn run_krom_test(test_name: &str) {
    let trace_path = PathBuf::from(format!("tests/cpu/{test_name}-trace.log"));
    let _rom_path = PathBuf::from(format!("tests/cpu/{test_name}.sfc"));

    for (i, line) in Trace::from_file(&trace_path).unwrap().enumerate() {
        line.context(format!("In line {i}")).unwrap();
    }
}
