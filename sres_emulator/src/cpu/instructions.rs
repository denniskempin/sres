use super::status::StatusFlags;
use super::Cpu;
use super::EmuVectorTable;
use super::NativeVectorTable;
use super::UInt;
use crate::bus::Bus;
use crate::cpu::operands::AddressMode;
use crate::cpu::operands::Operand;
use crate::cpu::operands::PeekWrapper;
use crate::cpu::operands::ReadWrapper;
use crate::cpu::operands::Rwm;
use crate::memory::Address;
use crate::memory::Wrap;

pub struct InstructionMeta {
    pub operation: &'static str,
    pub operand_str: Option<String>,
    pub effective_addr: Option<Address>,
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
                    cpu.pc = cpu.pc.add(1_u8, Wrap::WrapBank);
                },
                meta: |_, instruction_addr| {
                    (
                        InstructionMeta {
                            operation: stringify!($method),
                            operand_str: None,
                            effective_addr: None,
                        },
                        instruction_addr.add(1_u8, Wrap::WrapBank),
                    )
                },
            }
        };
        // Instruction with operand
        ($method: ident, $address_mode: expr, $rwm: expr) => {
            Instruction::<BusT> {
                execute: |cpu| {
                    let pc = cpu.pc;
                    let (operand, next_addr) =
                        Operand::decode(&mut ReadWrapper(cpu), pc, $address_mode, $rwm);
                    cpu.pc = next_addr;
                    $method(cpu, &operand);
                },
                meta: |cpu, instruction_addr| {
                    let (operand, next_addr) = Operand::decode(
                        &mut PeekWrapper(cpu),
                        instruction_addr,
                        $address_mode,
                        $rwm,
                    );
                    (
                        InstructionMeta {
                            operation: stringify!($method),
                            operand_str: Some(operand.format()),
                            effective_addr: operand.effective_addr(),
                        },
                        next_addr,
                    )
                },
            }
        };
        // Instruction with operand and variable register size
        ($method: ident, $address_mode: expr, $rwm: expr, $register: expr) => {
            Instruction::<BusT> {
                execute: |cpu| {
                    let pc = cpu.pc;
                    let (operand, next_addr) =
                        Operand::decode(&mut ReadWrapper(cpu), pc, $address_mode, $rwm);
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
                    let (operand, next_addr) = Operand::decode(
                        &mut PeekWrapper(cpu),
                        instruction_addr,
                        $address_mode,
                        $rwm,
                    );
                    (
                        InstructionMeta {
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

    let mut opcodes = [(); 256].map(|_| Instruction::<BusT> {
        execute: |_| {
            println!("Unimplemented instruction");
        },
        meta: |_, addr| {
            (
                InstructionMeta {
                    operation: "ill",
                    operand_str: None,
                    effective_addr: None,
                },
                addr,
            )
        },
    });

    use AddressMode::*;
    use Register::*;
    use Rwm::*;
    opcodes[0x00] = instruction!(brk);
    opcodes[0x01] = instruction!(ora, DirectPageXIndexedIndirect, Read, A);
    opcodes[0x02] = instruction!(cop, ImmediateU8, Read);
    opcodes[0x03] = instruction!(ora, StackRelative, Read, A);
    opcodes[0x04] = instruction!(tsb, DirectPage, Read, A);
    opcodes[0x05] = instruction!(ora, DirectPage, Read, A);
    opcodes[0x06] = instruction!(asl, DirectPage, Modify, A);
    opcodes[0x07] = instruction!(ora, DirectPageIndirectLong, Read, A);
    opcodes[0x08] = instruction!(php);
    opcodes[0x09] = instruction!(ora, ImmediateA, Read, A);
    opcodes[0x0A] = instruction!(asl, Accumulator, Modify, A);
    opcodes[0x0B] = instruction!(phd);
    opcodes[0x0C] = instruction!(tsb, AbsoluteData, Read, A);
    opcodes[0x0D] = instruction!(ora, AbsoluteData, Read, A);
    opcodes[0x0E] = instruction!(asl, AbsoluteData, Modify, A);
    opcodes[0x0F] = instruction!(ora, AbsoluteLong, Read, A);
    opcodes[0x10] = instruction!(bpl, Relative, Read);
    opcodes[0x10] = instruction!(bpl, Relative, Read);
    opcodes[0x11] = instruction!(ora, DirectPageIndirectYIndexed, Read, A);
    opcodes[0x12] = instruction!(ora, DirectPageIndirect, Read, A);
    opcodes[0x13] = instruction!(ora, StackRelativeIndirectYIndexed, Read, A);
    opcodes[0x14] = instruction!(trb, DirectPage, Read, A);
    opcodes[0x15] = instruction!(ora, DirectPageXIndexed, Read, A);
    opcodes[0x16] = instruction!(asl, DirectPageXIndexed, Modify, A);
    opcodes[0x17] = instruction!(ora, DirectPageIndirectYIndexedLong, Read, A);
    opcodes[0x18] = instruction!(clc);
    opcodes[0x19] = instruction!(ora, AbsoluteYIndexed, Read, A);
    opcodes[0x1A] = instruction!(inc, Accumulator, Modify, A);
    opcodes[0x1B] = instruction!(tcs);
    opcodes[0x1C] = instruction!(trb, AbsoluteData, Read, A);
    opcodes[0x1D] = instruction!(ora, AbsoluteXIndexed, Read, A);
    opcodes[0x1E] = instruction!(asl, AbsoluteXIndexed, Modify, A);
    opcodes[0x1F] = instruction!(ora, AbsoluteXIndexedLong, Read, A);
    opcodes[0x20] = instruction!(jsr, AbsoluteData, Read);
    opcodes[0x21] = instruction!(and, DirectPageXIndexedIndirect, Read, A);
    opcodes[0x22] = instruction!(jsl, AbsoluteLong, Read);
    opcodes[0x23] = instruction!(and, StackRelative, Read, A);
    opcodes[0x24] = instruction!(bit, DirectPage, Read, A);
    opcodes[0x25] = instruction!(and, DirectPage, Read, A);
    opcodes[0x26] = instruction!(rol, DirectPage, Modify, A);
    opcodes[0x27] = instruction!(and, DirectPageIndirectLong, Read, A);
    opcodes[0x28] = instruction!(plp);
    opcodes[0x29] = instruction!(and, ImmediateA, Read, A);
    opcodes[0x2A] = instruction!(rol, Accumulator, Modify, A);
    opcodes[0x2B] = instruction!(pld);
    opcodes[0x2C] = instruction!(bit, AbsoluteData, Read, A);
    opcodes[0x2D] = instruction!(and, AbsoluteData, Read, A);
    opcodes[0x2E] = instruction!(rol, AbsoluteData, Modify, A);
    opcodes[0x2F] = instruction!(and, AbsoluteLong, Read, A);
    opcodes[0x30] = instruction!(bmi, Relative, Read);
    opcodes[0x31] = instruction!(and, DirectPageIndirectYIndexed, Read, A);
    opcodes[0x32] = instruction!(and, DirectPageIndirect, Read, A);
    opcodes[0x33] = instruction!(and, StackRelativeIndirectYIndexed, Read, A);
    opcodes[0x34] = instruction!(bit, DirectPageXIndexed, Read, A);
    opcodes[0x35] = instruction!(and, DirectPageXIndexed, Read, A);
    opcodes[0x36] = instruction!(rol, DirectPageXIndexed, Modify, A);
    opcodes[0x37] = instruction!(and, DirectPageIndirectYIndexedLong, Read, A);
    opcodes[0x38] = instruction!(sec);
    opcodes[0x39] = instruction!(and, AbsoluteYIndexed, Read, A);
    opcodes[0x3A] = instruction!(dec, Accumulator, Write, A);
    opcodes[0x3B] = instruction!(tsc);
    opcodes[0x3C] = instruction!(bit, AbsoluteXIndexed, Read, A);
    opcodes[0x3D] = instruction!(and, AbsoluteXIndexed, Read, A);
    opcodes[0x3E] = instruction!(rol, AbsoluteXIndexed, Modify, A);
    opcodes[0x3F] = instruction!(and, AbsoluteXIndexedLong, Read, A);
    opcodes[0x40] = instruction!(rti);
    opcodes[0x41] = instruction!(eor, DirectPageXIndexedIndirect, Read, A);
    opcodes[0x42] = instruction!(wdm);
    opcodes[0x43] = instruction!(eor, StackRelative, Read, A);
    opcodes[0x45] = instruction!(eor, DirectPage, Read, A);
    opcodes[0x46] = instruction!(lsr, DirectPage, Modify, A);
    opcodes[0x47] = instruction!(eor, DirectPageIndirectLong, Read, A);
    opcodes[0x48] = instruction!(pha, Implied, Read, A);
    opcodes[0x49] = instruction!(eor, ImmediateA, Read, A);
    opcodes[0x4A] = instruction!(lsr, Accumulator, Modify, A);
    opcodes[0x4B] = instruction!(phk);
    opcodes[0x4C] = instruction!(jmp, AbsoluteJump, Read);
    opcodes[0x4D] = instruction!(eor, AbsoluteData, Read, A);
    opcodes[0x4E] = instruction!(lsr, AbsoluteData, Modify, A);
    opcodes[0x4F] = instruction!(eor, AbsoluteLong, Read, A);
    opcodes[0x50] = instruction!(bvc, Relative, Read);
    opcodes[0x51] = instruction!(eor, DirectPageIndirectYIndexed, Read, A);
    opcodes[0x52] = instruction!(eor, DirectPageIndirect, Read, A);
    opcodes[0x53] = instruction!(eor, StackRelativeIndirectYIndexed, Read, A);
    opcodes[0x55] = instruction!(eor, DirectPageXIndexed, Read, A);
    opcodes[0x56] = instruction!(lsr, DirectPageXIndexed, Modify, A);
    opcodes[0x57] = instruction!(eor, DirectPageIndirectYIndexedLong, Read, A);
    opcodes[0x58] = instruction!(cli);
    opcodes[0x59] = instruction!(eor, AbsoluteYIndexed, Read, A);
    opcodes[0x5A] = instruction!(phy, Implied, Read, Y);
    opcodes[0x5B] = instruction!(tcd);
    opcodes[0x5C] = instruction!(jml, AbsoluteLong, Read);
    opcodes[0x5D] = instruction!(eor, AbsoluteXIndexed, Read, A);
    opcodes[0x5E] = instruction!(lsr, AbsoluteXIndexed, Modify, A);
    opcodes[0x5F] = instruction!(eor, AbsoluteXIndexedLong, Read, A);
    opcodes[0x60] = instruction!(rts);
    opcodes[0x61] = instruction!(adc, DirectPageXIndexedIndirect, Read, A);
    opcodes[0x62] = instruction!(per, RelativeLong, Read);
    opcodes[0x63] = instruction!(adc, StackRelative, Read, A);
    opcodes[0x64] = instruction!(stz, DirectPage, Write, A);
    opcodes[0x65] = instruction!(adc, DirectPage, Read, A);
    opcodes[0x66] = instruction!(ror, DirectPage, Modify, A);
    opcodes[0x67] = instruction!(adc, DirectPageIndirectLong, Read, A);
    opcodes[0x68] = instruction!(pla, Implied, Read, A);
    opcodes[0x69] = instruction!(adc, ImmediateA, Read, A);
    opcodes[0x6A] = instruction!(ror, Accumulator, Modify, A);
    opcodes[0x6B] = instruction!(rtl);
    opcodes[0x6C] = instruction!(jmp, AbsoluteIndirectJump, Read);
    opcodes[0x6D] = instruction!(adc, AbsoluteData, Read, A);
    opcodes[0x6E] = instruction!(ror, AbsoluteData, Modify, A);
    opcodes[0x6F] = instruction!(adc, AbsoluteLong, Read, A);
    opcodes[0x70] = instruction!(bvs, Relative, Read);
    opcodes[0x71] = instruction!(adc, DirectPageIndirectYIndexed, Read, A);
    opcodes[0x72] = instruction!(adc, DirectPageIndirect, Read, A);
    opcodes[0x73] = instruction!(adc, StackRelativeIndirectYIndexed, Read, A);
    opcodes[0x74] = instruction!(stz, DirectPageXIndexed, Write, A);
    opcodes[0x75] = instruction!(adc, DirectPageXIndexed, Read, A);
    opcodes[0x76] = instruction!(ror, DirectPageXIndexed, Modify, A);
    opcodes[0x77] = instruction!(adc, DirectPageIndirectYIndexedLong, Read, A);
    opcodes[0x78] = instruction!(sei);
    opcodes[0x79] = instruction!(adc, AbsoluteYIndexed, Read, A);
    opcodes[0x7A] = instruction!(ply, Implied, Read, Y);
    opcodes[0x7B] = instruction!(tdc);
    opcodes[0x7C] = instruction!(jmp, AbsoluteXIndexedIndirectJump, Read);
    opcodes[0x7D] = instruction!(adc, AbsoluteXIndexed, Read, A);
    opcodes[0x7E] = instruction!(ror, AbsoluteXIndexed, Modify, A);
    opcodes[0x7F] = instruction!(adc, AbsoluteXIndexedLong, Read, A);
    opcodes[0x80] = instruction!(bra, Relative, Read);
    opcodes[0x81] = instruction!(sta, DirectPageXIndexedIndirect, Write, A);
    opcodes[0x82] = instruction!(brl, RelativeLong, Read);
    opcodes[0x83] = instruction!(sta, StackRelative, Write, A);
    opcodes[0x84] = instruction!(sty, DirectPage, Write, Y);
    opcodes[0x85] = instruction!(sta, DirectPage, Write, A);
    opcodes[0x86] = instruction!(stx, DirectPage, Write, X);
    opcodes[0x87] = instruction!(sta, DirectPageIndirectLong, Write, A);
    opcodes[0x88] = instruction!(dey, Implied, Read, Y);
    opcodes[0x89] = instruction!(bit, ImmediateA, Read, A);
    opcodes[0x8A] = instruction!(txa, Implied, Read, A);
    opcodes[0x8B] = instruction!(phb);
    opcodes[0x8C] = instruction!(sty, AbsoluteData, Write, Y);
    opcodes[0x8D] = instruction!(sta, AbsoluteData, Write, A);
    opcodes[0x8E] = instruction!(stx, AbsoluteData, Write, X);
    opcodes[0x8F] = instruction!(sta, AbsoluteLong, Write, A);
    opcodes[0x90] = instruction!(bcc, Relative, Read);
    opcodes[0x91] = instruction!(sta, DirectPageIndirectYIndexed, Write, A);
    opcodes[0x92] = instruction!(sta, DirectPageIndirect, Write, A);
    opcodes[0x93] = instruction!(sta, StackRelativeIndirectYIndexed, Write, A);
    opcodes[0x94] = instruction!(sty, DirectPageXIndexed, Write, Y);
    opcodes[0x95] = instruction!(sta, DirectPageXIndexed, Write, A);
    opcodes[0x96] = instruction!(stx, DirectPageYIndexed, Write, X);
    opcodes[0x97] = instruction!(sta, DirectPageIndirectYIndexedLong, Write, A);
    opcodes[0x98] = instruction!(tya, Implied, Read, A);
    opcodes[0x99] = instruction!(sta, AbsoluteYIndexed, Write, A);
    opcodes[0x9A] = instruction!(txs);
    opcodes[0x9B] = instruction!(txy, Implied, Read, X);
    opcodes[0x9C] = instruction!(stz, AbsoluteData, Write, A);
    opcodes[0x9D] = instruction!(sta, AbsoluteXIndexed, Write, A);
    opcodes[0x9E] = instruction!(stz, AbsoluteXIndexed, Write, A);
    opcodes[0x9F] = instruction!(sta, AbsoluteXIndexedLong, Write, A);
    opcodes[0xA0] = instruction!(ldy, ImmediateXY, Read, Y);
    opcodes[0xA1] = instruction!(lda, DirectPageXIndexedIndirect, Read, A);
    opcodes[0xA2] = instruction!(ldx, ImmediateXY, Read, X);
    opcodes[0xA3] = instruction!(lda, StackRelative, Read, A);
    opcodes[0xA4] = instruction!(ldy, DirectPage, Read, Y);
    opcodes[0xA5] = instruction!(lda, DirectPage, Read, A);
    opcodes[0xA6] = instruction!(ldx, DirectPage, Read, X);
    opcodes[0xA7] = instruction!(lda, DirectPageIndirectLong, Read, A);
    opcodes[0xA8] = instruction!(tay, Implied, Read, Y);
    opcodes[0xA9] = instruction!(lda, ImmediateA, Read, A);
    opcodes[0xAA] = instruction!(tax, Implied, Read, X);
    opcodes[0xAB] = instruction!(plb);
    opcodes[0xAC] = instruction!(ldy, AbsoluteData, Read, Y);
    opcodes[0xAD] = instruction!(lda, AbsoluteData, Read, A);
    opcodes[0xAE] = instruction!(ldx, AbsoluteData, Read, X);
    opcodes[0xAF] = instruction!(lda, AbsoluteLong, Read, A);
    opcodes[0xB0] = instruction!(bcs, Relative, Read);
    opcodes[0xB1] = instruction!(lda, DirectPageIndirectYIndexed, Read, A);
    opcodes[0xB2] = instruction!(lda, DirectPageIndirect, Read, A);
    opcodes[0xB3] = instruction!(lda, StackRelativeIndirectYIndexed, Read, A);
    opcodes[0xB4] = instruction!(ldy, DirectPageXIndexed, Read, Y);
    opcodes[0xB5] = instruction!(lda, DirectPageXIndexed, Read, A);
    opcodes[0xB6] = instruction!(ldx, DirectPageYIndexed, Read, X);
    opcodes[0xB7] = instruction!(lda, DirectPageIndirectYIndexedLong, Read, A);
    opcodes[0xB8] = instruction!(clv);
    opcodes[0xB9] = instruction!(lda, AbsoluteYIndexed, Read, A);
    opcodes[0xBA] = instruction!(tsx, Implied, Read, X);
    opcodes[0xBB] = instruction!(tyx, Implied, Read, Y);
    opcodes[0xBC] = instruction!(ldy, AbsoluteXIndexed, Read, Y);
    opcodes[0xBD] = instruction!(lda, AbsoluteXIndexed, Read, A);
    opcodes[0xBE] = instruction!(ldx, AbsoluteYIndexed, Read, X);
    opcodes[0xBF] = instruction!(lda, AbsoluteXIndexedLong, Read, A);
    opcodes[0xC0] = instruction!(cpy, ImmediateXY, Read, Y);
    opcodes[0xC1] = instruction!(cmp, DirectPageXIndexedIndirect, Read, A);
    opcodes[0xC2] = instruction!(rep, ImmediateU8, Read);
    opcodes[0xC3] = instruction!(cmp, StackRelative, Read, A);
    opcodes[0xC4] = instruction!(cpy, DirectPage, Read, Y);
    opcodes[0xC5] = instruction!(cmp, DirectPage, Read, A);
    opcodes[0xC6] = instruction!(dec, DirectPage, Write, A);
    opcodes[0xC7] = instruction!(cmp, DirectPageIndirectLong, Read, A);
    opcodes[0xC8] = instruction!(iny, Implied, Read, Y);
    opcodes[0xC9] = instruction!(cmp, ImmediateA, Read, A);
    opcodes[0xCA] = instruction!(dex, Implied, Read, X);
    opcodes[0xCB] = instruction!(wai);
    opcodes[0xCC] = instruction!(cpy, AbsoluteData, Read, Y);
    opcodes[0xCD] = instruction!(cmp, AbsoluteData, Read, A);
    opcodes[0xCD] = instruction!(cmp, AbsoluteData, Read, A);
    opcodes[0xCE] = instruction!(dec, AbsoluteData, Write, A);
    opcodes[0xCF] = instruction!(cmp, AbsoluteLong, Read, A);
    opcodes[0xD0] = instruction!(bne, Relative, Read);
    opcodes[0xD0] = instruction!(bne, Relative, Read);
    opcodes[0xD1] = instruction!(cmp, DirectPageIndirectYIndexed, Read, A);
    opcodes[0xD2] = instruction!(cmp, DirectPageIndirect, Read, A);
    opcodes[0xD3] = instruction!(cmp, StackRelativeIndirectYIndexed, Read, A);
    opcodes[0xD4] = instruction!(pei, DirectPageIndirect, Read);
    opcodes[0xD5] = instruction!(cmp, DirectPageXIndexed, Read, A);
    opcodes[0xD6] = instruction!(dec, DirectPageXIndexed, Write, A);
    opcodes[0xD7] = instruction!(cmp, DirectPageIndirectYIndexedLong, Read, A);
    opcodes[0xD8] = instruction!(cld);
    opcodes[0xD9] = instruction!(cmp, AbsoluteYIndexed, Read, A);
    opcodes[0xDA] = instruction!(phx, Implied, Read, X);
    opcodes[0xDB] = instruction!(stp);
    opcodes[0xDC] = instruction!(jmp, AbsoluteIndirectLong, Read);
    opcodes[0xDD] = instruction!(cmp, AbsoluteXIndexed, Read, A);
    opcodes[0xDE] = instruction!(dec, AbsoluteXIndexed, Write, A);
    opcodes[0xDF] = instruction!(cmp, AbsoluteXIndexedLong, Read, A);
    opcodes[0xE0] = instruction!(cpx, ImmediateXY, Read, X);
    opcodes[0xE1] = instruction!(sbc, DirectPageXIndexedIndirect, Read, A);
    opcodes[0xE2] = instruction!(sep, ImmediateU8, Read);
    opcodes[0xE3] = instruction!(sbc, StackRelative, Read, A);
    opcodes[0xE4] = instruction!(cpx, DirectPage, Read, X);
    opcodes[0xE5] = instruction!(sbc, DirectPage, Read, A);
    opcodes[0xE6] = instruction!(inc, DirectPage, Modify, A);
    opcodes[0xE7] = instruction!(sbc, DirectPageIndirectLong, Read, A);
    opcodes[0xE8] = instruction!(inx, Implied, Read, X);
    opcodes[0xE9] = instruction!(sbc, ImmediateA, Read, A);
    opcodes[0xEA] = instruction!(nop);
    opcodes[0xEB] = instruction!(xba);
    opcodes[0xEC] = instruction!(cpx, AbsoluteData, Read, X);
    opcodes[0xED] = instruction!(sbc, AbsoluteData, Read, A);
    opcodes[0xEE] = instruction!(inc, AbsoluteData, Modify, A);
    opcodes[0xEF] = instruction!(sbc, AbsoluteLong, Read, A);
    opcodes[0xF0] = instruction!(beq, Relative, Read);
    opcodes[0xF0] = instruction!(beq, Relative, Read);
    opcodes[0xF1] = instruction!(sbc, DirectPageIndirectYIndexed, Read, A);
    opcodes[0xF2] = instruction!(sbc, DirectPageIndirect, Read, A);
    opcodes[0xF3] = instruction!(sbc, StackRelativeIndirectYIndexed, Read, A);
    opcodes[0xF4] = instruction!(pea, AbsoluteData, Read);
    opcodes[0xF5] = instruction!(sbc, DirectPageXIndexed, Read, A);
    opcodes[0xF6] = instruction!(inc, DirectPageXIndexed, Modify, A);
    opcodes[0xF7] = instruction!(sbc, DirectPageIndirectYIndexedLong, Read, A);
    opcodes[0xF8] = instruction!(sed);
    opcodes[0xF9] = instruction!(sbc, AbsoluteYIndexed, Read, A);
    opcodes[0xFA] = instruction!(plx, Implied, Read, X);
    opcodes[0xFB] = instruction!(xce);
    opcodes[0xFC] = instruction!(jsr, AbsoluteXIndexedIndirectJump, Read);
    opcodes[0xFD] = instruction!(sbc, AbsoluteXIndexed, Read, A);
    opcodes[0xFE] = instruction!(inc, AbsoluteXIndexed, Modify, A);
    opcodes[0xFF] = instruction!(sbc, AbsoluteXIndexedLong, Read, A);
    opcodes
}

fn nop(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
}

fn sec(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.status.carry = true;
}

fn sed(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.status.decimal = true;
}

fn sei(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.status.irq_disable = true;
}

fn txs(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    if cpu.emulation_mode {
        cpu.s = 0x0100 + cpu.x.get::<u8>() as u16;
        cpu.update_negative_zero_flags(cpu.x.get::<u8>())
    }
    cpu.s = cpu.x.get::<u16>();
}

fn xba(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    let a = cpu.a.get::<u16>();
    cpu.a.set(a.swap_bytes());
    cpu.update_negative_zero_flags(cpu.a.get::<u8>());
}

fn tsx<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.x.set(T::from_u16(cpu.s));
    cpu.update_negative_zero_flags(cpu.x.get::<T>())
}

fn txy<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.y.set(cpu.x.get::<T>());
    cpu.update_negative_zero_flags(cpu.y.get::<T>());
}

fn txa<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.a.set(cpu.x.get::<T>());
    cpu.update_negative_zero_flags(cpu.a.get::<T>());
}

fn tya<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.a.set(cpu.y.get::<T>());
    cpu.update_negative_zero_flags(cpu.a.get::<T>());
}

fn tyx<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.x.set(cpu.y.get::<T>());
    cpu.update_negative_zero_flags(cpu.x.get::<T>());
}

fn tax<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.x.set(cpu.a.get::<T>());
    cpu.update_negative_zero_flags(cpu.x.get::<T>());
}

fn tay<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.y.set(cpu.a.get::<T>());
    cpu.update_negative_zero_flags(cpu.y.get::<T>());
}

fn tcs(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.s = cpu.a.get::<u16>();
}

fn tsc(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.a.set(cpu.s);
    cpu.update_negative_zero_flags(cpu.s);
}

fn tdc(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.a.set(cpu.d);
    cpu.update_negative_zero_flags(cpu.d);
}

fn trb<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let result = value & !cpu.a.get::<T>();
    operand.store(cpu, result);
    cpu.status.zero = (value & cpu.a.get::<T>()) == T::zero();
}

fn tsb<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let result = value | cpu.a.get::<T>();
    operand.store(cpu, result);
    cpu.status.zero = (value & cpu.a.get::<T>()) == T::zero();
}

fn jmp(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.pc = operand.effective_addr().unwrap();
}

fn jml(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.pc = operand.effective_addr().unwrap();
}

fn jsr(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    // JSR has an extra IO cycle in the Absolute addressing mode
    if let Operand::Address(_, address_mode, _) = operand {
        if *address_mode == AddressMode::AbsoluteData {
            cpu.bus.cycle_io();
        }
    }
    cpu.stack_push_u16(cpu.pc.offset - 1);
    cpu.pc.offset = operand.effective_addr().unwrap().offset;
}

fn jsl(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    // JSL has an extra IO cycle in the AbsoluteLong addressing mode
    if let Operand::Address(_, address_mode, _) = operand {
        if *address_mode == AddressMode::AbsoluteLong {
            cpu.bus.cycle_io();
        }
    }
    cpu.stack_push_u24(u32::from(cpu.pc) - 1);
    cpu.pc = operand.effective_addr().unwrap();
}

fn rts(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc.offset = cpu.stack_pop_u16();
    cpu.bus.cycle_io();
}

fn rti(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.status = StatusFlags::from(cpu.stack_pop_u8());
    cpu.update_register_sizes();
    cpu.pc = Address::from(cpu.stack_pop_u24());
}

fn rtl(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = cpu.stack_pop_u24().into();
}

fn rol<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let mut result = value << 1;
    result.set_bit(0, cpu.status.carry);
    operand.store(cpu, result);
    cpu.status.carry = value.msb();
    cpu.status.zero = result == T::zero();
    cpu.status.negative = result.msb();
}

fn ror<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let mut result = value >> 1;
    result.set_bit(T::N_BITS - 1, cpu.status.carry);
    operand.store(cpu, result);
    cpu.status.carry = value.lsb();
    cpu.status.zero = result == T::zero();
    cpu.status.negative = result.msb();
}

fn bra(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.bus.cycle_io();
    cpu.pc = operand.effective_addr().unwrap();
}

fn brl(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.bus.cycle_io();
    cpu.pc = operand.effective_addr().unwrap();
}

fn bcc(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if !cpu.status.carry {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

fn bcs(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.carry {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

fn beq(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.zero {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

fn bne(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if !cpu.status.zero {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

fn bpl(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if !cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

fn bmi(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

fn bvc(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if !cpu.status.overflow {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

fn bvs(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.overflow {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

fn brk(cpu: &mut Cpu<impl Bus>) {
    // read signature byte, even though it is unused.
    cpu.bus.cycle_read_u8(cpu.pc.add(1_u8, Wrap::WrapBank));
    cpu.stack_push_u24(u32::from(cpu.pc.add(1_u8, Wrap::WrapBank)));
    cpu.stack_push_u8(u8::from(cpu.status));
    cpu.status.irq_disable = true;
    let address = if cpu.emulation_mode {
        cpu.bus
            .cycle_read_u16(Address::new(0, EmuVectorTable::Break as u16), Wrap::NoWrap)
    } else {
        cpu.bus.cycle_read_u16(
            Address::new(0, NativeVectorTable::Break as u16),
            Wrap::NoWrap,
        )
    };
    cpu.pc = ((address as u32).saturating_sub(1)).into();
}

fn cop(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.stack_push_u24(u32::from(cpu.pc) - 1);
    cpu.stack_push_u8(u8::from(cpu.status));
    cpu.status.irq_disable = true;
    let address = if cpu.emulation_mode {
        cpu.bus
            .cycle_read_u16(Address::new(0, EmuVectorTable::Cop as u16), Wrap::NoWrap)
    } else {
        cpu.bus
            .cycle_read_u16(Address::new(0, NativeVectorTable::Cop as u16), Wrap::NoWrap)
    };
    cpu.pc = (address as u32).into();
}

fn clc(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.status.carry = false;
}

fn cld(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.status.decimal = false;
}

fn cli(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.status.irq_disable = false;
}

fn xce(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    (cpu.status.carry, cpu.emulation_mode) = (cpu.emulation_mode, cpu.status.carry);
    if cpu.emulation_mode {
        cpu.status.accumulator_register_size = true;
        cpu.status.index_register_size_or_break = true;
        cpu.s = 0x0100 + (cpu.s & 0x00ff);
        cpu.update_register_sizes();
    }
}

fn pea(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.stack_push_u16(operand.effective_addr().unwrap().offset);
}

fn pei(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.stack_push_u16(operand.effective_addr().unwrap().offset);
}

fn per(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.bus.cycle_io();
    cpu.stack_push_u16(operand.effective_addr().unwrap().offset);
}

fn phk(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.stack_push_u8(cpu.pc.bank);
}

fn php(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.stack_push_u8(u8::from(cpu.status));
}

fn pha<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.stack_push(cpu.a.get::<T>());
}

fn phb(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.stack_push_u8(cpu.db);
}

fn phd(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.d);
}

fn phx<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.stack_push(cpu.x.get::<T>());
}

fn phy<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.stack_push(cpu.y.get::<T>());
}

fn plx<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    let value = cpu.stack_pop::<T>();
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

fn ply<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    let value = cpu.stack_pop::<T>();
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
}

fn plb(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.db = cpu.stack_pop_u8();
    cpu.update_negative_zero_flags(cpu.db);
}

fn pld(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.d = cpu.stack_pop_u16();
    cpu.update_negative_zero_flags(cpu.d);
}

fn plp(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.status = cpu.stack_pop_u8().into();
    cpu.update_register_sizes();
}

fn pla<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    let value = cpu.stack_pop::<T>();
    cpu.a.set(value);
    cpu.update_negative_zero_flags(value);
}

fn clv(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.status.overflow = false;
}

fn rep(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.bus.cycle_io();
    let data: u8 = operand.load(cpu);
    cpu.status = (u8::from(cpu.status) & !data).into();
    cpu.update_register_sizes();
}

fn sep(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    cpu.bus.cycle_io();
    let data: u8 = operand.load(cpu);
    cpu.status = (u8::from(cpu.status) | data).into();
    cpu.update_register_sizes();
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

fn lsr<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let data: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let result = data >> 1;
    cpu.status.carry = data.lsb();
    cpu.update_negative_zero_flags(result);
    operand.store(cpu, result);
}

fn sta<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, cpu.a.get::<T>());
}

fn stp(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
}

fn sty<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, cpu.y.get::<T>());
}

fn stx<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, cpu.x.get::<T>());
}

fn stz<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    operand.store(cpu, T::zero());
}

fn inc<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load::<T>(cpu).wrapping_add(&T::one());
    cpu.bus.cycle_io();
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

fn inx<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    let value: T = cpu.x.get::<T>().wrapping_add(&T::one());
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

fn iny<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    cpu.bus.cycle_io();
    let value: T = cpu.y.get::<T>().wrapping_add(&T::one());
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
}

fn dec<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let value: T = operand.load::<T>(cpu).wrapping_sub(&T::one());
    cpu.bus.cycle_io();
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

fn dex<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value: T = cpu.x.get::<T>().wrapping_sub(&T::one());
    cpu.bus.cycle_io();
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

fn dey<T: UInt>(cpu: &mut Cpu<impl Bus>, _: &Operand) {
    let value: T = cpu.y.get::<T>().wrapping_sub(&T::one());
    cpu.bus.cycle_io();
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
}

fn ora<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let result = cpu.a.get::<T>() | operand_value;
    cpu.a.set(result);
    cpu.update_negative_zero_flags(result);
}

fn eor<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let result = cpu.a.get::<T>() ^ operand_value;
    cpu.a.set(result);
    cpu.update_negative_zero_flags(result);
}

fn tcd(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.d = cpu.a.get();
    cpu.update_negative_zero_flags(cpu.d);
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
        cpu.status.negative = operand_value.msb();
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
    cpu.bus.cycle_io();
    cpu.status.carry = data.msb();
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
        cpu.status.overflow = ((cpu.a.get::<T>() ^ result) & (value ^ result)).msb();
        cpu.a.set(result);
    }
}

fn sbc<T: UInt>(cpu: &mut Cpu<impl Bus>, operand: &Operand) {
    if cpu.status.decimal {
        let value: T = operand.load(cpu);
        let (result, overflow, carry) = cpu.a.get::<T>().sub_bcd(value, cpu.status.carry);
        cpu.update_negative_zero_flags(result);
        cpu.status.carry = carry;
        cpu.status.overflow = overflow;
        cpu.a.set(result);
    } else {
        let value: T = operand.load(cpu);
        let (mut result, mut overflow) = cpu.a.get::<T>().overflowing_sub(&value);
        if !cpu.status.carry {
            let (result2, overflow2) = result.overflowing_sub(&T::one());
            result = result2;
            overflow |= overflow2;
        }
        cpu.update_negative_zero_flags(result);
        cpu.status.carry = !overflow;
        cpu.status.overflow = ((cpu.a.get::<T>() ^ result) & (!value ^ result)).msb();
        cpu.a.set(result);
    }
}

fn wdm(cpu: &mut Cpu<impl Bus>) {
    // William D. Mensch, Jr (WDM) opcode
    // Acts like a 2-byte NOP but without reading the second byte.
    cpu.bus.cycle_io();
    cpu.pc = cpu.pc.add(1_u8, Wrap::WrapBank);
}

fn wai(cpu: &mut Cpu<impl Bus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
}
