use anyhow::Context;
use anyhow::Result;
use std::fmt::Display;
use std::str::FromStr;

use crate::common::address::AddressU16;

use super::Spc700;
use super::Spc700Bus;

// Representation of the state of [Spc700] in the same format as logged by BSNES.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Spc700TraceLine {
    pub pc: AddressU16,
    pub instruction: String,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: AddressU16,
    pub status: String,
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
            status: cpu.status.to_string(),
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
            status: s[62..70].to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

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
            status: "N.....ZC".to_string(),
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
