//! This module handles loading of operands used by instructions.
//!
//! Each instruction in the [opcode table](build_opcode_table) has an associated
//! address mode, which is decoded here to handle how the operand is loaded and stored.
use super::Cpu;
use super::MainBus;
use super::STACK_BASE;
use crate::common::address::Address;
use crate::common::address::AddressU24;
use crate::common::address::Wrap;
use crate::common::uint::U16Ext;
use crate::common::uint::UInt;

/// The address mode describes how to load the operand for an instruction.
#[derive(Clone, Copy, PartialEq)]
pub enum AddressMode {
    Implied,
    ImmediateU8,
    ImmediateA,  // Immediate value based on accumulator register size
    ImmediateXY, // Immediate value based on index register size
    Accumulator,
    AbsoluteData,
    AbsoluteJump,
    AbsoluteLong,
    AbsoluteXIndexed,
    AbsoluteXIndexedLong,
    AbsoluteYIndexed,
    AbsoluteIndirectJump,
    AbsoluteIndirectLong,
    AbsoluteXIndexedIndirectJump,
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
    MoveAddressPair, // Used by MVN and MVP
}

/// Describes how the instruction will access the operand. This may subtly affect the
/// load/store behavior.
#[derive(Copy, Clone, PartialEq)]
pub enum AccessMode {
    Read,
    Write,
    Modify,
}

/// A decoded operand. The operand may be an immediate value, the accumulator register, or lives
/// at a specific address in memory.
///
/// The associated methods of this enum are inlined to allow the compiler to optimize away any
/// unnecessary branches, especially when called with a constant address mode in the opcode table.
#[derive(Copy, Clone)]
pub enum Operand {
    Implied,
    Accumulator,
    ImmediateU8(u8),
    ImmediateU16(u16),
    Address(u32, AddressMode, AddressU24),
    MoveAddressPair(u8, u8),
}

impl Operand {
    /// Decodes the next operand located at the program counter.
    ///
    /// Returns the operand and address of the next instruction.
    #[inline]
    pub fn decode(
        cpu: &mut Cpu<impl MainBus>,
        mode: AddressMode,
        rwm: AccessMode,
    ) -> (Self, AddressU24) {
        let pc = cpu.pc;
        Self::decode_impl(&mut ReadWrapper(cpu), pc, mode, rwm)
    }

    /// Peeks at the operand at `instruction_addr` without modifying the system state.
    ///
    /// Returns the operand and address of the next instruction.
    #[inline]
    pub fn peek(
        cpu: &Cpu<impl MainBus>,
        instruction_addr: AddressU24,
        mode: AddressMode,
        rwm: AccessMode,
    ) -> (Self, AddressU24) {
        Self::decode_impl(&mut PeekWrapper(cpu), instruction_addr, mode, rwm)
    }

    /// Decodes an operand from the bus.
    ///
    /// Uses the [ReadOrPeekWrapper] to use the same logic for decoding operands during
    /// execution (where read cycles will modify the system state), and decoding
    /// operands for disassemply without modifying the system state.
    #[inline]
    fn decode_impl<'a, BusT: MainBus, WrapperT: ReadOrPeekWrapper<'a, BusT>>(
        bus: &'a mut WrapperT,
        instruction_addr: AddressU24,
        mode: AddressMode,
        rwm: AccessMode,
    ) -> (Self, AddressU24) {
        // The size of the operand part of the instruction depends on the address mode.
        let operand_size: u8 = match mode {
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
            AddressMode::AbsoluteData => 2,
            AddressMode::AbsoluteJump => 2,
            AddressMode::AbsoluteLong => 3,
            AddressMode::AbsoluteXIndexed => 2,
            AddressMode::AbsoluteXIndexedLong => 3,
            AddressMode::AbsoluteYIndexed => 2,
            AddressMode::AbsoluteIndirectJump => 2,
            AddressMode::AbsoluteIndirectLong => 2,
            AddressMode::AbsoluteXIndexedIndirectJump => 2,
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
            AddressMode::MoveAddressPair => 2,
        };

        // Regardless of how many bytes were read, store them all as u32 for simplicity.
        let next_addr = instruction_addr.add(1_u8, Wrap::WrapBank);
        let operand_data: u32 = match operand_size {
            0 => 0,
            1 => bus.cycle_read_u8(next_addr) as u32,
            2 => bus.cycle_read_u16(next_addr, Wrap::WrapBank) as u32,
            3 => bus.cycle_read_u24(next_addr, Wrap::WrapBank),
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
            AddressMode::MoveAddressPair => Operand::MoveAddressPair(
                (operand_data as u16).high_byte(),
                (operand_data as u16).low_byte(),
            ),
            // Operand is in memory, calculate the effective address
            _ => {
                let operand_addr: AddressU24 = match mode {
                    AddressMode::AbsoluteData => AddressU24 {
                        bank: bus.cpu().db,
                        offset: operand_data as u16,
                    },
                    AddressMode::AbsoluteJump => AddressU24 {
                        bank: bus.cpu().pc.bank,
                        offset: operand_data as u16,
                    },
                    AddressMode::AbsoluteLong => AddressU24::from(operand_data),

                    AddressMode::AbsoluteYIndexed => {
                        let (page_cross, addr) = AddressU24 {
                            bank: bus.cpu().db,
                            offset: operand_data as u16,
                        }
                        .add_detect_page_cross(bus.cpu().y.value, Wrap::NoWrap);
                        if !bus.cpu().status.index_register_size_or_break
                            || page_cross
                            || rwm != AccessMode::Read
                        {
                            bus.cycle_io();
                        }
                        addr
                    }
                    AddressMode::AbsoluteXIndexed => {
                        let (page_cross, addr) = AddressU24 {
                            bank: bus.cpu().db,
                            offset: operand_data as u16,
                        }
                        .add_detect_page_cross(bus.cpu().x.value, Wrap::NoWrap);
                        if !bus.cpu().status.index_register_size_or_break
                            || page_cross
                            || rwm != AccessMode::Read
                        {
                            bus.cycle_io();
                        }
                        addr
                    }
                    AddressMode::AbsoluteXIndexedLong => {
                        AddressU24::from(operand_data).add(bus.cpu().x.value, Wrap::NoWrap)
                    }
                    AddressMode::AbsoluteIndirectJump => AddressU24::new(
                        bus.cpu().pc.bank,
                        bus.cycle_read_u16(operand_data.into(), Wrap::NoWrap),
                    ),
                    AddressMode::AbsoluteIndirectLong => {
                        AddressU24::from(bus.cycle_read_u24(operand_data.into(), Wrap::WrapBank))
                    }
                    AddressMode::AbsoluteXIndexedIndirectJump => {
                        bus.cycle_io();
                        AddressU24::new(
                            bus.cpu().pc.bank,
                            bus.cycle_read_u16(
                                AddressU24::new(bus.cpu().pc.bank, operand_data as u16)
                                    .add(bus.cpu().x.value, Wrap::WrapBank),
                                Wrap::WrapBank,
                            ),
                        )
                    }
                    AddressMode::Relative => {
                        let relative_addr = operand_data as i8;
                        if relative_addr > 0 {
                            bus.cpu()
                                .pc
                                .add(2_u8, Wrap::WrapBank)
                                .add(relative_addr.unsigned_abs(), Wrap::WrapBank)
                        } else {
                            bus.cpu()
                                .pc
                                .add(2_u8, Wrap::WrapBank)
                                .sub(relative_addr.unsigned_abs(), Wrap::WrapBank)
                        }
                    }
                    AddressMode::RelativeLong => {
                        let relative_addr = operand_data as i16;
                        if relative_addr > 0 {
                            bus.cpu()
                                .pc
                                .add(3_u8, Wrap::WrapBank)
                                .add(relative_addr.unsigned_abs(), Wrap::WrapBank)
                        } else {
                            bus.cpu()
                                .pc
                                .add(3_u8, Wrap::WrapBank)
                                .sub(relative_addr.unsigned_abs(), Wrap::WrapBank)
                        }
                    }
                    AddressMode::StackRelative => {
                        bus.cycle_io();
                        AddressU24::new(0, bus.cpu().s + STACK_BASE)
                            .add(operand_data, Wrap::WrapBank)
                    }
                    AddressMode::StackRelativeIndirectYIndexed => {
                        bus.cycle_io();
                        let value = AddressU24 {
                            bank: bus.cpu().db,
                            offset: bus.cycle_read_u16(
                                AddressU24::new(0, bus.cpu().s).add(operand_data, Wrap::WrapBank),
                                Wrap::WrapBank,
                            ),
                        }
                        .add(bus.cpu().y.value, Wrap::NoWrap);
                        bus.cycle_io();
                        value
                    }
                    AddressMode::DirectPage => {
                        if bus.cpu().d.low_byte() != 0 {
                            bus.cycle_io();
                        }
                        AddressU24::new(0, bus.cpu().d).add(operand_data, Wrap::WrapBank)
                    }
                    AddressMode::DirectPageXIndexed => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        bus.cycle_io();
                        AddressU24::new(0, bus.cpu().d)
                            .add(operand_data, Wrap::WrapBank)
                            .add(bus.cpu().x.value, Wrap::WrapBank)
                    }
                    AddressMode::DirectPageYIndexed => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        bus.cycle_io();
                        AddressU24::new(0, bus.cpu().d)
                            .add(operand_data, Wrap::WrapBank)
                            .add(bus.cpu().y.value, Wrap::WrapBank)
                    }
                    AddressMode::DirectPageIndirect => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        AddressU24 {
                            bank: bus.cpu().db,
                            offset: bus.cycle_read_u16(
                                AddressU24::new(0, bus.cpu().d).add(operand_data, Wrap::WrapBank),
                                Wrap::NoWrap,
                            ),
                        }
                    }
                    AddressMode::DirectPageXIndexedIndirect => {
                        bus.cycle_io();
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }

                        AddressU24 {
                            bank: bus.cpu().db,
                            offset: bus.cycle_read_u16(
                                AddressU24::new(0, bus.cpu().d)
                                    .add(operand_data, Wrap::WrapBank)
                                    .add(bus.cpu().x.value, Wrap::WrapBank),
                                Wrap::WrapBank,
                            ),
                        }
                    }
                    AddressMode::DirectPageIndirectYIndexed => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        let addr = AddressU24 {
                            bank: bus.cpu().db,
                            offset: bus.cycle_read_u16(
                                AddressU24::new(0, bus.cpu().d).add(operand_data, Wrap::WrapBank),
                                Wrap::WrapBank,
                            ),
                        };
                        let (page_cross, addr) =
                            addr.add_detect_page_cross(bus.cpu().y.value, Wrap::NoWrap);
                        if !bus.cpu().status.index_register_size_or_break
                            || page_cross
                            || rwm != AccessMode::Read
                        {
                            bus.cycle_io();
                        }
                        addr
                    }
                    AddressMode::DirectPageIndirectYIndexedLong => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        AddressU24::from(bus.cycle_read_u24(
                            AddressU24::new(0, bus.cpu().d).add(operand_data, Wrap::WrapBank),
                            Wrap::WrapBank,
                        ))
                        .add(bus.cpu().y.value, Wrap::NoWrap)
                    }
                    AddressMode::DirectPageIndirectLong => {
                        if bus.cpu().d.low_byte() > 0 {
                            bus.cycle_io();
                        }
                        AddressU24::from(bus.cycle_read_u24(
                            AddressU24::new(0, bus.cpu().d).add(operand_data, Wrap::WrapBank),
                            Wrap::NoWrap,
                        ))
                    }
                    AddressMode::Implied
                    | AddressMode::ImmediateU8
                    | AddressMode::ImmediateA
                    | AddressMode::ImmediateXY
                    | AddressMode::Accumulator
                    | AddressMode::MoveAddressPair => unreachable!(),
                };
                Operand::Address(operand_data, mode, operand_addr)
            }
        };
        (
            operand,
            instruction_addr
                .add(1_u8, Wrap::WrapBank)
                .add(operand_size, Wrap::WrapBank),
        )
    }

    /// Returns the effective [Address] of the operand lies in memory. None otherwise.
    #[inline]
    pub fn effective_addr(&self) -> Option<AddressU24> {
        match self {
            Self::Implied
            | Self::Accumulator
            | Self::ImmediateU8(_)
            | Self::ImmediateU16(_)
            | Self::MoveAddressPair(_, _) => None,
            Self::Address(_, _, addr) => Some(*addr),
        }
    }

    /// Load the operand. This may perform [bus](Bus) cycles to load the operand from memory.
    ///
    /// This method supports both u8 and u16 operands.
    #[inline]
    pub fn load<T: UInt>(&self, cpu: &mut Cpu<impl MainBus>) -> T {
        match self {
            Self::Implied => panic!("loading implied operand"),
            Self::MoveAddressPair(_, _) => panic!("loading from MoveAddressPair"),
            Self::ImmediateU8(value) => T::from_u8(*value),
            Self::ImmediateU16(value) => T::from_u16(*value),
            Self::Accumulator => cpu.a.get(),
            Self::Address(_, address_mode, addr) => {
                let wrap = if matches!(*address_mode, AddressMode::DirectPage) {
                    Wrap::WrapBank
                } else {
                    Wrap::NoWrap
                };
                cpu.bus.cycle_read_generic::<T>(*addr, wrap)
            }
        }
    }

    /// Store the operand. This may perform [bus](Bus) cycles to save the operand to memory.
    ///
    /// This method supports both u8 and u16 operands.
    #[inline]
    pub fn store<T: UInt>(&self, cpu: &mut Cpu<impl MainBus>, value: T) {
        match self {
            Self::Implied => panic!("writing to implied operand"),
            Self::MoveAddressPair(_, _) => panic!("writing to MoveAddressPair"),
            Self::ImmediateU8(_) | Self::ImmediateU16(_) => panic!("writing to immediate operand"),
            Self::Accumulator => cpu.a.set(value),
            Self::Address(_, _, addr) => {
                cpu.bus
                    .cycle_write_generic::<T>(*addr, value, Wrap::WrapBank);
            }
        }
    }

    /// Formats the operand as a human-readable string.
    ///
    /// The format matches that of BSNES disassembly.
    #[inline]
    pub fn format(&self) -> String {
        match self {
            Self::Implied | Self::Accumulator => "".to_string(),
            Self::ImmediateU8(value) => format!("#${value:02x}"),
            Self::ImmediateU16(value) => format!("#${value:04x}"),
            Self::MoveAddressPair(s, d) => format!("${s:02x}, ${d:02x}"),
            Self::Address(value, mode, _) => match mode {
                AddressMode::AbsoluteData => format!("${value:04x}"),
                AddressMode::AbsoluteJump => format!("${value:04x}"),
                AddressMode::AbsoluteLong => format!("${value:06x}"),
                AddressMode::AbsoluteXIndexed => format!("${value:04x},x"),
                AddressMode::AbsoluteXIndexedLong => format!("${value:06x},x"),
                AddressMode::AbsoluteYIndexed => format!("${value:04x},y"),
                AddressMode::AbsoluteIndirectJump => format!("(${value:04x})"),
                AddressMode::AbsoluteIndirectLong => format!("[${value:04x}]"),
                AddressMode::AbsoluteXIndexedIndirectJump => format!("(${value:04x},x)"),
                AddressMode::StackRelative => format!("${value:02x},s"),
                AddressMode::StackRelativeIndirectYIndexed => format!("(${value:02x},s),y"),
                AddressMode::Relative => format!("{:+}", *value as i8),
                AddressMode::RelativeLong => format!("{:+}", *value as i16),
                AddressMode::DirectPage => format!("${value:02x}"),
                AddressMode::DirectPageIndirect => format!("(${value:02x})"),
                AddressMode::DirectPageIndirectLong => format!("[${value:02x}]"),
                AddressMode::DirectPageXIndexed => format!("${value:02x},x"),
                AddressMode::DirectPageXIndexedIndirect => format!("(${value:02x},x)"),
                AddressMode::DirectPageIndirectYIndexed => format!("(${value:02x}),y"),
                AddressMode::DirectPageIndirectYIndexedLong => format!("[${value:02x}],y"),
                AddressMode::DirectPageYIndexed => format!("${value:02x},y"),
                AddressMode::Implied
                | AddressMode::ImmediateU8
                | AddressMode::ImmediateA
                | AddressMode::ImmediateXY
                | AddressMode::Accumulator
                | AddressMode::MoveAddressPair => unreachable!(),
            },
        }
    }
}

/// A wrapper around a CPU that will either perform reads on a mutable bus or
/// peeks on an immutable bus.
///
/// This allows logic for decoding of operands to be re-used for execution (mutable bus)
/// and disassembly generation (immutable bus).
trait ReadOrPeekWrapper<'a, T: MainBus>
where
    Self: Sized,
{
    /// Returns an immutable reference to the underlying CPU.
    fn cpu(&self) -> &Cpu<T>;

    fn cycle_io(&mut self);
    fn cycle_read_u8(&mut self, addr: AddressU24) -> u8;

    fn cycle_read_u16(&mut self, addr: AddressU24, wrap: Wrap) -> u16 {
        u16::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add(1_u16, wrap)),
        ])
    }

    fn cycle_read_u24(&mut self, addr: AddressU24, wrap: Wrap) -> u32 {
        u32::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add(1_u16, wrap)),
            self.cycle_read_u8(addr.add(2_u16, wrap)),
            0,
        ])
    }
}

/// Implements ReadOrPeekWrapper for an immutable bus. Will perform peek's instead
/// of read's, since a read operation will modify the state of the system.
struct PeekWrapper<'a, T: MainBus>(pub &'a Cpu<T>);
impl<'a, T: MainBus> ReadOrPeekWrapper<'a, T> for PeekWrapper<'a, T> {
    fn cpu(&self) -> &Cpu<T> {
        self.0
    }

    fn cycle_io(&mut self) {}

    fn cycle_read_u8(&mut self, addr: AddressU24) -> u8 {
        self.0.bus.peek_u8(addr).unwrap_or_default()
    }
}

/// Implements ReadOrPeekWrapper for a mutable bus. Will perform read bus cycles that
/// will modify the system state.
struct ReadWrapper<'a, T: MainBus>(pub &'a mut Cpu<T>);
impl<'a, T: MainBus> ReadOrPeekWrapper<'a, T> for ReadWrapper<'a, T> {
    fn cpu(&self) -> &Cpu<T> {
        self.0
    }

    fn cycle_io(&mut self) {
        self.0.bus.cycle_io()
    }

    fn cycle_read_u8(&mut self, addr: AddressU24) -> u8 {
        self.0.bus.cycle_read_u8(addr)
    }
}
