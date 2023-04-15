use std::str::FromStr;

use anyhow::bail;
use packed_struct::prelude::*;

#[derive(PackedStruct, Clone, Debug, Default, Copy, PartialEq, Eq)]
#[packed_struct(bit_numbering = "msb0")]
pub struct StatusFlags {
    pub negative: bool,
    pub overflow: bool,
    pub accumulator_register_size: bool,
    pub index_register_size_or_break: bool,
    pub decimal: bool,
    pub irq_disable: bool,
    pub zero: bool,
    pub carry: bool,
}

// Shorthand to convert StatusFlags into and from u8 reqister value
impl From<u8> for StatusFlags {
    fn from(value: u8) -> Self {
        StatusFlags::unpack(&[value]).unwrap()
    }
}

impl From<StatusFlags> for u8 {
    fn from(value: StatusFlags) -> Self {
        value.pack().unwrap()[0]
    }
}

// Format the StatusFlags into a readable string
impl From<StatusFlags> for String {
    fn from(value: StatusFlags) -> Self {
        let mut parts: Vec<char> = Vec::with_capacity(8);
        parts.push(if value.negative { 'N' } else { '.' });
        parts.push(if value.overflow { 'V' } else { '.' });
        parts.push(if value.accumulator_register_size {
            'M'
        } else {
            '.'
        });
        parts.push(if value.index_register_size_or_break {
            'X'
        } else {
            '.'
        });
        parts.push(if value.decimal { 'D' } else { '.' });
        parts.push(if value.irq_disable { 'I' } else { '.' });
        parts.push(if value.zero { 'Z' } else { '.' });
        parts.push(if value.carry { 'C' } else { '.' });
        parts.into_iter().collect()
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
            negative: chars.next().unwrap() == 'N',
            overflow: chars.next().unwrap() == 'V',
            accumulator_register_size: chars.next().unwrap() == 'M',
            index_register_size_or_break: chars.next().unwrap() == 'X',
            decimal: chars.next().unwrap() == 'D',
            irq_disable: chars.next().unwrap() == 'I',
            zero: chars.next().unwrap() == 'Z',
            carry: chars.next().unwrap() == 'C',
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_status_flags_representation() {
        for i in 0..=255 {
            let flags = StatusFlags::from(i);
            let parsed: StatusFlags = String::from(flags).parse().unwrap();
            assert_eq!(flags, parsed);
        }
    }
}
