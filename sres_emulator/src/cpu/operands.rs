use super::Cpu;
use super::UInt;
use crate::bus::Bus;
use crate::memory::Address;
use crate::memory::ToAddress;

#[derive(Clone, Copy)]
pub enum AddressMode {
    Implied,
    ImmediateU8,
    ImmediateA,  // Immediate value based on accumulator register size
    ImmediateXY, // Immediate value based on index register size
    Accumulator,
    Absolute,
    AbsoluteLong,
    AbsoluteXIndexed,
    Relative,
    DirectPage,
}

#[derive(Copy, Clone)]
pub enum Operand {
    Implied,
    Accumulator,
    ImmediateU8(u8),
    ImmediateU16(u16),
    Absolute(Address),
    AbsoluteXIndexed(Address, Address),
    DirectPage(Address, Address),
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
            AddressMode::Implied => (Operand::Implied, instruction_addr + 1),
            AddressMode::Accumulator => (Operand::Accumulator, instruction_addr + 1),
            AddressMode::Absolute => (
                Operand::Absolute(
                    (cpu.bus.peek_u16(instruction_addr + 1).unwrap_or_default() as u32)
                        .to_address(),
                ),
                instruction_addr + 3,
            ),
            AddressMode::AbsoluteXIndexed => {
                let base = cpu.bus.peek_u16(instruction_addr + 1).unwrap_or_default() as u32;
                (
                    Operand::AbsoluteXIndexed(
                        base.to_address(),
                        (base + cpu.x.value as u32).to_address(),
                    ),
                    instruction_addr + 3,
                )
            }
            AddressMode::DirectPage => {
                let offset = cpu.bus.peek(instruction_addr + 1).unwrap_or_default() as u32;
                (
                    Operand::DirectPage(offset.to_address(), (cpu.d as u32 + offset).to_address()),
                    instruction_addr + 2,
                )
            }
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
            Self::Accumulator | Self::ImmediateU8(_) | Self::ImmediateU16(_) | Self::Implied => {
                None
            }
            Self::Absolute(addr)
            | Self::Relative(_, addr)
            | Self::AbsoluteLong(addr)
            | Self::DirectPage(_, addr)
            | Self::AbsoluteXIndexed(_, addr) => Some(*addr),
        }
    }

    #[inline]
    pub fn load<T: UInt>(&self, cpu: &mut Cpu<impl Bus>) -> T {
        match self {
            Self::ImmediateU8(value) => T::from_u8(*value),
            Self::ImmediateU16(value) => T::from_u16(*value),
            Self::Accumulator => cpu.a.get(),
            _ => T::read_from_bus(&mut cpu.bus, self.addr().unwrap()),
        }
    }

    #[inline]
    pub fn store<T: UInt>(&self, cpu: &mut Cpu<impl Bus>, value: T) {
        match self {
            Self::Accumulator => cpu.a.set(value),
            Self::ImmediateU8(_) | Self::ImmediateU16(_) => panic!("writing to immediate operand"),
            _ => value.write_to_bus(&mut cpu.bus, self.addr().unwrap()),
        }
    }

    #[inline]
    pub fn format(&self) -> String {
        match self {
            Self::Implied | Self::Accumulator => "".to_string(),
            Self::ImmediateU8(value) => format!("#${:02x}", value),
            Self::ImmediateU16(value) => format!("#${:04x}", value),
            Self::Absolute(addr) | Self::Relative(_, addr) => {
                format!("${:04x}", u32::from(*addr))
            }
            Self::DirectPage(offset, _) => {
                format!("${:02x}", u32::from(*offset))
            }
            Self::AbsoluteLong(addr) => {
                format!("${:06x}", u32::from(*addr))
            }
            Self::AbsoluteXIndexed(base_addr, _) => {
                format!("${:04x},x", u32::from(*base_addr))
            }
        }
    }
}
