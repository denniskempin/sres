use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;

use itertools::Itertools;
use sres_emulator::common::address::Address;
use sres_emulator::common::address::AddressU16;
use sres_emulator::common::address::AddressU24;
use sres_emulator::common::bus::Bus;
use sres_emulator::components::cpu::MainBus;
use sres_emulator::components::ppu::PpuTimer;
use sres_emulator::components::spc700::Spc700Bus;

/// A test implementation of the `Bus`.
///
/// Stores memore sparsely and records all bus cycles for comparison to the test data.
#[derive(Default)]
pub struct TestBus<AddressT: Address> {
    pub memory: SparseMemory<AddressT>,
    pub cycles: Vec<Cycle<AddressT>>,
    pub ppu_timer: PpuTimer,
}

impl<AddressT: Address> Bus<AddressT> for TestBus<AddressT> {
    fn peek_u8(&self, addr: AddressT) -> Option<u8> {
        self.memory.get(addr)
    }

    fn cycle_read_u8(&mut self, addr: AddressT) -> u8 {
        let value = self.peek_u8(addr);
        self.cycles.push(Cycle::Read(addr, value));
        value.unwrap_or_default()
    }

    #[allow(clippy::single_match)]
    fn cycle_write_u8(&mut self, addr: AddressT, val: u8) {
        self.cycles.push(Cycle::Write(addr, val));
        self.memory.set(addr, val);
    }

    fn cycle_io(&mut self) {
        self.cycles.push(Cycle::Internal);
    }

    fn reset(&mut self) {}
}

/// Description of a bus cycle
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Cycle<AddressT: Address> {
    /// The bus was in read mode: (addr, value read)
    Read(AddressT, Option<u8>),
    /// The bus was in write mode: (addr, value written)
    Write(AddressT, u8),
    /// The bus performed an internal operation
    Internal,
}

impl<AddressT: Address> Debug for Cycle<AddressT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Cycle::Read(addr, value) => {
                if let Some(value) = value {
                    write!(f, "R({})={:02X}", addr, value)
                } else {
                    write!(f, "R({})=XX", addr)
                }
            }
            Cycle::Write(addr, value) => write!(f, "W({})={:02X}", addr, value),
            Cycle::Internal => write!(f, "I"),
        }
    }
}

/// Implements a sparse memory HashMap with a readable display format.
#[derive(Default, PartialEq)]
pub struct SparseMemory<AddressT: Address> {
    pub memory: HashMap<AddressT, u8>,
}

impl<AddressT: Address> SparseMemory<AddressT> {
    pub fn get(&self, addr: AddressT) -> Option<u8> {
        self.memory.get(&addr).copied()
    }

    pub fn set(&mut self, addr: AddressT, value: u8) {
        self.memory.insert(addr, value);
    }
}

impl<AddressT: Address> Display for SparseMemory<AddressT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (addr, value) in self.memory.iter().sorted() {
            writeln!(f, "{}: {:02X}", addr, value)?;
        }
        Ok(())
    }
}

impl Spc700Bus for TestBus<AddressU16> {
    fn master_cycle(&self) -> u64 {
        0
    }
}

impl MainBus for TestBus<AddressU24> {
    fn check_nmi_interrupt(&mut self) -> bool {
        false
    }

    fn consume_timer_interrupt(&mut self) -> bool {
        false
    }

    fn ppu_timer(&self) -> &PpuTimer {
        &self.ppu_timer
    }
}
