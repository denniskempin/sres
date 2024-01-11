use crate::bus::Address;
use crate::bus::AddressU16;
use crate::bus::Wrap;
use crate::spc700::operands::AccessMode;
use crate::spc700::AddressMode;
use crate::spc700::Operand;
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
                    cpu.pc = cpu.pc.add(1_u8, Wrap::NoWrap);
                },
                meta: |_, instruction_addr| {
                    (
                        InstructionMeta {
                            address: instruction_addr,
                            operation: stringify!($method),
                            operand_str: None,
                            effective_addr: None,
                        },
                        instruction_addr.add(1_u8, Wrap::NoWrap),
                    )
                },
            }
        };
        // Single operand instruction
        ($method: ident, $address_mode: expr, $rwm: expr) => {
            InstructionDef::<BusT> {
                execute: |cpu| {
                    let (operand, next_addr) = Operand::decode(cpu, $address_mode, $rwm);
                    $method(cpu, operand);
                    cpu.pc = next_addr;
                },
                meta: |cpu, instruction_addr| {
                    let (operand, next_addr) =
                        Operand::peek(cpu, instruction_addr, $address_mode, $rwm);
                    (
                        InstructionMeta {
                            address: instruction_addr,
                            operation: stringify!($method),
                            operand_str: Some(operand.format()),
                            effective_addr: operand.effective_addr(),
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
    opcodes[0x00] = instruction!(nop);
    opcodes[0x0A] = instruction!(or1, AddressMode::AbsoluteBit, AccessMode::Read);
    opcodes[0x0B] = instruction!(asl, AddressMode::DirectPage, AccessMode::Modify);
    opcodes
}
