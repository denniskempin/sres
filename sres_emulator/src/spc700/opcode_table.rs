use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::operands::DecodedOperand;
use crate::spc700::operands::Operand;
use crate::spc700::Spc700;
use crate::spc700::Spc700Bus;

/// Metadata about a decoded instruction. Used to generate disassembly.
pub struct InstructionMeta {
    pub address: AddressU16,
    pub operation: &'static str,
    pub operand_str: Option<String>,
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
                },
                instruction_addr,
            )
        },
    });

    use crate::spc700::instructions::*;
    use crate::spc700::operands::AddressMode::*;
    use crate::spc700::operands::BitOperand::*;
    use crate::spc700::operands::Register::*;
    use crate::spc700::operands::U16Operand::*;
    use crate::spc700::operands::U8Operand::*;
    opcodes[0x00] = instruction!(nop);
    opcodes[0x01] = instruction!(tcall, Const(0));
    opcodes[0x02] = instruction!(set1, DpBit(0));
    opcodes[0x03] = instruction!(bbs, Immediate, DpBit(0));
    opcodes[0x04] = instruction!(or, Register(A), U8InMemory(Dp));
    opcodes[0x05] = instruction!(or, Register(A), U8InMemory(Abs));
    opcodes[0x06] = instruction!(or, Register(A), U8InMemory(XIndirect));
    opcodes[0x07] = instruction!(or, Register(A), U8InMemory(DpXIdxIndirect));
    opcodes[0x08] = instruction!(or, Register(A), Immediate);
    opcodes[0x09] = instruction!(or, U8InMemory(Dp), U8InMemory(Dp));
    opcodes[0x0A] = instruction!(or1, AbsBit);
    opcodes[0x0B] = instruction!(asl, U8InMemory(Dp));
    opcodes[0x0C] = instruction!(asl, U8InMemory(Abs));
    opcodes[0x0D] = instruction!(push, Register(Psw));
    opcodes[0x0E] = instruction!(tset1, U8InMemory(Abs));
    opcodes[0x0F] = instruction!(brk);
    opcodes[0x10] = instruction!(bpl, Immediate);
    opcodes[0x11] = instruction!(tcall, Const(1));
    opcodes[0x12] = instruction!(clr1, DpBit(0));
    opcodes[0x13] = instruction!(bbc, Immediate, DpBit(0));
    opcodes[0x14] = instruction!(or, Register(A), U8InMemory(DpXIdx));
    opcodes[0x15] = instruction!(or, Register(A), U8InMemory(AbsXIdx));
    opcodes[0x16] = instruction!(or, Register(A), U8InMemory(AbsYIdx));
    opcodes[0x17] = instruction!(or, Register(A), U8InMemory(DpIndirectYIdx));
    opcodes[0x18] = instruction!(or, U8InMemory(Dp), Immediate);
    opcodes[0x19] = instruction!(or, U8InMemory(XIndirect), U8InMemory(YIndirect));
    opcodes[0x1A] = instruction!(decw, U16InMemory(Dp));
    opcodes[0x1B] = instruction!(asl, U8InMemory(DpXIdx));
    opcodes[0x1C] = instruction!(asl, Register(A));
    opcodes[0x1D] = instruction!(dec, Register(X));
    opcodes[0x1E] = instruction!(cmp, Register(X), U8InMemory(Abs));
    opcodes[0x1F] = instruction!(jmp, JumpAddress(AbsXIdxIndirect));
    opcodes[0x20] = instruction!(clrp);
    opcodes[0x21] = instruction!(tcall, Const(2));
    opcodes[0x22] = instruction!(set1, DpBit(1));
    opcodes[0x23] = instruction!(bbs, Immediate, DpBit(1));
    opcodes[0x24] = instruction!(and, Register(A), U8InMemory(Dp));
    opcodes[0x25] = instruction!(and, Register(A), U8InMemory(Abs));
    opcodes[0x26] = instruction!(and, Register(A), U8InMemory(XIndirect));
    opcodes[0x27] = instruction!(and, Register(A), U8InMemory(DpXIdxIndirect));
    opcodes[0x28] = instruction!(and, Register(A), Immediate);
    opcodes[0x29] = instruction!(and, U8InMemory(Dp), U8InMemory(Dp));
    opcodes[0x2A] = instruction!(or1, AbsBitInv);
    opcodes[0x2B] = instruction!(rol, U8InMemory(Dp));
    opcodes[0x2C] = instruction!(rol, U8InMemory(Abs));
    opcodes[0x2D] = instruction!(push, Register(A));
    opcodes[0x2E] = instruction!(cbne, Immediate, U8InMemory(Dp));
    opcodes[0x2F] = instruction!(bra, Immediate);
    opcodes[0x30] = instruction!(bmi, Immediate);
    opcodes[0x31] = instruction!(tcall, Const(3));
    opcodes[0x32] = instruction!(clr1, DpBit(1));
    opcodes[0x33] = instruction!(bbc, Immediate, DpBit(1));
    opcodes[0x34] = instruction!(and, Register(A), U8InMemory(DpXIdx));
    opcodes[0x35] = instruction!(and, Register(A), U8InMemory(AbsXIdx));
    opcodes[0x36] = instruction!(and, Register(A), U8InMemory(AbsYIdx));
    opcodes[0x37] = instruction!(and, Register(A), U8InMemory(DpIndirectYIdx));
    opcodes[0x38] = instruction!(and, U8InMemory(Dp), Immediate);
    opcodes[0x39] = instruction!(and, U8InMemory(XIndirect), U8InMemory(YIndirect));
    opcodes[0x3A] = instruction!(incw, U16InMemory(Dp));
    opcodes[0x3B] = instruction!(rol, U8InMemory(DpXIdx));
    opcodes[0x3C] = instruction!(rol, Register(A));
    opcodes[0x3D] = instruction!(inc, Register(X));
    opcodes[0x3E] = instruction!(cmp, Register(X), U8InMemory(Dp));
    opcodes[0x3F] = instruction!(call, JumpAddress(Abs));
    opcodes[0x40] = instruction!(setp);
    opcodes[0x41] = instruction!(tcall, Const(4));
    opcodes[0x42] = instruction!(set1, DpBit(2));
    opcodes[0x43] = instruction!(bbs, Immediate, DpBit(2));
    opcodes[0x44] = instruction!(eor, Register(A), U8InMemory(Dp));
    opcodes[0x45] = instruction!(eor, Register(A), U8InMemory(Abs));
    opcodes[0x46] = instruction!(eor, Register(A), U8InMemory(XIndirect));
    opcodes[0x47] = instruction!(eor, Register(A), U8InMemory(DpXIdxIndirect));
    opcodes[0x48] = instruction!(eor, Register(A), Immediate);
    opcodes[0x49] = instruction!(eor, U8InMemory(Dp), U8InMemory(Dp));
    opcodes[0x4A] = instruction!(and1, AbsBit);
    opcodes[0x4B] = instruction!(lsr, U8InMemory(Dp));
    opcodes[0x4C] = instruction!(lsr, U8InMemory(Abs));
    opcodes[0x4D] = instruction!(push, Register(X));
    opcodes[0x4E] = instruction!(tclr1, U8InMemory(Abs));
    opcodes[0x4F] = instruction!(pcall, Immediate);
    opcodes[0x50] = instruction!(bvc, Immediate);
    opcodes[0x51] = instruction!(tcall, Const(5));
    opcodes[0x52] = instruction!(clr1, DpBit(2));
    opcodes[0x53] = instruction!(bbc, Immediate, DpBit(2));
    opcodes[0x54] = instruction!(eor, Register(A), U8InMemory(DpXIdx));
    opcodes[0x55] = instruction!(eor, Register(A), U8InMemory(AbsXIdx));
    opcodes[0x56] = instruction!(eor, Register(A), U8InMemory(AbsYIdx));
    opcodes[0x57] = instruction!(eor, Register(A), U8InMemory(DpIndirectYIdx));
    opcodes[0x58] = instruction!(eor, U8InMemory(Dp), Immediate);
    opcodes[0x59] = instruction!(eor, U8InMemory(XIndirect), U8InMemory(YIndirect));
    opcodes[0x5A] = instruction!(cmpw, RegisterYA, U16InMemory(Dp));
    opcodes[0x5B] = instruction!(lsr, U8InMemory(DpXIdx));
    opcodes[0x5C] = instruction!(lsr, Register(A));
    opcodes[0x5D] = instruction!(mov, Register(X), Register(A));
    opcodes[0x5E] = instruction!(cmp, Register(Y), U8InMemory(Abs));
    opcodes[0x5F] = instruction!(jmp, JumpAddress(Abs));
    opcodes[0x60] = instruction!(clrc);
    opcodes[0x61] = instruction!(tcall, Const(6));
    opcodes[0x62] = instruction!(set1, DpBit(3));
    opcodes[0x63] = instruction!(bbs, Immediate, DpBit(3));
    opcodes[0x64] = instruction!(cmp, Register(A), U8InMemory(Dp));
    opcodes[0x65] = instruction!(cmp, Register(A), U8InMemory(Abs));
    opcodes[0x66] = instruction!(cmp, Register(A), U8InMemory(XIndirect));
    opcodes[0x67] = instruction!(cmp, Register(A), U8InMemory(DpXIdxIndirect));
    opcodes[0x68] = instruction!(cmp, Register(A), Immediate);
    opcodes[0x69] = instruction!(cmp, U8InMemory(Dp), U8InMemory(Dp));
    opcodes[0x6A] = instruction!(and1, AbsBitInv);
    opcodes[0x6B] = instruction!(ror, U8InMemory(Dp));
    opcodes[0x6C] = instruction!(ror, U8InMemory(Abs));
    opcodes[0x6D] = instruction!(push, Register(Y));
    opcodes[0x6E] = instruction!(dbnz, U8InMemory(Dp), Immediate);
    opcodes[0x6F] = instruction!(ret);
    opcodes[0x70] = instruction!(bvs, Immediate);
    opcodes[0x71] = instruction!(tcall, Const(7));
    opcodes[0x72] = instruction!(clr1, DpBit(3));
    opcodes[0x73] = instruction!(bbc, Immediate, DpBit(3));
    opcodes[0x74] = instruction!(cmp, Register(A), U8InMemory(DpXIdx));
    opcodes[0x75] = instruction!(cmp, Register(A), U8InMemory(AbsXIdx));
    opcodes[0x76] = instruction!(cmp, Register(A), U8InMemory(AbsYIdx));
    opcodes[0x77] = instruction!(cmp, Register(A), U8InMemory(DpIndirectYIdx));
    opcodes[0x78] = instruction!(cmp, U8InMemory(Dp), Immediate);
    opcodes[0x79] = instruction!(cmp, U8InMemory(XIndirect), U8InMemory(YIndirect));
    opcodes[0x7A] = instruction!(addw, RegisterYA, U16InMemory(Dp));
    opcodes[0x7B] = instruction!(ror, U8InMemory(DpXIdx));
    opcodes[0x7C] = instruction!(ror, Register(A));
    opcodes[0x7D] = instruction!(mov, Register(A), Register(X));
    opcodes[0x7E] = instruction!(cmp, Register(Y), U8InMemory(Dp));
    opcodes[0x7F] = instruction!(reti);
    opcodes[0x80] = instruction!(setc);
    opcodes[0x81] = instruction!(tcall, Const(8));
    opcodes[0x82] = instruction!(set1, DpBit(4));
    opcodes[0x83] = instruction!(bbs, Immediate, DpBit(4));
    opcodes[0x84] = instruction!(adc, Register(A), U8InMemory(Dp));
    opcodes[0x85] = instruction!(adc, Register(A), U8InMemory(Abs));
    opcodes[0x86] = instruction!(adc, Register(A), U8InMemory(XIndirect));
    opcodes[0x87] = instruction!(adc, Register(A), U8InMemory(DpXIdxIndirect));
    opcodes[0x88] = instruction!(adc, Register(A), Immediate);
    opcodes[0x89] = instruction!(adc, U8InMemory(Dp), U8InMemory(Dp));
    opcodes[0x8A] = instruction!(eor1, AbsBit);
    opcodes[0x8B] = instruction!(dec, U8InMemory(Dp));
    opcodes[0x8C] = instruction!(dec, U8InMemory(Abs));
    opcodes[0x8D] = instruction!(mov, Register(Y), Immediate);
    opcodes[0x8E] = instruction!(pop, Register(Psw));
    opcodes[0x8F] = instruction!(mov, U8InMemory(Dp), Immediate);
    opcodes[0x90] = instruction!(bcc, Immediate);
    opcodes[0x91] = instruction!(tcall, Const(9));
    opcodes[0x92] = instruction!(clr1, DpBit(4));
    opcodes[0x93] = instruction!(bbc, Immediate, DpBit(4));
    opcodes[0x94] = instruction!(adc, Register(A), U8InMemory(DpXIdx));
    opcodes[0x95] = instruction!(adc, Register(A), U8InMemory(AbsXIdx));
    opcodes[0x96] = instruction!(adc, Register(A), U8InMemory(AbsYIdx));
    opcodes[0x97] = instruction!(adc, Register(A), U8InMemory(DpIndirectYIdx));
    opcodes[0x98] = instruction!(adc, U8InMemory(Dp), Immediate);
    opcodes[0x99] = instruction!(adc, U8InMemory(XIndirect), U8InMemory(YIndirect));
    opcodes[0x9A] = instruction!(subw, RegisterYA, U16InMemory(Dp));
    opcodes[0x9B] = instruction!(dec, U8InMemory(DpXIdx));
    opcodes[0x9C] = instruction!(dec, Register(A));
    opcodes[0x9D] = instruction!(mov, Register(X), Register(Sp));
    opcodes[0x9E] = instruction!(div);
    opcodes[0x9F] = instruction!(xcn);
    opcodes
}
