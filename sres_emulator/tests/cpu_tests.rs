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
    run_krom_test("CPUADC", true, 0);
}

#[test]
pub fn test_cpuand() {
    run_krom_test("CPUAND", true, 0);
}

#[test]
pub fn test_cpuasl() {
    run_krom_test("CPUASL", true, 0);
}

#[test]
pub fn test_cpubit() {
    run_krom_test("CPUBIT", true, 0);
}

#[test]
pub fn test_cpubra() {
    run_krom_test("CPUBRA", true, 0);
}

#[test]
pub fn test_cpucmp() {
    run_krom_test("CPUCMP", true, 0);
}

#[test]
pub fn test_cpudec() {
    run_krom_test("CPUDEC", true, 0);
}

#[test]
pub fn test_cpueor() {
    run_krom_test("CPUEOR", true, 0);
}

#[test]
pub fn test_cpuinc() {
    run_krom_test("CPUINC", true, 0);
}

#[test]
pub fn test_cpujmp() {
    run_krom_test("CPUJMP", false, 0);
}

#[test]
pub fn test_cpuldr() {
    run_krom_test("CPULDR", true, 0);
}

#[test]
pub fn test_cpulsr() {
    run_krom_test("CPULSR", true, 0);
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cpumov() {
    run_krom_test("CPUMOV", false, 0);
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cpumsc() {
    run_krom_test("CPUMSC", false, 0);
}

#[test]
pub fn test_cpuora() {
    run_krom_test("CPUORA", true, 0);
}

#[test]
pub fn test_cpuphl() {
    run_krom_test("CPUPHL", true, 0);
}

#[test]
pub fn test_cpupsr() {
    run_krom_test("CPUPSR", true, 0);
}

#[test]
pub fn test_cpuret() {
    run_krom_test("CPURET", false, 0);
}

#[test]
pub fn test_cpurol() {
    run_krom_test("CPUROL", true, 0);
}

#[test]
pub fn test_cpuror() {
    run_krom_test("CPUROR", true, 0);
}

#[test]
pub fn test_cpusbc() {
    run_krom_test("CPUSBC", true, 0);
}

#[test]
pub fn test_cpustr() {
    run_krom_test("CPUSTR", true, 0);
}

#[test]
pub fn test_cputrn() {
    run_krom_test("CPUTRN", true, 0);
}

#[test]
pub fn test_ppu_timing() {
    run_krom_test("PpuTiming", true, 0);
}

fn run_krom_test(test_name: &str, validate_cycles: bool, instruction_limit: u64) {
    let trace_path = PathBuf::from(format!("tests/cpu/{test_name}-trace.log.xz"));
    let rom_path = PathBuf::from(format!("tests/cpu/{test_name}.sfc"));

    let mut bus = TestBus::with_sfc(&rom_path).unwrap();
    if !validate_cycles {
        // Fake RDNMI register. NMI is always true.
        bus.write_u8(0x004210, 0xC2);
    }
    // CPUMSC reads 0x20 from $000000 at the first instruction. I cannot figure out why, it
    // should be mapped to RAM.
    bus.write_u8(0x000000, 0x20);

    let mut cpu = Cpu::new(bus);
    cpu.reset();

    let mut in_nmi_loop = false;
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
        if i > instruction_limit as usize && instruction_limit > 0 {
            println!("Reached instruction limit, stopping.");
            break;
        }

        let mut actual_line = cpu.trace(true);
        previous_lines.push_front(actual_line.clone());
        previous_lines.truncate(50);

        // krom tests will run a loop to wait for nmi:
        // bit $4210; bpl ...;
        // Skip those, to match our fake implementation that always return NMI
        if in_nmi_loop {
            if expected_line.status.negative {
                in_nmi_loop = false;
            } else {
                if validate_cycles {
                    cpu.step();
                }
                continue;
            }
        }

        if actual_line.instruction == "bit" {
            if let Some(addr) = actual_line.operand_addr {
                if addr.offset == 0x4210 {
                    in_nmi_loop = true;
                }
            }
        }

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

        // Comparison of PPU V, H, F cycles is done separately below.
        if actual_line.to_string()[..80] != expected_line.to_string()[..80] {
            println!("Assertion failure at instruction {i}");
            for line in previous_lines.iter().rev() {
                println!("{line}");
            }
            assert_eq!(
                actual_line.to_string()[..80],
                expected_line.to_string()[..80]
            )
        }

        if validate_cycles {
            // Convert F: V: H: from BSNES trace to master cycles to make it easier to compare how many
            // cycles each instruction takes (or should take).
            let expected_master_cycle =
                fvh_to_master_clock(expected_line.f, expected_line.v, expected_line.h);
            let expected_duration = expected_master_cycle.saturating_sub(previous_master_cycle);
            let actual_duration = cpu
                .bus
                .ppu_timer
                .master_clock
                .saturating_sub(previous_master_cycle);
            if expected_duration != actual_duration {
                println!("Assertion failure at instruction {i}");
                for line in previous_lines.iter().rev() {
                    println!("{line}");
                }
                panic!(
                    "Expected duration: {} - Actual: {}, diff: {}",
                    expected_duration,
                    actual_duration,
                    (expected_duration as i64) - (actual_duration as i64),
                );
            }
            previous_master_cycle = cpu.bus.ppu_timer.master_clock;
        }

        cpu.step();
    }
}
