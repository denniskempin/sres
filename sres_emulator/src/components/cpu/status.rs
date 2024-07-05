//! Type for status register and boilerplate for conversion and display
use packed_struct::prelude::*;

#[derive(PackedStruct, Clone, Debug, Copy, PartialEq, Eq)]
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
