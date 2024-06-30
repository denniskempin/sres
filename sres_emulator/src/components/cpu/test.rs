//! Executes CPU-only tests using test data at https://github.com/TomHarte/ProcessorTests
//!
//! The data provides thousands of test cases with initial CPU state and expected CPU state after
//! executing one instruction.

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

use crate::common::address::AddressU24;
use crate::common::bus::Bus;
use crate::common::debugger::DebuggerRef;
use crate::common::logging;
use crate::common::test_bus::Cycle;
use crate::common::test_bus::TestBus;

use super::Cpu;
use super::StatusFlags;

const SKIP_OPCODES: &[u8] = &[
    0x44, // MVP test cases seem to follow a very different implementation
    0x54, // MVN test cases seem to follow a very different implementation
];

#[test]
pub fn test_opcodes_0x() {
    run_tomharte_test("0x");
}

#[test]
pub fn test_opcodes_1x() {
    run_tomharte_test("1x");
}

#[test]
pub fn test_opcodes_2x() {
    run_tomharte_test("2x");
}

#[test]
pub fn test_opcodes_3x() {
    run_tomharte_test("3x");
}

#[test]
pub fn test_opcodes_4x() {
    run_tomharte_test("4x");
}

#[test]
pub fn test_opcodes_5x() {
    run_tomharte_test("5x");
}

#[test]
pub fn test_opcodes_6x() {
    run_tomharte_test("6x");
}

#[test]
pub fn test_opcodes_7x() {
    run_tomharte_test("7x");
}

#[test]
pub fn test_opcodes_8x() {
    run_tomharte_test("8x");
}

#[test]
pub fn test_opcodes_9x() {
    run_tomharte_test("9x");
}

#[test]
pub fn test_opcodes_ax() {
    run_tomharte_test("ax");
}

#[test]
pub fn test_opcodes_bx() {
    run_tomharte_test("bx");
}

#[test]
pub fn test_opcodes_cx() {
    run_tomharte_test("cx");
}

#[test]
pub fn test_opcodes_dx() {
    run_tomharte_test("dx");
}

#[test]
pub fn test_opcodes_ex() {
    run_tomharte_test("ex");
}

#[test]
pub fn test_opcodes_fx() {
    run_tomharte_test("fx");
}

/// Executes the test cases provided by tomharte_cpu/{test_name}.json.xz
fn run_tomharte_test(test_name: &str) {
    logging::test_init(false);
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let json_path = root_dir.join(format!("src/components/cpu/test/{test_name}.json.xz"));
    let mut failed_opcodes: HashMap<u8, u32> = HashMap::new();

    for test_case in TestCase::from_xz_file(&json_path) {
        let mut actual_state = test_case.initial.create_cpu();
        let expected_state = test_case.final_.create_cpu();
        let opcode = actual_state
            .bus
            .peek_u8(actual_state.pc)
            .unwrap_or_default();
        if SKIP_OPCODES.contains(&opcode) {
            continue;
        }

        actual_state.step();

        // Compare before asserting to print additional information on failures
        let state_matches = &actual_state.trace() == &expected_state.trace();
        let memory_matches = actual_state.bus.memory == expected_state.bus.memory;
        // Only compare cycle count. No need to be perfectly accurate with the order.
        let cycles_match = actual_state.bus.cycles.len() == test_case.cycles().len();

        if state_matches && memory_matches && cycles_match {
            // Test case passed!
            continue;
        }

        *failed_opcodes.entry(opcode).or_insert(0) += 1;

        println!();
        println!(
            "Case {:2X}: {}",
            opcode,
            &test_case.initial.create_cpu().trace()
        );
        println!(
            "Result: {}",
            StrComparison::new(
                &actual_state.trace().to_string(),
                &expected_state.trace().to_string()
            )
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
    s: u16,
    p: u8,
    a: u16,
    x: u16,
    y: u16,
    dbr: u8,
    d: u16,
    pbr: u8,
    e: u8,
    ram: Vec<(u32, u8)>,
}

impl TestCpuState {
    /// Create a CPU instance with the state described in the test case.
    fn create_cpu(&self) -> Cpu<TestBus<AddressU24>> {
        let mut bus = TestBus::default();
        for (addr, value) in &self.ram {
            bus.memory.set(AddressU24::from(*addr), *value);
        }
        let mut cpu = Cpu::new(bus, DebuggerRef::new());
        cpu.pc = AddressU24 {
            bank: self.pbr,
            offset: self.pc,
        };
        cpu.s = self.s;
        cpu.status = StatusFlags::from(self.p);
        cpu.a.value = self.a;
        cpu.x.value = self.x;
        cpu.y.value = self.y;
        cpu.db = self.dbr;
        cpu.d = self.d;
        cpu.emulation_mode = self.e == 1;
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
    raw_cycles: Vec<(Option<u32>, Option<u8>, String)>,
}

impl TestCase {
    /// Returns an iterator of test cases read from the compressed JSON file at `path`.
    fn from_xz_file(path: &PathBuf) -> impl Iterator<Item = Self> {
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(XzDecoder::new(file));
        // To speed things up, read json file line-by-line instead of reading the whole vector at
        // once.
        reader.lines().map(|line| {
            let line = line.unwrap();
            // Trim array syntax from each line
            let trimmed = line
                .trim_end_matches(']')
                .trim_end_matches(',')
                .trim_start_matches('[');
            serde_json::from_str::<Self>(trimmed).unwrap()
        })
    }

    /// Translates the JSON format cycles into the `Cycle` format.
    fn cycles(&self) -> Vec<Cycle<AddressU24>> {
        self.raw_cycles
            .iter()
            .map(|(addr, value, state)| {
                if !(state.contains('p') || state.contains('d')) {
                    Cycle::Internal
                } else if state.contains('r') {
                    Cycle::Read(addr.unwrap_or_default().into(), *value)
                } else if state.contains('w') {
                    Cycle::Write(addr.unwrap_or_default().into(), value.unwrap_or_default())
                } else {
                    Cycle::Internal
                }
            })
            .collect()
    }
}
