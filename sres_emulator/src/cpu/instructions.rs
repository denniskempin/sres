use super::Cpu;
use super::UInt;
use crate::bus::Bus;
use crate::cpu::operands::AddressMode;
use crate::cpu::operands::Operand;
use crate::memory::Address;
use crate::memory::ToAddress;

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

    let mut opcodes = [(); 256].map(|_| Instruction::<BusT> {
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
    opcodes[0x61] = instruction!(adc, DirectPageXIndexedIndirect, A);
    opcodes[0x63] = instruction!(adc, StackRelative, A);
    opcodes[0x65] = instruction!(adc, DirectPage, A);
    opcodes[0x67] = instruction!(adc, DirectPageIndirectLong, A);
    opcodes[0x69] = instruction!(adc, ImmediateA, A);
    opcodes[0x6D] = instruction!(adc, Absolute, A);
    opcodes[0x6F] = instruction!(adc, AbsoluteLong, A);
    opcodes[0x71] = instruction!(adc, DirectPageIndirectYIndexed, A);
    opcodes[0x72] = instruction!(adc, DirectPageIndirect, A);
    opcodes[0x73] = instruction!(adc, StackRelativeIndirectYIndexed, A);
    opcodes[0x75] = instruction!(adc, DirectPageXIndexed, A);
    opcodes[0x77] = instruction!(adc, DirectPageIndirectYIndexedLong, A);
    opcodes[0x79] = instruction!(adc, AbsoluteYIndexed, A);
    opcodes[0x7D] = instruction!(adc, AbsoluteXIndexed, A);
    opcodes[0x7F] = instruction!(adc, AbsoluteXIndexedLong, A);
    opcodes[0x21] = instruction!(and, DirectPageXIndexedIndirect, A);
    opcodes[0x23] = instruction!(and, StackRelative, A);
    opcodes[0x25] = instruction!(and, DirectPage, A);
    opcodes[0x27] = instruction!(and, DirectPageIndirectLong, A);
    opcodes[0x29] = instruction!(and, ImmediateA, A);
    opcodes[0x2D] = instruction!(and, Absolute, A);
    opcodes[0x2F] = instruction!(and, AbsoluteLong, A);
    opcodes[0x31] = instruction!(and, DirectPageIndirectYIndexed, A);
    opcodes[0x32] = instruction!(and, DirectPageIndirect, A);
    opcodes[0x33] = instruction!(and, StackRelativeIndirectYIndexed, A);
    opcodes[0x35] = instruction!(and, DirectPageXIndexed, A);
    opcodes[0x37] = instruction!(and, DirectPageIndirectYIndexedLong, A);
    opcodes[0x39] = instruction!(and, AbsoluteYIndexed, A);
    opcodes[0x3D] = instruction!(and, AbsoluteXIndexed, A);
    opcodes[0x3F] = instruction!(and, AbsoluteXIndexedLong, A);
    opcodes[0x06] = instruction!(asl, DirectPage, A);
    opcodes[0x0A] = instruction!(asl, Accumulator, A);
    opcodes[0x0E] = instruction!(asl, Absolute, A);
    opcodes[0x16] = instruction!(asl, DirectPageXIndexed, A);
    opcodes[0x1E] = instruction!(asl, AbsoluteXIndexed, A);
    opcodes[0x90] = instruction!(bcc, Relative);
    opcodes[0xB0] = instruction!(bcs, Relative);
    opcodes[0xF0] = instruction!(beq, Relative);
    opcodes[0xF0] = instruction!(beq, Relative);
    opcodes[0x24] = instruction!(bit, DirectPage, A);
    opcodes[0x2C] = instruction!(bit, Absolute, A);
    opcodes[0x34] = instruction!(bit, DirectPageXIndexed, A);
    opcodes[0x3C] = instruction!(bit, AbsoluteXIndexed, A);
    opcodes[0x89] = instruction!(bit, ImmediateA, A);
    opcodes[0x30] = instruction!(bmi, Relative);
    opcodes[0xD0] = instruction!(bne, Relative);
    opcodes[0xD0] = instruction!(bne, Relative);
    opcodes[0x10] = instruction!(bpl, Relative);
    opcodes[0x10] = instruction!(bpl, Relative);
    opcodes[0x80] = instruction!(bra, Relative);
    opcodes[0x82] = instruction!(brl, RelativeLong);
    opcodes[0x50] = instruction!(bvc, Relative);
    opcodes[0x70] = instruction!(bvs, Relative);
    opcodes[0x18] = instruction!(clc);
    opcodes[0xB8] = instruction!(clv);
    opcodes[0xC1] = instruction!(cmp, DirectPageXIndexedIndirect, A);
    opcodes[0xC3] = instruction!(cmp, StackRelative, A);
    opcodes[0xC5] = instruction!(cmp, DirectPage, A);
    opcodes[0xC7] = instruction!(cmp, DirectPageIndirectLong, A);
    opcodes[0xC9] = instruction!(cmp, ImmediateA, A);
    opcodes[0xCD] = instruction!(cmp, Absolute, A);
    opcodes[0xCD] = instruction!(cmp, Absolute, A);
    opcodes[0xCF] = instruction!(cmp, AbsoluteLong, A);
    opcodes[0xD1] = instruction!(cmp, DirectPageIndirectYIndexed, A);
    opcodes[0xD2] = instruction!(cmp, DirectPageIndirect, A);
    opcodes[0xD3] = instruction!(cmp, StackRelativeIndirectYIndexed, A);
    opcodes[0xD5] = instruction!(cmp, DirectPageXIndexed, A);
    opcodes[0xD7] = instruction!(cmp, DirectPageIndirectYIndexedLong, A);
    opcodes[0xD9] = instruction!(cmp, AbsoluteYIndexed, A);
    opcodes[0xDD] = instruction!(cmp, AbsoluteXIndexed, A);
    opcodes[0xDF] = instruction!(cmp, AbsoluteXIndexedLong, A);
    opcodes[0xE0] = instruction!(cpx, ImmediateXY, X);
    opcodes[0xE4] = instruction!(cpx, DirectPage, X);
    opcodes[0xEC] = instruction!(cpx, Absolute, X);
    opcodes[0xC0] = instruction!(cpy, ImmediateXY, Y);
    opcodes[0xC4] = instruction!(cpy, DirectPage, Y);
    opcodes[0xCC] = instruction!(cpy, Absolute, Y);
    opcodes[0x3A] = instruction!(dec, Accumulator, A);
    opcodes[0xC6] = instruction!(dec, DirectPage, A);
    opcodes[0xCE] = instruction!(dec, Absolute, A);
    opcodes[0xD6] = instruction!(dec, DirectPageXIndexed, A);
    opcodes[0xDE] = instruction!(dec, AbsoluteXIndexed, A);
    opcodes[0xCA] = instruction!(dex, Implied, X);
    opcodes[0x88] = instruction!(dey, Implied, Y);
    opcodes[0x41] = instruction!(eor, DirectPageXIndexedIndirect, A);
    opcodes[0x43] = instruction!(eor, StackRelative, A);
    opcodes[0x49] = instruction!(eor, ImmediateA, A);
    opcodes[0x45] = instruction!(eor, DirectPage, A);
    opcodes[0x47] = instruction!(eor, DirectPageIndirectLong, A);
    opcodes[0x4D] = instruction!(eor, Absolute, A);
    opcodes[0x4F] = instruction!(eor, AbsoluteLong, A);
    opcodes[0x51] = instruction!(eor, DirectPageIndirectYIndexed, A);
    opcodes[0x52] = instruction!(eor, DirectPageIndirect, A);
    opcodes[0x53] = instruction!(eor, StackRelativeIndirectYIndexed, A);
    opcodes[0x55] = instruction!(eor, DirectPageXIndexed, A);
    opcodes[0x57] = instruction!(eor, DirectPageIndirectYIndexedLong, A);
    opcodes[0x59] = instruction!(eor, AbsoluteYIndexed, A);
    opcodes[0x5D] = instruction!(eor, AbsoluteXIndexed, A);
    opcodes[0x5F] = instruction!(eor, AbsoluteXIndexedLong, A);
    opcodes[0x1A] = instruction!(inc, Accumulator, A);
    opcodes[0xE6] = instruction!(inc, DirectPage, A);
    opcodes[0xEE] = instruction!(inc, Absolute, A);
    opcodes[0xF6] = instruction!(inc, DirectPageXIndexed, A);
    opcodes[0xFE] = instruction!(inc, AbsoluteXIndexed, A);
    opcodes[0xE8] = instruction!(inx, Implied, X);
    opcodes[0xC8] = instruction!(iny, Implied, Y);
    opcodes[0x5C] = instruction!(jml, AbsoluteLong);
    opcodes[0x4C] = instruction!(jmp, Absolute);
    opcodes[0x6C] = instruction!(jmp, AbsoluteIndirect);
    opcodes[0x7C] = instruction!(jmp, AbsoluteXIndexedIndirect);
    opcodes[0xDC] = instruction!(jmp, AbsoluteIndirectLong);
    opcodes[0x20] = instruction!(jsr, Absolute);
    opcodes[0xFC] = instruction!(jsr, AbsoluteXIndexedIndirect);
    opcodes[0x22] = instruction!(jsl, AbsoluteLong);
    opcodes[0xA1] = instruction!(lda, DirectPageXIndexedIndirect, A);
    opcodes[0xA3] = instruction!(lda, StackRelative, A);
    opcodes[0xA5] = instruction!(lda, DirectPage, A);
    opcodes[0xA7] = instruction!(lda, DirectPageIndirectLong, A);
    opcodes[0xA9] = instruction!(lda, ImmediateA, A);
    opcodes[0xAD] = instruction!(lda, Absolute, A);
    opcodes[0xAF] = instruction!(lda, AbsoluteLong, A);
    opcodes[0xB1] = instruction!(lda, DirectPageIndirectYIndexed, A);
    opcodes[0xB2] = instruction!(lda, DirectPageIndirect, A);
    opcodes[0xB3] = instruction!(lda, StackRelativeIndirectYIndexed, A);
    opcodes[0xB5] = instruction!(lda, DirectPageXIndexed, A);
    opcodes[0xB7] = instruction!(lda, DirectPageIndirectYIndexedLong, A);
    opcodes[0xB9] = instruction!(lda, AbsoluteYIndexed, A);
    opcodes[0xBD] = instruction!(lda, AbsoluteXIndexed, A);
    opcodes[0xBF] = instruction!(lda, AbsoluteXIndexedLong, A);
    opcodes[0xA2] = instruction!(ldx, ImmediateXY, X);
    opcodes[0xA6] = instruction!(ldx, DirectPage, X);
    opcodes[0xAE] = instruction!(ldx, Absolute, X);
    opcodes[0xB6] = instruction!(ldx, DirectPageYIndexed, X);
    opcodes[0xBE] = instruction!(ldx, AbsoluteYIndexed, X);
    opcodes[0xA0] = instruction!(ldy, ImmediateXY, Y);
    opcodes[0xA4] = instruction!(ldy, DirectPage, Y);
    opcodes[0xAC] = instruction!(ldy, Absolute, Y);
    opcodes[0xB4] = instruction!(ldy, DirectPageXIndexed, Y);
    opcodes[0xBC] = instruction!(ldy, AbsoluteXIndexed, Y);
    opcodes[0x4E] = instruction!(lsr, Absolute);
    opcodes[0x46] = instruction!(lsr, DirectPage);
    opcodes[0x4A] = instruction!(lsr, Accumulator);
    opcodes[0xEA] = instruction!(nop);
    opcodes[0x48] = instruction!(pha, Implied, A);
    opcodes[0x4B] = instruction!(phk);
    opcodes[0x08] = instruction!(php);
    opcodes[0xDA] = instruction!(phx, Implied, X);
    opcodes[0x68] = instruction!(pla, Implied, A);
    opcodes[0xAB] = instruction!(plb);
    opcodes[0xC2] = instruction!(rep, ImmediateU8);
    opcodes[0x6B] = instruction!(rtl);
    opcodes[0x60] = instruction!(rts);
    opcodes[0x38] = instruction!(sec);
    opcodes[0x78] = instruction!(sei);
    opcodes[0xE2] = instruction!(sep, ImmediateU8);
    opcodes[0x85] = instruction!(sta, DirectPage, A);
    opcodes[0x8D] = instruction!(sta, Absolute, A);
    opcodes[0x86] = instruction!(stx, DirectPage, X);
    opcodes[0x8E] = instruction!(stx, Absolute, X);
    opcodes[0x84] = instruction!(sty, DirectPage, Y);
    opcodes[0x8C] = instruction!(sty, Absolute, Y);
    opcodes[0x9C] = instruction!(stz, Absolute);
    opcodes[0x5B] = instruction!(tcd);
    opcodes[0x14] = instruction!(trb, DirectPage, A);
    opcodes[0x1C] = instruction!(trb, Absolute, A);
    opcodes[0x04] = instruction!(tsb, DirectPage, A);
    opcodes[0x0C] = instruction!(tsb, Absolute, A);
    opcodes[0x9A] = instruction!(txs, Implied, X);
    opcodes[0xFB] = instruction!(xce);

    opcodes
}

fn nop(_: &mut Cpu<impl Bus>) {}

fn sec(cpu: &mut Cpu<impl Bus>) {
    cpu.status.carry = true;
}

fn trb<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    let result = value & !cpu.a.get::<T>();
    operand.store(cpu, result);
    cpu.status.zero = (value & cpu.a.get::<T>()) == T::zero();
}

fn tsb<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    let result = value | cpu.a.get::<T>();
    operand.store(cpu, result);
    cpu.status.zero = (value & cpu.a.get::<T>()) == T::zero();
}

fn jsr(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.stack_push_u16(cpu.pc.offset - 1);
    cpu.pc = operand.addr().unwrap();
}

fn jsl(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.stack_push_u24(u32::from(cpu.pc) - 1);
    cpu.pc = operand.addr().unwrap();
}

fn rts(cpu: &mut Cpu<impl Bus>) {
    cpu.pc.offset = cpu.stack_pop_u16();
}

fn rtl(cpu: &mut Cpu<impl Bus>) {
    cpu.pc = cpu.stack_pop_u24().to_address();
}

fn bra(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.pc = operand.addr().unwrap();
}

fn brl(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.pc = operand.addr().unwrap();
}

fn bcc(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if !cpu.status.carry {
        cpu.pc = operand.addr().unwrap();
    }
}

fn bcs(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.carry {
        cpu.pc = operand.addr().unwrap();
    }
}

fn beq(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.zero {
        cpu.pc = operand.addr().unwrap();
    }
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

fn bmi(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.negative {
        cpu.pc = operand.addr().unwrap();
    }
}

fn bvc(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if !cpu.status.overflow {
        cpu.pc = operand.addr().unwrap();
    }
}

fn bvs(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.overflow {
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
    cpu.stack_push_u8(cpu.db);
}

fn php(cpu: &mut Cpu<impl Bus>) {
    cpu.stack_push_u8(u8::from(cpu.status));
}

fn pha<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.stack_push(cpu.a.get::<T>());
}
fn phx<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.stack_push(cpu.x.get::<T>());
}

fn plb(cpu: &mut Cpu<impl Bus>) {
    cpu.db = cpu.stack_pop_u8();
    cpu.update_negative_zero_flags(cpu.db);
}

fn pla<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value = cpu.stack_pop::<T>();
    cpu.a.set(value);
    cpu.update_negative_zero_flags(value);
}

fn clv(cpu: &mut Cpu<impl Bus>) {
    cpu.status.overflow = false;
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
    cpu.s = value.to_u16();
    cpu.update_negative_zero_flags(value);
}

fn inc<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load::<T>(cpu).wrapping_add(&T::one());
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

fn inx<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value: T = cpu.x.get::<T>().wrapping_add(&T::one());
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

fn iny<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value: T = cpu.y.get::<T>().wrapping_add(&T::one());
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
}

fn dec<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load::<T>(cpu).wrapping_sub(&T::one());
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
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

fn eor<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let result = cpu.a.get::<T>() ^ operand_value;
    cpu.a.set(result);
    cpu.update_negative_zero_flags(result);
}

fn tcd(cpu: &mut Cpu<impl Bus>) {
    cpu.d = cpu.a.get();
    cpu.update_negative_zero_flags(cpu.d);
}

fn jmp(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.pc = operand.addr().unwrap();
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

fn bit<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let result: T = cpu.a.get::<T>() & operand_value;
    if let Operand::Address(_, _, _) = operand {
        cpu.status.negative = operand_value.bit(T::N_BITS - 1);
        cpu.status.overflow = operand_value.bit(T::N_BITS - 2);
    }
    cpu.status.zero = result == T::zero();
}

fn cpx<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let (value, overflow) = cpu.x.get::<T>().overflowing_sub(&operand_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !overflow;
}

fn cpy<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let (value, overflow) = cpu.y.get::<T>().overflowing_sub(&operand_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !overflow;
}

fn cmp<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let (value, overflow) = cpu.a.get::<T>().overflowing_sub(&operand_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !overflow;
}

fn asl<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data: T = operand.load(cpu);
    cpu.status.carry = data.bit(T::N_BITS - 1);
    let result = data << 1;
    cpu.update_negative_zero_flags(result);
    operand.store(cpu, result);
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
