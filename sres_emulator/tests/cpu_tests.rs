use std::collections::VecDeque;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use sres_emulator::bus::fvh_to_master_clock;
use sres_emulator::bus::TestBus;
use sres_emulator::cpu::Cpu;
use sres_emulator::memory::Memory;
use sres_emulator::trace::Trace;

#[test]
pub fn test_cpuadc() {
    run_krom_test("CPUADC");
}

#[test]
pub fn test_cpuand() {
    run_krom_test("CPUAND");
}

#[test]
pub fn test_cpuasl() {
    run_krom_test("CPUASL");
}

#[test]
pub fn test_cpubit() {
    run_krom_test("CPUBIT");
}

#[test]
pub fn test_cpubra() {
    run_krom_test("CPUBRA");
}

#[test]
pub fn test_cpucmp() {
    run_krom_test("CPUCMP");
}

#[test]
pub fn test_cpudec() {
    run_krom_test("CPUDEC");
}

#[test]
pub fn test_cpueor() {
    run_krom_test("CPUEOR");
}

#[test]
pub fn test_cpuinc() {
    run_krom_test("CPUINC");
}

#[test]
pub fn test_cpujmp() {
    run_krom_test("CPUJMP");
}

#[test]
pub fn test_cpuldr() {
    run_krom_test("CPULDR");
}

#[test]
pub fn test_cpulsr() {
    run_krom_test("CPULSR");
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cpumov() {
    run_krom_test("CPUMOV");
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cpumsc() {
    run_krom_test("CPUMSC");
}

#[test]
pub fn test_cpuora() {
    run_krom_test("CPUORA");
}

#[test]
pub fn test_cpuphl() {
    run_krom_test("CPUPHL");
}

#[test]
pub fn test_cpupsr() {
    run_krom_test("CPUPSR");
}

#[test]
pub fn test_cpuret() {
    run_krom_test("CPURET");
}

#[test]
pub fn test_cpurol() {
    run_krom_test("CPUROL");
}

#[test]
pub fn test_cpuror() {
    run_krom_test("CPUROR");
}

#[test]
pub fn test_cpusbc() {
    run_krom_test("CPUSBC");
}

#[test]
pub fn test_cpustr() {
    run_krom_test("CPUSTR");
}

#[test]
pub fn test_cputrn() {
    run_krom_test("CPUTRN");
}

#[test]
pub fn test_ppu_timing() {
    run_krom_test("PpuTiming");
}

fn run_krom_test(test_name: &str) {
    let trace_path = PathBuf::from(format!("tests/cpu/{test_name}-trace.log.xz"));
    let rom_path = PathBuf::from(format!("tests/cpu/{test_name}.sfc"));

    let mut bus = TestBus::with_sfc(&rom_path).unwrap();
    // CPUMSC reads 0x20 from $000000 at the first instruction. I cannot figure out why, it
    // should be mapped to RAM.
    bus.write_u8(0x000000, 0x20);

    let mut cpu = Cpu::new(bus);
    cpu.reset();

    let mut previous_master_cycle = 0;
    let mut previous_lines: VecDeque<Trace> = VecDeque::new();
    for (i, expected_line) in Trace::from_xz_file(&trace_path).unwrap().enumerate() {
        let mut expected_line = expected_line.unwrap();
        if i == 0 {
            assert_eq!(
                expected_line.h, 186,
                "Trace file is using dots not H-position"
            );
        }

        let mut actual_line = cpu.trace(true);
        previous_lines.push_front(actual_line.clone());
        previous_lines.truncate(50);

        // Fix some BSNES trace inconsistencies:

        // Disassembly for branch instructions prints the absolute operand address, not the
        // relative address.
        if expected_line.instruction.starts_with('b') && expected_line.instruction != "bit" {
            actual_line.operand = "".to_string();
            expected_line.operand = "".to_string();
        }
        // `per` instruction prints relative address as effective address, not the calculated
        // absolute address.
        if expected_line.instruction == "per" {
            actual_line.operand = "".to_string();
            expected_line.operand = "".to_string();
            actual_line.operand_addr = None;
            expected_line.operand_addr = None;
        }
        // `jmp` instructions in bsnes print an inconsistent effective address. Skip comparison.
        if expected_line.instruction.starts_with('j') {
            expected_line.operand_addr = None;
            actual_line.operand_addr = None;
        }

        if actual_line != expected_line {
            println!("Assertion failure at instruction {i}");
            for line in previous_lines.iter().rev() {
                println!("{line}");
            }

            // Convert F: V: H: from BSNES trace to master cycles to make it easier to compare how
            // many cycles each instruction takes (or should take).
            let expected_master_cycle =
                fvh_to_master_clock(expected_line.f, expected_line.v, expected_line.h);
            let expected_duration = expected_master_cycle.saturating_sub(previous_master_cycle);
            let actual_duration = cpu
                .bus
                .ppu_timer
                .master_clock
                .saturating_sub(previous_master_cycle);
            if expected_duration != actual_duration {
                println!(
                    "Expected duration: {} - Actual: {}, diff: {}",
                    expected_duration,
                    actual_duration,
                    (expected_duration as i64) - (actual_duration as i64),
                );
            }

            // Compare as strings to get a nice diff.
            assert_eq!(actual_line.to_string(), expected_line.to_string())
        }

        previous_master_cycle = cpu.bus.ppu_timer.master_clock;
        cpu.step();
    }
}
