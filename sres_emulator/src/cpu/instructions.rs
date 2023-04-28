use super::Cpu;
use crate::bus::Bus;
use crate::cpu::operands::AddressMode;
use crate::cpu::operands::Operand;
use crate::cpu::operands::Register;
use crate::memory::Address;

pub struct InstructionMeta {
    pub operation: &'static str,
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
                            operation: stringify!($method),
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
                        Operand::decode(cpu, instruction_addr, $address_mode, $register);
                    $method(cpu, &operand);
                    next_addr
                },
                meta: |cpu, instruction_addr| {
                    let (operand, next_addr) =
                        Operand::decode(cpu, instruction_addr, $address_mode, $register);
                    (
                        InstructionMeta {
                            operation: stringify!($method),
                            operand_str: Some(operand.format()),
                            operand_addr: operand.addr(),
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
                    operation: "ill",
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
    table[0xE2] = instruction!(sep, AddressMode::Immediate, Register::FixedU8);
    table[0xC2] = instruction!(rep, AddressMode::Immediate, Register::FixedU8);
    table[0xA9] = instruction!(lda, AddressMode::Immediate, Register::A);
    table[0xA2] = instruction!(ldx, AddressMode::Immediate, Register::X);
    table[0x8D] = instruction!(sta, AddressMode::Absolute, Register::A);
    table[0x9C] = instruction!(stz, AddressMode::Absolute, Register::FixedU8);
    table[0x9A] = instruction!(txs);
    table[0x5B] = instruction!(tcd);
    table[0xCA] = instruction!(dex);
    table[0xD0] = instruction!(bne, AddressMode::Relative, Register::FixedU8);
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
    cpu.update_negative_zero_flags(cpu.db);
}

fn rep(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data = operand.load(cpu);
    cpu.status = (u8::from(cpu.status) & !data).into();
}

fn sep(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data = operand.load(cpu);
    cpu.status = (u8::from(cpu.status) | data).into();
}

fn ldx(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.index_register_size_or_break {
        cpu.x = operand.load(cpu) as u16;
        cpu.update_negative_zero_flags(cpu.x as u8);
    } else {
        cpu.x = operand.load_u16(cpu);
        cpu.update_negative_zero_flags_u16(cpu.x);
    }
}

fn lda(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.accumulator_register_size {
        cpu.a = operand.load(cpu) as u16;
        cpu.update_negative_zero_flags(cpu.a as u8);
    } else {
        cpu.a = operand.load_u16(cpu);
        cpu.update_negative_zero_flags_u16(cpu.a);
    }
}

fn sta(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, cpu.a as u8);
}

fn stz(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, 0);
}

fn txs(cpu: &mut Cpu<impl Bus>) {
    cpu.s = cpu.x;
    cpu.update_negative_zero_flags_u16(cpu.s);
}

fn dex(cpu: &mut Cpu<impl Bus>) {
    cpu.x = cpu.x.wrapping_sub(1);
    cpu.update_negative_zero_flags_u16(cpu.x);
}

fn bne(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if !cpu.status.zero {
        cpu.pc = operand.addr().unwrap();
    }
}

fn tcd(cpu: &mut Cpu<impl Bus>) {
    cpu.d = cpu.a;
    cpu.update_negative_zero_flags_u16(cpu.d);
}
