// Executes SPC700 tests using test data at https://github.com/TomHarte/ProcessorTests
//
// The data provides thousands of test cases with initial CPU state and expected CPU state after
// executing one instruction.

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use itertools::Itertools;
use pretty_assertions::Comparison;
use pretty_assertions::StrComparison;
use serde::Deserialize;
use serde::Serialize;
use xz2::read::XzDecoder;

use crate::common::address::AddressU16;
use crate::common::bus::Bus;
use crate::debugger::DebuggerRef;
use crate::test_util::test_bus::Cycle;
use crate::test_util::test_bus::TestBus;
use crate::util::logging;

use super::Spc700;
use super::Spc700Bus;
use super::Spc700StatusFlags;

#[rustfmt::skip]
const SKIP_OPCODES: &[u8] = &[];

/// For some opcodes we only want to ensure the correct number of cycles have been
/// have been executed and ignore the details of what those cycles actually do.
#[rustfmt::skip]
const IGNORE_CYCLE_DETAILS: &[u8] = &[
    // mov1: io cycle between read/write of AbsBit
    0xCA,
    // mov [d]+Y, A: io cycle in an odd place
    0xD7,
    // dbnz: Test cases read open bus and a value from the same address
    0xFE,
];

#[test]
pub fn test_spc700_opcodes_0x() {
    run_tomharte_test("0x");
}

#[test]
pub fn test_spc700_opcodes_1x() {
    run_tomharte_test("1x");
}

#[test]
pub fn test_spc700_opcodes_2x() {
    run_tomharte_test("2x");
}

#[test]
pub fn test_spc700_opcodes_3x() {
    run_tomharte_test("3x");
}

#[test]
pub fn test_spc700_opcodes_4x() {
    run_tomharte_test("4x");
}

#[test]
pub fn test_spc700_opcodes_5x() {
    run_tomharte_test("5x");
}

#[test]
pub fn test_spc700_opcodes_6x() {
    run_tomharte_test("6x");
}

#[test]
pub fn test_spc700_opcodes_7x() {
    run_tomharte_test("7x");
}

#[test]
pub fn test_spc700_opcodes_8x() {
    run_tomharte_test("8x");
}

#[test]
pub fn test_spc700_opcodes_9x() {
    run_tomharte_test("9x");
}

#[test]
pub fn test_spc700_opcodes_ax() {
    run_tomharte_test("ax");
}

#[test]
pub fn test_spc700_opcodes_bx() {
    run_tomharte_test("bx");
}

#[test]
pub fn test_spc700_opcodes_cx() {
    run_tomharte_test("cx");
}

#[test]
pub fn test_spc700_opcodes_dx() {
    run_tomharte_test("dx");
}

#[test]
pub fn test_spc700_opcodes_ex() {
    run_tomharte_test("ex");
}

#[test]
pub fn test_spc700_opcodes_fx() {
    run_tomharte_test("fx");
}

/// Executes the test cases provided by tomharte_spc700/{test_name}.json.xz
fn run_tomharte_test(test_name: &str) {
    logging::test_init(false);
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let json_path = root_dir.join(format!("src/components/spc700/test/{test_name}.json.xz"));
    let mut failed_opcodes: HashMap<u8, u32> = HashMap::new();

    for test_case in TestCase::from_xz_file(&json_path) {
        let mut actual_state = test_case.initial.create_spc700();
        let expected_state = test_case.final_.create_spc700();
        let opcode = actual_state
            .bus
            .peek_u8(actual_state.pc)
            .unwrap_or_default();
        if SKIP_OPCODES.contains(&opcode) {
            continue;
        }

        actual_state.step();

        // Compare before asserting to print additional information on failures
        let state_matches = actual_state.to_string() == expected_state.to_string();
        let memory_matches = actual_state.bus.memory == expected_state.bus.memory;
        let cycles_match = if IGNORE_CYCLE_DETAILS.contains(&opcode) {
            actual_state.bus.cycles.len() == test_case.cycles().len()
        } else {
            actual_state.bus.cycles == test_case.cycles()
        };

        if state_matches && memory_matches && cycles_match {
            // Test case passed!
            continue;
        }

        *failed_opcodes.entry(opcode).or_insert(0) += 1;

        println!();
        println!("Case {:02X}: {}", opcode, test_case.initial.create_spc700());
        println!(
            "Result: {}",
            StrComparison::new(&actual_state.to_string(), &expected_state.to_string())
        );
        println!(
            "Memory: {}",
            StrComparison::new(
                &actual_state.bus.memory.to_string(),
                &expected_state.bus.memory.to_string()
            )
        );
        println!(
            "Cycles: {}",
            Comparison::new(&actual_state.bus.cycles, &test_case.cycles())
        );
    }

    if !failed_opcodes.is_empty() {
        println!("Failing tests by opcode:");
        for failed_opcode in failed_opcodes.iter().sorted() {
            println!("0x{:02X}: {}", failed_opcode.0, failed_opcode.1);
        }
        panic!("Some tests failed");
    }
}

/// CPU State format of the format described in
/// https://github.com/TomHarte/ProcessorTests/tree/main/65816
#[derive(Serialize, Deserialize)]
struct TestCpuState {
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    psw: u8,
    ram: Vec<(u16, u8)>,
}

impl TestCpuState {
    /// Create a SPC700 instance with the state described in the test case.
    fn create_spc700(&self) -> Spc700<TestBus<AddressU16>> {
        let mut bus = TestBus::default();
        for (addr, value) in &self.ram {
            bus.memory.set(AddressU16(*addr), *value);
        }
        let mut cpu = Spc700::new(bus, DebuggerRef::new());
        cpu.pc = AddressU16(self.pc);
        cpu.a = self.a;
        cpu.x = self.x;
        cpu.y = self.y;
        cpu.sp = self.sp;
        cpu.status = Spc700StatusFlags::from(self.psw);
        cpu
    }
}

/// A single test case, parsed from the JSON format described in
/// https://github.com/TomHarte/ProcessorTests/tree/main/65816
#[derive(Serialize, Deserialize)]
struct TestCase {
    /// Human readable name of the test case
    name: String,
    /// CPU state before execution
    initial: TestCpuState,
    /// CPU state after execution
    #[serde(rename = "final")]
    final_: TestCpuState,
    /// Bus cycles during execution (address, value, state string)
    #[serde(rename = "cycles")]
    raw_cycles: Vec<(Option<u16>, Option<u8>, String)>,
}

impl TestCase {
    /// Returns an iterator of test cases read from the compressed JSON file at `path`.
    fn from_xz_file(path: &PathBuf) -> impl Iterator<Item = Self> {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(XzDecoder::new(file));
        // The json files have been reformatted to be one json object per line to speed up parsing.
        reader
            .lines()
            .map(|line| serde_json::from_str(&line.unwrap()).unwrap())
    }

    /// Translates the JSON format cycles into the `Cycle` format.
    fn cycles(&self) -> Vec<Cycle<AddressU16>> {
        self.raw_cycles
            .iter()
            .map(|(addr, value, state)| {
                if state == "wait" {
                    Cycle::Internal
                } else if state == "read" {
                    Cycle::Read(AddressU16(addr.unwrap_or_default()), *value)
                } else if state == "write" {
                    Cycle::Write(
                        AddressU16(addr.unwrap_or_default()),
                        value.unwrap_or_default(),
                    )
                } else {
                    panic!("Unknown cycle state: {}", state);
                }
            })
            .collect()
    }
}

impl Spc700Bus for TestBus<AddressU16> {
    fn master_cycle(&self) -> u64 {
        0
    }
}
