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
    AbsoluteXIndexedLong,
    Relative,
    DirectPage,
    DirectPageIndirect,
    DirectPageIndirectLong,
}

#[derive(Copy, Clone)]
pub enum Operand {
    Implied,
    Accumulator,
    ImmediateU8(u8),
    ImmediateU16(u16),
    Address(u32, AddressMode, Address),
}

impl Operand {
    #[inline]
    pub fn decode(
        cpu: &Cpu<impl Bus>,
        instruction_addr: Address,
        mode: AddressMode,
    ) -> (Self, Address) {
        // The size of the operand part of the instruction depends on the address mode.
        let operand_size = match mode {
            AddressMode::Implied => 0,
            AddressMode::Accumulator => 0,
            AddressMode::ImmediateU8 => 1,
            AddressMode::ImmediateA => {
                if cpu.status.accumulator_register_size {
                    1
                } else {
                    2
                }
            }
            AddressMode::ImmediateXY => {
                if cpu.status.index_register_size_or_break {
                    1
                } else {
                    2
                }
            }
            AddressMode::Absolute => 2,
            AddressMode::AbsoluteLong => 3,
            AddressMode::AbsoluteXIndexed => 2,
            AddressMode::AbsoluteXIndexedLong => 3,
            AddressMode::Relative => 1,
            AddressMode::DirectPage => 1,
            AddressMode::DirectPageIndirect => 1,
            AddressMode::DirectPageIndirectLong => 1,
        };

        // Regardless of how many bytes were read, store them all as u32 for simplicity.
        let operand_data: u32 = match operand_size {
            0 => 0,
            1 => cpu.bus.peek(instruction_addr + 1).unwrap_or_default() as u32,
            2 => cpu.bus.peek_u16(instruction_addr + 1).unwrap_or_default() as u32,
            3 => cpu.bus.peek_u24(instruction_addr + 1).unwrap_or_default(),
            _ => unreachable!(),
        };

        // Interpret the address mode to figure out where the operand is located.
        let operand = match mode {
            AddressMode::Implied => Operand::Implied,
            AddressMode::Accumulator => Operand::Accumulator,
            AddressMode::ImmediateU8 => Operand::ImmediateU8(operand_data as u8),
            AddressMode::ImmediateA => {
                if cpu.status.accumulator_register_size {
                    Operand::ImmediateU8(operand_data as u8)
                } else {
                    Operand::ImmediateU16(operand_data as u16)
                }
            }
            AddressMode::ImmediateXY => {
                if cpu.status.index_register_size_or_break {
                    Operand::ImmediateU8(operand_data as u8)
                } else {
                    Operand::ImmediateU16(operand_data as u16)
                }
            }
            AddressMode::Absolute | AddressMode::AbsoluteLong => {
                Operand::Address(operand_data, mode, operand_data.to_address())
            }
            AddressMode::AbsoluteXIndexed | AddressMode::AbsoluteXIndexedLong => Operand::Address(
                operand_data,
                mode,
                (operand_data + cpu.x.value as u32).to_address(),
            ),
            AddressMode::Relative => {
                let relative_addr = operand_data as i8;
                let operand_addr = if relative_addr > 0 {
                    u32::from(cpu.pc + 2).wrapping_add(relative_addr.unsigned_abs() as u32)
                } else {
                    u32::from(cpu.pc + 2).wrapping_sub(relative_addr.unsigned_abs() as u32)
                };
                Operand::Address(operand_data, mode, operand_addr.to_address())
            }
            AddressMode::DirectPage => Operand::Address(
                operand_data,
                mode,
                (cpu.d as u32 + operand_data).to_address(),
            ),
            AddressMode::DirectPageIndirect => {
                let indirect_addr = cpu
                    .bus
                    .peek_u16(cpu.d as u32 + operand_data)
                    .unwrap_or_default() as u32;
                Operand::Address(operand_data, mode, indirect_addr.to_address())
            }
            AddressMode::DirectPageIndirectLong => {
                let indirect_addr = cpu
                    .bus
                    .peek_u24(cpu.d as u32 + operand_data)
                    .unwrap_or_default();
                Operand::Address(operand_data, mode, indirect_addr.to_address())
            }
        };
        (operand, instruction_addr + 1 + operand_size)
    }

    #[inline]
    pub fn addr(&self) -> Option<Address> {
        match self {
            Self::Implied | Self::Accumulator | Self::ImmediateU8(_) | Self::ImmediateU16(_) => {
                None
            }
            Self::Address(_, _, addr) => Some(*addr),
        }
    }

    #[inline]
    pub fn load<T: UInt>(&self, cpu: &mut Cpu<impl Bus>) -> T {
        match self {
            Self::Implied => panic!("loading implied operand"),
            Self::ImmediateU8(value) => T::from_u8(*value),
            Self::ImmediateU16(value) => T::from_u16(*value),
            Self::Accumulator => cpu.a.get(),
            _ => T::read_from_bus(&mut cpu.bus, self.addr().unwrap()),
        }
    }

    #[inline]
    pub fn store<T: UInt>(&self, cpu: &mut Cpu<impl Bus>, value: T) {
        match self {
            Self::Implied => panic!("writing to implied operand"),
            Self::ImmediateU8(_) | Self::ImmediateU16(_) => panic!("writing to immediate operand"),
            Self::Accumulator => cpu.a.set(value),
            _ => value.write_to_bus(&mut cpu.bus, self.addr().unwrap()),
        }
    }

    #[inline]
    pub fn format(&self) -> String {
        match self {
            Self::Implied | Self::Accumulator => "".to_string(),
            Self::ImmediateU8(value) => format!("#${:02x}", value),
            Self::ImmediateU16(value) => format!("#${:04x}", value),
            Self::Address(value, mode, operand_addr) => match mode {
                AddressMode::Absolute => format!("${:04x}", value),
                AddressMode::AbsoluteLong => format!("${:06x}", value),
                AddressMode::AbsoluteXIndexed => format!("${:04x},x", value),
                AddressMode::AbsoluteXIndexedLong => format!("${:06x},x", value),
                AddressMode::Relative => format!("${:04x}", u32::from(*operand_addr)),
                AddressMode::DirectPage => format!("${:02x}", value),
                AddressMode::DirectPageIndirect => format!("(${:02x})", value),
                AddressMode::DirectPageIndirectLong => format!("[${:02x}]", value),
                _ => unreachable!(),
            },
        }
    }
}
