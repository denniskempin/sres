use intbits::Bits;

use super::Cpu;
use super::UInt;
use super::STACK_BASE;
use crate::bus::Bus;
use crate::memory::Address;
use crate::memory::Wrap;
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

pub trait ReadOrPeekWrapper<'a, T: Bus>
where
    Self: Sized,
{
    fn cpu(&self) -> &Cpu<T>;
    fn cycle_io(&mut self);
    fn cycle_read_u8(&mut self, addr: Address) -> u8;

    fn cycle_read_u16(&mut self, addr: Address) -> u16 {
        u16::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add2(1_u16, Wrap::NoWrap)),
        ])
    }

    fn cycle_read_u24(&mut self, addr: Address) -> u32 {
        u32::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add2(1_u16, Wrap::NoWrap)),
            self.cycle_read_u8(addr.add2(2_u16, Wrap::NoWrap)),
            0,
        ])
    }
}

pub struct PeekWrapper<'a, T: Bus>(pub &'a Cpu<T>);
impl<'a, T: Bus> ReadOrPeekWrapper<'a, T> for PeekWrapper<'a, T> {
    fn cpu(&self) -> &Cpu<T> {
        self.0
    }

    fn cycle_io(&mut self) {}

    fn cycle_read_u8(&mut self, addr: Address) -> u8 {
        self.0.bus.peek_u8(addr).unwrap_or_default()
    }
}

pub struct ReadWrapper<'a, T: Bus>(pub &'a mut Cpu<T>);
impl<'a, T: Bus> ReadOrPeekWrapper<'a, T> for ReadWrapper<'a, T> {
    fn cpu(&self) -> &Cpu<T> {
        self.0
    }

    fn cycle_io(&mut self) {
        self.0.bus.cycle_io()
    }

    fn cycle_read_u8(&mut self, addr: Address) -> u8 {
        self.0.bus.cycle_read_u8(addr)
    }
}

impl Operand {
    #[inline]
    pub fn decode<'a, BusT: Bus, WrapperT: ReadOrPeekWrapper<'a, BusT>>(
        bus: &'a mut WrapperT,
        instruction_addr: Address,
        mode: AddressMode,
    ) -> (Self, Address) {
        // The size of the operand part of the instruction depends on the address mode.
        let operand_size = match mode {
            AddressMode::Implied => 0,
            AddressMode::Accumulator => 0,
            AddressMode::ImmediateU8 => 1,
            AddressMode::ImmediateA => {
                if bus.cpu().status.accumulator_register_size {
                    1
                } else {
                    2
                }
            }
            AddressMode::ImmediateXY => {
                if bus.cpu().status.index_register_size_or_break {
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
            1 => bus.cycle_read_u8(instruction_addr + 1) as u32,
            // Do not use read_u16. The program counter will wrap around in the memory bank.
            2 => u16::from_le_bytes([
                bus.cycle_read_u8(instruction_addr + 1),
                bus.cycle_read_u8(instruction_addr + 2),
            ]) as u32,
            // Do not use read_u24. The program counter will wrap around in the memory bank.
            3 => u32::from_le_bytes([
                bus.cycle_read_u8(instruction_addr + 1),
                bus.cycle_read_u8(instruction_addr + 2),
                bus.cycle_read_u8(instruction_addr + 3),
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
                if bus.cpu().status.accumulator_register_size {
                    Operand::ImmediateU8(operand_data as u8)
                } else {
                    Operand::ImmediateU16(operand_data as u16)
                }
            }
            AddressMode::ImmediateXY => {
                if bus.cpu().status.index_register_size_or_break {
                    Operand::ImmediateU8(operand_data as u8)
                } else {
                    Operand::ImmediateU16(operand_data as u16)
                }
            }
            // Operand is in memory, calculate the effective address
            _ => {
                let operand_addr: u32 = match mode {
                    AddressMode::Absolute => Address {
                        bank: bus.cpu().db,
                        offset: operand_data as u16,
                    }
                    .into(),
                    AddressMode::AbsoluteLong => operand_data,

                    AddressMode::AbsoluteYIndexed => {
                        let page_cross = operand_data.bits(0..8) + bus.cpu().y.value as u32 > 0xff;
                        if !bus.cpu().status.index_register_size_or_break || page_cross {
                            bus.cycle_io();
                        }
                        u32::from(Address {
                            bank: bus.cpu().db,
                            offset: operand_data as u16,
                        }) + bus.cpu().y.value as u32
                    }
                    AddressMode::AbsoluteXIndexed => {
                        let page_cross =
                            operand_data.bits(0..8) + bus.cpu().x.value.bits(0..8) as u32 > 0xff;
                        if !bus.cpu().status.index_register_size_or_break || page_cross {
                            bus.cycle_io();
                        }
                        u32::from(Address {
                            bank: bus.cpu().db,
                            offset: operand_data as u16,
                        }) + bus.cpu().x.value as u32
                    }
                    AddressMode::AbsoluteXIndexedLong => operand_data + bus.cpu().x.value as u32,
                    AddressMode::AbsoluteIndirect => bus.cycle_read_u16(operand_data.into()) as u32,
                    AddressMode::AbsoluteIndirectLong => bus.cycle_read_u24(operand_data.into()),
                    AddressMode::AbsoluteXIndexedIndirect => {
                        bus.cycle_io();
                        bus.cycle_read_u16(Address::from(operand_data) + bus.cpu().x.value as u32)
                            as u32
                    }
                    AddressMode::Relative => {
                        let relative_addr = operand_data as i8;
                        if relative_addr > 0 {
                            Address {
                                bank: bus.cpu().pc.bank,
                                offset: (bus.cpu().pc.offset.wrapping_add(2))
                                    .wrapping_add(relative_addr.unsigned_abs() as u16),
                            }
                        } else {
                            Address {
                                bank: bus.cpu().pc.bank,
                                offset: (bus.cpu().pc.offset.wrapping_add(2))
                                    .wrapping_sub(relative_addr.unsigned_abs() as u16),
                            }
                        }
                        .into()
                    }
                    AddressMode::RelativeLong => {
                        let relative_addr = operand_data as i16;
                        if relative_addr > 0 {
                            u32::from(bus.cpu().pc + 3)
                                .wrapping_add(relative_addr.unsigned_abs() as u32)
                        } else {
                            u32::from(bus.cpu().pc + 3)
                                .wrapping_sub(relative_addr.unsigned_abs() as u32)
                        }
                    }
                    AddressMode::StackRelative => {
                        bus.cycle_io();
                        (Address::new(0, bus.cpu().s + STACK_BASE) + operand_data).into()
                    }
                    AddressMode::StackRelativeIndirectYIndexed => {
                        bus.cycle_io();
                        let value = u32::from(Address {
                            bank: bus.cpu().db,
                            offset: bus.cycle_read_u16(Address::new(0, bus.cpu().s) + operand_data),
                        }) + bus.cpu().y.value as u32;
                        bus.cycle_io();
                        value
                    }
                    AddressMode::DirectPage => {
                        if bus.cpu().d.low_byte() != 0 {
                            bus.cycle_io();
                        }
                        (Address::new(0, bus.cpu().d) + operand_data).into()
                    }
                    AddressMode::DirectPageXIndexed => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        bus.cycle_io();
                        (Address::new(0, bus.cpu().d) + operand_data as u16 + bus.cpu().x.value)
                            .into()
                    }
                    AddressMode::DirectPageYIndexed => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        bus.cycle_io();
                        (Address::new(0, bus.cpu().d) + operand_data as u16 + bus.cpu().y.value)
                            .into()
                    }
                    AddressMode::DirectPageIndirect => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        Address {
                            bank: bus.cpu().db,
                            offset: bus.cycle_read_u16(Address::new(0, bus.cpu().d) + operand_data),
                        }
                        .into()
                    }
                    AddressMode::DirectPageXIndexedIndirect => {
                        bus.cycle_io();
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }

                        Address {
                            bank: bus.cpu().db,
                            offset: bus.cycle_read_u16(
                                Address::new(0, bus.cpu().d)
                                    + operand_data
                                    + bus.cpu().x.value as u32,
                            ),
                        }
                        .into()
                    }
                    AddressMode::DirectPageIndirectYIndexed => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        let addr = Address {
                            bank: bus.cpu().db,
                            offset: bus.cycle_read_u16(Address::new(0, bus.cpu().d) + operand_data),
                        };

                        if !bus.cpu().status.index_register_size_or_break
                            || addr.offset.low_byte() as u16 + bus.cpu().y.value.low_byte() as u16
                                > 0xff
                        {
                            bus.cycle_io();
                        }
                        u32::from(addr) + bus.cpu().y.value as u32
                    }
                    AddressMode::DirectPageIndirectYIndexedLong => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        bus.cycle_read_u24(Address::new(0, bus.cpu().d) + operand_data)
                            + bus.cpu().y.value as u32
                    }
                    AddressMode::DirectPageIndirectLong => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        bus.cycle_read_u24(Address::new(0, bus.cpu().d) + operand_data)
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
