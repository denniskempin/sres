use super::status::StatusFlags;
use super::Cpu;
use crate::bus::Bus;
use crate::cpu::operands::AddressMode;
use crate::cpu::operands::Operand;
use crate::memory::Address;

pub struct InstructionMeta {
    pub operation: &'static str,
    pub operand_str: Option<String>,
    pub operand_addr: Option<Address>,
}

pub struct Instruction<BusT: Bus> {
    pub execute: fn(&mut Cpu<BusT>),
    pub meta: fn(&Cpu<BusT>, Address) -> (InstructionMeta, Address),
}

pub fn build_opcode_table<BusT: Bus>() -> [Instruction<BusT>; 256] {
    macro_rules! instruction {
        // Operand-less instruction
        ($method: ident) => {
            Instruction::<BusT> {
                execute: |cpu| {
                    $method(cpu);
                    cpu.pc = cpu.pc + 1;
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
        ($method: ident, $address_mode: expr) => {
            Instruction::<BusT> {
                execute: |cpu| {
                    let (operand, next_addr) = Operand::decode(cpu, cpu.pc, $address_mode);
                    cpu.pc = next_addr;
                    $method(cpu, &operand);
                },
                meta: |cpu, instruction_addr| {
                    let (operand, next_addr) =
                        Operand::decode(cpu, instruction_addr, $address_mode);
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
        execute: |_| {
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
    table[0x08] = instruction!(php);
    table[0xAB] = instruction!(plb);
    table[0x68] = instruction!(pla);
    table[0x69] = instruction!(adc, AddressMode::ImmediateA);
    table[0xE2] = instruction!(sep, AddressMode::ImmediateU8);
    table[0xC2] = instruction!(rep, AddressMode::ImmediateU8);
    table[0xA9] = instruction!(lda, AddressMode::ImmediateA);
    table[0xBD] = instruction!(lda, AddressMode::AbsoluteXIndexed);
    table[0xA2] = instruction!(ldx, AddressMode::ImmediateXY);
    table[0xA0] = instruction!(ldy, AddressMode::ImmediateXY);
    table[0x8D] = instruction!(sta, AddressMode::Absolute);
    table[0x85] = instruction!(sta, AddressMode::DirectPage);
    table[0x8E] = instruction!(stx, AddressMode::Absolute);
    table[0x8C] = instruction!(sty, AddressMode::Absolute);
    table[0x9C] = instruction!(stz, AddressMode::Absolute);
    table[0x5C] = instruction!(jml, AddressMode::AbsoluteLong);
    table[0x9A] = instruction!(txs);
    table[0x5B] = instruction!(tcd);
    table[0xCA] = instruction!(dex);
    table[0x88] = instruction!(dey);
    table[0xE8] = instruction!(inx);
    table[0xEA] = instruction!(nop);
    table[0xC9] = instruction!(cmp, AddressMode::ImmediateA);
    table[0x4A] = instruction!(lsr, AddressMode::Accumulator);
    table[0x2C] = instruction!(bit, AddressMode::Absolute);
    table[0xD0] = instruction!(bne, AddressMode::Relative);
    table[0x10] = instruction!(bpl, AddressMode::Relative);
    table[0x80] = instruction!(bra, AddressMode::Relative);
    table[0xE0] = instruction!(cpx, AddressMode::ImmediateXY);
    table[0x29] = instruction!(and, AddressMode::ImmediateA);
    table[0x20] = instruction!(jsr, AddressMode::Absolute);
    table[0x24] = instruction!(bit, AddressMode::DirectPage);
    table[0x60] = instruction!(rts);
    table[0xA5] = instruction!(lda, AddressMode::DirectPage);
    table[0xCD] = instruction!(cmp, AddressMode::Absolute);
    table[0xF0] = instruction!(beq, AddressMode::Relative);
    table[0x38] = instruction!(sec);
    table[0xA6] = instruction!(ldx, AddressMode::DirectPage);
    table[0xEC] = instruction!(cpx, AddressMode::Absolute);
    table
}

fn nop(_: &mut Cpu<impl Bus>) {}

fn sec(cpu: &mut Cpu<impl Bus>) {
    cpu.status.carry = true;
}

fn jsr(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.stack_push_u16(cpu.pc.offset - 1);
    cpu.pc = operand.addr().unwrap();
}

fn rts(cpu: &mut Cpu<impl Bus>) {
    cpu.pc.offset = cpu.stack_pop_u16();
}

fn beq(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.zero {
        cpu.pc = operand.addr().unwrap();
    }
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

fn php(cpu: &mut Cpu<impl Bus>) {
    cpu.stack_push(u8::from(cpu.status));
}

fn plb(cpu: &mut Cpu<impl Bus>) {
    cpu.db = cpu.stack_pop();
    cpu.update_negative_zero_flags(cpu.db);
}

fn pla(cpu: &mut Cpu<impl Bus>) {
    if cpu.status.accumulator_register_size {
        cpu.a = cpu.stack_pop() as u16;
        cpu.update_negative_zero_flags(cpu.a as u8);
    } else {
        cpu.a = cpu.stack_pop_u16();
        cpu.update_negative_zero_flags_u16(cpu.a);
    }
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

fn ldy(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.index_register_size_or_break {
        cpu.y = operand.load(cpu) as u16;
        cpu.update_negative_zero_flags(cpu.y as u8);
    } else {
        cpu.y = operand.load_u16(cpu);
        cpu.update_negative_zero_flags_u16(cpu.y);
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

fn lsr(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data = operand.load(cpu);
    let result = data >> 1;
    cpu.status.carry = data & 1 != 0;
    cpu.update_negative_zero_flags(result);
    operand.store(cpu, result);
}

fn sta(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, cpu.a as u8);
}

fn sty(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.index_register_size_or_break {
        operand.store(cpu, cpu.y as u8);
    } else {
        operand.store_u16(cpu, cpu.y);
    }
}

fn stx(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.index_register_size_or_break {
        operand.store(cpu, cpu.x as u8);
    } else {
        operand.store_u16(cpu, cpu.x);
    }
}

fn stz(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, 0);
}

fn txs(cpu: &mut Cpu<impl Bus>) {
    cpu.s = cpu.x;
    cpu.update_negative_zero_flags_u16(cpu.s);
}

fn inx(cpu: &mut Cpu<impl Bus>) {
    cpu.x = cpu.x.wrapping_add(1);
    cpu.update_negative_zero_flags_u16(cpu.x);
}

fn dex(cpu: &mut Cpu<impl Bus>) {
    cpu.x = cpu.x.wrapping_sub(1);
    cpu.update_negative_zero_flags_u16(cpu.x);
}

fn dey(cpu: &mut Cpu<impl Bus>) {
    cpu.y = cpu.y.wrapping_sub(1);
    cpu.update_negative_zero_flags_u16(cpu.y);
}

fn bne(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if !cpu.status.zero {
        cpu.pc = operand.addr().unwrap();
    }
}

fn bpl(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if !cpu.status.negative {
        cpu.pc = operand.addr().unwrap();
    }
}

fn bra(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.pc = operand.addr().unwrap();
}

fn tcd(cpu: &mut Cpu<impl Bus>) {
    cpu.d = cpu.a;
    cpu.update_negative_zero_flags_u16(cpu.d);
}

fn jml(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.pc = operand.addr().unwrap();
}

fn and(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.accumulator_register_size {
        cpu.a &= operand.load(cpu) as u16;
        cpu.update_negative_zero_flags(cpu.a as u8);
    } else {
        cpu.a &= operand.load_u16(cpu);
        cpu.update_negative_zero_flags_u16(cpu.a);
    }
}

fn bit(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value = operand.load(cpu);
    let flags = StatusFlags::from(value);
    cpu.status.negative = flags.negative;
    cpu.status.overflow = flags.overflow;
    cpu.status.zero = (value & cpu.a as u8) == 0;
}

fn cpx(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.index_register_size_or_break {
        let (value, overflow) = (cpu.x as u8).overflowing_sub(operand.load(cpu));
        cpu.update_negative_zero_flags(value);
        cpu.status.carry = !overflow;
    } else {
        let (value, overflow) = cpu.x.overflowing_sub(operand.load_u16(cpu));
        cpu.update_negative_zero_flags_u16(value);
        cpu.status.carry = !overflow;
    }
}

fn cmp(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.accumulator_register_size {
        let (value, overflow) = (cpu.a as u8).overflowing_sub(operand.load(cpu));
        cpu.update_negative_zero_flags(value);
        cpu.status.carry = !overflow;
    } else {
        let (value, overflow) = cpu.a.overflowing_sub(operand.load_u16(cpu));
        cpu.update_negative_zero_flags_u16(value);
        cpu.status.carry = !overflow;
    }
}

fn adc(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.accumulator_register_size {
        let value = operand.load(cpu);
        let (mut result, mut overflow) = (cpu.a as u8).overflowing_add(value);
        if cpu.status.carry {
            let (result2, overflow2) = result.overflowing_add(1);
            result = result2;
            overflow |= overflow2;
        }
        cpu.update_negative_zero_flags(result);
        cpu.status.carry = overflow;
        cpu.status.overflow = ((cpu.a as u8) ^ result) & (value ^ result) & 0x80 != 0;
        cpu.a = result as u16;
    } else {
        let value = operand.load_u16(cpu);
        let (mut result, mut overflow) = cpu.a.overflowing_add(value);
        if cpu.status.carry {
            let (result2, overflow2) = result.overflowing_add(1);
            result = result2;
            overflow |= overflow2;
        }
        cpu.update_negative_zero_flags_u16(result);
        cpu.status.carry = overflow;
        cpu.status.overflow = (cpu.a ^ result) & (value ^ result) & 0x8000 != 0;
        cpu.a = result;
    }
}
