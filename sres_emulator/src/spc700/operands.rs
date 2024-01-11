//! This module handles loading of operands used by instructions.
//!
//! Each instruction in the [opcode table](build_opcode_table) has an associated
//! address mode, which is decoded here to handle how the operand is loaded and stored.
use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::util::uint::UInt;

/// The address mode describes how to load the operand for an instruction.
#[derive(Clone, Copy, PartialEq)]
pub enum AddressMode {
    Implied,
    DirectPage,
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
    Address(u16, AddressMode, AddressU16),
}

impl Operand {
    /// Decodes the next operand located at the program counter.
    ///
    /// Returns the operand and address of the next instruction.
    #[inline]
    pub fn decode(
        cpu: &mut Spc700<impl Spc700Bus>,
        mode: AddressMode,
        rwm: AccessMode,
    ) -> (Self, AddressU16) {
        let pc = cpu.pc;
        Self::decode_impl(&mut ReadWrapper(cpu), pc, mode, rwm)
    }

    /// Peeks at the operand at `instruction_addr` without modifying the system state.
    ///
    /// Returns the operand and address of the next instruction.
    #[inline]
    pub fn peek(
        cpu: &Spc700<impl Spc700Bus>,
        instruction_addr: AddressU16,
        mode: AddressMode,
        rwm: AccessMode,
    ) -> (Self, AddressU16) {
        Self::decode_impl(&mut PeekWrapper(cpu), instruction_addr, mode, rwm)
    }

    /// Decodes an operand from the bus.
    ///
    /// Uses the [ReadOrPeekWrapper] to use the same logic for decoding operands during
    /// execution (where read cycles will modify the system state), and decoding
    /// operands for disassemply without modifying the system state.
    #[inline]
    fn decode_impl<'a, BusT: Spc700Bus, WrapperT: ReadOrPeekWrapper<'a, BusT>>(
        bus: &'a mut WrapperT,
        instruction_addr: AddressU16,
        mode: AddressMode,
        rwm: AccessMode,
    ) -> (Self, AddressU16) {
        // The size of the operand part of the instruction depends on the address mode.
        let operand_size: u8 = match mode {
            AddressMode::Implied => 0,
            AddressMode::DirectPage => 1,
        };

        // Regardless of how many bytes were read, store them all as u16 for simplicity.
        let next_addr = instruction_addr.add(1_u8, Wrap::WrapBank);
        let operand_data: u16 = match operand_size {
            0 => 0,
            1 => bus.cycle_read_u8(next_addr) as u16,
            2 => bus.cycle_read_u16(next_addr, Wrap::WrapBank) as u16,
            _ => unreachable!(),
        };

        // Interpret the address mode to figure out where the operand is located.
        let operand = match mode {
            AddressMode::Implied => Operand::Implied,
            // Operand is in memory, calculate the effective address
            _ => {
                let operand_addr: AddressU16 = match mode {
                    AddressMode::DirectPage => {
                        AddressU16::new_direct_page(bus.cpu().dsw, operand_data as u8)
                    }
                    AddressMode::Implied => unreachable!(),
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
    pub fn effective_addr(&self) -> Option<AddressU16> {
        match self {
            Self::Implied => None,
            Self::Address(_, _, addr) => Some(*addr),
        }
    }

    /// Load the operand. This may perform [bus](Bus) cycles to load the operand from memory.
    ///
    /// This method supports both u8 and u16 operands.
    #[inline]
    pub fn load<T: UInt>(&self, cpu: &mut Spc700<impl Spc700Bus>) -> T {
        match self {
            Self::Implied => panic!("loading implied operand"),
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
    pub fn store<T: UInt>(&self, cpu: &mut Spc700<impl Spc700Bus>, value: T) {
        match self {
            Self::Implied => panic!("writing to implied operand"),
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
            Self::Implied => "".to_string(),
            Self::Address(value, mode, _) => match mode {
                AddressMode::DirectPage => format!("${:02x}", value),
                AddressMode::Implied => unreachable!(),
            },
        }
    }
}

/// A wrapper around a CPU that will either perform reads on a mutable bus or
/// peeks on an immutable bus.
///
/// This allows logic for decoding of operands to be re-used for execution (mutable bus)
/// and disassembly generation (immutable bus).
trait ReadOrPeekWrapper<'a, T: Spc700Bus>
where
    Self: Sized,
{
    /// Returns an immutable reference to the underlying CPU.
    fn cpu(&self) -> &Spc700<T>;

    fn cycle_io(&mut self);
    fn cycle_read_u8(&mut self, addr: AddressU16) -> u8;

    fn cycle_read_u16(&mut self, addr: AddressU16, wrap: Wrap) -> u16 {
        u16::from_le_bytes([
            self.cycle_read_u8(addr),
            self.cycle_read_u8(addr.add(1_u16, wrap)),
        ])
    }

    fn cycle_read_u24(&mut self, addr: AddressU16, wrap: Wrap) -> u32 {
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
struct PeekWrapper<'a, T: Spc700Bus>(pub &'a Spc700<T>);
impl<'a, T: Spc700Bus> ReadOrPeekWrapper<'a, T> for PeekWrapper<'a, T> {
    fn cpu(&self) -> &Spc700<T> {
        self.0
    }

    fn cycle_io(&mut self) {}

    fn cycle_read_u8(&mut self, addr: AddressU16) -> u8 {
        self.0.bus.peek_u8(addr).unwrap_or_default()
    }
}

/// Implements ReadOrPeekWrapper for a mutable bus. Will perform read bus cycles that
/// will modify the system state.
struct ReadWrapper<'a, T: Spc700Bus>(pub &'a mut Spc700<T>);
impl<'a, T: Spc700Bus> ReadOrPeekWrapper<'a, T> for ReadWrapper<'a, T> {
    fn cpu(&self) -> &Spc700<T> {
        self.0
    }

    fn cycle_io(&mut self) {
        self.0.bus.cycle_io()
    }

    fn cycle_read_u8(&mut self, addr: AddressU16) -> u8 {
        self.0.bus.cycle_read_u8(addr)
    }
}
