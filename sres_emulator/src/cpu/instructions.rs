use super::operands::OperandU8;
use super::Cpu;
use crate::bus::Bus;
use crate::cpu::operands::OperandU8Immediate;
use crate::memory::Address;

pub struct InstructionMeta {
    pub name: &'static str,
    pub operand_str: Option<String>,
    pub operand_addr: Option<Address>,
}

pub struct Instruction<BusT: Bus> {
    pub size: usize,
    pub execute: fn(&mut Cpu<BusT>, Address),
    pub meta: fn(&BusT, Address) -> InstructionMeta,
}

pub fn build_opcode_table<BusT: Bus>() -> [Instruction<BusT>; 256] {
    macro_rules! instruction {
        // Operand-less instruction
        ($method: ident) => {
            Instruction::<BusT> {
                size: 1,
                execute: |cpu, _| $method(cpu),
                meta: |_, _| InstructionMeta {
                    name: stringify!($method),
                    operand_str: None,
                    operand_addr: None,
                },
            }
        };
        // Instruction with operand
        ($method: ident, $operand: ident) => {
            Instruction::<BusT> {
                size: $operand::SIZE + 1,
                execute: |cpu, addr| $method(cpu, $operand::new(&cpu.bus, addr + 1)),
                meta: |bus, addr| {
                    let operand = $operand::new(bus, addr + 1);
                    InstructionMeta {
                        name: stringify!(rep),
                        operand_str: operand.format(),
                        operand_addr: operand.addr(),
                    }
                },
            }
        };
    }

    let mut table = [(); 256].map(|_| Instruction::<BusT> {
        size: 1,
        execute: |_, _| {
            panic!("Unimplemented instruction");
        },
        meta: |_, _| InstructionMeta {
            name: "ill",
            operand_str: None,
            operand_addr: None,
        },
    });
    table[0x78] = instruction!(sei);
    table[0x18] = instruction!(clc);
    table[0xFB] = instruction!(xce);
    table[0x4B] = instruction!(phk);
    table[0xAB] = instruction!(plb);
    table[0xC2] = instruction!(rep, OperandU8Immediate);
    table
}

fn sei<BusT: Bus>(cpu: &mut Cpu<BusT>) {
    cpu.status.irq_disable = true;
}

fn clc<BusT: Bus>(cpu: &mut Cpu<BusT>) {
    cpu.status.carry = false;
}

fn xce<BusT: Bus>(cpu: &mut Cpu<BusT>) {
    (cpu.status.carry, cpu.emulation_mode) = (cpu.emulation_mode, cpu.status.carry);
}

fn phk<BusT: Bus>(cpu: &mut Cpu<BusT>) {
    cpu.stack_push(cpu.db);
}

fn plb<BusT: Bus>(cpu: &mut Cpu<BusT>) {
    cpu.db = cpu.stack_pop();
    cpu.update_negative_zero_flags(cpu.db);
}

fn rep<BusT: Bus, OperandT: OperandU8>(cpu: &mut Cpu<BusT>, operand: OperandT) {
    let data = operand.load();
    cpu.status = (u8::from(cpu.status) & !data).into();
}
