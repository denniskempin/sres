use std::path::PathBuf;

use pretty_assertions::assert_eq;
use sres_emulator::bus::SresBus;
use sres_emulator::cpu::Cpu;
use sres_emulator::memory::Memory;
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

    let mut in_nmi_loop = false;
    for (i, expected_line) in Trace::from_file(&trace_path).unwrap().enumerate() {
        // Exit test after unimplemented part
        if i == 5000 {
            break;
        }
        let expected_line = expected_line.unwrap();
        let actual_line = cpu.trace();

        // krom tests will run a loop to wait for nmi:
        // bit $4210
        // bpl ...
        // Skip those, so the fake implementation can always return NMI
        if in_nmi_loop {
            if expected_line.status.negative {
                println!("Line {:06}: End skip", i);
                in_nmi_loop = false;
            } else {
                continue;
            }
        }

        println!("Line {:06}: opcode {:02X}", i, cpu.bus.read(cpu.pc));
        assert_eq!(actual_line.to_string(), expected_line.to_string());
        cpu.step();

        if actual_line.instruction == "bit" {
            if let Some(addr) = actual_line.operand_addr {
                if addr.offset == 0x4210 {
                    in_nmi_loop = true;
                    println!("Line {:06}: Skipping NMI loop", i);
                }
            }
        }
    }
}
