use super::status::StatusFlags;
use super::Cpu;
use super::UInt;
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

enum Register {
    A,
    X,
    Y,
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
        // Instruction with operand and variable register size
        ($method: ident, $address_mode: expr, $register: expr) => {
            Instruction::<BusT> {
                execute: |cpu| {
                    let (operand, next_addr) = Operand::decode(cpu, cpu.pc, $address_mode);
                    cpu.pc = next_addr;
                    let is_u8 = match $register {
                        Register::A => cpu.status.accumulator_register_size,
                        Register::X => cpu.status.index_register_size_or_break,
                        Register::Y => cpu.status.index_register_size_or_break,
                    };
                    if is_u8 {
                        $method::<u8>(cpu, &operand);
                    } else {
                        $method::<u16>(cpu, &operand);
                    }
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

    use AddressMode::*;
    use Register::*;
    table[0x78] = instruction!(sei);
    table[0x18] = instruction!(clc);
    table[0xFB] = instruction!(xce);
    table[0x4B] = instruction!(phk);
    table[0x08] = instruction!(php);
    table[0xAB] = instruction!(plb);
    table[0x48] = instruction!(pha, Implied, A);
    table[0x68] = instruction!(pla, Implied, A);
    table[0x69] = instruction!(adc, ImmediateA, A);
    table[0x6D] = instruction!(adc, Absolute, A);
    table[0x6F] = instruction!(adc, AbsoluteLong, A);
    table[0x65] = instruction!(adc, DirectPage, A);
    table[0x72] = instruction!(adc, DirectPageIndirect, A);
    table[0x67] = instruction!(adc, DirectPageIndirectLong, A);
    table[0x7D] = instruction!(adc, AbsoluteXIndexed, A);
    table[0x7F] = instruction!(adc, AbsoluteXIndexedLong, A);
    table[0x79] = instruction!(adc, AbsoluteYIndexed, A);
    table[0x75] = instruction!(adc, DirectPageXIndexed, A);
    table[0x61] = instruction!(adc, DirectPageXIndexedIndirect, A);
    table[0x71] = instruction!(adc, DirectPageIndirectYIndexed, A);
    table[0x77] = instruction!(adc, DirectPageIndirectYIndexedLong, A);
    table[0x63] = instruction!(adc, StackRelative, A);
    table[0xE2] = instruction!(sep, ImmediateU8);
    table[0xC2] = instruction!(rep, ImmediateU8);
    table[0xA9] = instruction!(lda, ImmediateA, A);
    table[0xBD] = instruction!(lda, AbsoluteXIndexed, A);
    table[0xA2] = instruction!(ldx, ImmediateXY, X);
    table[0xA0] = instruction!(ldy, ImmediateXY, Y);
    table[0x8D] = instruction!(sta, Absolute, A);
    table[0x85] = instruction!(sta, DirectPage, A);
    table[0x8E] = instruction!(stx, Absolute, X);
    table[0x86] = instruction!(stx, DirectPage, X);
    table[0x8C] = instruction!(sty, Absolute, Y);
    table[0x9C] = instruction!(stz, Absolute);
    table[0x5C] = instruction!(jml, AbsoluteLong);
    table[0x9A] = instruction!(txs, Implied, X);
    table[0x5B] = instruction!(tcd);
    table[0xCA] = instruction!(dex, Implied, X);
    table[0x88] = instruction!(dey, Implied, Y);
    table[0xE8] = instruction!(inx, Implied, X);
    table[0xEA] = instruction!(nop);
    table[0xC9] = instruction!(cmp, ImmediateA, A);
    table[0x4A] = instruction!(lsr, Accumulator);
    table[0x2C] = instruction!(bit, Absolute);
    table[0xD0] = instruction!(bne, Relative);
    table[0x10] = instruction!(bpl, Relative);
    table[0x80] = instruction!(bra, Relative);
    table[0xE0] = instruction!(cpx, ImmediateXY, X);
    table[0x29] = instruction!(and, ImmediateA, A);
    table[0x20] = instruction!(jsr, Absolute);
    table[0x24] = instruction!(bit, DirectPage);
    table[0x60] = instruction!(rts);
    table[0xA5] = instruction!(lda, DirectPage, A);
    table[0xCD] = instruction!(cmp, Absolute, A);
    table[0xF0] = instruction!(beq, Relative);
    table[0x38] = instruction!(sec);
    table[0xA6] = instruction!(ldx, DirectPage, X);
    table[0xEC] = instruction!(cpx, Absolute, X);
    table
}

fn nop(_: &mut Cpu<impl Bus>) {}

fn sec(cpu: &mut Cpu<impl Bus>) {
    cpu.status.carry = true;
}

fn jsr(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.stack_push(cpu.pc.offset - 1);
    cpu.pc = operand.addr().unwrap();
}

fn rts(cpu: &mut Cpu<impl Bus>) {
    cpu.pc.offset = cpu.stack_pop();
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

fn pha<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.stack_push(cpu.a.get::<T>());
}

fn plb(cpu: &mut Cpu<impl Bus>) {
    cpu.db = cpu.stack_pop();
    cpu.update_negative_zero_flags(cpu.db);
}

fn pla<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value: T = cpu.stack_pop();
    cpu.a.set(value);
    cpu.update_negative_zero_flags(value);
}

fn rep(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data: u8 = operand.load(cpu);
    cpu.status = (u8::from(cpu.status) & !data).into();
}

fn sep(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data: u8 = operand.load(cpu);
    cpu.status = (u8::from(cpu.status) | data).into();
}

fn ldx<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

fn ldy<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
}

fn lda<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.a.set(value);
    cpu.update_negative_zero_flags(value);
}

fn lsr(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data: u8 = operand.load(cpu);
    let result = data >> 1;
    cpu.status.carry = data & 1 != 0;
    cpu.update_negative_zero_flags(result);
    operand.store(cpu, result);
}

fn sta<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, cpu.a.get::<T>());
}

fn sty<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, cpu.y.get::<T>());
}

fn stx<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, cpu.x.get::<T>());
}

fn stz(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, 0_u8);
}

fn txs<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value: T = cpu.x.get();
    cpu.s = value.to_u16().unwrap();
    cpu.update_negative_zero_flags(value);
}

fn inx<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value: T = cpu.x.get::<T>().wrapping_add(&T::one());
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

fn dex<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value: T = cpu.x.get::<T>().wrapping_sub(&T::one());
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

fn dey<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value: T = cpu.y.get::<T>().wrapping_sub(&T::one());
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
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
    cpu.d = cpu.a.get();
    cpu.update_negative_zero_flags(cpu.d);
}

fn jml(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.pc = operand.addr().unwrap();
}

fn and<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let result = cpu.a.get::<T>() & operand_value;
    cpu.a.set(result);
    cpu.update_negative_zero_flags(result);
}

fn bit(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value = operand.load::<u8>(cpu);
    let flags = StatusFlags::from(value);
    cpu.status.negative = flags.negative;
    cpu.status.overflow = flags.overflow;
    cpu.status.zero = (value & cpu.a.get::<u8>()) == 0;
}

fn cpx<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let (value, overflow) = cpu.x.get::<T>().overflowing_sub(&operand_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !overflow;
}

fn cmp<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let (value, overflow) = cpu.a.get::<T>().overflowing_sub(&operand_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !overflow;
}

fn adc<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.decimal {
        let value: T = operand.load(cpu);
        let (result, overflow, carry) = cpu.a.get::<T>().add_bcd(value, cpu.status.carry);
        cpu.update_negative_zero_flags(result);
        cpu.status.carry = carry;
        cpu.status.overflow = overflow;
        cpu.a.set(result);
    } else {
        let value: T = operand.load(cpu);
        let (mut result, mut overflow) = cpu.a.get::<T>().overflowing_add(&value);
        if cpu.status.carry {
            let (result2, overflow2) = result.overflowing_add(&T::one());
            result = result2;
            overflow |= overflow2;
        }
        cpu.update_negative_zero_flags(result);
        cpu.status.carry = overflow;
        cpu.status.overflow = ((cpu.a.get::<T>() ^ result) & (value ^ result)).bit(T::N_BITS - 1);
        cpu.a.set(result);
    }
}
