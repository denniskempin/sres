use super::Cpu;
use crate::bus::Bus;
use crate::memory::{Address, ToAddress};

#[derive(Clone, Copy)]
pub enum AddressMode {
    Immediate,
    Absolute,
}

#[derive(Clone, Copy)]
pub enum Register {
    A,
    X,
    Y,
    Status,
}

#[derive(Clone, Copy)]
pub enum RegisterSize {
    U8,
    U16,
}

impl From<bool> for RegisterSize {
    fn from(b: bool) -> Self {
        if b {
            RegisterSize::U8
        } else {
            RegisterSize::U16
        }
    }
}

#[derive(Clone)]
pub struct Operand {
    pub value: u32,
    pub mode: AddressMode,
    pub register_size: RegisterSize,
}

fn get_register_size(cpu: &Cpu<impl Bus>, register: Register) -> RegisterSize {
    match register {
        Register::A => RegisterSize::from(cpu.status.accumulator_register_size),
        Register::X | Register::Y => RegisterSize::from(cpu.status.index_register_size_or_break),
        Register::Status => RegisterSize::U8,
    }
}

impl Operand {
    pub fn new(
        cpu: &Cpu<impl Bus>,
        instruction_addr: Address,
        mode: AddressMode,
        register: Register,
    ) -> (Self, Address) {
        let register_size = get_register_size(cpu, register);
        let operand_size = match mode {
            AddressMode::Immediate => register_size,
            AddressMode::Absolute => RegisterSize::U16,
        };
        let value = match operand_size {
            RegisterSize::U8 => cpu.bus.peek(instruction_addr + 1).unwrap_or_default() as u32,
            RegisterSize::U16 => u16::from_le_bytes([
                cpu.bus.peek(instruction_addr + 1).unwrap_or_default(),
                cpu.bus.peek(instruction_addr + 2).unwrap_or_default(),
            ]) as u32,
        };
        let instruction_size = match operand_size {
            RegisterSize::U8 => 2,
            RegisterSize::U16 => 3,
        };
        (
            Self {
                value,
                mode,
                register_size,
            },
            instruction_addr + instruction_size,
        )
    }

    pub fn load(&self, cpu: &mut Cpu<impl Bus>) -> u16 {
        match self.mode {
            AddressMode::Immediate => self.value as u16,
            AddressMode::Absolute => {
                let addr = self.addr().unwrap();
                match self.register_size {
                    RegisterSize::U8 => cpu.bus.read(addr) as u16,
                    RegisterSize::U16 => {
                        u16::from_le_bytes([cpu.bus.read(addr), cpu.bus.read(addr + 1)])
                    }
                }
            }
        }
    }

    pub fn addr(&self) -> Option<Address> {
        match self.mode {
            AddressMode::Immediate => None,
            AddressMode::Absolute => Some(self.value.to_address()),
        }
    }

    pub fn store(&self, cpu: &mut Cpu<impl Bus>, value: u16) {
        match self.mode {
            AddressMode::Immediate => (),
            AddressMode::Absolute => {
                let addr = self.addr().unwrap();
                match self.register_size {
                    RegisterSize::U8 => cpu.bus.write(addr, value as u8),
                    RegisterSize::U16 => {
                        let bytes = value.to_le_bytes();
                        cpu.bus.write(addr, bytes[0]);
                        cpu.bus.write(addr + 1, bytes[1]);
                    }
                }
            }
        }
    }

    pub fn get_meta(&self) -> (String, Option<Address>) {
        match self.mode {
            AddressMode::Immediate => match self.register_size {
                RegisterSize::U8 => (format!("#${:02x}", self.value), None),
                RegisterSize::U16 => (format!("#${:04x}", self.value), None),
            },
            AddressMode::Absolute => (format!("${:04x}", self.value), self.addr()),
        }
    }
}
