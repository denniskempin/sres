//! Each instruction in the [opcode table](build_opcode_table) has an associated
//!
//! address mode, which is decoded here to handle how the operand is loaded and stored.
use core::panic;

use intbits::Bits;

use super::Spc700;
use super::Spc700Bus;
use crate::common::address::Address;
use crate::common::address::AddressU16;
use crate::common::address::Wrap;
use crate::common::uint::U16Ext;

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
#[derive(Clone, Copy, PartialEq)]
pub enum Operand {
    Immediate,
    Register(Register),
    Const(u8),
    InMemory(AddressMode),
    JumpAddress(AddressMode),
    Relative,
    Carry,
    AbsBit,
    AbsBitInv,
    DpBit(usize),
}

impl Operand {
    /// Consumes program bytes to decode an operand from the bus.
    #[inline]
    pub fn decode(&self, cpu: &mut Spc700<impl Spc700Bus>) -> DecodedOperand {
        match self {
            Self::Immediate => DecodedOperand::Immediate(cpu.fetch_program_u8()),
            Self::Register(register) => DecodedOperand::Register(*register),
            Self::Const(value) => DecodedOperand::Const(*value),
            Self::InMemory(mode) => DecodedOperand::InMemory(*mode, mode.decode(cpu)),
            Self::JumpAddress(mode) => DecodedOperand::JumpAddress(*mode, mode.decode(cpu)),
            Self::Relative => {
                let relative = cpu.fetch_program_u8() as i8;
                DecodedOperand::Relative(
                    relative,
                    cpu.pc
                        .add(1_u16, Wrap::NoWrap)
                        .add_signed(relative.into(), Wrap::NoWrap),
                )
            }
            Self::Carry => DecodedOperand::Carry,
            Self::AbsBit => {
                let operand_data = cpu.fetch_program_u16();
                DecodedOperand::BitInMemory(
                    AddressU16(operand_data.bits(0..13)),
                    operand_data.bits(13..16) as usize,
                )
            }
            Self::AbsBitInv => {
                let operand_data = cpu.fetch_program_u16();
                DecodedOperand::BitInMemoryInv(
                    AddressU16(operand_data.bits(0..13)),
                    operand_data.bits(13..16) as usize,
                )
            }
            Self::DpBit(bit) => {
                let dp_addr = cpu.fetch_program_u8();
                DecodedOperand::BitInMemory(cpu.direct_page_addr(dp_addr), *bit)
            }
        }
    }

    #[inline]
    pub fn disassembly(
        &self,
        cpu: &Spc700<impl Spc700Bus>,
        addr: AddressU16,
    ) -> (String, AddressU16) {
        let operand_size: u16 = match self {
            Operand::Immediate => 1,
            Operand::Register(_) => 0,
            Operand::Const(_) => 0,
            Operand::Carry => 0,
            Operand::InMemory(mode) => mode.operand_size(),
            Operand::JumpAddress(mode) => mode.operand_size(),
            Operand::Relative => 1,
            Operand::AbsBit => 2,
            Operand::AbsBitInv => 2,
            Operand::DpBit(_) => 1,
        };
        let value: u16 = match operand_size {
            0 => 0,
            1 => cpu.bus.peek_u8(addr).unwrap_or_default() as u16,
            2 => cpu.bus.peek_u16(addr, Wrap::NoWrap).unwrap_or_default(),
            _ => unreachable!(),
        };
        let disassembly = match self {
            Self::Immediate => format!("#${:02x}", value),
            Self::Register(register) => format!("{:}", register),
            Self::Const(value) => format!("{:02x}", value),
            Self::InMemory(mode) => mode.disassembly(cpu, addr),
            Self::JumpAddress(mode) => mode.disassembly(cpu, addr),
            Self::Relative => {
                let relative = cpu.bus.peek_u8(addr).unwrap_or_default() as i8;
                format!(
                    "${:04x}",
                    addr.add(1_u16, Wrap::NoWrap)
                        .add_signed(relative.into(), Wrap::NoWrap)
                        .0
                )
            }
            Self::Carry => "C".to_string(),
            Self::AbsBit => format!("${:04x}.{}", value.bits(0..13), value.bits(13..16)),
            Self::AbsBitInv => format!("/${:04x}.{}", value.bits(0..13), value.bits(13..16)),
            Self::DpBit(bit) => format!("${:02x}.{}", value, bit),
        };
        (disassembly, addr.add(operand_size, Wrap::NoWrap))
    }

    #[inline]
    pub fn is_address_mode(&self, address_mode: AddressMode) -> bool {
        if let Self::InMemory(mode) = self {
            *mode == address_mode
        } else {
            false
        }
    }

    #[inline]
    pub fn is_in_memory(&self) -> bool {
        matches!(self, Self::InMemory(_))
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
pub enum DecodedOperand {
    Immediate(u8),
    Register(Register),
    Const(u8),
    InMemory(AddressMode, AddressU16),
    JumpAddress(AddressMode, AddressU16),
    Relative(i8, AddressU16),
    Carry,
    BitInMemory(AddressU16, usize),
    BitInMemoryInv(AddressU16, usize),
}

impl DecodedOperand {
    #[inline]
    pub fn load(&self, cpu: &mut Spc700<impl Spc700Bus>) -> u8 {
        match self {
            Self::Immediate(value) => *value,
            Self::Register(Register::A) => cpu.a,
            Self::Register(Register::X) => cpu.x,
            Self::Register(Register::Y) => cpu.y,
            Self::Register(Register::YA) => cpu.a,
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
            Self::JumpAddress(_, addr) => addr.0.low_byte(),
            Self::Relative(_, addr) => addr.0.low_byte(),
            Self::Carry => cpu.status.into(),
            Self::BitInMemory(addr, _bit) => cpu.bus.cycle_read_u8(*addr),
            Self::BitInMemoryInv(addr, _bit) => !cpu.bus.cycle_read_u8(*addr),
        }
    }

    #[inline]
    pub fn store(&self, cpu: &mut Spc700<impl Spc700Bus>, value: u8) {
        match self {
            Self::Immediate(_) => panic!("storing immediate operand"),
            Self::Register(Register::A) => cpu.a = value,
            Self::Register(Register::X) => cpu.x = value,
            Self::Register(Register::Y) => cpu.y = value,
            Self::Register(Register::YA) => cpu.a = value,
            Self::Register(Register::Sp) => cpu.sp = value,
            Self::Register(Register::Psw) => cpu.status = value.into(),
            Self::Const(_) => panic!("storing const operand"),
            Self::InMemory(AddressMode::XIndirectAutoInc, addr) => {
                cpu.x = cpu.x.wrapping_add(1);
                cpu.bus.cycle_write_u8(*addr, value)
            }
            Self::InMemory(_, addr) => cpu.bus.cycle_write_u8(*addr, value),
            Self::JumpAddress(_, _) => panic!("read only"),
            Self::Relative(_, _addr) => panic!("read only"),
            Self::Carry => cpu.status = value.into(),
            Self::BitInMemory(addr, _bit) => {
                cpu.bus.cycle_write_u8(*addr, value);
            }
            Self::BitInMemoryInv(addr, _bit) => {
                cpu.bus.cycle_write_u8(*addr, value);
            }
        }
    }

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
            Self::Register(Register::A)
                | Self::Register(Register::X)
                | Self::Register(Register::Y)
                | Self::Register(Register::YA)
        )
    }

    #[inline]
    pub fn is_in_memory(&self) -> bool {
        matches!(self, Self::InMemory(_, _))
    }

    #[inline]
    pub fn load_u16(&self, cpu: &mut Spc700<impl Spc700Bus>) -> u16 {
        u16::from_le_bytes([self.load(cpu), self.load_high(cpu)])
    }

    #[inline]
    pub fn store_u16(&self, cpu: &mut Spc700<impl Spc700Bus>, value: u16) {
        self.store(cpu, value.low_byte());
        self.store_high(cpu, value.high_byte());
    }

    #[inline]
    pub fn load_high(&self, cpu: &mut Spc700<impl Spc700Bus>) -> u8 {
        match self {
            Self::JumpAddress(_, addr) => addr.0.high_byte(),
            Self::Relative(_, addr) => addr.0.high_byte(),
            Self::InMemory(mode, addr) => cpu.bus.cycle_read_u8(addr.add(1_u16, mode.wrap_mode())),
            Self::Register(Register::YA) => cpu.y,
            _ => panic!("Not a u16 operand"),
        }
    }

    #[inline]
    pub fn store_high(&self, cpu: &mut Spc700<impl Spc700Bus>, value: u8) {
        match self {
            Self::InMemory(mode, addr) => cpu
                .bus
                .cycle_write_u8(addr.add(1_u16, mode.wrap_mode()), value),
            Self::Register(Register::YA) => cpu.y = value,
            _ => panic!("Not a writeable u16 operand"),
        }
    }

    #[inline]
    pub fn bit(&self) -> usize {
        match self {
            Self::Carry => 0,
            Self::BitInMemory(_, bit) => *bit,
            Self::BitInMemoryInv(_, bit) => *bit,
            _ => panic!("Not a bit operand"),
        }
    }
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
    /// Consumes program bytes to decode the operand data.
    #[inline]
    pub fn decode(&self, cpu: &mut Spc700<impl Spc700Bus>) -> AddressU16 {
        // Regardless of how many bytes were read, store them all as u16 for simplicity.
        let operand_size = self.operand_size();
        let operand_data: u16 = match operand_size {
            0 => 0,
            1 => cpu.fetch_program_u8() as u16,
            2 => cpu.fetch_program_u16(),
            _ => unreachable!(),
        };

        match self {
            AddressMode::Dp => cpu.direct_page_addr(operand_data as u8),
            AddressMode::DpXIdx => {
                cpu.bus.cycle_io();
                cpu.direct_page_addr(operand_data as u8)
                    .add(cpu.x as u16, Wrap::WrapPage)
            }
            AddressMode::DpYIdx => {
                cpu.bus.cycle_io();
                cpu.direct_page_addr(operand_data as u8)
                    .add(cpu.y as u16, Wrap::WrapPage)
            }
            AddressMode::DpXIdxIndirect => {
                cpu.bus.cycle_io();
                let addr = cpu
                    .direct_page_addr(operand_data as u8)
                    .add(cpu.x as u16, Wrap::WrapPage);
                AddressU16(cpu.bus.cycle_read_u16(addr, Wrap::WrapPage))
            }
            AddressMode::DpIndirectYIdx => {
                cpu.bus.cycle_io();
                let indirect_addr = cpu.direct_page_addr(operand_data as u8);
                let addr = AddressU16(cpu.bus.cycle_read_u16(indirect_addr, Wrap::WrapPage));
                addr.add(cpu.y, Wrap::NoWrap)
            }
            AddressMode::XIndirect => cpu.direct_page_addr(cpu.x),
            AddressMode::YIndirect => cpu.direct_page_addr(cpu.y),
            AddressMode::XIndirectAutoInc => cpu.direct_page_addr(cpu.x),
            AddressMode::Abs => AddressU16(operand_data),
            AddressMode::AbsXIdx => {
                cpu.bus.cycle_io();
                AddressU16(operand_data).add(cpu.x as u16, Wrap::NoWrap)
            }
            AddressMode::AbsYIdx => {
                cpu.bus.cycle_io();
                AddressU16(operand_data).add(cpu.y as u16, Wrap::NoWrap)
            }
            AddressMode::AbsXIdxIndirect => {
                let addr = AddressU16(operand_data).add(cpu.x as u16, Wrap::NoWrap);
                cpu.bus.cycle_io();
                AddressU16(cpu.bus.cycle_read_u16(addr, Wrap::NoWrap))
            }
        }
    }

    #[inline]
    pub fn disassembly(&self, cpu: &Spc700<impl Spc700Bus>, addr: AddressU16) -> String {
        let operand_size: u16 = self.operand_size();
        let value: u16 = match operand_size {
            0 => 0,
            1 => cpu.bus.peek_u8(addr).unwrap_or_default() as u16,
            2 => cpu.bus.peek_u16(addr, Wrap::NoWrap).unwrap_or_default(),
            _ => unreachable!(),
        };

        match self {
            Self::Dp => {
                if cpu.status.direct_page {
                    format!("$1{:02x}", value)
                } else {
                    format!("$0{:02x}", value)
                }
            }
            Self::DpXIdx => format!("${:02x}+x", value),
            Self::DpYIdx => format!("${:02x}+y", value),
            Self::DpXIdxIndirect => format!("[${:02x}+x]", value),
            Self::DpIndirectYIdx => format!("[${:02x}]+y", value),
            Self::XIndirect => "(x)".to_string(),
            Self::YIndirect => "(y)".to_string(),
            Self::XIndirectAutoInc => "(x++)".to_string(),
            Self::Abs => format!("${:04x}", value),
            Self::AbsXIdx => format!("${:04x}+x", value),
            Self::AbsYIdx => format!("${:04x}+y", value),
            Self::AbsXIdxIndirect => format!("[${:04x}+x]", value),
        }
    }

    pub fn operand_size(&self) -> u16 {
        match self {
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
        }
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

#[derive(Clone, Copy, PartialEq, strum::Display)]
pub enum Register {
    A,
    X,
    Y,
    YA,
    Psw,
    Sp,
}
