use std::fmt::Display;
use std::str::FromStr;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;

use super::Spc700;
use super::Spc700Bus;
use crate::common::address::AddressU16;
use crate::common::address::InstructionMeta;

#[derive(Debug, Clone, PartialEq)]
pub enum Spc700Event {
    Step(Spc700State),
}

pub struct Spc700Debug<'a, BusT: Spc700Bus>(pub &'a Spc700<BusT>);

impl<BusT: Spc700Bus> Spc700Debug<'_, BusT> {
    pub fn state(&self) -> Spc700State {
        Spc700State {
            instruction: self.disassembly(self.0.pc).0,
            a: self.0.a,
            x: self.0.x,
            y: self.0.y,
            sp: AddressU16(0x0100 + self.0.sp as u16),
            status: self.0.status.to_string(),
        }
    }

    pub fn disassembly(&self, addr: AddressU16) -> (InstructionMeta<AddressU16>, AddressU16) {
        let opcode = self.0.bus.peek_u8(addr).unwrap_or_default();
        let instruction = &self.0.opcode_table[opcode as usize];
        (instruction.disassembly)(self.0, addr)
    }
}

/// The Spc700State formats into a string compatible with BSNES traces.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Spc700State {
    pub instruction: InstructionMeta<AddressU16>,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: AddressU16,
    pub status: String,
}

impl FromStr for Spc700State {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // Example:
        //
        // ..ffe2 mov   ($000)+y, a       A:8f X:cc Y:f9 SP:01ef YA:f98f N......C
        // 0      7     13                  33   38   43    49      57   62
        if &s[31..=32] != "A:" {
            bail!("Invalid trace format.")
        }
        Ok(Self {
            instruction: InstructionMeta {
                address: AddressU16(u16::from_str_radix(&s[2..6], 16).with_context(|| "pc")?),
                operation: s[7..13].trim().to_string(),
                operand_str: Some(s[13..30].trim().to_string()),
                effective_addr_and_value: None,
            },
            a: u8::from_str_radix(&s[33..35], 16).with_context(|| "a")?,
            x: u8::from_str_radix(&s[38..40], 16).with_context(|| "x")?,
            y: u8::from_str_radix(&s[43..45], 16).with_context(|| "y")?,
            sp: AddressU16(u16::from_str_radix(&s[49..53], 16).with_context(|| "y")?),
            status: s[62..70].to_string(),
        })
    }
}

impl Display for Spc700State {
    /// Format a trace object into a BSNES trace line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "..{:04x} {:<5} {:<17} A:{:02x} X:{:02x} Y:{:02x} SP:{:04x} YA:{:02x}{:02x} {}",
            self.instruction.address.0,
            self.instruction.operation,
            self.instruction.operand_str.as_deref().unwrap_or(""),
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

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_SPC700_TRACE: &str =
        r"..ffe2 mov   ($000)+y, a       A:8f X:cc Y:f9 SP:01ef YA:f98f N.....ZC";

    fn example_spc700_trace() -> Spc700State {
        Spc700State {
            instruction: InstructionMeta {
                address: AddressU16(0xffe2),
                operation: "mov".to_string(),
                operand_str: Some("($000)+y, a".to_string()),
                effective_addr_and_value: None,
            },
            a: 0x8f,
            x: 0xcc,
            y: 0xf9,
            sp: AddressU16(0x01ef),
            status: "N.....ZC".to_string(),
        }
    }

    #[test]
    pub fn test_spc700_from_str() {
        assert_eq!(
            EXAMPLE_SPC700_TRACE.parse::<Spc700State>().unwrap(),
            example_spc700_trace()
        );
    }

    #[test]
    pub fn test_spc700_to_str() {
        assert_eq!(format!("{}", example_spc700_trace()), EXAMPLE_SPC700_TRACE);
    }
}
