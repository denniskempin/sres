use crate::common::address::Address;
use crate::common::address::AddressU16;
use crate::common::address::AddressU24;

/// Represents a snapshot of the Cpu state for debugging purposes
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CpuState {
    pub instruction: InstructionMeta<AddressU24>,
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub s: u16,
    pub d: u16,
    pub db: u8,
    pub status: CpuStatusFlags,
    // TODO: Use ClockInfo
    pub v: u64,
    pub h: u64,
    pub f: u64,
}

/// Metadata about a decoded instruction. Used to generate disassembly.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct InstructionMeta<AddressT: Address> {
    pub address: AddressT,
    pub operation: String,
    pub operand_str: Option<String>,
    pub effective_addr: Option<AddressT>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CpuStatusFlags {
    pub negative: bool,
    pub overflow: bool,
    pub accumulator_register_size: bool,
    pub index_register_size_or_break: bool,
    pub decimal: bool,
    pub irq_disable: bool,
    pub zero: bool,
    pub carry: bool,
}

#[derive(Clone, Copy, PartialEq, Debug, strum::Display, strum::EnumString)]
pub enum NativeVectorTable {
    #[strum(serialize = "cop")]
    Cop = 0xFFE4,
    #[strum(serialize = "break", serialize = "brk")]
    Break = 0xFFE6,
    #[strum(serialize = "nmi")]
    Nmi = 0xFFEA,
    #[strum(serialize = "irq")]
    Irq = 0xFFEE,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct ClockInfo {
    pub master_clock: u64,
    pub v: u64,
    pub h_counter: u64,
    pub f: u64,
}

impl ClockInfo {
    pub fn hdot(&self) -> u64 {
        let mut counter = self.h_counter;
        if self.f % 2 == 0 || self.v != 240 {
            // Dot 323 and 327 take 6 cycles on non-short scanlines.
            if self.h_counter > 1292 {
                counter -= 2;
            }
            if self.h_counter > 1310 {
                counter -= 2;
            }
        }
        counter / 4
    }
}

// Representation of the state of [Spc700] in the same format as logged by BSNES.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Spc700State {
    pub instruction: InstructionMeta<AddressU16>,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: AddressU16,
    pub status: String,
}
