//! Implements the trace log format used by BSNES to compare the emulator to BSNES.
//!
//! Also a useful, compact format for debugging emulator execution.
use std::fmt::Display;
use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;

use crate::bus::AddressU16;
use crate::cpu::Cpu;
use crate::cpu::InstructionMeta;
use crate::cpu::StatusFlags;
use crate::main_bus::MainBus;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::spc700::Spc700StatusFlags;

#[allow(dead_code)]
pub enum TraceLine {
    Cpu(CpuTraceLine),
    Spc700(Spc700TraceLine),
}

impl FromStr for TraceLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with("..") {
            Ok(TraceLine::Spc700(s.parse()?))
        } else {
            Ok(TraceLine::Cpu(s.parse()?))
        }
    }
}

impl Display for TraceLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TraceLine::Cpu(line) => write!(f, "{}", line),
            TraceLine::Spc700(line) => write!(f, "{}", line),
        }
    }
}

// Representation of the state of [Spc700] in the same format as logged by BSNES.
#[derive(Debug, Eq, PartialEq)]
pub struct Spc700TraceLine {
    pub pc: AddressU16,
    pub instruction: String,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: AddressU16,
    pub status: Spc700StatusFlags,
}

impl Spc700TraceLine {
    pub fn from_spc700(cpu: &Spc700<impl Spc700Bus>) -> Self {
        Self {
            pc: cpu.pc,
            instruction: cpu.disassembly(cpu.pc).0,
            a: cpu.a,
            x: cpu.x,
            y: cpu.y,
            sp: AddressU16(0x0100 + cpu.sp as u16),
            status: cpu.status,
        }
    }
}

impl FromStr for Spc700TraceLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // Example:
        //
        // ..ffe2 mov   ($000)+y, a       A:8f X:cc Y:f9 SP:01ef YA:f98f N......C
        // 0      7     13                  33   38   43    49      57   62
        Ok(Self {
            pc: AddressU16(u16::from_str_radix(&s[2..6], 16).with_context(|| "pc")?),
            instruction: s[7..30].trim().to_string(),
            a: u8::from_str_radix(&s[33..35], 16).with_context(|| "a")?,
            x: u8::from_str_radix(&s[38..40], 16).with_context(|| "x")?,
            y: u8::from_str_radix(&s[43..45], 16).with_context(|| "y")?,
            sp: AddressU16(u16::from_str_radix(&s[49..53], 16).with_context(|| "y")?),
            status: s[62..70].trim().parse().with_context(|| "status")?,
        })
    }
}

impl Display for Spc700TraceLine {
    /// Format a trace object into a BSNES trace line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "..{:04x} {:<23} A:{:02x} X:{:02x} Y:{:02x} SP:{:04x} YA:{:02x}{:02x} {}",
            self.pc.0,
            self.instruction,
            self.a,
            self.x,
            self.y,
            self.sp.0,
            self.y,
            self.a,
            self.status,
        )
    }
}

/// Represents a snapshot of the current state of the system.
/// Can be formatted and parsed in the BSNES trace format to allow comparison to BSNES.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CpuTraceLine {
    pub instruction: InstructionMeta,
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

impl Display for CpuTraceLine {
    /// Format a trace object into a BSNES trace line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:06x} {} {:<10} {:8} A:{:04x} X:{:04x} Y:{:04x} S:{:04x} D:{:04x} DB:{:02x} {} V:{:03} H:{:03} F:{:02}",
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
            &String::from(self.status),
            self.v,
            self.h,
            self.f,
        )
    }
}

impl FromStr for CpuTraceLine {
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
        let operand_str = s[11..21].trim().to_string();
        Ok(CpuTraceLine {
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
            v: u64::from_str(s[83..86].trim()).with_context(|| "v")?,
            h: u64::from_str(s[89..(if is_hcounter { 94 } else { 93 })].trim())
                .with_context(|| "h")?,
            f: u64::from_str(s[(if is_hcounter { 96 } else { 95 })..].trim())
                .with_context(|| "f")?,
        })
    }
}

impl CpuTraceLine {
    pub fn from_cpu(cpu: &Cpu<impl MainBus>) -> Self {
        let (instruction, _) = cpu.load_instruction_meta(cpu.pc);
        let ppu_timer = cpu.bus.ppu_timer();
        CpuTraceLine {
            instruction,
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
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::bus::AddressU24;

    static EXAMPLE_BSNES_TRACE: &str = r"00e811 bpl $e80e      [00e80e] A:9901 X:0100 Y:0000 S:1ff3 D:0000 DB:00 .VM..IZC V:261 H:236 F:32";

    fn example_trace() -> CpuTraceLine {
        CpuTraceLine {
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
            EXAMPLE_BSNES_TRACE.parse::<CpuTraceLine>().unwrap(),
            example_trace()
        );
    }

    #[test]
    pub fn test_to_str() {
        assert_eq!(format!("{}", example_trace()), EXAMPLE_BSNES_TRACE);
    }

    static EXAMPLE_SPC700_TRACE: &str =
        r"..ffe2 mov   ($000)+y, a       A:8f X:cc Y:f9 SP:01ef YA:f98f N.....ZC";

    fn example_spc700_trace() -> Spc700TraceLine {
        Spc700TraceLine {
            pc: AddressU16(0xffe2),
            instruction: "mov   ($000)+y, a".to_string(),
            a: 0x8f,
            x: 0xcc,
            y: 0xf9,
            sp: AddressU16(0x01ef),
            status: Spc700StatusFlags {
                carry: true,
                zero: true,
                irq_enable: false,
                half_carry: false,
                break_command: false,
                direct_page: false,
                overflow: false,
                negative: true,
            },
        }
    }

    #[test]
    pub fn test_spc700_from_str() {
        assert_eq!(
            EXAMPLE_SPC700_TRACE.parse::<Spc700TraceLine>().unwrap(),
            example_spc700_trace()
        );
    }

    #[test]
    pub fn test_spc700_to_str() {
        assert_eq!(format!("{}", example_spc700_trace()), EXAMPLE_SPC700_TRACE);
    }
}
