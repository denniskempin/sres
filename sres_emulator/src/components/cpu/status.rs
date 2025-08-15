//! Type for status register and boilerplate for conversion and display
use intbits::Bits;

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
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

impl Default for StatusFlags {
    fn default() -> Self {
        Self {
            negative: false,
            overflow: false,
            accumulator_register_size: true,
            index_register_size_or_break: true,
            decimal: false,
            irq_disable: true,
            zero: false,
            carry: false,
        }
    }
}

impl std::fmt::Display for StatusFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            if self.negative { "N" } else { "n" },
            if self.overflow { "V" } else { "v" },
            if self.accumulator_register_size {
                "M"
            } else {
                "m"
            },
            if self.index_register_size_or_break {
                "X"
            } else {
                "x"
            },
            if self.decimal { "D" } else { "d" },
            if self.irq_disable { "I" } else { "i" },
            if self.zero { "Z" } else { "z" },
            if self.carry { "C" } else { "c" }
        )
    }
}

// Shorthand to convert StatusFlags into and from u8 reqister value
impl From<u8> for StatusFlags {
    fn from(value: u8) -> Self {
        Self {
            negative: value.bit(7),
            overflow: value.bit(6),
            accumulator_register_size: value.bit(5),
            index_register_size_or_break: value.bit(4),
            decimal: value.bit(3),
            irq_disable: value.bit(2),
            zero: value.bit(1),
            carry: value.bit(0),
        }
    }
}

impl From<StatusFlags> for u8 {
    fn from(value: StatusFlags) -> Self {
        0u8.with_bit(7, value.negative)
            .with_bit(6, value.overflow)
            .with_bit(5, value.accumulator_register_size)
            .with_bit(4, value.index_register_size_or_break)
            .with_bit(3, value.decimal)
            .with_bit(2, value.irq_disable)
            .with_bit(1, value.zero)
            .with_bit(0, value.carry)
    }
}
