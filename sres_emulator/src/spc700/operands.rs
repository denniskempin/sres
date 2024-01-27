//! This module handles loading of operands used by instructions.
//!
//! Each instruction in the [opcode table](build_opcode_table) has an associated
//! address mode, which is decoded here to handle how the operand is loaded and stored.
use intbits::Bits;

use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;

/// The address mode describes how to load the operand for an instruction.
#[derive(Clone, Copy, PartialEq)]
pub enum AddressMode {
    DirectPage,
    Absolute,
    IndirectX,
    DirectPageXIndexed,
}

impl AddressMode {
    #[inline]
    pub fn operand_size(&self) -> u8 {
        match self {
            AddressMode::DirectPage => 1,
            AddressMode::Absolute => 2,
            AddressMode::IndirectX => 0,
            AddressMode::DirectPageXIndexed => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Register {
    Accumulator,
    X,
    Y,
    Psw,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OperandDef {
    Implied,
    Immediate,
    Register(Register),
    Const(u8),
    InMemory(AddressMode),
    AbsoluteBit,
    DirectPageBit(u8),
}

impl OperandDef {
    /// Decodes the next operand located at the program counter.
    ///
    /// Returns the operand and address of the next instruction.
    #[inline]
    pub fn decode(&self, cpu: &mut Spc700<impl Spc700Bus>) -> (Operand, AddressU16) {
        let pc = cpu.pc;
        self.decode_impl(&mut ReadWrapper(cpu), pc)
    }

    /// Peeks at the operand at `instruction_addr` without modifying the system state.
    ///
    /// Returns the operand and address of the next instruction.
    #[inline]
    pub fn peek(
        &self,
        cpu: &Spc700<impl Spc700Bus>,
        operand_addr: AddressU16,
    ) -> (Operand, AddressU16) {
        self.decode_impl(&mut PeekWrapper(cpu), operand_addr)
    }

    /// Decodes an operand from the bus.
    ///
    /// Uses the [ReadOrPeekWrapper] to use the same logic for decoding operands during
    /// execution (where read cycles will modify the system state), and decoding
    /// operands for disassemply without modifying the system state.
    #[inline]
    fn decode_impl<'a, BusT: Spc700Bus, WrapperT: ReadOrPeekWrapper<'a, BusT>>(
        &self,
        bus: &'a mut WrapperT,
        operand_addr: AddressU16,
    ) -> (Operand, AddressU16) {
        // Regardless of how many bytes were read, store them all as u16 for simplicity.
        let operand_data: u16 = match self.operand_size() {
            0 => 0,
            1 => bus.cycle_read_u8(operand_addr) as u16,
            2 => bus.cycle_read_u16(operand_addr, Wrap::NoWrap),
            _ => unreachable!(),
        };

        // Interpret the address mode to figure out where the operand is located.
        let operand = match self {
            OperandDef::Implied => Operand::Implied,
            OperandDef::Immediate => Operand::Immediate(operand_data),
            OperandDef::Register(register) => Operand::Register(*register),
            OperandDef::Const(value) => Operand::Const(*value),
            OperandDef::InMemory(mode) => {
                let addr = match mode {
                    AddressMode::DirectPage => bus.cpu().direct_page_addr(operand_data as u8),
                    AddressMode::Absolute => AddressU16(operand_data),
                    AddressMode::IndirectX => {
                        bus.cycle_read_u8(operand_addr); // Dummy read
                        let addr = bus.cpu().direct_page_addr(bus.cpu().x);
                        addr
                    }
                    AddressMode::DirectPageXIndexed => {
                        bus.cycle_io();
                        let addr = bus
                            .cpu()
                            .direct_page_addr(operand_data as u8)
                            .add(bus.cpu().x, Wrap::WrapPage);
                        AddressU16(bus.cycle_read_u16(addr, Wrap::WrapPage))
                    }
                };
                Operand::InMemory(operand_data, *mode, addr)
            }
            OperandDef::AbsoluteBit => {
                let bit = operand_data.bits(13..16);
                let addr = AddressU16(operand_data.bits(0..13));
                Operand::InMemoryBit(addr, bit as u8)
            }
            OperandDef::DirectPageBit(bit) => {
                let addr = bus.cpu().direct_page_addr(operand_data as u8);
                Operand::InMemoryBit(addr, *bit)
            }
        };
        (operand, operand_addr.add(self.operand_size(), Wrap::NoWrap))
    }

    #[inline]
    fn operand_size(&self) -> u8 {
        match self {
            Self::Implied => 0,
            Self::Immediate => 1,
            Self::Register(_) => 0,
            Self::Const(_) => 0,
            Self::InMemory(mode) => mode.operand_size(),
            Self::AbsoluteBit => 2,
            Self::DirectPageBit(_) => 1,
        }
    }
}

/// A decoded operand. The operand may be an immediate value, the accumulator register, or lives
/// at a specific address in memory.
///
/// The associated methods of this enum are inlined to allow the compiler to optimize away any
/// unnecessary branches, especially when called with a constant address mode in the opcode table.
#[derive(Copy, Clone)]
pub enum Operand {
    Implied,
    Immediate(u16),
    Register(Register),
    Const(u8),
    InMemory(u16, AddressMode, AddressU16),
    InMemoryInverted(u16, AddressMode, AddressU16),
    InMemoryBit(AddressU16, u8),
}

impl Operand {
    /// Returns the effective [Address] of the operand lies in memory. None otherwise.
    #[inline]
    pub fn effective_addr(&self) -> Option<AddressU16> {
        match self {
            Self::Implied => None,
            Self::Immediate(_) => None,
            Self::Register(_) => None,
            Self::Const(_) => None,
            Self::InMemory(_, _, addr) => Some(*addr),
            Self::InMemoryInverted(_, _, addr) => Some(*addr),
            Self::InMemoryBit(addr, _) => Some(*addr),
        }
    }

    /// Load the operand. This may perform [bus](Bus) cycles to load the operand from memory.
    ///
    /// This method supports both u8 and u16 operands.
    #[inline]
    pub fn load_u8(&self, cpu: &mut Spc700<impl Spc700Bus>) -> u8 {
        match self {
            Self::Implied => panic!("loading implied operand"),
            Self::Immediate(value) => *value as u8,
            Self::Register(Register::Accumulator) => cpu.a,
            Self::Register(Register::X) => cpu.x,
            Self::Register(Register::Y) => cpu.y,
            Self::Register(Register::Psw) => cpu.status.into(),
            Self::Const(value) => *value,
            Self::InMemory(_, _, addr) => cpu.bus.cycle_read_u8(*addr),
            Self::InMemoryInverted(_, _, addr) => !cpu.bus.cycle_read_u8(*addr),
            Self::InMemoryBit(addr, _) => cpu.bus.cycle_read_u8(*addr),
        }
    }

    #[inline]
    pub fn bit_idx(&self) -> usize {
        match self {
            Self::InMemoryBit(_, bit) => *bit as usize,
            _ => panic!("Not a bit operand"),
        }
    }

    /// Store the operand. This may perform [bus](Bus) cycles to save the operand to memory.
    ///
    /// This method supports both u8 and u16 operands.
    #[inline]
    pub fn store_u8(&self, cpu: &mut Spc700<impl Spc700Bus>, value: u8) {
        match self {
            Self::Implied => panic!("storing implied operand"),
            Self::Immediate(_) => panic!("storing immediate operand"),
            Self::Register(Register::Accumulator) => cpu.a = value,
            Self::Register(Register::X) => cpu.x = value,
            Self::Register(Register::Y) => cpu.y = value,
            Self::Register(Register::Psw) => cpu.status = value.into(),
            Self::Const(_) => panic!("storing const operand"),
            Self::InMemory(_, _, addr) => cpu.bus.cycle_write_u8(*addr, value),
            Self::InMemoryInverted(_, _, addr) => cpu.bus.cycle_write_u8(*addr, value ^ 0xFF),
            Self::InMemoryBit(addr, _) => cpu.bus.cycle_write_u8(*addr, value),
        }
    }

    /// Formats the operand as a human-readable string.
    ///
    /// The format matches that of BSNES disassembly.
    #[inline]
    pub fn format(&self) -> String {
        match self {
            Self::Implied => String::new(),
            Self::Immediate(value) => format!("#${:02X}", value),
            Self::Register(Register::Accumulator) => "A".to_string(),
            Self::Register(Register::X) => "X".to_string(),
            Self::Register(Register::Y) => "Y".to_string(),
            Self::Register(Register::Psw) => "PSW".to_string(),
            Self::Const(value) => format!("{:02X}", value),
            Self::InMemory(_, _, addr) => format!("${:04X}", addr.0),
            Self::InMemoryInverted(_, _, addr) => format!("!${:04X}", addr.0),
            Self::InMemoryBit(addr, bit) => format!("(${:04X}.{})", addr.0, bit),
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
