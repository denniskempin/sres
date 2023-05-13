use std::path::PathBuf;

use pretty_assertions::assert_eq;
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
#[ignore = "Instructions not implemented yet"]
pub fn test_cpupsr() {
    run_krom_test("CPUPSR");
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cpuret() {
    run_krom_test("CPURET");
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cpurol() {
    run_krom_test("CPUROL");
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cpuror() {
    run_krom_test("CPUROR");
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cpusbc() {
    run_krom_test("CPUSBC");
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cpustr() {
    run_krom_test("CPUSTR");
}

#[test]
#[ignore = "Instructions not implemented yet"]
pub fn test_cputrn() {
    run_krom_test("CPUTRN");
}

fn run_krom_test(test_name: &str) {
    let trace_path = PathBuf::from(format!("tests/cpu/{test_name}-trace.log"));
    let rom_path = PathBuf::from(format!("tests/cpu/{test_name}.sfc"));

    let mut bus = TestBus::with_sfc(&rom_path).unwrap();
    // Fake RDNMI register. NMI is always true.
    bus.write_u8(0x004210, 0xC2);
    let mut cpu = Cpu::new(bus);
    cpu.reset();

    let mut in_nmi_loop = false;
    for (i, expected_line) in Trace::from_file(&trace_path).unwrap().enumerate() {
        let mut expected_line = expected_line.unwrap();
        let mut actual_line = cpu.trace();

        // krom tests will run a loop to wait for nmi:
        // bit $4210; bpl ...;
        // Skip those, to match our fake implementation that always return NMI
        if in_nmi_loop {
            if expected_line.status.negative {
                println!("Line {:06}: End skip", i);
                in_nmi_loop = false;
            } else {
                continue;
            }
        }

        if actual_line.instruction == "bit" {
            if let Some(addr) = actual_line.operand_addr {
                if addr.offset == 0x4210 {
                    in_nmi_loop = true;
                    println!("Line {:06}: Skipping NMI loop", i);
                }
            }
        }

        println!(
            "{:06} ({:02X}): {}",
            i,
            cpu.bus.read_u8(cpu.pc),
            actual_line
        );

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
        assert_eq!(actual_line.to_string(), expected_line.to_string());
        cpu.step();
    }
}
