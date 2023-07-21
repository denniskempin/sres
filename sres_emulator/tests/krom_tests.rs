use std::collections::VecDeque;
use std::path::PathBuf;

use pretty_assertions::assert_eq;
use sres_emulator::bus::fvh_to_master_clock;
use sres_emulator::bus::Bus;
use sres_emulator::bus::TestBus;
use sres_emulator::cpu::Cpu;
use sres_emulator::memory::Wrap;
use sres_emulator::trace::Trace;

#[test]
pub fn test_nmi_sub_cycle_accuracy() {
    static TEST_CASES: &[(u64, u64, bool, bool)] = &[
        // The `bit $4210` instruction is often used to check the NMI signal, to wait for VSYNC.
        // This makes the instruction very sensitive to sub-cpu-cycle timing, as the result will
        // depend on when exactly the signal is read.
        //
        // The list below is the result of `bit $4210` executed at various points in the frame. This
        // matches the behavior of BSNES.
        //
        // Starting 1334, the bit instruction will end after NMI and the internal NMI flag will
        // be set after the instruction is executed.
        //
        // Starting 1340, NMI will be high by the time the bit instruction reads the state. Usually
        // reads from $4210 will reset the NMI flag, but not for the first 4 cycles.
        //
        // (V, H, nmi returned by `bit`, internal nmi flag)
        (224, 1330, false, false),
        (224, 1332, false, false),
        (224, 1334, false, true),
        (224, 1336, false, true),
        (224, 1338, false, true),
        (224, 1340, true, true),
        (224, 1342, true, true),
        (224, 1344, true, false),
        (224, 1346, true, false),
        (224, 1348, true, false),
        (224, 1350, true, false),
        (224, 1352, true, false),
        (224, 1354, true, false),
        (224, 1356, true, false),
        (224, 1358, true, false),
        (224, 1360, true, false),
        (224, 1362, true, false),
        (225, 0, true, false),
    ];
    for (v, h, expected_nmi, expected_internal_nmi) in TEST_CASES {
        // Create CPU with `bit $4210` program in memory
        let mut bus = TestBus::default();
        bus.cycle_write_u16(0x00.into(), 0x2C, Wrap::NoWrap);
        bus.cycle_write_u16(0x01.into(), 0x4210, Wrap::NoWrap);
        let mut cpu = Cpu::new(bus);
        cpu.reset();

        // Advance PPU timer until (v, h) is reached
        while cpu.bus.ppu_timer.v != *v || cpu.bus.ppu_timer.h_counter != *h {
            cpu.bus.ppu_timer.advance_master_clock(2);
        }

        // Execute `bit $4210` instruction
        println!("before: {}", cpu.trace(true));
        cpu.step();
        println!("after: {}", cpu.trace(true));

        // If the NMI bit is set, the negative status bit will be true.
        assert_eq!(cpu.status.negative, *expected_nmi);
        // For the first 4 cycles NMI will remain high, so the internal nmi_flag will still be set.
        assert_eq!(cpu.bus.ppu_timer.nmi_flag, *expected_internal_nmi);
    }
}

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
    let trace_path = PathBuf::from(format!("tests/krom_tests/{test_name}-trace.log.xz"));
    let rom_path = PathBuf::from(format!("tests/krom_tests/{test_name}.sfc"));

    let mut bus = TestBus::with_sfc(&rom_path).unwrap();
    // CPUMSC reads 0x20 from $000000 at the first instruction. I cannot figure out why, it
    // should be mapped to RAM.
    bus.cycle_write_u8(0x000000.into(), 0x20);

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
