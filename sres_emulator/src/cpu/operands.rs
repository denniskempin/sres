use crate::bus::Bus;
use crate::memory::Address;

pub trait OperandU8 {
    const SIZE: usize = 0;

    fn new<BusT: Bus>(bus: &BusT, addr: Address) -> Self;
    fn format(&self) -> Option<String>;
    fn addr(&self) -> Option<Address>;
    fn load(&self) -> u8;
}

pub struct OperandU8Immediate {
    pub value: u8,
}

impl OperandU8 for OperandU8Immediate {
    const SIZE: usize = 1;

    fn new<BusT: Bus>(bus: &BusT, addr: Address) -> Self {
        Self {
            value: bus.peek(addr).unwrap_or_default(),
        }
    }

    fn format(&self) -> Option<String> {
        Some(format!("#${:02X}", self.value))
    }

    fn load(&self) -> u8 {
        self.value
    }

    fn addr(&self) -> Option<Address> {
        None
    }
}
