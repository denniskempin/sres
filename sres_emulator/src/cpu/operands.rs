use super::Cpu;
use super::UInt;
use super::STACK_BASE;
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
    AbsoluteYIndexed,
    AbsoluteIndirect,
    AbsoluteIndirectLong,
    AbsoluteXIndexedIndirect,
    StackRelative,
    StackRelativeIndirectYIndexed,
    Relative,
    RelativeLong,
    DirectPage,
    DirectPageXIndexed,
    DirectPageYIndexed,
    DirectPageXIndexedIndirect,
    DirectPageIndirectYIndexed,
    DirectPageIndirectYIndexedLong,
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
        cpu: &mut Cpu<impl Bus>,
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
            AddressMode::AbsoluteYIndexed => 2,
            AddressMode::AbsoluteIndirect => 2,
            AddressMode::AbsoluteIndirectLong => 2,
            AddressMode::AbsoluteXIndexedIndirect => 2,
            AddressMode::StackRelative => 1,
            AddressMode::StackRelativeIndirectYIndexed => 1,
            AddressMode::Relative => 1,
            AddressMode::RelativeLong => 2,
            AddressMode::DirectPage => 1,
            AddressMode::DirectPageXIndexed => 1,
            AddressMode::DirectPageXIndexedIndirect => 1,
            AddressMode::DirectPageYIndexed => 1,
            AddressMode::DirectPageIndirectYIndexed => 1,
            AddressMode::DirectPageIndirectYIndexedLong => 1,
            AddressMode::DirectPageIndirect => 1,
            AddressMode::DirectPageIndirectLong => 1,
        };

        // Regardless of how many bytes were read, store them all as u32 for simplicity.
        let operand_data: u32 = match operand_size {
            0 => 0,
            1 => cpu.bus.read_u8(instruction_addr + 1) as u32,
            2 => cpu.bus.read_u16(instruction_addr + 1) as u32,
            3 => cpu.bus.read_u24(instruction_addr + 1),
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
            // Operand is in memory, calculate the effective address
            _ => {
                let operand_addr: u32 = match mode {
                    AddressMode::Absolute | AddressMode::AbsoluteLong => operand_data,
                    AddressMode::AbsoluteYIndexed => operand_data + cpu.y.value as u32,
                    AddressMode::AbsoluteXIndexed => {
                        cpu.bus.internal_operation_cycle();
                        operand_data + cpu.x.value as u32
                    }
                    AddressMode::AbsoluteXIndexedLong => operand_data + cpu.x.value as u32,
                    AddressMode::AbsoluteIndirect => cpu.bus.read_u16(operand_data) as u32,
                    AddressMode::AbsoluteIndirectLong => cpu.bus.read_u24(operand_data),
                    AddressMode::AbsoluteXIndexedIndirect => {
                        cpu.bus.read_u16(operand_data + cpu.x.value as u32) as u32
                    }
                    AddressMode::Relative => {
                        let relative_addr = operand_data as i8;
                        if relative_addr > 0 {
                            u32::from(cpu.pc + 2).wrapping_add(relative_addr.unsigned_abs() as u32)
                        } else {
                            u32::from(cpu.pc + 2).wrapping_sub(relative_addr.unsigned_abs() as u32)
                        }
                    }
                    AddressMode::RelativeLong => {
                        let relative_addr = operand_data as i16;
                        if relative_addr > 0 {
                            u32::from(cpu.pc + 3).wrapping_add(relative_addr.unsigned_abs() as u32)
                        } else {
                            u32::from(cpu.pc + 3).wrapping_sub(relative_addr.unsigned_abs() as u32)
                        }
                    }
                    AddressMode::StackRelative => operand_data + cpu.s as u32 + STACK_BASE,
                    AddressMode::StackRelativeIndirectYIndexed => {
                        cpu.bus.read_u16(cpu.s as u32 + operand_data) as u32 + cpu.y.value as u32
                    }
                    AddressMode::DirectPage => cpu.d as u32 + operand_data,
                    AddressMode::DirectPageXIndexed => {
                        cpu.d as u32 + operand_data + cpu.x.value as u32
                    }
                    AddressMode::DirectPageYIndexed => {
                        cpu.d as u32 + operand_data + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirect => {
                        cpu.bus.read_u16(cpu.d as u32 + operand_data) as u32
                    }
                    AddressMode::DirectPageXIndexedIndirect => cpu
                        .bus
                        .read_u16(cpu.d as u32 + operand_data + cpu.x.value as u32)
                        as u32,
                    AddressMode::DirectPageIndirectYIndexed => {
                        cpu.bus.read_u16(cpu.d as u32 + operand_data) as u32 + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirectYIndexedLong => {
                        cpu.bus.read_u24(cpu.d as u32 + operand_data) + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirectLong => {
                        cpu.bus.read_u24(cpu.d as u32 + operand_data)
                    }
                    AddressMode::Implied
                    | AddressMode::ImmediateU8
                    | AddressMode::ImmediateA
                    | AddressMode::ImmediateXY
                    | AddressMode::Accumulator => unreachable!(),
                };
                Operand::Address(operand_data, mode, operand_addr.to_address())
            }
        };
        (operand, instruction_addr + 1 + operand_size)
    }

    #[inline]
    pub fn peek(
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
            AddressMode::AbsoluteYIndexed => 2,
            AddressMode::AbsoluteIndirect => 2,
            AddressMode::AbsoluteIndirectLong => 2,
            AddressMode::AbsoluteXIndexedIndirect => 2,
            AddressMode::StackRelative => 1,
            AddressMode::StackRelativeIndirectYIndexed => 1,
            AddressMode::Relative => 1,
            AddressMode::RelativeLong => 2,
            AddressMode::DirectPage => 1,
            AddressMode::DirectPageXIndexed => 1,
            AddressMode::DirectPageXIndexedIndirect => 1,
            AddressMode::DirectPageYIndexed => 1,
            AddressMode::DirectPageIndirectYIndexed => 1,
            AddressMode::DirectPageIndirectYIndexedLong => 1,
            AddressMode::DirectPageIndirect => 1,
            AddressMode::DirectPageIndirectLong => 1,
        };

        // Regardless of how many bytes were read, store them all as u32 for simplicity.
        let operand_data: u32 = match operand_size {
            0 => 0,
            1 => cpu.bus.peek_u8(instruction_addr + 1).unwrap_or_default() as u32,
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
            // Operand is in memory, calculate the effective address
            _ => {
                let operand_addr: u32 = match mode {
                    AddressMode::Absolute | AddressMode::AbsoluteLong => operand_data,
                    AddressMode::AbsoluteYIndexed => operand_data + cpu.y.value as u32,
                    AddressMode::AbsoluteXIndexed | AddressMode::AbsoluteXIndexedLong => {
                        operand_data + cpu.x.value as u32
                    }
                    AddressMode::AbsoluteIndirect => {
                        cpu.bus.peek_u16(operand_data).unwrap_or_default() as u32
                    }
                    AddressMode::AbsoluteIndirectLong => {
                        cpu.bus.peek_u24(operand_data).unwrap_or_default()
                    }
                    AddressMode::AbsoluteXIndexedIndirect => {
                        cpu.bus
                            .peek_u16(operand_data + cpu.x.value as u32)
                            .unwrap_or_default() as u32
                    }
                    AddressMode::Relative => {
                        let relative_addr = operand_data as i8;
                        if relative_addr > 0 {
                            u32::from(cpu.pc + 2).wrapping_add(relative_addr.unsigned_abs() as u32)
                        } else {
                            u32::from(cpu.pc + 2).wrapping_sub(relative_addr.unsigned_abs() as u32)
                        }
                    }
                    AddressMode::RelativeLong => {
                        let relative_addr = operand_data as i16;
                        if relative_addr > 0 {
                            u32::from(cpu.pc + 3).wrapping_add(relative_addr.unsigned_abs() as u32)
                        } else {
                            u32::from(cpu.pc + 3).wrapping_sub(relative_addr.unsigned_abs() as u32)
                        }
                    }
                    AddressMode::StackRelative => operand_data + cpu.s as u32 + STACK_BASE,
                    AddressMode::StackRelativeIndirectYIndexed => {
                        cpu.bus
                            .peek_u16(cpu.s as u32 + operand_data)
                            .unwrap_or_default() as u32
                            + cpu.y.value as u32
                    }
                    AddressMode::DirectPage => cpu.d as u32 + operand_data,
                    AddressMode::DirectPageXIndexed => {
                        cpu.d as u32 + operand_data + cpu.x.value as u32
                    }
                    AddressMode::DirectPageYIndexed => {
                        cpu.d as u32 + operand_data + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirect => {
                        cpu.bus
                            .peek_u16(cpu.d as u32 + operand_data)
                            .unwrap_or_default() as u32
                    }
                    AddressMode::DirectPageXIndexedIndirect => {
                        cpu.bus
                            .peek_u16(cpu.d as u32 + operand_data + cpu.x.value as u32)
                            .unwrap_or_default() as u32
                    }
                    AddressMode::DirectPageIndirectYIndexed => {
                        cpu.bus
                            .peek_u16(cpu.d as u32 + operand_data)
                            .unwrap_or_default() as u32
                            + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirectYIndexedLong => {
                        cpu.bus
                            .peek_u24(cpu.d as u32 + operand_data)
                            .unwrap_or_default()
                            + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirectLong => cpu
                        .bus
                        .peek_u24(cpu.d as u32 + operand_data)
                        .unwrap_or_default(),
                    AddressMode::Implied
                    | AddressMode::ImmediateU8
                    | AddressMode::ImmediateA
                    | AddressMode::ImmediateXY
                    | AddressMode::Accumulator => unreachable!(),
                };
                Operand::Address(operand_data, mode, operand_addr.to_address())
            }
        };
        (operand, instruction_addr + 1 + operand_size)
    }

    #[inline]
    pub fn effective_addr(&self) -> Option<Address> {
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
            _ => cpu.bus.read_generic::<T>(self.effective_addr().unwrap()),
        }
    }

    #[inline]
    pub fn store<T: UInt>(&self, cpu: &mut Cpu<impl Bus>, value: T) {
        match self {
            Self::Implied => panic!("writing to implied operand"),
            Self::ImmediateU8(_) | Self::ImmediateU16(_) => panic!("writing to immediate operand"),
            Self::Accumulator => cpu.a.set(value),
            _ => cpu
                .bus
                .write_generic::<T>(self.effective_addr().unwrap(), value),
        }
    }

    #[inline]
    pub fn format(&self) -> String {
        match self {
            Self::Implied | Self::Accumulator => "".to_string(),
            Self::ImmediateU8(value) => format!("#${:02x}", value),
            Self::ImmediateU16(value) => format!("#${:04x}", value),
            Self::Address(value, mode, _) => match mode {
                AddressMode::Absolute => format!("${:04x}", value),
                AddressMode::AbsoluteLong => format!("${:06x}", value),
                AddressMode::AbsoluteXIndexed => format!("${:04x},x", value),
                AddressMode::AbsoluteXIndexedLong => format!("${:06x},x", value),
                AddressMode::AbsoluteYIndexed => format!("${:04x},y", value),
                AddressMode::AbsoluteIndirect => format!("(${:04x})", value),
                AddressMode::AbsoluteIndirectLong => format!("[${:04x}]", value),
                AddressMode::AbsoluteXIndexedIndirect => format!("(${:04x},x)", value),
                AddressMode::StackRelative => format!("${:02x},s", value),
                AddressMode::StackRelativeIndirectYIndexed => format!("(${:02x},s),y", value),
                AddressMode::Relative => format!("{:+03x}", *value as i8),
                AddressMode::RelativeLong => format!("{:+05x}", *value as i16),
                AddressMode::DirectPage => format!("${:02x}", value),
                AddressMode::DirectPageIndirect => format!("(${:02x})", value),
                AddressMode::DirectPageIndirectLong => format!("[${:02x}]", value),
                AddressMode::DirectPageXIndexed => format!("${:02x},x", value),
                AddressMode::DirectPageXIndexedIndirect => format!("(${:02x},x)", value),
                AddressMode::DirectPageIndirectYIndexed => format!("(${:02x}),y", value),
                AddressMode::DirectPageIndirectYIndexedLong => format!("[${:02x}],y", value),
                AddressMode::DirectPageYIndexed => format!("${:02x},y", value),
                AddressMode::Implied
                | AddressMode::ImmediateU8
                | AddressMode::ImmediateA
                | AddressMode::ImmediateXY
                | AddressMode::Accumulator => unreachable!(),
            },
        }
    }
}
