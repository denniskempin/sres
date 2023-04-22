use super::Cpu;
use crate::bus::Bus;
use crate::memory::Address;

pub trait Operand {
    fn new(cpu: &Cpu<impl Bus>, addr: Address) -> (Address, Self);
    fn format(&self, cpu: &Cpu<impl Bus>) -> Option<String>;
    fn addr(&self) -> Option<Address>;
    fn load(&self) -> u16;
}

pub struct ImmediateOperand {
    pub value: u8,
}

impl Operand for ImmediateOperand {
    fn new(cpu: &Cpu<impl Bus>, addr: Address) -> (Address, Self) {
        (
            addr + 1,
            Self {
                value: cpu.bus.peek(addr).unwrap_or_default(),
            },
        )
    }

    fn format(&self, _: &Cpu<impl Bus>) -> Option<String> {
        Some(format!("#${:02x}", self.value))
    }

    fn addr(&self) -> Option<Address> {
        None
    }
    fn load(&self) -> u16 {
        self.value as u16
    }
}

pub struct ImmediateOperandU16 {
    pub value: u16,
}

impl Operand for ImmediateOperandU16 {
    fn new(cpu: &Cpu<impl Bus>, addr: Address) -> (Address, Self) {
        (
            addr + 2,
            Self {
                value: u16::from_le_bytes([
                    cpu.bus.peek(addr).unwrap_or_default(),
                    cpu.bus.peek(addr + 1).unwrap_or_default(),
                ]),
            },
        )
    }

    fn format(&self, _: &Cpu<impl Bus>) -> Option<String> {
        Some(format!("#${:04x}", self.value))
    }

    fn addr(&self) -> Option<Address> {
        None
    }
    fn load(&self) -> u16 {
        self.value
    }
}
pub struct ImmediateOperandA {
    pub value: u16,
}

impl Operand for ImmediateOperandA {
    fn new(cpu: &Cpu<impl Bus>, addr: Address) -> (Address, Self) {
        if cpu.status.accumulator_register_size {
            (
                addr + 1,
                Self {
                    value: cpu.bus.peek(addr).unwrap_or_default() as u16,
                },
            )
        } else {
            (
                addr + 2,
                Self {
                    value: u16::from_le_bytes([
                        cpu.bus.peek(addr).unwrap_or_default(),
                        cpu.bus.peek(addr + 1).unwrap_or_default(),
                    ]),
                },
            )
        }
    }

    fn format(&self, cpu: &Cpu<impl Bus>) -> Option<String> {
        if !cpu.status.accumulator_register_size {
            Some(format!("#${:04x}", self.value))
        } else {
            Some(format!("#${:02x}", self.value))
        }
    }

    fn load(&self) -> u16 {
        self.value
    }

    fn addr(&self) -> Option<Address> {
        None
    }
}
