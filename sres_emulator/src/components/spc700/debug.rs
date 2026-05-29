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
            spc_cycle: self.0.bus.spc_cycle(),
            master_cycle: self.0.bus.master_clock(),
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
    // TODO: Replace with Spc700StatusFlags struct
    pub status: String,
    pub spc_cycle: u64,
    pub master_cycle: u64,
}

impl Spc700State {
    // Using the following custom format:
    // [Disassembly][EffectiveAddress] [MemoryValue,h][Align,38] A:[A,2h] X:[X,2h] Y:[Y,2h] S:[SP,2h] P:[P,8] C:[CycleCount]
    pub fn parse_mesen_trace(s: &str) -> Result<Self> {
        // Example:
        //
        // FFC5  MOV (X),A [$00EF] = $71          A:00 X:EF Y:00 S:EF P:nvpbhiZc C:0
        // 0     6   10                           39   44   49   54   59         70
        if &s[39..=40] != "A:" {
            bail!("Invalid trace format.")
        }

        let disassembly: Vec<&str> = s[6..39].split_whitespace().collect();
        let operation = disassembly[0];
        let operand_str = if disassembly.len() > 1 {
            Some(disassembly[1].to_string())
        } else {
            None
        };
        let effective_addr = if disassembly.len() > 2 {
            let effective_addr_str = disassembly[2].trim_matches(['[', ']', '$']);
            if let Ok(addr) = u32::from_str_radix(effective_addr_str, 16) {
                Some(AddressU16(addr as u16))
            } else {
                None
            }
        } else {
            None
        };

        Ok(Self {
            instruction: InstructionMeta {
                address: AddressU16(u16::from_str_radix(&s[0..4], 16).with_context(|| "pc")?),
                operation: operation.to_string(),
                operand_str,
                effective_addr,
            },
            a: u8::from_str_radix(&s[41..43], 16).with_context(|| "a")?,
            x: u8::from_str_radix(&s[46..48], 16).with_context(|| "x")?,
            y: u8::from_str_radix(&s[51..53], 16).with_context(|| "y")?,
            sp: AddressU16(0x0100 + u16::from_str_radix(&s[56..58], 16).with_context(|| "s")?),
            status: s[61..69].to_string(),
            spc_cycle: u64::from_str(s[72..].trim()).with_context(|| "v")?,
            master_cycle: 0,
        })
    }
}

impl Display for Spc700State {
    /// Format a trace object into a BSNES trace line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operation = self.instruction.operation.to_uppercase();
        let operand = self
            .instruction
            .operand_str
            .as_deref()
            .unwrap_or("")
            .to_uppercase();
        write!(
            f,
            "{:08} [{:04X}]  {:<3} {:<29} A:{:02X} X:{:02X} Y:{:02X} S:{:02X} P:{} C:{}",
            self.master_cycle,
            self.instruction.address.0,
            operation,
            operand,
            self.a,
            self.x,
            self.y,
            self.sp.0 as u8,
            self.status,
            self.spc_cycle,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_MESEN_TRACE: &str =
        r"FFC5  MOV (X),A [$00EF] = $71          A:00 X:EF Y:00 S:EF P:nvpbhiZc C:54";

    static EXAMPLE_SPC700_TRACE: &str =
        r"00000088 [FFC5]  MOV (X),A                         A:00 X:EF Y:00 S:EF P:nvpbhiZc C:54";

    fn example_spc700_trace() -> Spc700State {
        Spc700State {
            instruction: InstructionMeta {
                address: AddressU16(0xffC5),
                operation: "MOV".to_string(),
                operand_str: Some("(X),A".to_string()),
                effective_addr: Some(AddressU16::from(0x00EF)),
            },
            a: 0x00,
            x: 0xEF,
            y: 0x00,
            sp: AddressU16(0x01ef),
            status: "nvpbhiZc".to_string(),
            spc_cycle: 54,
            master_cycle: 88,
        }
    }

    #[test]
    pub fn test_parse_mesen_trace() {
        // Mesen does not support master cycle printing.
        let example_trace = Spc700State {
            master_cycle: 0,
            ..example_spc700_trace()
        };
        assert_eq!(
            Spc700State::parse_mesen_trace(&EXAMPLE_MESEN_TRACE).unwrap(),
            example_trace
        );
    }

    #[test]
    pub fn test_spc700_to_str() {
        assert_eq!(format!("{}", example_spc700_trace()), EXAMPLE_SPC700_TRACE);
    }
}
