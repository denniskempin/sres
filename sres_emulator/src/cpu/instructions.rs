use super::operands::RegisterSize;
use super::Cpu;
use crate::bus::Bus;
use crate::cpu::operands::AddressMode;
use crate::cpu::operands::Operand;
use crate::cpu::operands::Register;

use crate::memory::Address;

pub struct InstructionMeta {
    pub name: &'static str,
    pub operand_str: Option<String>,
    pub operand_addr: Option<Address>,
}

pub struct Instruction<BusT: Bus> {
    pub execute: fn(&mut Cpu<BusT>, Address) -> Address,
    pub meta: fn(&Cpu<BusT>, Address) -> (InstructionMeta, Address),
}

pub fn build_opcode_table<BusT: Bus>() -> [Instruction<BusT>; 256] {
    macro_rules! instruction {
        // Operand-less instruction
        ($method: ident) => {
            Instruction::<BusT> {
                execute: |cpu, instruction_addr| {
                    $method(cpu);
                    instruction_addr + 1
                },
                meta: |_, instruction_addr| {
                    (
                        InstructionMeta {
                            name: stringify!($method),
                            operand_str: None,
                            operand_addr: None,
                        },
                        instruction_addr + 1,
                    )
                },
            }
        };
        // Instruction with operand
        ($method: ident, $address_mode: expr, $register: expr) => {
            Instruction::<BusT> {
                execute: |cpu, instruction_addr| {
                    let (operand, next_addr) =
                        Operand::new(cpu, instruction_addr, $address_mode, $register);
                    $method(cpu, &operand);
                    next_addr
                },
                meta: |cpu, instruction_addr| {
                    let (operand, next_addr) =
                        Operand::new(cpu, instruction_addr, $address_mode, $register);
                    let (operand_str, operand_addr) = operand.get_meta();
                    (
                        InstructionMeta {
                            name: stringify!($method),
                            operand_str: Some(operand_str),
                            operand_addr,
                        },
                        next_addr,
                    )
                },
            }
        };
    }

    let mut table = [(); 256].map(|_| Instruction::<BusT> {
        execute: |_, _| {
            panic!("Unimplemented instruction");
        },
        meta: |_, addr| {
            (
                InstructionMeta {
                    name: "ill",
                    operand_str: None,
                    operand_addr: None,
                },
                addr,
            )
        },
    });
    table[0x78] = instruction!(sei);
    table[0x18] = instruction!(clc);
    table[0xFB] = instruction!(xce);
    table[0x4B] = instruction!(phk);
    table[0xAB] = instruction!(plb);
    table[0xE2] = instruction!(sep, AddressMode::Immediate, Register::Status);
    table[0xC2] = instruction!(rep, AddressMode::Immediate, Register::Status);
    table[0xA9] = instruction!(lda, AddressMode::Immediate, Register::A);
    table[0xA2] = instruction!(ldx, AddressMode::Immediate, Register::X);
    table[0x8D] = instruction!(sta, AddressMode::Absolute, Register::A);
    table[0x9A] = instruction!(txs);
    table[0x5B] = instruction!(tcd);
    table
}

fn sei(cpu: &mut Cpu<impl Bus>) {
    cpu.status.irq_disable = true;
}

fn clc(cpu: &mut Cpu<impl Bus>) {
    cpu.status.carry = false;
}

fn xce(cpu: &mut Cpu<impl Bus>) {
    (cpu.status.carry, cpu.emulation_mode) = (cpu.emulation_mode, cpu.status.carry);
}

fn phk(cpu: &mut Cpu<impl Bus>) {
    cpu.stack_push(cpu.db);
}

fn plb(cpu: &mut Cpu<impl Bus>) {
    cpu.db = cpu.stack_pop();
    cpu.update_negative_zero_flags(cpu.db as u16, RegisterSize::U8);
}

fn rep(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data = operand.load(cpu) as u8;
    cpu.status = (u8::from(cpu.status) & !data).into();
}

fn sep(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data = operand.load(cpu) as u8;
    cpu.status = (u8::from(cpu.status) | data).into();
}

fn ldx(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.x = operand.load(cpu);
    cpu.update_negative_zero_flags(cpu.x, operand.register_size);
}

fn lda(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.a = operand.load(cpu);
    cpu.update_negative_zero_flags(cpu.a, operand.register_size);
}

fn sta(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, cpu.a);
}

fn txs(cpu: &mut Cpu<impl Bus>) {
    cpu.s = cpu.x;
    cpu.update_negative_zero_flags(cpu.s, RegisterSize::U16);
}

fn tcd(cpu: &mut Cpu<impl Bus>) {
    cpu.d = cpu.a;
    cpu.update_negative_zero_flags(cpu.d, RegisterSize::U16);
}
