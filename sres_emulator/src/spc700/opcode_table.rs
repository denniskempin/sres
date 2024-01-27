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
                    let (operand, next_addr) = $operand_def.decode(cpu);
                    cpu.pc = next_addr;
                    $method(cpu, operand);
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
                    let (right, next_addr) = $right_def.decode(cpu);
                    cpu.pc = next_addr;
                    let (left, next_addr) = $left_def.decode(cpu);
                    cpu.pc = next_addr;
                    $method(cpu, left, right);
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
    opcodes[0x02] = instruction!(set1, DirectPageBit(0));
    opcodes[0x03] = instruction!(bbs, Immediate, DirectPageBit(0));
    opcodes[0x04] = instruction!(or, Register(Accumulator), InMemory(DirectPage));
    opcodes[0x05] = instruction!(or, Register(Accumulator), InMemory(Absolute));
    opcodes[0x06] = instruction!(or, Register(Accumulator), InMemory(IndirectX));
    opcodes[0x07] = instruction!(or, Register(Accumulator), InMemory(DirectPageXIndexed));
    opcodes[0x08] = instruction!(or, Register(Accumulator), Immediate);
    opcodes[0x09] = instruction!(or, InMemory(DirectPage), InMemory(DirectPage));
    opcodes[0x0A] = instruction!(or1, AbsoluteBit);
    opcodes[0x0B] = instruction!(asl, InMemory(DirectPage));
    opcodes[0x0C] = instruction!(asl, InMemory(Absolute));
    opcodes[0x0D] = instruction!(push, Register(Psw));
    opcodes[0x0E] = instruction!(tset1, InMemory(Absolute));
    opcodes[0x0F] = instruction!(brk);
    opcodes
}
