/// Executes CPU-only tests using test data at https://github.com/TomHarte/ProcessorTests
///
/// The data provides thousands of test cases with initial CPU state and expected CPU state after
/// executing one instruction.
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::PathBuf;

use itertools::Itertools;
use pretty_assertions::Comparison;
use pretty_assertions::StrComparison;
use serde::Deserialize;
use serde::Serialize;
use sres_emulator::apu::spc700::Spc700;
use sres_emulator::apu::spc700::Spc700Bus;
use sres_emulator::apu::spc700::Spc700StatusFlags;
use sres_emulator::debugger::DebuggerRef;
use sres_emulator::util::logging;
use sres_emulator::util::memory::AddressU16;
use sres_emulator::util::memory::Bus;
use sres_emulator::util::memory::SparseMemory;
use xz2::read::XzDecoder;

const SKIP_OPCODES: &[u8] = &[];

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_0x() {
    run_tomharte_test("0x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_1x() {
    run_tomharte_test("1x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_2x() {
    run_tomharte_test("2x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_3x() {
    run_tomharte_test("3x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_4x() {
    run_tomharte_test("4x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_5x() {
    run_tomharte_test("5x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_6x() {
    run_tomharte_test("6x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_7x() {
    run_tomharte_test("7x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_8x() {
    run_tomharte_test("8x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_9x() {
    run_tomharte_test("9x");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_ax() {
    run_tomharte_test("ax");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_bx() {
    run_tomharte_test("bx");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_cx() {
    run_tomharte_test("cx");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_dx() {
    run_tomharte_test("dx");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_ex() {
    run_tomharte_test("ex");
}

#[test]
#[ignore = "Not yet implemented"]
pub fn test_opcodes_fx() {
    run_tomharte_test("fx");
}

/// Executes the test cases provided by tomharte_spc700/{test_name}.json.xz
fn run_tomharte_test(test_name: &str) {
    logging::test_init(false);
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let json_path = root_dir.join(format!("tests/tomharte_spc700/{test_name}.json.xz"));
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
        let cycles_match = actual_state.bus.cycles == test_case.cycles();

        if state_matches && memory_matches && cycles_match {
            // Test case passed!
            continue;
        }

        *failed_opcodes.entry(opcode).or_insert(0) += 1;

        println!();
        println!("Case {:2X}: {}", opcode, test_case.initial.create_spc700());
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
    fn create_spc700(&self) -> Spc700<TestBus> {
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

/// A test implementation of the `Bus`.
///
/// Stores memore sparsely and records all bus cycles for comparison to the test data.
#[derive(Default)]
struct TestBus {
    pub memory: SparseMemory<AddressU16>,
    pub cycles: Vec<Cycle>,
}

impl Bus<AddressU16> for TestBus {
    fn peek_u8(&self, addr: AddressU16) -> Option<u8> {
        self.memory.get(addr)
    }

    fn cycle_read_u8(&mut self, addr: AddressU16) -> u8 {
        let value = self.peek_u8(addr).unwrap_or_default();
        self.cycles.push(Cycle::Read(addr, value));
        value
    }

    #[allow(clippy::single_match)]
    fn cycle_write_u8(&mut self, addr: AddressU16, val: u8) {
        self.cycles.push(Cycle::Write(addr, val));
        self.memory.set(addr, val);
    }

    fn cycle_io(&mut self) {
        self.cycles.push(Cycle::Internal);
    }

    fn reset(&mut self) {}
}

impl Spc700Bus for TestBus {}

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
    fn cycles(&self) -> Vec<Cycle> {
        self.raw_cycles
            .iter()
            .map(|(addr, value, state)| {
                if state == "wait" {
                    Cycle::Internal
                } else if state == "read" {
                    Cycle::Read(
                        AddressU16(addr.unwrap_or_default()),
                        value.unwrap_or_default(),
                    )
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

/// Description of a bus cycle
#[derive(Clone, Copy, Eq, PartialEq)]
enum Cycle {
    /// The bus was in read mode: (addr, value read)
    Read(AddressU16, u8),
    /// The bus was in write mode: (addr, value written)
    Write(AddressU16, u8),
    /// The bus performed an internal operation
    Internal,
}

impl Debug for Cycle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Cycle::Read(addr, value) => write!(f, "R({})={:02X}", addr, value),
            Cycle::Write(addr, value) => write!(f, "W({})={:02X}", addr, value),
            Cycle::Internal => write!(f, "I"),
        }
    }
}
