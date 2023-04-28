use super::Cpu;
use crate::bus::Bus;
use crate::memory::Address;
use crate::memory::ToAddress;

#[derive(Clone, Copy)]
pub enum AddressMode {
    Immediate,
    Absolute,
    Relative,
}

#[derive(Clone, Copy)]
pub enum Register {
    A,
    X,
    FixedU8,
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

#[derive(Copy, Clone)]
pub enum Operand {
    ImmediateU8(u8),
    ImmediateU16(u16),
    Absolute(Address),
    Relative(i8, Address),
}

fn get_register_size(cpu: &Cpu<impl Bus>, register: Register) -> RegisterSize {
    match register {
        Register::A => RegisterSize::from(cpu.status.accumulator_register_size),
        Register::X => RegisterSize::from(cpu.status.index_register_size_or_break),
        Register::FixedU8 => RegisterSize::U8,
    }
}

impl Operand {
    #[inline]
    pub fn decode(
        cpu: &Cpu<impl Bus>,
        instruction_addr: Address,
        mode: AddressMode,
        register: Register,
    ) -> (Self, Address) {
        match mode {
            AddressMode::Immediate => match get_register_size(cpu, register) {
                RegisterSize::U8 => (
                    Operand::ImmediateU8(cpu.bus.peek(instruction_addr + 1).unwrap_or_default()),
                    instruction_addr + 2,
                ),

                RegisterSize::U16 => (
                    Operand::ImmediateU16(
                        cpu.bus.peek_u16(instruction_addr + 1).unwrap_or_default(),
                    ),
                    instruction_addr + 3,
                ),
            },
            AddressMode::Absolute => (
                Operand::Absolute(
                    (cpu.bus.peek_u16(instruction_addr + 1).unwrap_or_default() as u32)
                        .to_address(),
                ),
                instruction_addr + 3,
            ),
            AddressMode::Relative => {
                let relative_addr = cpu.bus.peek(instruction_addr + 1).unwrap_or_default() as i8;
                let operand_addr = if relative_addr > 0 {
                    u32::from(cpu.pc + 2).wrapping_add(relative_addr.unsigned_abs() as u32)
                } else {
                    u32::from(cpu.pc + 2).wrapping_sub(relative_addr.unsigned_abs() as u32)
                };
                (
                    Operand::Relative(relative_addr, operand_addr.to_address()),
                    instruction_addr + 2,
                )
            }
        }
    }

    #[inline]
    pub fn addr(&self) -> Option<Address> {
        match self {
            Self::ImmediateU8(_) => None,
            Self::ImmediateU16(_) => None,
            Self::Absolute(addr) | Self::Relative(_, addr) => Some(*addr),
        }
    }

    #[inline]
    pub fn load(&self, cpu: &mut Cpu<impl Bus>) -> u8 {
        match self {
            Self::ImmediateU8(value) => *value,
            Self::ImmediateU16(_) => panic!("loading u8 from u16 operand"),
            Self::Absolute(addr) | Self::Relative(_, addr) => cpu.bus.read(*addr),
        }
    }

    #[inline]
    pub fn load_u16(&self, cpu: &mut Cpu<impl Bus>) -> u16 {
        match self {
            Self::ImmediateU8(_) => panic!("loading u16 from u8 operand"),
            Self::ImmediateU16(value) => *value,
            Self::Absolute(addr) | Self::Relative(_, addr) => cpu.bus.read_u16(*addr),
        }
    }

    #[inline]
    pub fn store(&self, cpu: &mut Cpu<impl Bus>, value: u8) {
        match self {
            Self::ImmediateU8(_) => panic!("writing to immediate operand"),
            Self::ImmediateU16(_) => panic!("writing to immediate operand"),
            Self::Absolute(addr) | Self::Relative(_, addr) => cpu.bus.write(*addr, value),
        }
    }

    #[inline]
    pub fn format(&self) -> String {
        match self {
            Self::ImmediateU8(value) => format!("#${:02x}", value),
            Self::ImmediateU16(value) => format!("#${:04x}", value),
            Self::Absolute(addr) | Self::Relative(_, addr) => {
                format!("${:04x}", u32::from(*addr))
            }
        }
    }
}
