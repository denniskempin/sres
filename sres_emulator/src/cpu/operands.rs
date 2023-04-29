use super::Cpu;
use crate::bus::Bus;
use crate::memory::Address;
use crate::memory::ToAddress;

#[derive(Clone, Copy)]
pub enum AddressMode {
    ImmediateU8,
    ImmediateA,  // Immediate value based on accumulator register size
    ImmediateXY, // Immediate value based on index register size
    Absolute,
    AbsoluteLong,
    Relative,
}

#[derive(Copy, Clone)]
pub enum Operand {
    ImmediateU8(u8),
    ImmediateU16(u16),
    Absolute(Address),
    AbsoluteLong(Address),
    Relative(i8, Address),
}

impl Operand {
    #[inline]
    pub fn decode(
        cpu: &Cpu<impl Bus>,
        instruction_addr: Address,
        mode: AddressMode,
    ) -> (Self, Address) {
        match mode {
            AddressMode::Absolute => (
                Operand::Absolute(
                    (cpu.bus.peek_u16(instruction_addr + 1).unwrap_or_default() as u32)
                        .to_address(),
                ),
                instruction_addr + 3,
            ),
            AddressMode::AbsoluteLong => (
                Operand::AbsoluteLong(
                    (cpu.bus.peek_u24(instruction_addr + 1).unwrap_or_default()).to_address(),
                ),
                instruction_addr + 4,
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
            AddressMode::ImmediateU8 => (
                Operand::ImmediateU8(cpu.bus.peek(instruction_addr + 1).unwrap_or_default()),
                instruction_addr + 2,
            ),
            AddressMode::ImmediateA => {
                if cpu.status.accumulator_register_size {
                    (
                        Operand::ImmediateU8(
                            cpu.bus.peek(instruction_addr + 1).unwrap_or_default(),
                        ),
                        instruction_addr + 2,
                    )
                } else {
                    (
                        Operand::ImmediateU16(
                            cpu.bus.peek_u16(instruction_addr + 1).unwrap_or_default(),
                        ),
                        instruction_addr + 3,
                    )
                }
            }
            AddressMode::ImmediateXY => {
                if cpu.status.index_register_size_or_break {
                    (
                        Operand::ImmediateU8(
                            cpu.bus.peek(instruction_addr + 1).unwrap_or_default(),
                        ),
                        instruction_addr + 2,
                    )
                } else {
                    (
                        Operand::ImmediateU16(
                            cpu.bus.peek_u16(instruction_addr + 1).unwrap_or_default(),
                        ),
                        instruction_addr + 3,
                    )
                }
            }
        }
    }

    #[inline]
    pub fn addr(&self) -> Option<Address> {
        match self {
            Self::ImmediateU8(_) => None,
            Self::ImmediateU16(_) => None,
            Self::Absolute(addr) | Self::Relative(_, addr) | Self::AbsoluteLong(addr) => {
                Some(*addr)
            }
        }
    }

    #[inline]
    pub fn load(&self, cpu: &mut Cpu<impl Bus>) -> u8 {
        match self {
            Self::ImmediateU8(value) => *value,
            Self::ImmediateU16(_) => panic!("loading u8 from u16 operand"),
            Self::Absolute(addr) | Self::Relative(_, addr) | Self::AbsoluteLong(addr) => {
                cpu.bus.read(*addr)
            }
        }
    }

    #[inline]
    pub fn load_u16(&self, cpu: &mut Cpu<impl Bus>) -> u16 {
        match self {
            Self::ImmediateU8(_) => panic!("loading u16 from u8 operand"),
            Self::ImmediateU16(value) => *value,
            Self::Absolute(addr) | Self::Relative(_, addr) | Self::AbsoluteLong(addr) => {
                cpu.bus.read_u16(*addr)
            }
        }
    }

    #[inline]
    pub fn store(&self, cpu: &mut Cpu<impl Bus>, value: u8) {
        match self {
            Self::ImmediateU8(_) => panic!("writing to immediate operand"),
            Self::ImmediateU16(_) => panic!("writing to immediate operand"),
            Self::Absolute(addr) | Self::Relative(_, addr) | Self::AbsoluteLong(addr) => {
                cpu.bus.write(*addr, value)
            }
        }
    }

    #[inline]
    pub fn store_u16(&self, cpu: &mut Cpu<impl Bus>, value: u16) {
        match self {
            Self::ImmediateU8(_) => panic!("writing to immediate operand"),
            Self::ImmediateU16(_) => panic!("writing to immediate operand"),
            Self::Absolute(addr) | Self::Relative(_, addr) | Self::AbsoluteLong(addr) => {
                cpu.bus.write_u16(*addr, value)
            }
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
            Self::AbsoluteLong(addr) => {
                format!("${:06x}", u32::from(*addr))
            }
        }
    }
}
