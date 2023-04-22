use super::operands::Operand;
use super::Cpu;
use crate::bus::Bus;
use crate::cpu::operands::ImmediateOperand;
use crate::cpu::operands::ImmediateOperandA;
use crate::cpu::operands::ImmediateOperandU16;
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
                execute: |cpu, operand_addr| {
                    $method(cpu);
                    operand_addr
                },
                meta: |_, operand_addr| {
                    (
                        InstructionMeta {
                            name: stringify!($method),
                            operand_str: None,
                            operand_addr: None,
                        },
                        operand_addr,
                    )
                },
            }
        };
        // Instruction with operand
        ($method: ident, $operand: ident) => {
            Instruction::<BusT> {
                execute: |cpu, operand_addr| {
                    let (next_addr, operand) = $operand::new(cpu, operand_addr);
                    println!(
                        "Loaded operand at {:}. Next addr: {:}",
                        operand_addr, next_addr
                    );
                    $method(cpu, operand);
                    next_addr
                },
                meta: |cpu, operand_addr| {
                    let (next_addr, operand) = $operand::new(cpu, operand_addr);
                    (
                        InstructionMeta {
                            name: stringify!($method),
                            operand_str: operand.format(cpu),
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
    table[0xE2] = instruction!(sep, ImmediateOperand);
    table[0xC2] = instruction!(rep, ImmediateOperand);
    table[0xA9] = instruction!(lda, ImmediateOperandA);
    table[0xA2] = instruction!(ldx, ImmediateOperandU16);
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
    cpu.update_negative_zero_flags_u8(cpu.db);
}

fn rep(cpu: &mut Cpu<impl Bus>, operand: impl Operand) {
    let data = operand.load() as u8;
    cpu.status = (u8::from(cpu.status) & !data).into();
}

fn sep(cpu: &mut Cpu<impl Bus>, operand: impl Operand) {
    let data = operand.load() as u8;
    cpu.status = (u8::from(cpu.status) | data).into();
}

fn ldx(cpu: &mut Cpu<impl Bus>, operand: impl Operand) {
    cpu.x = operand.load();
    cpu.update_negative_zero_flags_u16(cpu.x);
}

fn lda(cpu: &mut Cpu<impl Bus>, operand: impl Operand) {
    cpu.a = operand.load();
    cpu.update_negative_zero_flags_u16(cpu.a);
}

fn txs(cpu: &mut Cpu<impl Bus>) {
    cpu.s = cpu.x;
    cpu.update_negative_zero_flags_u16(cpu.s);
}

fn tcd(cpu: &mut Cpu<impl Bus>) {
    cpu.d = cpu.a;
    cpu.update_negative_zero_flags_u16(cpu.d);
}
