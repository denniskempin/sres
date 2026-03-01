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
            if self.negative { "N" } else { "n" },
            if self.overflow { "V" } else { "v" },
            if self.direct_page { "P" } else { "p" },
            if self.break_command { "B" } else { "b" },
            if self.half_carry { "H" } else { "h" },
            if self.irq_enable { "I" } else { "i" },
            if self.zero { "Z" } else { "z" },
            if self.carry { "C" } else { "c" },
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
            negative: chars.next().unwrap() == 'N',
            overflow: chars.next().unwrap() == 'V',
            direct_page: chars.next().unwrap() == 'P',
            break_command: chars.next().unwrap() == 'B',
            half_carry: chars.next().unwrap() == 'H',
            irq_enable: chars.next().unwrap() == 'I',
            zero: chars.next().unwrap() == 'Z',
            carry: chars.next().unwrap() == 'C',
        })
    }
}
