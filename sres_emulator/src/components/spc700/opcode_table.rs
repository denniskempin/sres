use super::Spc700;
use super::Spc700Bus;
use crate::common::address::Address;
use crate::common::address::AddressU16;
use crate::common::address::InstructionMeta;
use crate::common::address::Wrap;

/// An entry in the opcode table
pub struct InstructionDef<BusT: Spc700Bus> {
    /// Execute the instruction on the provided CPU.
    pub execute: fn(&mut Spc700<BusT>),

    /// Return metadata about this instruction. Can be used on an immutable CPU.
    pub disassembly: fn(&Spc700<BusT>, AddressU16) -> (InstructionMeta<AddressU16>, AddressU16),
}

pub fn build_opcode_table<BusT: Spc700Bus>() -> [InstructionDef<BusT>; 256] {
    macro_rules! instruction {
        // Operand-less instruction
        ($method: ident) => {
            InstructionDef::<BusT> {
                execute: |cpu| {
                    cpu.pc = cpu.pc.add(1_u8, Wrap::NoWrap);
                    cpu.$method();
                },
                disassembly: |_, instruction_addr| {
                    (
                        InstructionMeta {
                            address: instruction_addr,
                            operation: stringify!($method).to_string(),
                            operand_str: None,
                            effective_addr_and_value: None,
                        },
                        instruction_addr.add(1_u8, Wrap::NoWrap),
                    )
                },
            }
        };
        // Single operand instruction
        ($method: ident, $operand_def: expr) => {
            InstructionDef::<BusT> {
                execute: |cpu| {
                    cpu.pc = cpu.pc.add(1_u8, Wrap::NoWrap);
                    cpu.$method($operand_def);
                },
                disassembly: |cpu, instruction_addr| {
                    let (operand, next_addr) =
                        $operand_def.disassembly(cpu, instruction_addr.add(1_u8, Wrap::NoWrap));
                    (
                        InstructionMeta {
                            address: instruction_addr,
                            operation: stringify!($method).to_string(),
                            operand_str: Some(operand),
                            effective_addr_and_value: None,
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
                    cpu.pc = cpu.pc.add(1_u8, Wrap::NoWrap);
                    cpu.$method($left_def, $right_def);
                },
                disassembly: |cpu, instruction_addr| {
                    let (right, next_addr) =
                        $right_def.disassembly(cpu, instruction_addr.add(1_u8, Wrap::NoWrap));
                    let (left, next_addr) = $left_def.disassembly(cpu, next_addr);
                    (
                        InstructionMeta {
                            address: instruction_addr,
                            operation: stringify!($method).to_string(),
                            operand_str: Some(format!("{}, {}", left, right)),
                            effective_addr_and_value: None,
                        },
                        next_addr,
                    )
                },
            }
        };
    }

    let mut opcodes = [(); 256].map(|_| InstructionDef::<BusT> {
        execute: |_| {},
        disassembly: |_, instruction_addr| {
            (
                InstructionMeta {
                    address: instruction_addr,
                    operation: "ill".to_string(),
                    operand_str: None,
                    effective_addr_and_value: None,
                },
                instruction_addr.add(1_u8, Wrap::NoWrap),
            )
        },
    });

    use super::operands::AddressMode::*;
    use super::operands::Operand::*;
    use super::operands::Register::*;
    opcodes[0x00] = instruction!(nop);
    opcodes[0x01] = instruction!(tcall, Const(0));
    opcodes[0x02] = instruction!(set1, DpBit(0));
    opcodes[0x03] = instruction!(bbs, Relative, DpBit(0));
    opcodes[0x04] = instruction!(or, Register(A), InMemory(Dp));
    opcodes[0x05] = instruction!(or, Register(A), InMemory(Abs));
    opcodes[0x06] = instruction!(or, Register(A), InMemory(XIndirect));
    opcodes[0x07] = instruction!(or, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0x08] = instruction!(or, Register(A), Immediate);
    opcodes[0x09] = instruction!(or, InMemory(Dp), InMemory(Dp));
    opcodes[0x0A] = instruction!(or1, AbsBit);
    opcodes[0x0B] = instruction!(asl, InMemory(Dp));
    opcodes[0x0C] = instruction!(asl, InMemory(Abs));
    opcodes[0x0D] = instruction!(push, Register(Psw));
    opcodes[0x0E] = instruction!(tset1, InMemory(Abs));
    opcodes[0x0F] = instruction!(brk);
    opcodes[0x10] = instruction!(bpl, Relative);
    opcodes[0x11] = instruction!(tcall, Const(1));
    opcodes[0x12] = instruction!(clr1, DpBit(0));
    opcodes[0x13] = instruction!(bbc, Relative, DpBit(0));
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
    opcodes[0x1F] = instruction!(jmp, JumpAddress(AbsXIdxIndirect));
    opcodes[0x20] = instruction!(clrp);
    opcodes[0x21] = instruction!(tcall, Const(2));
    opcodes[0x22] = instruction!(set1, DpBit(1));
    opcodes[0x23] = instruction!(bbs, Relative, DpBit(1));
    opcodes[0x24] = instruction!(and, Register(A), InMemory(Dp));
    opcodes[0x25] = instruction!(and, Register(A), InMemory(Abs));
    opcodes[0x26] = instruction!(and, Register(A), InMemory(XIndirect));
    opcodes[0x27] = instruction!(and, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0x28] = instruction!(and, Register(A), Immediate);
    opcodes[0x29] = instruction!(and, InMemory(Dp), InMemory(Dp));
    opcodes[0x2A] = instruction!(or1, AbsBitInv);
    opcodes[0x2B] = instruction!(rol, InMemory(Dp));
    opcodes[0x2C] = instruction!(rol, InMemory(Abs));
    opcodes[0x2D] = instruction!(push, Register(A));
    opcodes[0x2E] = instruction!(cbne, Relative, InMemory(Dp));
    opcodes[0x2F] = instruction!(bra, Relative);
    opcodes[0x30] = instruction!(bmi, Relative);
    opcodes[0x31] = instruction!(tcall, Const(3));
    opcodes[0x32] = instruction!(clr1, DpBit(1));
    opcodes[0x33] = instruction!(bbc, Relative, DpBit(1));
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
    opcodes[0x3F] = instruction!(call, JumpAddress(Abs));
    opcodes[0x40] = instruction!(setp);
    opcodes[0x41] = instruction!(tcall, Const(4));
    opcodes[0x42] = instruction!(set1, DpBit(2));
    opcodes[0x43] = instruction!(bbs, Relative, DpBit(2));
    opcodes[0x44] = instruction!(eor, Register(A), InMemory(Dp));
    opcodes[0x45] = instruction!(eor, Register(A), InMemory(Abs));
    opcodes[0x46] = instruction!(eor, Register(A), InMemory(XIndirect));
    opcodes[0x47] = instruction!(eor, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0x48] = instruction!(eor, Register(A), Immediate);
    opcodes[0x49] = instruction!(eor, InMemory(Dp), InMemory(Dp));
    opcodes[0x4A] = instruction!(and1, AbsBit);
    opcodes[0x4B] = instruction!(lsr, InMemory(Dp));
    opcodes[0x4C] = instruction!(lsr, InMemory(Abs));
    opcodes[0x4D] = instruction!(push, Register(X));
    opcodes[0x4E] = instruction!(tclr1, InMemory(Abs));
    opcodes[0x4F] = instruction!(pcall, Immediate);
    opcodes[0x50] = instruction!(bvc, Relative);
    opcodes[0x51] = instruction!(tcall, Const(5));
    opcodes[0x52] = instruction!(clr1, DpBit(2));
    opcodes[0x53] = instruction!(bbc, Relative, DpBit(2));
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
    opcodes[0x5F] = instruction!(jmp, JumpAddress(Abs));
    opcodes[0x60] = instruction!(clrc);
    opcodes[0x61] = instruction!(tcall, Const(6));
    opcodes[0x62] = instruction!(set1, DpBit(3));
    opcodes[0x63] = instruction!(bbs, Relative, DpBit(3));
    opcodes[0x64] = instruction!(cmp, Register(A), InMemory(Dp));
    opcodes[0x65] = instruction!(cmp, Register(A), InMemory(Abs));
    opcodes[0x66] = instruction!(cmp, Register(A), InMemory(XIndirect));
    opcodes[0x67] = instruction!(cmp, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0x68] = instruction!(cmp, Register(A), Immediate);
    opcodes[0x69] = instruction!(cmp, InMemory(Dp), InMemory(Dp));
    opcodes[0x6A] = instruction!(and1, AbsBitInv);
    opcodes[0x6B] = instruction!(ror, InMemory(Dp));
    opcodes[0x6C] = instruction!(ror, InMemory(Abs));
    opcodes[0x6D] = instruction!(push, Register(Y));
    opcodes[0x6E] = instruction!(dbnz, InMemory(Dp), Relative);
    opcodes[0x6F] = instruction!(ret);
    opcodes[0x70] = instruction!(bvs, Relative);
    opcodes[0x71] = instruction!(tcall, Const(7));
    opcodes[0x72] = instruction!(clr1, DpBit(3));
    opcodes[0x73] = instruction!(bbc, Relative, DpBit(3));
    opcodes[0x74] = instruction!(cmp, Register(A), InMemory(DpXIdx));
    opcodes[0x75] = instruction!(cmp, Register(A), InMemory(AbsXIdx));
    opcodes[0x76] = instruction!(cmp, Register(A), InMemory(AbsYIdx));
    opcodes[0x77] = instruction!(cmp, Register(A), InMemory(DpIndirectYIdx));
    opcodes[0x78] = instruction!(cmp, InMemory(Dp), Immediate);
    opcodes[0x79] = instruction!(cmp, InMemory(XIndirect), InMemory(YIndirect));
    opcodes[0x7A] = instruction!(addw, InMemory(Dp));
    opcodes[0x7B] = instruction!(ror, InMemory(DpXIdx));
    opcodes[0x7C] = instruction!(ror, Register(A));
    opcodes[0x7D] = instruction!(mov, Register(A), Register(X));
    opcodes[0x7E] = instruction!(cmp, Register(Y), InMemory(Dp));
    opcodes[0x7F] = instruction!(reti);
    opcodes[0x80] = instruction!(setc);
    opcodes[0x81] = instruction!(tcall, Const(8));
    opcodes[0x82] = instruction!(set1, DpBit(4));
    opcodes[0x83] = instruction!(bbs, Relative, DpBit(4));
    opcodes[0x84] = instruction!(adc, Register(A), InMemory(Dp));
    opcodes[0x85] = instruction!(adc, Register(A), InMemory(Abs));
    opcodes[0x86] = instruction!(adc, Register(A), InMemory(XIndirect));
    opcodes[0x87] = instruction!(adc, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0x88] = instruction!(adc, Register(A), Immediate);
    opcodes[0x89] = instruction!(adc, InMemory(Dp), InMemory(Dp));
    opcodes[0x8A] = instruction!(eor1, AbsBit);
    opcodes[0x8B] = instruction!(dec, InMemory(Dp));
    opcodes[0x8C] = instruction!(dec, InMemory(Abs));
    opcodes[0x8D] = instruction!(mov, Register(Y), Immediate);
    opcodes[0x8E] = instruction!(pop, Register(Psw));
    opcodes[0x8F] = instruction!(mov, InMemory(Dp), Immediate);
    opcodes[0x90] = instruction!(bcc, Relative);
    opcodes[0x91] = instruction!(tcall, Const(9));
    opcodes[0x92] = instruction!(clr1, DpBit(4));
    opcodes[0x93] = instruction!(bbc, Relative, DpBit(4));
    opcodes[0x94] = instruction!(adc, Register(A), InMemory(DpXIdx));
    opcodes[0x95] = instruction!(adc, Register(A), InMemory(AbsXIdx));
    opcodes[0x96] = instruction!(adc, Register(A), InMemory(AbsYIdx));
    opcodes[0x97] = instruction!(adc, Register(A), InMemory(DpIndirectYIdx));
    opcodes[0x98] = instruction!(adc, InMemory(Dp), Immediate);
    opcodes[0x99] = instruction!(adc, InMemory(XIndirect), InMemory(YIndirect));
    opcodes[0x9A] = instruction!(subw, InMemory(Dp));
    opcodes[0x9B] = instruction!(dec, InMemory(DpXIdx));
    opcodes[0x9C] = instruction!(dec, Register(A));
    opcodes[0x9D] = instruction!(mov, Register(X), Register(Sp));
    opcodes[0x9E] = instruction!(div);
    opcodes[0x9F] = instruction!(xcn);
    opcodes[0xA0] = instruction!(ei);
    opcodes[0xA1] = instruction!(tcall, Const(10));
    opcodes[0xA2] = instruction!(set1, DpBit(5));
    opcodes[0xA3] = instruction!(bbs, Relative, DpBit(5));
    opcodes[0xA4] = instruction!(sbc, Register(A), InMemory(Dp));
    opcodes[0xA5] = instruction!(sbc, Register(A), InMemory(Abs));
    opcodes[0xA6] = instruction!(sbc, Register(A), InMemory(XIndirect));
    opcodes[0xA7] = instruction!(sbc, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0xA8] = instruction!(sbc, Register(A), Immediate);
    opcodes[0xA9] = instruction!(sbc, InMemory(Dp), InMemory(Dp));
    opcodes[0xAA] = instruction!(mov1, Carry, AbsBit);
    opcodes[0xAB] = instruction!(inc, InMemory(Dp));
    opcodes[0xAC] = instruction!(inc, InMemory(Abs));
    opcodes[0xAD] = instruction!(cmp, Register(Y), Immediate);
    opcodes[0xAE] = instruction!(pop, Register(A));
    opcodes[0xAF] = instruction!(mov, InMemory(XIndirectAutoInc), Register(A));
    opcodes[0xB0] = instruction!(bcs, Relative);
    opcodes[0xB1] = instruction!(tcall, Const(11));
    opcodes[0xB2] = instruction!(clr1, DpBit(5));
    opcodes[0xB3] = instruction!(bbc, Relative, DpBit(5));
    opcodes[0xB4] = instruction!(sbc, Register(A), InMemory(DpXIdx));
    opcodes[0xB5] = instruction!(sbc, Register(A), InMemory(AbsXIdx));
    opcodes[0xB6] = instruction!(sbc, Register(A), InMemory(AbsYIdx));
    opcodes[0xB7] = instruction!(sbc, Register(A), InMemory(DpIndirectYIdx));
    opcodes[0xB8] = instruction!(sbc, InMemory(Dp), Immediate);
    opcodes[0xB9] = instruction!(sbc, InMemory(XIndirect), InMemory(YIndirect));
    opcodes[0xBA] = instruction!(movw, Register(YA), InMemory(Dp));
    opcodes[0xBB] = instruction!(inc, InMemory(DpXIdx));
    opcodes[0xBC] = instruction!(inc, Register(A));
    opcodes[0xBD] = instruction!(mov, Register(Sp), Register(X));
    opcodes[0xBE] = instruction!(das);
    opcodes[0xBF] = instruction!(mov, Register(A), InMemory(XIndirectAutoInc));
    opcodes[0xC0] = instruction!(di);
    opcodes[0xC1] = instruction!(tcall, Const(12));
    opcodes[0xC2] = instruction!(set1, DpBit(6));
    opcodes[0xC3] = instruction!(bbs, Relative, DpBit(6));
    opcodes[0xC4] = instruction!(mov, InMemory(Dp), Register(A));
    opcodes[0xC5] = instruction!(mov, InMemory(Abs), Register(A));
    opcodes[0xC6] = instruction!(mov, InMemory(XIndirect), Register(A));
    opcodes[0xC7] = instruction!(mov, InMemory(DpXIdxIndirect), Register(A));
    opcodes[0xC8] = instruction!(cmp, Register(X), Immediate);
    opcodes[0xC9] = instruction!(mov, InMemory(Abs), Register(X));
    opcodes[0xCA] = instruction!(mov1, AbsBit, Carry);
    opcodes[0xCB] = instruction!(mov, InMemory(Dp), Register(Y));
    opcodes[0xCC] = instruction!(mov, InMemory(Abs), Register(Y));
    opcodes[0xCD] = instruction!(mov, Register(X), Immediate);
    opcodes[0xCE] = instruction!(pop, Register(X));
    opcodes[0xCF] = instruction!(mul);
    opcodes[0xD0] = instruction!(bne, Relative);
    opcodes[0xD1] = instruction!(tcall, Const(13));
    opcodes[0xD2] = instruction!(clr1, DpBit(6));
    opcodes[0xD3] = instruction!(bbc, Relative, DpBit(6));
    opcodes[0xD4] = instruction!(mov, InMemory(DpXIdx), Register(A));
    opcodes[0xD5] = instruction!(mov, InMemory(AbsXIdx), Register(A));
    opcodes[0xD6] = instruction!(mov, InMemory(AbsYIdx), Register(A));
    opcodes[0xD7] = instruction!(mov, InMemory(DpIndirectYIdx), Register(A));
    opcodes[0xD8] = instruction!(mov, InMemory(Dp), Register(X));
    opcodes[0xD9] = instruction!(mov, InMemory(DpYIdx), Register(X));
    opcodes[0xDA] = instruction!(movw, InMemory(Dp), Register(YA));
    opcodes[0xDB] = instruction!(mov, InMemory(DpXIdx), Register(Y));
    opcodes[0xDC] = instruction!(dec, Register(Y));
    opcodes[0xDD] = instruction!(mov, Register(A), Register(Y));
    opcodes[0xDE] = instruction!(cbne, Relative, InMemory(DpXIdx));
    opcodes[0xDF] = instruction!(daa);
    opcodes[0xE0] = instruction!(clrv);
    opcodes[0xE1] = instruction!(tcall, Const(14));
    opcodes[0xE2] = instruction!(set1, DpBit(7));
    opcodes[0xE3] = instruction!(bbs, Relative, DpBit(7));
    opcodes[0xE4] = instruction!(mov, Register(A), InMemory(Dp));
    opcodes[0xE5] = instruction!(mov, Register(A), InMemory(Abs));
    opcodes[0xE6] = instruction!(mov, Register(A), InMemory(XIndirect));
    opcodes[0xE7] = instruction!(mov, Register(A), InMemory(DpXIdxIndirect));
    opcodes[0xE8] = instruction!(mov, Register(A), Immediate);
    opcodes[0xE9] = instruction!(mov, Register(X), InMemory(Abs));
    opcodes[0xEA] = instruction!(not1, AbsBit);
    opcodes[0xEB] = instruction!(mov, Register(Y), InMemory(Dp));
    opcodes[0xEC] = instruction!(mov, Register(Y), InMemory(Abs));
    opcodes[0xED] = instruction!(not1, Carry);
    opcodes[0xEE] = instruction!(pop, Register(Y));
    opcodes[0xEF] = instruction!(sleep);
    opcodes[0xF0] = instruction!(beq, Relative);
    opcodes[0xF1] = instruction!(tcall, Const(15));
    opcodes[0xF2] = instruction!(clr1, DpBit(7));
    opcodes[0xF3] = instruction!(bbc, Relative, DpBit(7));
    opcodes[0xF4] = instruction!(mov, Register(A), InMemory(DpXIdx));
    opcodes[0xF5] = instruction!(mov, Register(A), InMemory(AbsXIdx));
    opcodes[0xF6] = instruction!(mov, Register(A), InMemory(AbsYIdx));
    opcodes[0xF7] = instruction!(mov, Register(A), InMemory(DpIndirectYIdx));
    opcodes[0xF8] = instruction!(mov, Register(X), InMemory(Dp));
    opcodes[0xF9] = instruction!(mov, Register(X), InMemory(DpYIdx));
    opcodes[0xFA] = instruction!(mov, InMemory(Dp), InMemory(Dp));
    opcodes[0xFB] = instruction!(mov, Register(Y), InMemory(DpXIdx));
    opcodes[0xFC] = instruction!(inc, Register(Y));
    opcodes[0xFD] = instruction!(mov, Register(Y), Register(A));
    opcodes[0xFE] = instruction!(dbnz, Register(Y), Relative);
    opcodes[0xFF] = instruction!(stop);
    opcodes
}
