use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;

/// Metadata about a decoded instruction. Used to generate disassembly.
pub struct InstructionMeta {
    pub address: AddressU16,
    pub operation: &'static str,
    pub operand_str: Option<String>,
    pub effective_addr: Option<AddressU16>,
}

/// An entry in the opcode table
pub struct InstructionDef<BusT: Spc700Bus> {
    /// Execute the instruction on the provided CPU.
    pub execute: fn(&mut Spc700<BusT>),

    /// Return metadata about this instruction. Can be used on an immutable CPU.
    pub meta: fn(&Spc700<BusT>, AddressU16) -> (InstructionMeta, AddressU16),
}

pub fn build_opcode_table<BusT: Spc700Bus>() -> [InstructionDef<BusT>; 256] {
    macro_rules! instruction {
        // Operand-less instruction
        ($method: ident) => {
            InstructionDef::<BusT> {
                execute: |cpu| {
                    $method(cpu);
                },
                meta: |_, operand_addr| {
                    (
                        InstructionMeta {
                            address: operand_addr,
                            operation: stringify!($method),
                            operand_str: None,
                            effective_addr: None,
                        },
                        operand_addr,
                    )
                },
            }
        };
        // Single operand instruction
        ($method: ident, $operand_def: expr) => {
            InstructionDef::<BusT> {
                execute: |cpu| {
                    $method(cpu, $operand_def);
                },
                meta: |cpu, operand_addr| {
                    let (operand, next_addr) = $operand_def.peek(cpu, operand_addr);
                    (
                        InstructionMeta {
                            address: operand_addr,
                            operation: stringify!($method),
                            operand_str: Some(operand.format()),
                            effective_addr: operand.effective_addr(),
                        },
                        next_addr,
                    )
                },
            }
        };
        // Two operand instruction
        ($method: ident, $left_def: expr, $right_def: expr) => {
            InstructionDef::<BusT> {
                execute: |cpu| {
                    $method(cpu, $left_def, $right_def);
                },
                meta: |cpu, operand_addr| {
                    let (right, next_addr) = $right_def.peek(cpu, operand_addr);
                    let (left, next_addr) = $left_def.peek(cpu, next_addr);
                    (
                        InstructionMeta {
                            address: operand_addr,
                            operation: stringify!($method),
                            operand_str: Some(left.format() + ", " + &right.format()),
                            effective_addr: None,
                        },
                        next_addr,
                    )
                },
            }
        };
    }

    let mut opcodes = [(); 256].map(|_| InstructionDef::<BusT> {
        execute: |_| {},
        meta: |_, instruction_addr| {
            (
                InstructionMeta {
                    address: instruction_addr,
                    operation: "ill",
                    operand_str: None,
                    effective_addr: None,
                },
                instruction_addr,
            )
        },
    });

    use crate::spc700::instructions::*;
    use crate::spc700::operands::AddressMode::*;
    use crate::spc700::operands::OperandDef::*;
    use crate::spc700::operands::Register::*;
    opcodes[0x00] = instruction!(nop);
    opcodes[0x01] = instruction!(tcall, Const(0));
    opcodes[0x02] = instruction!(set1, DpBit(0));
    opcodes[0x03] = instruction!(bbs, Immediate, DpBit(0));
    opcodes[0x04] = instruction!(or, Register(A), InMemory(Dp));
    opcodes[0x05] = instruction!(or, Register(A), InMemory(Abs));
    opcodes[0x06] = instruction!(or, Register(A), InMemory(XIndirect));
    opcodes[0x07] = instruction!(or, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0x08] = instruction!(or, Register(A), Immediate);
    opcodes[0x09] = instruction!(or, InMemory(Dp), InMemory(Dp));
    opcodes[0x0A] = instruction!(or1, AbsoluteBit);
    opcodes[0x0B] = instruction!(asl, InMemory(Dp));
    opcodes[0x0C] = instruction!(asl, InMemory(Abs));
    opcodes[0x0D] = instruction!(push, Register(Psw));
    opcodes[0x0E] = instruction!(tset1, InMemory(Abs));
    opcodes[0x0F] = instruction!(brk);
    opcodes[0x10] = instruction!(bpl, Immediate);
    opcodes[0x11] = instruction!(tcall, Const(1));
    opcodes[0x12] = instruction!(clr1, DpBit(0));
    opcodes[0x13] = instruction!(bbc, Immediate, DpBit(0));
    opcodes[0x14] = instruction!(or, Register(A), InMemory(DpXIdx));
    opcodes[0x15] = instruction!(or, Register(A), InMemory(AbsXIdx));
    opcodes[0x16] = instruction!(or, Register(A), InMemory(AbsYIdx));
    opcodes[0x17] = instruction!(or, Register(A), InMemory(DpIndirectYIdx));
    opcodes[0x18] = instruction!(or, InMemory(Dp), Immediate);
    opcodes[0x19] = instruction!(or, InMemory(XIndirect), InMemory(YIndirect));
    opcodes[0x1A] = instruction!(decw, InMemory(Dp));
    opcodes[0x1B] = instruction!(asl, InMemory(DpXIdx));
    opcodes[0x1C] = instruction!(asl, Register(A));
    opcodes[0x1D] = instruction!(dec, Register(X));
    opcodes[0x1E] = instruction!(cmp, Register(X), InMemory(Abs));
    opcodes[0x1F] = instruction!(jmp, InMemory(AbsXIdxIndirect));
    opcodes[0x20] = instruction!(clrp);
    opcodes[0x21] = instruction!(tcall, Const(2));
    opcodes[0x22] = instruction!(set1, DpBit(1));
    opcodes[0x23] = instruction!(bbs, Immediate, DpBit(1));
    opcodes[0x24] = instruction!(and, Register(A), InMemory(Dp));
    opcodes[0x25] = instruction!(and, Register(A), InMemory(Abs));
    opcodes[0x26] = instruction!(and, Register(A), InMemory(XIndirect));
    opcodes[0x27] = instruction!(and, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0x28] = instruction!(and, Register(A), Immediate);
    opcodes[0x29] = instruction!(and, InMemory(Dp), InMemory(Dp));
    opcodes[0x2A] = instruction!(or1, AbsoluteBitInv);
    opcodes[0x2B] = instruction!(rol, InMemory(Dp));
    opcodes[0x2C] = instruction!(rol, InMemory(Abs));
    opcodes[0x2D] = instruction!(push, Register(A));
    opcodes[0x2E] = instruction!(cbne, Immediate, InMemory(Dp));
    opcodes[0x2F] = instruction!(bra, Immediate);
    opcodes[0x30] = instruction!(bmi, Immediate);
    opcodes[0x31] = instruction!(tcall, Const(3));
    opcodes[0x32] = instruction!(clr1, DpBit(1));
    opcodes[0x33] = instruction!(bbc, Immediate, DpBit(1));
    opcodes[0x34] = instruction!(and, Register(A), InMemory(DpXIdx));
    opcodes[0x35] = instruction!(and, Register(A), InMemory(AbsXIdx));
    opcodes[0x36] = instruction!(and, Register(A), InMemory(AbsYIdx));
    opcodes[0x37] = instruction!(and, Register(A), InMemory(DpIndirectYIdx));
    opcodes[0x38] = instruction!(and, InMemory(Dp), Immediate);
    opcodes[0x39] = instruction!(and, InMemory(XIndirect), InMemory(YIndirect));
    opcodes[0x3A] = instruction!(incw, InMemory(Dp));
    opcodes[0x3B] = instruction!(rol, InMemory(DpXIdx));
    opcodes[0x3C] = instruction!(rol, Register(A));
    opcodes[0x3D] = instruction!(inc, Register(X));
    opcodes[0x3E] = instruction!(cmp, Register(X), InMemory(Dp));
    opcodes[0x3F] = instruction!(call, InMemory(Abs));
    opcodes[0x40] = instruction!(setp);
    opcodes[0x41] = instruction!(tcall, Const(4));
    opcodes[0x42] = instruction!(set1, DpBit(2));
    opcodes[0x43] = instruction!(bbs, Immediate, DpBit(2));
    opcodes[0x44] = instruction!(eor, Register(A), InMemory(Dp));
    opcodes[0x45] = instruction!(eor, Register(A), InMemory(Abs));
    opcodes[0x46] = instruction!(eor, Register(A), InMemory(XIndirect));
    opcodes[0x47] = instruction!(eor, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0x48] = instruction!(eor, Register(A), Immediate);
    opcodes[0x49] = instruction!(eor, InMemory(Dp), InMemory(Dp));
    opcodes[0x4A] = instruction!(and1, AbsoluteBit);
    opcodes[0x4B] = instruction!(lsr, InMemory(Dp));
    opcodes[0x4C] = instruction!(lsr, InMemory(Abs));
    opcodes[0x4D] = instruction!(push, Register(X));
    opcodes[0x4E] = instruction!(tclr1, InMemory(Abs));
    opcodes[0x4F] = instruction!(pcall, Immediate);
    opcodes[0x50] = instruction!(bvc, Immediate);
    opcodes[0x51] = instruction!(tcall, Const(5));
    opcodes[0x52] = instruction!(clr1, DpBit(2));
    opcodes[0x53] = instruction!(bbc, Immediate, DpBit(2));
    opcodes[0x54] = instruction!(eor, Register(A), InMemory(DpXIdx));
    opcodes[0x55] = instruction!(eor, Register(A), InMemory(AbsXIdx));
    opcodes[0x56] = instruction!(eor, Register(A), InMemory(AbsYIdx));
    opcodes[0x57] = instruction!(eor, Register(A), InMemory(DpIndirectYIdx));
    opcodes[0x58] = instruction!(eor, InMemory(Dp), Immediate);
    opcodes[0x59] = instruction!(eor, InMemory(XIndirect), InMemory(YIndirect));
    opcodes[0x5A] = instruction!(cmpw, Register(YA), InMemory(Dp));
    opcodes[0x5B] = instruction!(lsr, InMemory(DpXIdx));
    opcodes[0x5C] = instruction!(lsr, Register(A));
    opcodes[0x5D] = instruction!(mov, Register(X), Register(A));
    opcodes[0x5E] = instruction!(cmp, Register(Y), InMemory(Abs));
    opcodes[0x5F] = instruction!(jmp, InMemory(Abs));
    opcodes[0x60] = instruction!(clrc);
    opcodes[0x61] = instruction!(tcall, Const(6));
    opcodes[0x62] = instruction!(set1, DpBit(3));
    opcodes[0x63] = instruction!(bbs, Immediate, DpBit(3));
    opcodes[0x64] = instruction!(cmp, Register(A), InMemory(Dp));
    opcodes[0x65] = instruction!(cmp, Register(A), InMemory(Abs));
    opcodes[0x66] = instruction!(cmp, Register(A), InMemory(XIndirect));
    opcodes[0x67] = instruction!(cmp, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0x68] = instruction!(cmp, Register(A), Immediate);
    opcodes[0x69] = instruction!(cmp, InMemory(Dp), InMemory(Dp));
    opcodes[0x6A] = instruction!(and1, AbsoluteBitInv);
    opcodes[0x6B] = instruction!(ror, InMemory(Dp));
    opcodes[0x6C] = instruction!(ror, InMemory(Abs));
    opcodes[0x6D] = instruction!(push, Register(Y));
    opcodes[0x6E] = instruction!(dbnz, InMemory(Dp), Immediate);
    opcodes[0x6F] = instruction!(ret);
    opcodes
}
