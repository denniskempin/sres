//! This module handles loading of operands used by instructions.
//! Each instruction in the [opcode table](build_opcode_table) has an associated
//!
//! address mode, which is decoded here to handle how the operand is loaded and stored.
use intbits::Bits;

use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;
use crate::util::uint::U16Ext;

/// Each instruction is defined by it's opcode and a variable number of bytes to descibe the
/// operands of the instruction.
///
/// The operands are defined for each opcode in `opcode_table.rs`, and then passed to the
/// instructions in `instructions.rs`.
///
/// To access operands, they first need to be decoded (i.e. consume and interpret bytes of program
/// memory) into a [DecodedOperand], then can be loaded or stored. This are separate steps since
/// the order in which memory is accessed varies, and instructions may want to decode once, but
/// load/store multiple times.
///
/// Design note:
///
/// In retrospect this abstraction just does not work well and overcomplicates things
/// way too much. It only works well with single operands.
/// With multiple operands, the cycle order of decoding and loading the operands
/// cannot be easily abstracted. There are too many irregularities based on the combination
/// of operand types (e.g. look at the mess in [crate::spc700::instructions::mov]) or the
/// special handling needed for [AddressMode::XIndirect] since `MOV (X) (Y)` has different idle
/// cycles than other uses of `(X)` or `(Y)`.
/// U16 operands are especially complex as decoding, loading and storing of the operands is
/// interleaved.
pub trait Operand<ValueT, DecodedT: DecodedOperand<ValueT>> {
    /// Consumes enough bytes of program memory to decode this operand
    ///
    /// Advances the program counter and returns the decoded operand
    #[inline]
    fn decode(&self, cpu: &mut Spc700<impl Spc700Bus>) -> DecodedT {
        let pc = cpu.pc;
        let (operand, next_pc) = self.decode_impl(&mut ReadWrapper(cpu), pc);
        cpu.pc = next_pc;
        operand
    }

    /// Peeks at the operand at `operand_addr` without modifying the system state.
    ///
    /// Returns the decoded operand and address of the next instruction (or next operand).
    #[inline]
    fn peek(
        &self,
        cpu: &Spc700<impl Spc700Bus>,
        operand_addr: AddressU16,
    ) -> (DecodedT, AddressU16) {
        self.decode_impl(&mut PeekWrapper(cpu), operand_addr)
    }

    /// Internal implementation of both `decode` and `peek`, using the [ReadOrPeekWrapper] to use
    /// the same implementation for both mutable and immutable Cpu's.
    fn decode_impl<'a, BusT: Spc700Bus, WrapperT: ReadOrPeekWrapper<'a, BusT>>(
        &self,
        bus: &'a mut WrapperT,
        operand_addr: AddressU16,
    ) -> (DecodedT, AddressU16);
}

/// Provides access to the operand value after decoding.
/// SPC700 has 3 types of Operands: 8 and 16 bit operands, as well as single bit operands.
pub trait DecodedOperand<T> {
    /// Loads the operand. May perform the memory reads needed to read the operand from memory.
    fn load(&self, cpu: &mut Spc700<impl Spc700Bus>) -> T;
    /// Stores the operand. May perform the memory writes needed to write the operand to memory.
    fn store(&self, cpu: &mut Spc700<impl Spc700Bus>, value: T);
    /// Returns a string to produce disassembly.
    fn format(&self) -> String;
}

/// The address mode describes where in memory an operand is located
#[derive(Clone, Copy, PartialEq)]
pub enum AddressMode {
    // d: Direct page
    Dp,
    // d+X: Direct page, X indexed
    DpXIdx,
    // d+Y: Direct page, X indexed
    DpYIdx,
    // [d+X]: Direct page, X indexed, indirect
    DpXIdxIndirect,
    // [d]+Y: Direct page, indirect, Y indexed
    DpIndirectYIdx,
    // (X): Indirect X
    XIndirect,
    // (Y): Indirect Y
    YIndirect,
    // (X++): X indexed, auto increment
    XIndirectAutoInc,
    // !a: Absolute
    Abs,
    // !a+X: Absolute, X indexed
    AbsXIdx,
    // !a+Y: Absolute, Y indexed
    AbsYIdx,
    // [!a+X]: Absolute, X indexed, indirect
    AbsXIdxIndirect,
}

impl AddressMode {
    /// Decodes the address mode from program memory, and returns the target address and how many
    /// program bytes have been read.
    #[inline]
    fn decode<'a, BusT: Spc700Bus, WrapperT: ReadOrPeekWrapper<'a, BusT>>(
        &self,
        bus: &'a mut WrapperT,
        operand_addr: AddressU16,
    ) -> (AddressU16, u8) {
        let operand_size = match self {
            Self::Dp => 1,
            Self::DpXIdx => 1,
            Self::DpYIdx => 1,
            Self::DpXIdxIndirect => 1,
            Self::DpIndirectYIdx => 1,
            Self::XIndirect => 0,
            Self::YIndirect => 0,
            Self::XIndirectAutoInc => 0,
            Self::Abs => 2,
            Self::AbsXIdx => 2,
            Self::AbsYIdx => 2,
            Self::AbsXIdxIndirect => 2,
        };

        // Regardless of how many bytes were read, store them all as u16 for simplicity.
        let operand_data: u16 = match operand_size {
            0 => 0,
            1 => bus.cycle_read_u8(operand_addr) as u16,
            2 => bus.cycle_read_u16(operand_addr, Wrap::NoWrap),
            _ => unreachable!(),
        };

        let addr = match self {
            AddressMode::Dp => bus.cpu().direct_page_addr(operand_data as u8),
            AddressMode::DpXIdx => {
                bus.cycle_io();
                bus.cpu()
                    .direct_page_addr(operand_data as u8)
                    .add(bus.cpu().x as u16, Wrap::WrapPage)
            }
            AddressMode::DpYIdx => {
                bus.cycle_io();
                bus.cpu()
                    .direct_page_addr(operand_data as u8)
                    .add(bus.cpu().y as u16, Wrap::WrapPage)
            }
            AddressMode::DpXIdxIndirect => {
                bus.cycle_io();
                let addr = bus
                    .cpu()
                    .direct_page_addr(operand_data as u8)
                    .add(bus.cpu().x as u16, Wrap::WrapPage);
                AddressU16(bus.cycle_read_u16(addr, Wrap::WrapPage))
            }
            AddressMode::DpIndirectYIdx => {
                bus.cycle_io();
                let indirect_addr = bus.cpu().direct_page_addr(operand_data as u8);
                let addr = AddressU16(bus.cycle_read_u16(indirect_addr, Wrap::WrapPage));
                addr.add(bus.cpu().y, Wrap::NoWrap)
            }
            AddressMode::XIndirect => bus.cpu().direct_page_addr(bus.cpu().x),
            AddressMode::YIndirect => bus.cpu().direct_page_addr(bus.cpu().y),
            AddressMode::XIndirectAutoInc => bus.cpu().direct_page_addr(bus.cpu().x),
            AddressMode::Abs => AddressU16(operand_data),
            AddressMode::AbsXIdx => {
                bus.cycle_io();
                AddressU16(operand_data).add(bus.cpu().x as u16, Wrap::NoWrap)
            }
            AddressMode::AbsYIdx => {
                bus.cycle_io();
                AddressU16(operand_data).add(bus.cpu().y as u16, Wrap::NoWrap)
            }
            AddressMode::AbsXIdxIndirect => {
                let addr = AddressU16(operand_data).add(bus.cpu().x as u16, Wrap::NoWrap);
                bus.cycle_io();
                AddressU16(bus.cycle_read_u16(addr, Wrap::NoWrap))
            }
        };
        (addr, operand_size)
    }

    /// How memory addresse overflows are wrapped when using this address mode.
    #[inline]
    pub fn wrap_mode(&self) -> Wrap {
        match self {
            Self::Dp => Wrap::WrapPage,
            Self::DpXIdx => Wrap::WrapPage,
            Self::DpYIdx => Wrap::WrapPage,
            Self::DpXIdxIndirect => Wrap::WrapPage,
            Self::DpIndirectYIdx => Wrap::WrapPage,
            Self::XIndirect => Wrap::NoWrap,
            Self::YIndirect => Wrap::NoWrap,
            Self::XIndirectAutoInc => Wrap::NoWrap,
            Self::Abs => Wrap::NoWrap,
            Self::AbsXIdx => Wrap::NoWrap,
            Self::AbsYIdx => Wrap::NoWrap,
            Self::AbsXIdxIndirect => Wrap::NoWrap,
        }
    }
}

/// Defines an 8-bit operand. See [Operand]
#[derive(Clone, Copy, PartialEq)]
pub enum U8Operand {
    Immediate,
    Register(Register),
    Const(u8),
    U8InMemory(AddressMode),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Register {
    A,
    X,
    Y,
    Psw,
    Sp,
}

impl Operand<u8, DecodedU8Operand> for U8Operand {
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
    ) -> (DecodedU8Operand, AddressU16) {
        let (operand, operand_size) = match self {
            U8Operand::Immediate => (
                DecodedU8Operand::Immediate(bus.cycle_read_u8(operand_addr)),
                1,
            ),
            U8Operand::Register(register) => (DecodedU8Operand::Register(*register), 0),
            U8Operand::Const(value) => (DecodedU8Operand::Const(*value), 0),
            U8Operand::U8InMemory(mode) => {
                let (addr, operand_size) = mode.decode(bus, operand_addr);
                (DecodedU8Operand::InMemory(*mode, addr), operand_size)
            }
        };
        (operand, operand_addr.add(operand_size, Wrap::NoWrap))
    }
}

impl U8Operand {
    #[inline]
    pub fn is_address_mode(&self, address_mode: AddressMode) -> bool {
        if let Self::U8InMemory(mode) = self {
            *mode == address_mode
        } else {
            false
        }
    }

    #[inline]
    pub fn is_in_memory(&self) -> bool {
        matches!(self, Self::U8InMemory(_))
    }

    #[inline]
    pub fn is_alu_register(&self) -> bool {
        matches!(
            self,
            Self::Register(Register::A) | Self::Register(Register::X) | Self::Register(Register::Y)
        )
    }
}

#[derive(Copy, Clone)]
pub enum DecodedU8Operand {
    Immediate(u8),
    Register(Register),
    Const(u8),
    InMemory(AddressMode, AddressU16),
}

impl DecodedOperand<u8> for DecodedU8Operand {
    #[inline]
    fn load(&self, cpu: &mut Spc700<impl Spc700Bus>) -> u8 {
        match self {
            Self::Immediate(value) => *value,
            Self::Register(Register::A) => cpu.a,
            Self::Register(Register::X) => cpu.x,
            Self::Register(Register::Y) => cpu.y,
            Self::Register(Register::Sp) => {
                cpu.bus.cycle_read_u8(cpu.pc);
                cpu.sp
            }
            Self::Register(Register::Psw) => cpu.status.into(),
            Self::Const(value) => *value,
            Self::InMemory(AddressMode::XIndirectAutoInc, addr) => {
                cpu.bus.cycle_read_u8(cpu.pc);
                cpu.x = cpu.x.wrapping_add(1);
                let value = cpu.bus.cycle_read_u8(*addr);
                cpu.bus.cycle_io();
                value
            }
            Self::InMemory(_, addr) => cpu.bus.cycle_read_u8(*addr),
        }
    }

    fn store(&self, cpu: &mut Spc700<impl Spc700Bus>, value: u8) {
        match self {
            Self::Immediate(_) => panic!("storing immediate operand"),
            Self::Register(Register::A) => cpu.a = value,
            Self::Register(Register::X) => cpu.x = value,
            Self::Register(Register::Y) => cpu.y = value,
            Self::Register(Register::Sp) => cpu.sp = value,
            Self::Register(Register::Psw) => cpu.status = value.into(),
            Self::Const(_) => panic!("storing const operand"),
            Self::InMemory(AddressMode::XIndirectAutoInc, addr) => {
                cpu.x = cpu.x.wrapping_add(1);
                cpu.bus.cycle_write_u8(*addr, value)
            }
            Self::InMemory(_, addr) => cpu.bus.cycle_write_u8(*addr, value),
        }
    }

    #[inline]
    fn format(&self) -> String {
        match self {
            Self::Immediate(value) => format!("#${:02X}", value),
            Self::Register(Register::A) => "A".to_string(),
            Self::Register(Register::X) => "X".to_string(),
            Self::Register(Register::Y) => "Y".to_string(),
            Self::Register(Register::Sp) => "SP".to_string(),
            Self::Register(Register::Psw) => "PSW".to_string(),
            Self::Const(value) => format!("{:02X}", value),
            Self::InMemory(_, addr) => format!("${:04X}", addr.0),
        }
    }
}

impl DecodedU8Operand {
    #[inline]
    pub fn is_address_mode(&self, address_mode: AddressMode) -> bool {
        if let Self::InMemory(mode, _) = self {
            *mode == address_mode
        } else {
            false
        }
    }

    #[inline]
    pub fn is_alu_register(&self) -> bool {
        matches!(
            self,
            Self::Register(Register::A) | Self::Register(Register::X) | Self::Register(Register::Y)
        )
    }

    #[inline]
    pub fn is_in_memory(&self) -> bool {
        matches!(self, Self::InMemory(_, _))
    }
}

/// Implementation of 16-bit operands. See [Operand]
pub enum U16Operand {
    RegisterYA,
    JumpAddress(AddressMode),
    U16InMemory(AddressMode),
}

impl Operand<u16, DecodedU16Operand> for U16Operand {
    fn decode_impl<'a, BusT: Spc700Bus, WrapperT: ReadOrPeekWrapper<'a, BusT>>(
        &self,
        bus: &'a mut WrapperT,
        operand_addr: AddressU16,
    ) -> (DecodedU16Operand, AddressU16) {
        let (operand, operand_size) = match self {
            Self::RegisterYA => (DecodedU16Operand::RegisterYA, 0),
            Self::JumpAddress(mode) => {
                let (addr, operand_size) = mode.decode(bus, operand_addr);
                (DecodedU16Operand::JumpAddress(*mode, addr), operand_size)
            }
            Self::U16InMemory(mode) => {
                let (addr, operand_size) = mode.decode(bus, operand_addr);
                (DecodedU16Operand::InMemory(*mode, addr), operand_size)
            }
        };
        (operand, operand_addr.add(operand_size, Wrap::NoWrap))
    }
}

#[derive(Copy, Clone)]
pub enum DecodedU16Operand {
    RegisterYA,
    JumpAddress(AddressMode, AddressU16),
    InMemory(AddressMode, AddressU16),
}

impl DecodedOperand<u16> for DecodedU16Operand {
    #[inline]
    fn load(&self, cpu: &mut Spc700<impl Spc700Bus>) -> u16 {
        let low = self.load_low(cpu);
        let high = self.load_high(cpu);
        u16::from_le_bytes([low, high])
    }

    #[inline]
    fn store(&self, cpu: &mut Spc700<impl Spc700Bus>, value: u16) {
        self.store_low(cpu, value.low_byte());
        self.store_high(cpu, value.high_byte());
    }

    #[inline]
    fn format(&self) -> String {
        match self {
            Self::RegisterYA => "YA".to_string(),
            Self::JumpAddress(_, addr) => format!("${:04X}", addr.0),
            Self::InMemory(_, addr) => format!("${:04X}", addr.0),
        }
    }
}

impl DecodedU16Operand {
    #[inline]
    pub fn load_low(&self, cpu: &mut Spc700<impl Spc700Bus>) -> u8 {
        match self {
            Self::JumpAddress(_mode, addr) => addr.0.low_byte(),
            Self::InMemory(_mode, addr) => cpu.bus.cycle_read_u8(*addr),
            Self::RegisterYA => cpu.a,
        }
    }

    #[inline]
    pub fn load_high(&self, cpu: &mut Spc700<impl Spc700Bus>) -> u8 {
        match self {
            Self::JumpAddress(_mode, addr) => addr.0.high_byte(),
            Self::InMemory(mode, addr) => cpu.bus.cycle_read_u8(addr.add(1_u16, mode.wrap_mode())),
            Self::RegisterYA => cpu.y,
        }
    }

    pub fn store_low(&self, cpu: &mut Spc700<impl Spc700Bus>, value: u8) {
        match self {
            Self::JumpAddress(_, _) => panic!("Cannot store into address"),
            Self::InMemory(_mode, addr) => cpu.bus.cycle_write_u8(*addr, value),
            Self::RegisterYA => cpu.a = value,
        }
    }

    pub fn store_high(&self, cpu: &mut Spc700<impl Spc700Bus>, value: u8) {
        match self {
            Self::JumpAddress(_, _) => panic!("Cannot store into address"),
            Self::InMemory(mode, addr) => cpu
                .bus
                .cycle_write_u8(addr.add(1_u16, mode.wrap_mode()), value),
            Self::RegisterYA => cpu.y = value,
        }
    }

    #[inline]
    pub fn is_register(&self) -> bool {
        matches!(self, Self::RegisterYA)
    }
}

/// Implementation of single bit operands. See [Operand]
#[derive(Copy, Clone)]
pub enum BitOperand {
    Carry,
    AbsBit,
    AbsBitInv,
    DpBit(usize),
}

impl Operand<bool, DecodedBitOperand> for BitOperand {
    fn decode_impl<'a, BusT: Spc700Bus, WrapperT: ReadOrPeekWrapper<'a, BusT>>(
        &self,
        bus: &'a mut WrapperT,
        operand_addr: AddressU16,
    ) -> (DecodedBitOperand, AddressU16) {
        let (operand, operand_size) = match self {
            Self::Carry => (DecodedBitOperand::Carry, 0_u16),
            Self::AbsBit => {
                let operand_data = bus.cycle_read_u16(operand_addr, Wrap::NoWrap);
                (
                    DecodedBitOperand::InMemory(
                        AddressU16(operand_data.bits(0..13)),
                        operand_data.bits(13..16) as usize,
                    ),
                    2,
                )
            }
            Self::AbsBitInv => {
                let operand_data = bus.cycle_read_u16(operand_addr, Wrap::NoWrap);
                (
                    DecodedBitOperand::InMemoryInv(
                        AddressU16(operand_data.bits(0..13)),
                        operand_data.bits(13..16) as usize,
                    ),
                    2,
                )
            }
            Self::DpBit(bit) => {
                let dp_addr = bus.cycle_read_u8(operand_addr);
                (
                    DecodedBitOperand::InMemory(bus.cpu().direct_page_addr(dp_addr), *bit),
                    1,
                )
            }
        };
        (operand, operand_addr.add(operand_size, Wrap::NoWrap))
    }
}

impl BitOperand {
    #[inline]
    pub fn is_carry(&self) -> bool {
        matches!(self, Self::Carry)
    }
}

#[derive(Copy, Clone)]
pub enum DecodedBitOperand {
    Carry,
    InMemory(AddressU16, usize),
    InMemoryInv(AddressU16, usize),
}

impl DecodedOperand<bool> for DecodedBitOperand {
    #[inline]
    fn load(&self, cpu: &mut Spc700<impl Spc700Bus>) -> bool {
        match self {
            Self::Carry => cpu.status.carry,
            Self::InMemory(addr, bit) => cpu.bus.cycle_read_u8(*addr).bit(*bit),
            Self::InMemoryInv(addr, bit) => !cpu.bus.cycle_read_u8(*addr).bit(*bit),
        }
    }

    #[inline]
    fn store(&self, cpu: &mut Spc700<impl Spc700Bus>, value: bool) {
        match self {
            Self::Carry => cpu.status.carry = value,
            Self::InMemory(addr, bit) => {
                let mut data = cpu.bus.cycle_read_u8(*addr);
                data.set_bit(*bit, value);
                cpu.bus.cycle_write_u8(*addr, data);
            }
            Self::InMemoryInv(addr, bit) => {
                let mut data = cpu.bus.cycle_read_u8(*addr);
                data.set_bit(*bit, !value);
                cpu.bus.cycle_write_u8(*addr, data);
            }
        }
    }

    #[inline]
    fn format(&self) -> String {
        match self {
            Self::Carry => "C".to_string(),
            Self::InMemory(addr, bit) => format!("(${:04X}.{})", addr.0, bit),
            Self::InMemoryInv(addr, bit) => format!("(/${:04X}.{})", addr.0, bit),
        }
    }
}

impl DecodedBitOperand {
    #[inline]
    pub fn is_carry(&self) -> bool {
        matches!(self, Self::Carry)
    }

    #[inline]
    pub fn peek(&self, cpu: &mut Spc700<impl Spc700Bus>) -> bool {
        match self {
            Self::Carry => cpu.status.carry,
            Self::InMemory(addr, bit) => cpu.bus.peek_u8(*addr).unwrap_or_default().bit(*bit),
            Self::InMemoryInv(addr, bit) => !cpu.bus.peek_u8(*addr).unwrap_or_default().bit(*bit),
        }
    }
}

/// A wrapper around a CPU that will either perform reads on a mutable bus or
/// peeks on an immutable bus.
///
/// This allows logic for decoding of operands to be re-used for execution (mutable bus)
/// and disassembly generation (immutable bus).
pub trait ReadOrPeekWrapper<'a, T: Spc700Bus>
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
