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
use sres_emulator::bus::Bus;
use sres_emulator::bus::PpuTimer;
use sres_emulator::cpu::status::StatusFlags;
use sres_emulator::cpu::Cpu;
use sres_emulator::memory::Address;
use sres_emulator::memory::Memory;
use sres_emulator::memory::ToAddress;
use xz2::read::XzDecoder;

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
    fn create_cpu(&self) -> Cpu<TestBus> {
        let mut bus = TestBus::default();
        for (addr, value) in &self.ram {
            bus.memory.insert(*addr, *value);
        }
        let mut cpu = Cpu::new(bus);
        cpu.pc = Address {
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

#[derive(Serialize, Deserialize)]
struct TestCase {
    name: String,
    initial: TestCpuState,
    #[serde(rename = "final")]
    final_: TestCpuState,
    cycles: Vec<(Option<u32>, Option<u8>, String)>,
}

impl TestCase {
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
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Cycle {
    Read(u32, u8),
    Write(u32, u8),
    Internal,
}

#[derive(Default)]
struct TestBus {
    pub memory: HashMap<u32, u8>,
    pub cycles: Vec<Cycle>,
}

impl Memory for TestBus {
    fn peek_u8(&self, addr: impl ToAddress) -> Option<u8> {
        let addr = addr.to_address();
        Some(*self.memory.get(&u32::from(addr)).unwrap_or(&0))
    }

    fn read_u8(&mut self, addr: impl ToAddress) -> u8 {
        let addr = addr.to_address();
        self.cycles.push(Cycle::Read(u32::from(addr), 0));
        self.peek_u8(addr).unwrap_or_default()
    }

    #[allow(clippy::single_match)]
    fn write_u8(&mut self, addr: impl ToAddress, val: u8) {
        let addr = addr.to_address();
        self.cycles.push(Cycle::Write(u32::from(addr), 0));
        self.memory.insert(u32::from(addr), val);
    }
}

impl Bus for TestBus {
    fn internal_operation_cycle(&mut self) {
        self.cycles.push(Cycle::Internal);
    }

    fn advance_master_clock(&mut self, _: u64) {}

    fn ppu_timer(&self) -> PpuTimer {
        PpuTimer::default()
    }

    fn reset(&mut self) {}
}

const SKIP_OPCODES: &[u8] = &[];

fn run_tomharte_test(test_name: &str) {
    let json_path = PathBuf::from(format!("tests/tomharte_tests/{test_name}.json.xz"));
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

        let state_matches = actual_state.trace(true) == expected_state.trace(true);
        let memory_matches = actual_state.bus.memory == expected_state.bus.memory;
        let cycles_match = actual_state.bus.cycles.len() == test_case.cycles.len();
        if state_matches && memory_matches && cycles_match {
            continue;
        }

        *failed_opcodes.entry(opcode).or_insert(0) += 1;

        println!("Case:   {}", test_case.initial.create_cpu().trace(true));
        if !state_matches {
            println!(
                "Result: {}",
                StrComparison::new(
                    &actual_state.trace(true).to_string(),
                    &expected_state.trace(true).to_string()
                )
            )
        }
        if !memory_matches {
            println!(
                "Memory: {}",
                Comparison::new(&actual_state.bus.memory, &expected_state.bus.memory)
            )
        }
        if !cycles_match {
            println!(
                "Cycles: {}",
                Comparison::new(&actual_state.bus.cycles.len(), &test_case.cycles.len())
            )
        }
    }

    if !failed_opcodes.is_empty() {
        println!("Failing tests by opcode:");
        for failed_opcode in failed_opcodes.iter().sorted() {
            println!("0x{:02X}: {}", failed_opcode.0, failed_opcode.1);
        }
        panic!("Some tests failed");
    }
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_0x() {
    run_tomharte_test("0x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_1x() {
    run_tomharte_test("1x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_2x() {
    run_tomharte_test("2x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_3x() {
    run_tomharte_test("3x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_4x() {
    run_tomharte_test("4x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_5x() {
    run_tomharte_test("5x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_6x() {
    run_tomharte_test("6x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_7x() {
    run_tomharte_test("7x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_8x() {
    run_tomharte_test("8x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_9x() {
    run_tomharte_test("9x");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_ax() {
    run_tomharte_test("ax");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_bx() {
    run_tomharte_test("bx");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_cx() {
    run_tomharte_test("cx");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_dx() {
    run_tomharte_test("dx");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_ex() {
    run_tomharte_test("ex");
}

#[test]
#[ignore = "not passing yet"]
pub fn test_opcodes_fx() {
    run_tomharte_test("fx");
}

#[test]
#[ignore = "only used temporarily for collecting stats about failing tests"]
fn test_result_stats() {
    let mut success_cases: HashMap<u8, u32> = HashMap::new();
    let mut failed_cases: HashMap<u8, u32> = HashMap::new();

    for test_name in [
        "0x", "1x", "2x", "3x", "4x", "5x", "6x", "7x", "8x", "9x", "ax", "bx", "cx", "dx", "ex",
        "fx",
    ] {
        println!("Testing {}...", test_name);

        let json_path = PathBuf::from(format!("tests/tomharte_tests/{test_name}.json.xz"));

        for test_case in TestCase::from_xz_file(&json_path) {
            let initial_state = test_case.initial.create_cpu();
            let expected_state = test_case.final_.create_cpu();
            let mut actual_state = test_case.initial.create_cpu();
            actual_state.step();
            let opcode = initial_state
                .bus
                .peek_u8(initial_state.pc)
                .unwrap_or_default();

            let state_matches = actual_state.trace(true) == expected_state.trace(true);
            let memory_matches = actual_state.bus.memory == expected_state.bus.memory;
            let cycles_match = actual_state.bus.cycles.len() == test_case.cycles.len();
            if !state_matches || !memory_matches || !cycles_match {
                *failed_cases.entry(opcode).or_insert(0) += 1;
            } else {
                *success_cases.entry(opcode).or_insert(0) += 1;
            }
        }
    }
    for opcode in 0..=0xFF {
        let success_count = success_cases.get(&opcode).unwrap_or(&0);
        let failure_count = failed_cases.get(&opcode).unwrap_or(&0);
        if *success_count == 0 && *failure_count == 0 {
            continue;
        }
        println!("0x{:02X}: {:6}/{:}", opcode, success_count, failure_count);
    }

    println!("Total success: {}", success_cases.values().sum::<u32>());
    println!("Total failed: {}", failed_cases.values().sum::<u32>());
}
