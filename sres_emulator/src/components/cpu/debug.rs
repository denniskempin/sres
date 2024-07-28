use std::fmt::Display;
use std::fmt::Write;
use std::str::FromStr;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;

use crate::common::address::AddressU24;
use crate::common::address::InstructionMeta;
use crate::common::clock::ClockInfo;

use super::Cpu;
use super::MainBus;
use super::NativeVectorTable;
use super::StatusFlags;

#[derive(Debug, Clone, PartialEq)]
pub enum CpuEvent {
    Step(CpuState),
    Interrupt(NativeVectorTable),
}

pub struct CpuDebug<'a, BusT: MainBus>(pub &'a Cpu<BusT>);

impl<'a, BusT: MainBus> CpuDebug<'a, BusT> {
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

impl Display for CpuState {
    /// Format a trace object into a BSNES trace line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:06x} {} {:<10} {:8} A:{:04x} X:{:04x} Y:{:04x} S:{:04x} D:{:04x} DB:{:02x} ",
            u32::from(self.instruction.address),
            self.instruction.operation,
            self.instruction.operand_str.as_deref().unwrap_or_default(),
            if let Some(addr) = &self.instruction.effective_addr {
                format!("[{:06x}]", u32::from(*addr))
            } else {
                "".to_string()
            },
            self.a,
            self.x,
            self.y,
            self.s,
            self.d,
            self.db,
        )?;
        f.write_char(if self.status.negative { 'N' } else { '.' })?;
        f.write_char(if self.status.overflow { 'V' } else { '.' })?;
        f.write_char(if self.status.accumulator_register_size {
            'M'
        } else {
            '.'
        })?;
        f.write_char(if self.status.index_register_size_or_break {
            'X'
        } else {
            '.'
        })?;
        f.write_char(if self.status.decimal { 'D' } else { '.' })?;
        f.write_char(if self.status.irq_disable { 'I' } else { '.' })?;
        f.write_char(if self.status.zero { 'Z' } else { '.' })?;
        f.write_char(if self.status.carry { 'C' } else { '.' })?;
        write!(
            f,
            " V:{:03} H:{:04} F:{:02}",
            self.clock.v, self.clock.h_counter, self.clock.f,
        )?;
        Ok(())
    }
}

impl FromStr for CpuState {
    type Err = anyhow::Error;

    /// Parse a BSNES trace line into a Trace object
    fn from_str(s: &str) -> Result<Self> {
        // The trace format has a fixed width, which allows us to use direct indexing to parse
        // instead of much slower regex or nom parsing.
        //
        // Example with indices of the start of each item:
        //
        // 00e811 bpl $e80e      [00e80e] A:9901 X:0100 Y:0000 S:1ff3 D:0000 DB:00 .VM..IZC V:261 H:236 F:32
        // 0      7   11          23        33     40     47     54     61      69 72         83    89    95

        // BSNES can output h in clocks instead of pixels. This will require an additional character
        // for H: and shifts F: by one index.
        if s[94..=95].trim() != "F:" {
            bail!("Trace format using h lines instead of h dots.");
        }
        let operand_str = s[11..21].trim().to_string();
        Ok(CpuState {
            instruction: InstructionMeta {
                address: u32::from_str_radix(&s[0..6], 16)
                    .with_context(|| "pc")?
                    .into(),
                operation: s[7..10].trim().to_string(),
                operand_str: if operand_str.is_empty() {
                    None
                } else {
                    Some(operand_str)
                },
                effective_addr: {
                    let addr = s[23..29].trim();
                    if addr.is_empty() {
                        None
                    } else {
                        Some(
                            u32::from_str_radix(addr, 16)
                                .with_context(|| "operand_addr")?
                                .into(),
                        )
                    }
                },
            },
            a: u16::from_str_radix(&s[33..37], 16).with_context(|| "a")?,
            x: u16::from_str_radix(&s[40..44], 16).with_context(|| "x")?,
            y: u16::from_str_radix(&s[47..51], 16).with_context(|| "y")?,
            s: u16::from_str_radix(&s[54..58], 16).with_context(|| "s")?,
            d: u16::from_str_radix(&s[61..65], 16).with_context(|| "d")?,
            db: u8::from_str_radix(&s[69..71], 16).with_context(|| "db")?,
            status: s[72..80].trim().parse().with_context(|| "status")?,
            clock: ClockInfo::from_vhf(
                u64::from_str(s[83..86].trim()).with_context(|| "v")?,
                u64::from_str(s[89..94].trim()).with_context(|| "h")?,
                u64::from_str(s[96..].trim()).with_context(|| "f")?,
            ),
        })
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
            negative: chars.next().unwrap() != '.',
            overflow: chars.next().unwrap() != '.',
            accumulator_register_size: chars.next().unwrap() != '.',
            index_register_size_or_break: chars.next().unwrap() != '.',
            decimal: chars.next().unwrap() != '.',
            irq_disable: chars.next().unwrap() != '.',
            zero: chars.next().unwrap() != '.',
            carry: chars.next().unwrap() != '.',
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    static EXAMPLE_BSNES_TRACE: &str = r"00e811 bpl $e80e      [00e80e] A:9901 X:0100 Y:0000 S:1ff3 D:0000 DB:00 .VM..IZC V:261 H:0236 F:32";

    fn example_trace() -> CpuState {
        CpuState {
            instruction: InstructionMeta {
                address: AddressU24 {
                    bank: 0,
                    offset: 0xe811,
                },
                operation: "bpl".to_string(),
                operand_str: Some("$e80e".to_string()),
                effective_addr: Some(AddressU24 {
                    bank: 0,
                    offset: 0xe80e,
                }),
            },
            a: 0x9901,
            x: 0x0100,
            y: 0x0000,
            s: 0x1ff3,
            d: 0x0000,
            db: 0x00,
            status: ".VM..IZC".parse().unwrap(),
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
            EXAMPLE_BSNES_TRACE.parse::<CpuState>().unwrap(),
            example_trace()
        );
    }

    #[test]
    pub fn test_to_str() {
        assert_eq!(format!("{}", example_trace()), EXAMPLE_BSNES_TRACE);
    }
}
