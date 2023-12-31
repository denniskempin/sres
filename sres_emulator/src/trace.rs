//! Implements the trace log format used by BSNES to compare the emulator to BSNES.
//!
//! Also a useful, compact format for debugging emulator execution.
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;

use crate::bus::AddressU24;
use crate::cpu::Cpu;
use crate::cpu::StatusFlags;
use crate::main_bus::MainBus;
use crate::main_bus::MainBusImpl;

/// Represents a snapshot of the current state of the system.
/// Can be formatted and parsed in the BSNES trace format to allow comparison to BSNES.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TraceLine {
    pub pc: AddressU24,
    pub instruction: String,
    pub operand: String,
    pub operand_addr: Option<AddressU24>,
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub s: u16,
    pub d: u16,
    pub db: u8,
    pub status: StatusFlags,
    pub v: u64,
    pub h: u64,
    pub f: u64,
}

impl Display for TraceLine {
    /// Format a trace object into a BSNES trace line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:06x} {} {:<10} {:8} A:{:04x} X:{:04x} Y:{:04x} S:{:04x} D:{:04x} DB:{:02x} {} V:{:03} H:{:03} F:{:02}",
            u32::from(self.pc),
            self.instruction,
            self.operand,
            if let Some(addr) = &self.operand_addr {
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
            &String::from(self.status),
            self.v,
            self.h,
            self.f,
        )
    }
}

impl FromStr for TraceLine {
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
        let is_hcounter = s[94..=95].trim() == "F:";
        Ok(TraceLine {
            pc: u32::from_str_radix(&s[0..6], 16)
                .with_context(|| "pc")?
                .into(),
            instruction: s[7..10].trim().to_string(),
            operand: s[11..21].trim().to_string(),
            operand_addr: {
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
            a: u16::from_str_radix(&s[33..37], 16).with_context(|| "a")?,
            x: u16::from_str_radix(&s[40..44], 16).with_context(|| "x")?,
            y: u16::from_str_radix(&s[47..51], 16).with_context(|| "y")?,
            s: u16::from_str_radix(&s[54..58], 16).with_context(|| "s")?,
            d: u16::from_str_radix(&s[61..65], 16).with_context(|| "d")?,
            db: u8::from_str_radix(&s[69..71], 16).with_context(|| "db")?,
            status: s[72..80].trim().parse().with_context(|| "status")?,
            v: u64::from_str(s[83..86].trim()).with_context(|| "v")?,
            h: u64::from_str(s[89..(if is_hcounter { 94 } else { 93 })].trim())
                .with_context(|| "h")?,
            f: u64::from_str(s[(if is_hcounter { 96 } else { 95 })..].trim())
                .with_context(|| "f")?,
        })
    }
}

impl TraceLine {
    pub fn from_file(path: &Path) -> Result<impl Iterator<Item = Result<Self>>> {
        let trace_reader = io::BufReader::new(File::open(path)?);
        Ok(trace_reader.lines().map(|l| l?.parse()))
    }

    pub fn from_sres_cpu(cpu: &Cpu<MainBusImpl>) -> Self {
        let (instruction, _) = cpu.load_instruction_meta(cpu.pc);
        let ppu_timer = cpu.bus.ppu.timer;
        TraceLine {
            pc: cpu.pc,
            instruction: instruction.operation.to_string(),
            operand: instruction.operand_str.unwrap_or_default(),
            operand_addr: instruction.effective_addr,
            a: cpu.a.value,
            x: cpu.x.value,
            y: cpu.y.value,
            s: cpu.s,
            d: cpu.d,
            db: cpu.db,
            status: cpu.status,
            v: ppu_timer.v,
            h: ppu_timer.h_counter,
            f: ppu_timer.f,
        }
    }

    pub fn from_cpu(cpu: &Cpu<impl MainBus>) -> Self {
        let (instruction, _) = cpu.load_instruction_meta(cpu.pc);
        TraceLine {
            pc: cpu.pc,
            instruction: instruction.operation.to_string(),
            operand: instruction.operand_str.unwrap_or_default(),
            operand_addr: instruction.effective_addr,
            a: cpu.a.value,
            x: cpu.x.value,
            y: cpu.y.value,
            s: cpu.s,
            d: cpu.d,
            db: cpu.db,
            status: cpu.status,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    static EXAMPLE_BSNES_TRACE: &str = r"00e811 bpl $e80e      [00e80e] A:9901 X:0100 Y:0000 S:1ff3 D:0000 DB:00 .VM..IZC V:261 H:236 F:32";

    fn example_trace() -> TraceLine {
        TraceLine {
            pc: AddressU24 {
                bank: 0,
                offset: 0xe811,
            },
            instruction: "bpl".to_string(),
            operand: "$e80e".to_string(),
            operand_addr: Some(AddressU24 {
                bank: 0,
                offset: 0xe80e,
            }),
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
            v: 261,
            h: 236,
            f: 32,
        }
    }

    #[test]
    pub fn test_from_str() {
        assert_eq!(
            EXAMPLE_BSNES_TRACE.parse::<TraceLine>().unwrap(),
            example_trace()
        );
    }

    #[test]
    pub fn test_to_str() {
        assert_eq!(format!("{}", example_trace()), EXAMPLE_BSNES_TRACE);
    }
}
