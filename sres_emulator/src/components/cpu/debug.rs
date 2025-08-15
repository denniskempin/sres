use std::fmt::Display;
use std::str::FromStr;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use itertools::Itertools;

use super::Cpu;
use super::MainBus;
use super::NativeVectorTable;
use super::StatusFlags;
use crate::common::address::AddressU24;
use crate::common::address::InstructionMeta;
use crate::common::clock::ClockInfo;

#[derive(Debug, Clone, PartialEq)]
pub enum CpuEvent {
    Step(CpuState),
    Interrupt(NativeVectorTable),
}

pub struct CpuDebug<'a, BusT: MainBus>(pub &'a Cpu<BusT>);

impl<BusT: MainBus> CpuDebug<'_, BusT> {
    pub fn state(&self) -> CpuState {
        let (instruction, _) = self.load_instruction_meta(self.0.pc);
        CpuState {
            instruction,
            a: self.0.a.value,
            x: self.0.x.value,
            y: self.0.y.value,
            s: self.0.s,
            d: self.0.d,
            db: self.0.db,
            status: StatusFlags {
                negative: self.0.status.negative,
                overflow: self.0.status.overflow,
                accumulator_register_size: self.0.status.accumulator_register_size,
                index_register_size_or_break: self.0.status.index_register_size_or_break,
                decimal: self.0.status.decimal,
                irq_disable: self.0.status.irq_disable,
                zero: self.0.status.zero,
                carry: self.0.status.carry,
            },
            clock: self.0.bus.clock_info(),
        }
    }

    /// Return the instruction meta data for the instruction at the given address
    pub fn load_instruction_meta(
        &self,
        addr: AddressU24,
    ) -> (InstructionMeta<AddressU24>, AddressU24) {
        let opcode = self.0.bus.peek_u8(addr).unwrap_or_default();
        (self.0.instruction_table[opcode as usize].meta)(self.0, addr)
    }

    pub fn peek_next_operations(
        &self,
        count: usize,
    ) -> impl Iterator<Item = InstructionMeta<AddressU24>> + '_ {
        let mut pc = self.0.pc;
        (0..count).map(move |_| {
            let (meta, new_pc) = self.load_instruction_meta(pc);
            pc = new_pc;
            meta
        })
    }
}

/// Represents a snapshot of the Cpu state for debugging purposes
/// It formats into a string compatible with BSNES traces. Since the format changes
/// slightly between versions, this implementation is designed to be compatible with
/// bsnes-plus-v05.105. All traces in tests are generated with this version.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CpuState {
    pub instruction: InstructionMeta<AddressU24>,
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub s: u16,
    pub d: u16,
    pub db: u8,
    pub status: StatusFlags,
    pub clock: ClockInfo,
}

impl CpuState {
    /// Parse a Mesen trace line into a CpuState object
    pub fn from_mesen_trace(s: &str) -> Result<Self> {
        // The trace format has a fixed width, which allows us to use direct indexing to parse
        // instead of much slower regex or nom parsing.
        //
        // Example with indices of the start of each item:
        //
        // 00e811  BPL $E80E                      A:9901 X:0100 Y:0000 S:1FF3 D:0000 DB:00 P:nVMxdIZC Cycle:11791952
        // 0       8   12                           41     48     55     62     69      77   82             97

        if &s[39..=40] != "A:" {
            bail!("Invalid Mesen trace format - missing A: field");
        }
        if &s[80..=81] != "P:" {
            bail!("Invalid Mesen trace format - missing P: field");
        }
        if !s.contains("Cycle:") {
            bail!("Invalid Mesen trace format - missing Cycle: field");
        }

        let (operation, operand_str, io) = Self::parse_disassembly(s[8..39].trim())?;
        Ok(CpuState {
            instruction: InstructionMeta {
                address: u32::from_str_radix(&s[0..6], 16)
                    .with_context(|| "pc")?
                    .into(),
                operation,
                operand_str,
                effective_addr: io.map(|(addr, _)| addr),
            },
            a: u16::from_str_radix(&s[41..45], 16).with_context(|| "a")?,
            x: u16::from_str_radix(&s[48..52], 16).with_context(|| "x")?,
            y: u16::from_str_radix(&s[55..59], 16).with_context(|| "y")?,
            s: u16::from_str_radix(&s[62..66], 16).with_context(|| "s")?,
            d: u16::from_str_radix(&s[69..73], 16).with_context(|| "d")?,
            db: u8::from_str_radix(&s[77..79], 16).with_context(|| "db")?,
            status: {
                let status_str = &s[82..90];
                if status_str.len() != 8 {
                    bail!("StatusFlags string must be 8 characters long");
                }
                let mut chars = status_str.chars();
                StatusFlags {
                    negative: chars.next().unwrap().is_uppercase(),
                    overflow: chars.next().unwrap().is_uppercase(),
                    accumulator_register_size: chars.next().unwrap().is_uppercase(),
                    index_register_size_or_break: chars.next().unwrap().is_uppercase(),
                    decimal: chars.next().unwrap().is_uppercase(),
                    irq_disable: chars.next().unwrap().is_uppercase(),
                    zero: chars.next().unwrap().is_uppercase(),
                    carry: chars.next().unwrap().is_uppercase(),
                }
            },
            clock: ClockInfo::from_master_clock(
                u64::from_str(&s[97..].trim()).with_context(|| "cycle")?,
            ),
        })
    }

    pub fn parse_disassembly(
        disassembly: &str,
    ) -> Result<(String, Option<String>, Option<(AddressU24, u8)>)> {
        let pieces = disassembly.trim().split_ascii_whitespace().collect_vec();
        match pieces.len() {
            // e.g. "NMI"
            1 => Ok((pieces[0].to_string(), None, None)),
            // e.g. "BPL $1234"
            2 => Ok((pieces[0].to_string(), Some(pieces[1].to_string()), None)),
            // e.g. LDA $1234 [001234] = 42
            5 => {
                let effective_addr = AddressU24::from(
                    u32::from_str_radix(pieces[2].trim_matches(&['[', ']']), 16)
                        .with_context(|| "effective_addr")?,
                );
                Ok((
                    pieces[0].to_string(),
                    Some(pieces[1].to_string()),
                    Some((
                        effective_addr,
                        u8::from_str_radix(pieces[4], 16).with_context(|| "value")?,
                    )),
                ))
            }
            _ => {
                bail!("Invalid disassembly format `{disassembly}`")
            }
        }
    }

    pub fn format_disassembly(
        operation: &str,
        operand_str: &Option<String>,
        io: &Option<(AddressU24, u8)>,
    ) -> String {
        match (operand_str, io) {
            (None, None) => operation.to_string(),
            (Some(operand_str), None) => format!("{operation} {operand_str}"),
            (Some(operand_str), Some((addr, value))) => {
                let addr: u32 = (*addr).into();
                format!("{operation} {operand_str} [{addr:06X}] = {value:02X}")
            }
            (None, Some(_)) => panic!("Cannot format disassembly: No operand but IO info"),
        }
    }

    /// Format a CpuState into a Mesen trace line
    pub fn to_mesen_trace(&self) -> String {
        format!(
            "{:06x}  {:<30} A:{:04X} X:{:04X} Y:{:04X} S:{:04X} D:{:04X} DB:{:02X} P:{} Cycle:{}",
            u32::from(self.instruction.address),
            Self::format_disassembly(
                &self.instruction.operation,
                &self.instruction.operand_str,
                &None
            ),
            self.a,
            self.x,
            self.y,
            self.s,
            self.d,
            self.db,
            format!(
                "{}{}{}{}{}{}{}{}",
                if self.status.negative { 'N' } else { 'n' },
                if self.status.overflow { 'V' } else { 'v' },
                if self.status.accumulator_register_size {
                    'M'
                } else {
                    'm'
                },
                if self.status.index_register_size_or_break {
                    'X'
                } else {
                    'x'
                },
                if self.status.decimal { 'D' } else { 'd' },
                if self.status.irq_disable { 'I' } else { 'i' },
                if self.status.zero { 'Z' } else { 'z' },
                if self.status.carry { 'C' } else { 'c' }
            ),
            self.clock.master_clock
        )
    }
}

impl Display for CpuState {
    /// Format a trace object into a BSNES trace line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_mesen_trace())
    }
}

impl FromStr for CpuState {
    type Err = anyhow::Error;

    /// Parse a BSNES trace line into a CpuState object
    fn from_str(s: &str) -> Result<Self> {
        Self::from_mesen_trace(s)
    }
}

impl FromStr for StatusFlags {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            bail!("StatusFlags string must be 8 characters long");
        }
        let mut chars = s.chars();
        Ok(StatusFlags {
            negative: chars.next().unwrap().is_ascii_uppercase(),
            overflow: chars.next().unwrap().is_ascii_uppercase(),
            accumulator_register_size: chars.next().unwrap().is_ascii_uppercase(),
            index_register_size_or_break: chars.next().unwrap().is_ascii_uppercase(),
            decimal: chars.next().unwrap().is_ascii_uppercase(),
            irq_disable: chars.next().unwrap().is_ascii_uppercase(),
            zero: chars.next().unwrap().is_ascii_uppercase(),
            carry: chars.next().unwrap().is_ascii_uppercase(),
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    static EXAMPLE_MESEN_TRACE: &str = r"00e811  BPL $E80E                      A:9901 X:0100 Y:0000 S:1FF3 D:0000 DB:00 P:nVMxdIZC Cycle:11791952";

    fn example_trace() -> CpuState {
        CpuState {
            instruction: InstructionMeta {
                address: AddressU24 {
                    bank: 0,
                    offset: 0xe811,
                },
                operation: "BPL".to_string(),
                operand_str: Some("$E80E".to_string()),
                effective_addr: None,
            },
            a: 0x9901,
            x: 0x0100,
            y: 0x0000,
            s: 0x1ff3,
            d: 0x0000,
            db: 0x00,
            status: StatusFlags {
                negative: false,
                overflow: true,
                accumulator_register_size: true,
                index_register_size_or_break: false,
                decimal: false,
                irq_disable: true,
                zero: true,
                carry: true,
            },
            clock: ClockInfo {
                master_clock: 11791952,
                v: 261,
                h_counter: 236,
                f: 32,
            },
        }
    }

    #[test]
    pub fn test_from_str() {
        assert_eq!(
            EXAMPLE_MESEN_TRACE.parse::<CpuState>().unwrap(),
            example_trace()
        );
    }

    #[test]
    pub fn test_to_str() {
        assert_eq!(format!("{}", example_trace()), EXAMPLE_MESEN_TRACE);
    }

    #[test]
    pub fn test_disassembly_roundtrip() {
        let cases = ["CLC", "BPL $1234", "LDA $1234 [001234] = 00"];
        for case in cases {
            let (operation, operand_str, io) = CpuState::parse_disassembly(case).unwrap();
            let formatted = CpuState::format_disassembly(&operation, &operand_str, &io);
            assert_eq!(formatted, case);
        }
    }
}
