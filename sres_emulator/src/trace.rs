use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

use anyhow::Context;
use anyhow::Result;

use crate::cpu::status::StatusFlags;
use crate::memory::Address;
use crate::memory::ToAddress;

/// Represents a snapshot of the current state of the system.
/// Can be formatted and parsed in the BSNES trace format to allow comparison to BSNES.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Trace {
    pub pc: Address,
    pub instruction: String,
    pub operand: String,
    pub operand_addr: Option<Address>,
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub s: u16,
    pub d: u16,
    pub db: u8,
    pub status: StatusFlags,
}

impl Display for Trace {
    /// Format a trace object into a BSNES trace line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:06x} {} {:<10} {:8} A:{:04x} X:{:04x} Y:{:04x} S:{:04x} D:{:04x} DB:{:02x} {}",
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
        )
    }
}

impl FromStr for Trace {
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

        Ok(Trace {
            pc: u32::from_str_radix(&s[0..6], 16)
                .with_context(|| "pc")?
                .to_address(),
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
                            .to_address(),
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
        })
    }
}

impl Trace {
    pub fn from_file(path: &Path) -> Result<impl Iterator<Item = Result<Self>>> {
        let trace_reader = io::BufReader::new(File::open(path)?);
        Ok(trace_reader.lines().map(|l| l?.parse()))
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    static EXAMPLE_BSNES_TRACE: &str = r"00e811 bpl $e80e      [00e80e] A:9901 X:0100 Y:0000 S:1ff3 D:0000 DB:00 .VM..IZC V:261 H:236 F:32";

    fn example_trace() -> Trace {
        Trace {
            pc: Address {
                bank: 0,
                offset: 0xe811,
            },
            instruction: "bpl".to_string(),
            operand: "$e80e".to_string(),
            operand_addr: Some(Address {
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
        }
    }

    #[test]
    pub fn test_from_str() {
        assert_eq!(
            EXAMPLE_BSNES_TRACE.parse::<Trace>().unwrap(),
            example_trace()
        );
    }

    #[test]
    pub fn test_to_str() {
        assert_eq!(format!("{}", example_trace()), EXAMPLE_BSNES_TRACE[0..80]);
    }
}
