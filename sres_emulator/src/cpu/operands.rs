use intbits::Bits;

use super::Cpu;
use super::UInt;
use super::STACK_BASE;
use crate::bus::Bus;
use crate::memory::Address;
use crate::uint::U16Ext;

#[derive(Clone, Copy, PartialEq)]
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
            1 => cpu.bus.cycle_read_u8(instruction_addr + 1) as u32,
            // Do not use read_u16. The program counter will wrap around in the memory bank.
            2 => u16::from_le_bytes([
                cpu.bus.cycle_read_u8(instruction_addr + 1),
                cpu.bus.cycle_read_u8(instruction_addr + 2),
            ]) as u32,
            // Do not use read_u24. The program counter will wrap around in the memory bank.
            3 => u32::from_le_bytes([
                cpu.bus.cycle_read_u8(instruction_addr + 1),
                cpu.bus.cycle_read_u8(instruction_addr + 2),
                cpu.bus.cycle_read_u8(instruction_addr + 3),
                0,
            ]),
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
                    AddressMode::Absolute => Address {
                        bank: cpu.db,
                        offset: operand_data as u16,
                    }
                    .into(),
                    AddressMode::AbsoluteLong => operand_data,

                    AddressMode::AbsoluteYIndexed => {
                        let page_cross = operand_data.bits(0..8) + cpu.y.value as u32 > 0xff;
                        if !cpu.status.index_register_size_or_break || page_cross {
                            cpu.bus.cycle_io();
                        }
                        u32::from(Address {
                            bank: cpu.db,
                            offset: operand_data as u16,
                        }) + cpu.y.value as u32
                    }
                    AddressMode::AbsoluteXIndexed => {
                        let page_cross =
                            operand_data.bits(0..8) + cpu.x.value.bits(0..8) as u32 > 0xff;
                        if !cpu.status.index_register_size_or_break || page_cross {
                            cpu.bus.cycle_io();
                        }
                        u32::from(Address {
                            bank: cpu.db,
                            offset: operand_data as u16,
                        }) + cpu.x.value as u32
                    }
                    AddressMode::AbsoluteXIndexedLong => operand_data + cpu.x.value as u32,
                    AddressMode::AbsoluteIndirect => {
                        cpu.bus.cycle_read_u16(operand_data.into()) as u32
                    }
                    AddressMode::AbsoluteIndirectLong => {
                        cpu.bus.cycle_read_u24(operand_data.into())
                    }
                    AddressMode::AbsoluteXIndexedIndirect => {
                        cpu.bus.cycle_io();
                        cpu.bus
                            .cycle_read_u16(Address::from(operand_data) + cpu.x.value as u32)
                            as u32
                    }
                    AddressMode::Relative => {
                        let relative_addr = operand_data as i8;
                        if relative_addr > 0 {
                            Address {
                                bank: cpu.pc.bank,
                                offset: (cpu.pc.offset.wrapping_add(2))
                                    .wrapping_add(relative_addr.unsigned_abs() as u16),
                            }
                        } else {
                            Address {
                                bank: cpu.pc.bank,
                                offset: (cpu.pc.offset.wrapping_add(2))
                                    .wrapping_sub(relative_addr.unsigned_abs() as u16),
                            }
                        }
                        .into()
                    }
                    AddressMode::RelativeLong => {
                        let relative_addr = operand_data as i16;
                        if relative_addr > 0 {
                            u32::from(cpu.pc + 3).wrapping_add(relative_addr.unsigned_abs() as u32)
                        } else {
                            u32::from(cpu.pc + 3).wrapping_sub(relative_addr.unsigned_abs() as u32)
                        }
                    }
                    AddressMode::StackRelative => {
                        cpu.bus.cycle_io();
                        (Address::new(0, cpu.s + STACK_BASE) + operand_data).into()
                    }
                    AddressMode::StackRelativeIndirectYIndexed => {
                        cpu.bus.cycle_io();
                        let value = u32::from(Address {
                            bank: cpu.db,
                            offset: cpu
                                .bus
                                .cycle_read_u16(Address::new(0, cpu.s) + operand_data),
                        }) + cpu.y.value as u32;
                        cpu.bus.cycle_io();
                        value
                    }
                    AddressMode::DirectPage => {
                        if cpu.d.low_byte() != 0 {
                            cpu.bus.cycle_io();
                        }
                        (Address::new(0, cpu.d) + operand_data).into()
                    }
                    AddressMode::DirectPageXIndexed => {
                        if cpu.d.low_byte() > 0 {
                            cpu.bus.cycle_io();
                        }
                        cpu.bus.cycle_io();
                        (Address::new(0, cpu.d) + operand_data as u16 + cpu.x.value).into()
                    }
                    AddressMode::DirectPageYIndexed => {
                        if cpu.d.low_byte() > 0 {
                            cpu.bus.cycle_io();
                        }
                        cpu.bus.cycle_io();
                        (Address::new(0, cpu.d) + operand_data as u16 + cpu.y.value).into()
                    }
                    AddressMode::DirectPageIndirect => {
                        if cpu.d.low_byte() > 0 {
                            cpu.bus.cycle_io();
                        }
                        Address {
                            bank: cpu.db,
                            offset: cpu
                                .bus
                                .cycle_read_u16(Address::new(0, cpu.d) + operand_data),
                        }
                        .into()
                    }
                    AddressMode::DirectPageXIndexedIndirect => {
                        cpu.bus.cycle_io();
                        if cpu.d.low_byte() > 0 {
                            cpu.bus.cycle_io();
                        }

                        Address {
                            bank: cpu.db,
                            offset: cpu.bus.cycle_read_u16(
                                Address::new(0, cpu.d) + operand_data + cpu.x.value as u32,
                            ),
                        }
                        .into()
                    }
                    AddressMode::DirectPageIndirectYIndexed => {
                        if cpu.d.low_byte() > 0 {
                            cpu.bus.cycle_io();
                        }
                        let addr = Address {
                            bank: cpu.db,
                            offset: cpu
                                .bus
                                .cycle_read_u16(Address::new(0, cpu.d) + operand_data),
                        };

                        if !cpu.status.index_register_size_or_break
                            || addr.offset.low_byte() as u16 + cpu.y.value.low_byte() as u16 > 0xff
                        {
                            cpu.bus.cycle_io();
                        }
                        u32::from(addr) + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirectYIndexedLong => {
                        if cpu.d.low_byte() > 0 {
                            cpu.bus.cycle_io();
                        }
                        cpu.bus
                            .cycle_read_u24(Address::new(0, cpu.d) + operand_data)
                            + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirectLong => {
                        if cpu.d.low_byte() > 0 {
                            cpu.bus.cycle_io();
                        }
                        cpu.bus
                            .cycle_read_u24(Address::new(0, cpu.d) + operand_data)
                    }
                    AddressMode::Implied
                    | AddressMode::ImmediateU8
                    | AddressMode::ImmediateA
                    | AddressMode::ImmediateXY
                    | AddressMode::Accumulator => unreachable!(),
                };
                Operand::Address(operand_data, mode, operand_addr.into())
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
            // Do not use read_u16. The program counter will wrap around in the memory bank.
            2 => u16::from_le_bytes([
                cpu.bus.peek_u8(instruction_addr + 1).unwrap_or_default(),
                cpu.bus.peek_u8(instruction_addr + 2).unwrap_or_default(),
            ]) as u32,
            // Do not use read_u24. The program counter will wrap around in the memory bank.
            3 => u32::from_le_bytes([
                cpu.bus.peek_u8(instruction_addr + 1).unwrap_or_default(),
                cpu.bus.peek_u8(instruction_addr + 2).unwrap_or_default(),
                cpu.bus.peek_u8(instruction_addr + 3).unwrap_or_default(),
                0,
            ]),
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
                    AddressMode::Absolute => Address {
                        bank: cpu.db,
                        offset: operand_data as u16,
                    }
                    .into(),
                    AddressMode::AbsoluteLong => operand_data,

                    AddressMode::AbsoluteYIndexed => {
                        let page_cross = operand_data.bits(0..8) + cpu.y.value as u32 > 0xff;
                        u32::from(Address {
                            bank: cpu.db,
                            offset: operand_data as u16,
                        }) + cpu.y.value as u32
                    }
                    AddressMode::AbsoluteXIndexed => {
                        let page_cross =
                            operand_data.bits(0..8) + cpu.x.value.bits(0..8) as u32 > 0xff;
                        u32::from(Address {
                            bank: cpu.db,
                            offset: operand_data as u16,
                        }) + cpu.x.value as u32
                    }
                    AddressMode::AbsoluteXIndexedLong => operand_data + cpu.x.value as u32,
                    AddressMode::AbsoluteIndirect => {
                        cpu.bus.peek_u16(operand_data.into()).unwrap_or_default() as u32
                    }
                    AddressMode::AbsoluteIndirectLong => {
                        cpu.bus.peek_u24(operand_data.into()).unwrap_or_default()
                    }
                    AddressMode::AbsoluteXIndexedIndirect => {
                        cpu.bus
                            .peek_u16(Address::from(operand_data) + cpu.x.value as u32)
                            .unwrap_or_default() as u32
                    }
                    AddressMode::Relative => {
                        let relative_addr = operand_data as i8;
                        if relative_addr > 0 {
                            Address {
                                bank: cpu.pc.bank,
                                offset: (cpu.pc.offset.wrapping_add(2))
                                    .wrapping_add(relative_addr.unsigned_abs() as u16),
                            }
                        } else {
                            Address {
                                bank: cpu.pc.bank,
                                offset: (cpu.pc.offset.wrapping_add(2))
                                    .wrapping_sub(relative_addr.unsigned_abs() as u16),
                            }
                        }
                        .into()
                    }
                    AddressMode::RelativeLong => {
                        let relative_addr = operand_data as i16;
                        if relative_addr > 0 {
                            u32::from(cpu.pc + 3).wrapping_add(relative_addr.unsigned_abs() as u32)
                        } else {
                            u32::from(cpu.pc + 3).wrapping_sub(relative_addr.unsigned_abs() as u32)
                        }
                    }
                    AddressMode::StackRelative => {
                        (Address::new(0, cpu.s + STACK_BASE) + operand_data).into()
                    }
                    AddressMode::StackRelativeIndirectYIndexed => {
                        let value = u32::from(Address {
                            bank: cpu.db,
                            offset: cpu
                                .bus
                                .peek_u16(Address::new(0, cpu.s) + operand_data)
                                .unwrap_or_default(),
                        }) + cpu.y.value as u32;
                        value
                    }
                    AddressMode::DirectPage => (Address::new(0, cpu.d) + operand_data).into(),
                    AddressMode::DirectPageXIndexed => {
                        (Address::new(0, cpu.d) + operand_data as u16 + cpu.x.value).into()
                    }
                    AddressMode::DirectPageYIndexed => {
                        (Address::new(0, cpu.d) + operand_data as u16 + cpu.y.value).into()
                    }
                    AddressMode::DirectPageIndirect => Address {
                        bank: cpu.db,
                        offset: cpu
                            .bus
                            .peek_u16(Address::new(0, cpu.d) + operand_data)
                            .unwrap_or_default(),
                    }
                    .into(),
                    AddressMode::DirectPageXIndexedIndirect => Address {
                        bank: cpu.db,
                        offset: cpu
                            .bus
                            .peek_u16(Address::new(0, cpu.d) + operand_data + cpu.x.value as u32)
                            .unwrap_or_default(),
                    }
                    .into(),
                    AddressMode::DirectPageIndirectYIndexed => {
                        let addr = Address {
                            bank: cpu.db,
                            offset: cpu
                                .bus
                                .peek_u16(Address::new(0, cpu.d) + operand_data)
                                .unwrap_or_default(),
                        };

                        u32::from(addr) + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirectYIndexedLong => {
                        cpu.bus
                            .peek_u24(Address::new(0, cpu.d) + operand_data)
                            .unwrap_or_default()
                            + cpu.y.value as u32
                    }
                    AddressMode::DirectPageIndirectLong => cpu
                        .bus
                        .peek_u24(Address::new(0, cpu.d) + operand_data)
                        .unwrap_or_default(),
                    AddressMode::Implied
                    | AddressMode::ImmediateU8
                    | AddressMode::ImmediateA
                    | AddressMode::ImmediateXY
                    | AddressMode::Accumulator => unreachable!(),
                };
                Operand::Address(operand_data, mode, operand_addr.into())
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
            _ => cpu
                .bus
                .cycle_read_generic::<T>(self.effective_addr().unwrap()),
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
                .cycle_write_generic::<T>(self.effective_addr().unwrap(), value),
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
