use std::fmt::Display;
use std::str::FromStr;

use anyhow::bail;
use intbits::Bits;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub struct Spc700StatusFlags {
    pub carry: bool,
    pub zero: bool,
    pub irq_enable: bool,
    pub half_carry: bool,
    pub break_command: bool,
    pub direct_page: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl From<u8> for Spc700StatusFlags {
    fn from(value: u8) -> Self {
        Self {
            carry: value.bit(0),
            zero: value.bit(1),
            irq_enable: value.bit(2),
            half_carry: value.bit(3),
            break_command: value.bit(4),
            direct_page: value.bit(5),
            overflow: value.bit(6),
            negative: value.bit(7),
        }
    }
}

impl From<Spc700StatusFlags> for u8 {
    fn from(value: Spc700StatusFlags) -> Self {
        0_u8.with_bit(0, value.carry)
            .with_bit(1, value.zero)
            .with_bit(2, value.irq_enable)
            .with_bit(3, value.half_carry)
            .with_bit(4, value.break_command)
            .with_bit(5, value.direct_page)
            .with_bit(6, value.overflow)
            .with_bit(7, value.negative)
    }
}

impl Display for Spc700StatusFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            if self.negative { "N" } else { "." },
            if self.overflow { "V" } else { "." },
            if self.direct_page { "D" } else { "." },
            if self.break_command { "B" } else { "." },
            if self.half_carry { "H" } else { "." },
            if self.irq_enable { "I" } else { "." },
            if self.zero { "Z" } else { "." },
            if self.carry { "C" } else { "." },
        )
    }
}

impl FromStr for Spc700StatusFlags {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            bail!("StatusFlags string must be 8 characters long");
        }
        let mut chars = s.chars();
        Ok(Self {
            negative: chars.next().unwrap() != '.',
            overflow: chars.next().unwrap() != '.',
            direct_page: chars.next().unwrap() != '.',
            break_command: chars.next().unwrap() != '.',
            half_carry: chars.next().unwrap() != '.',
            irq_enable: chars.next().unwrap() != '.',
            zero: chars.next().unwrap() != '.',
            carry: chars.next().unwrap() != '.',
        })
    }
}
