//! Implements the 65816 CPU instruction set.
//!
//! One function per instruction, each named after the mnemonic. They are mapped to opcodes by the
//! (opcode table)[build_opcode_table].
//!
//! Some functions are implemented for u8 and u16 using generics. This allows the opcode table
//! to use the same implementation for operating on both u8 and u16 sized registers.
use super::operands::AddressMode;
use super::operands::Operand;
use super::status::StatusFlags;
use super::Cpu;
use super::EmuVectorTable;
use super::MainBus;
use super::NativeVectorTable;
use crate::common::address::Address;
use crate::common::address::AddressU24;
use crate::common::address::Wrap;
use crate::common::uint::UInt;

pub fn nop(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
}

pub fn sec(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.status.carry = true;
}

pub fn sed(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.status.decimal = true;
}

pub fn sei(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.status.irq_disable = true;
}

pub fn txs(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    if cpu.emulation_mode {
        cpu.s = 0x0100 + cpu.x.get::<u8>() as u16;
        cpu.update_negative_zero_flags(cpu.x.get::<u8>())
    }
    cpu.s = cpu.x.get::<u16>();
}

pub fn xba(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    let a = cpu.a.get::<u16>();
    cpu.a.set(a.swap_bytes());
    cpu.update_negative_zero_flags(cpu.a.get::<u8>());
}

pub fn tsx<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.x.set(T::from_u16(cpu.s));
    cpu.update_negative_zero_flags(cpu.x.get::<T>())
}

pub fn txy<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.y.set(cpu.x.get::<T>());
    cpu.update_negative_zero_flags(cpu.y.get::<T>());
}

pub fn txa<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.a.set(cpu.x.get::<T>());
    cpu.update_negative_zero_flags(cpu.a.get::<T>());
}

pub fn tya<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.a.set(cpu.y.get::<T>());
    cpu.update_negative_zero_flags(cpu.a.get::<T>());
}

pub fn tyx<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.x.set(cpu.y.get::<T>());
    cpu.update_negative_zero_flags(cpu.x.get::<T>());
}

pub fn tax<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.x.set(cpu.a.get::<T>());
    cpu.update_negative_zero_flags(cpu.x.get::<T>());
}

pub fn tay<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.y.set(cpu.a.get::<T>());
    cpu.update_negative_zero_flags(cpu.y.get::<T>());
}

pub fn tcs(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.s = cpu.a.get::<u16>();
}

pub fn tsc(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.a.set(cpu.s);
    cpu.update_negative_zero_flags(cpu.s);
}

pub fn tdc(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.a.set(cpu.d);
    cpu.update_negative_zero_flags(cpu.d);
}

pub fn trb<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let result = value & !cpu.a.get::<T>();
    operand.store(cpu, result);
    cpu.status.zero = (value & cpu.a.get::<T>()) == T::zero();
}

pub fn tsb<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let result = value | cpu.a.get::<T>();
    operand.store(cpu, result);
    cpu.status.zero = (value & cpu.a.get::<T>()) == T::zero();
}

pub fn jmp(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    cpu.pc = operand.effective_addr().unwrap();
}

pub fn jml(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    cpu.pc = operand.effective_addr().unwrap();
}

pub fn jsr(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    // JSR has an extra IO cycle in the Absolute addressing mode
    if let Operand::Address(_, address_mode, _) = operand {
        if *address_mode == AddressMode::AbsoluteData {
            cpu.bus.cycle_io();
        }
    }
    cpu.stack_push_u16(cpu.pc.offset - 1);
    cpu.pc.offset = operand.effective_addr().unwrap().offset;
}

pub fn jsl(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    // JSL has an extra IO cycle in the AbsoluteLong addressing mode
    if let Operand::Address(_, address_mode, _) = operand {
        if *address_mode == AddressMode::AbsoluteLong {
            cpu.bus.cycle_io();
        }
    }
    cpu.stack_push_u24(u32::from(cpu.pc) - 1);
    cpu.pc = operand.effective_addr().unwrap();
}

pub fn rts(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc.offset = cpu.stack_pop_u16();
    cpu.bus.cycle_io();
}

pub fn rti(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.status = StatusFlags::from(cpu.stack_pop_u8());
    cpu.update_register_sizes();
    cpu.pc = AddressU24::from(cpu.stack_pop_u24()).sub(1_u8, Wrap::WrapBank);
}

pub fn rtl(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.pc = cpu.stack_pop_u24().into();
}

pub fn rol<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let mut result = value << 1;
    result.set_bit(0, cpu.status.carry);
    operand.store(cpu, result);
    cpu.status.carry = value.msb();
    cpu.status.zero = result == T::zero();
    cpu.status.negative = result.msb();
}

pub fn ror<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let mut result = value >> 1;
    result.set_bit(T::N_BITS - 1, cpu.status.carry);
    operand.store(cpu, result);
    cpu.status.carry = value.lsb();
    cpu.status.zero = result == T::zero();
    cpu.status.negative = result.msb();
}

pub fn bra(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    cpu.bus.cycle_io();
    cpu.pc = operand.effective_addr().unwrap();
}

pub fn brl(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    cpu.bus.cycle_io();
    cpu.pc = operand.effective_addr().unwrap();
}

pub fn bcc(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if !cpu.status.carry {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

pub fn bcs(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if cpu.status.carry {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

pub fn beq(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if cpu.status.zero {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

pub fn bne(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if !cpu.status.zero {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

pub fn bpl(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if !cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

pub fn bmi(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if cpu.status.negative {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

pub fn bvc(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if !cpu.status.overflow {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

pub fn bvs(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if cpu.status.overflow {
        cpu.bus.cycle_io();
        cpu.pc = operand.effective_addr().unwrap();
    }
}

pub fn brk(cpu: &mut Cpu<impl MainBus>) {
    // read signature byte, even though it is unused.
    cpu.bus.cycle_read_u8(cpu.pc.add(1_u8, Wrap::WrapBank));
    cpu.stack_push_u24(u32::from(cpu.pc.add(2_u8, Wrap::WrapBank)));
    cpu.stack_push_u8(u8::from(cpu.status));
    cpu.status.irq_disable = true;
    cpu.status.decimal = false;
    let address = AddressU24::new(
        0,
        if cpu.emulation_mode {
            cpu.bus.cycle_read_u16(
                AddressU24::new(0, EmuVectorTable::Break as u16),
                Wrap::NoWrap,
            )
        } else {
            cpu.bus.cycle_read_u16(
                AddressU24::new(0, NativeVectorTable::Break as u16),
                Wrap::NoWrap,
            )
        },
    );
    cpu.pc = address.sub(1_u8, Wrap::NoWrap);
}

pub fn cop(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.stack_push_u24(u32::from(cpu.pc));
    cpu.stack_push_u8(u8::from(cpu.status));
    cpu.status.irq_disable = true;
    cpu.status.decimal = false;
    let address = if cpu.emulation_mode {
        cpu.bus
            .cycle_read_u16(AddressU24::new(0, EmuVectorTable::Cop as u16), Wrap::NoWrap)
    } else {
        cpu.bus.cycle_read_u16(
            AddressU24::new(0, NativeVectorTable::Cop as u16),
            Wrap::NoWrap,
        )
    };
    cpu.pc = (address as u32).into();
}

pub fn clc(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.status.carry = false;
}

pub fn cld(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.status.decimal = false;
}

pub fn cli(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.status.irq_disable = false;
}

pub fn xce(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    (cpu.status.carry, cpu.emulation_mode) = (cpu.emulation_mode, cpu.status.carry);
    if cpu.emulation_mode {
        cpu.status.accumulator_register_size = true;
        cpu.status.index_register_size_or_break = true;
        cpu.s = 0x0100 + (cpu.s & 0x00ff);
        cpu.update_register_sizes();
    }
}

pub fn pea(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    cpu.stack_push_u16(operand.effective_addr().unwrap().offset);
}

pub fn pei(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    cpu.stack_push_u16(operand.effective_addr().unwrap().offset);
}

pub fn per(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    cpu.bus.cycle_io();
    cpu.stack_push_u16(operand.effective_addr().unwrap().offset);
}

pub fn phk(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.stack_push_u8(cpu.pc.bank);
}

pub fn php(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.stack_push_u8(u8::from(cpu.status));
}

pub fn pha<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.stack_push(cpu.a.get::<T>());
}

pub fn phb(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.stack_push_u8(cpu.db);
}

pub fn phd(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.stack_push_u16(cpu.d);
}

pub fn phx<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.stack_push(cpu.x.get::<T>());
}

pub fn phy<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.stack_push(cpu.y.get::<T>());
}

pub fn plx<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    let value = cpu.stack_pop::<T>();
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn ply<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    let value = cpu.stack_pop::<T>();
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn plb(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.db = cpu.stack_pop_u8();
    cpu.update_negative_zero_flags(cpu.db);
}

pub fn pld(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.d = cpu.stack_pop_u16();
    cpu.update_negative_zero_flags(cpu.d);
}

pub fn plp(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.status = cpu.stack_pop_u8().into();
    cpu.update_register_sizes();
}

pub fn pla<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    let value = cpu.stack_pop::<T>();
    cpu.a.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn clv(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.status.overflow = false;
}

pub fn rep(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    cpu.bus.cycle_io();
    let data: u8 = operand.load(cpu);
    cpu.status = (u8::from(cpu.status) & !data).into();
    cpu.update_register_sizes();
}

pub fn sep(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    cpu.bus.cycle_io();
    let data: u8 = operand.load(cpu);
    cpu.status = (u8::from(cpu.status) | data).into();
    cpu.update_register_sizes();
}

pub fn ldx<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn ldy<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn lda<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let value: T = operand.load(cpu);
    cpu.a.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn lsr<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let data: T = operand.load(cpu);
    cpu.bus.cycle_io();
    let result = data >> 1;
    cpu.status.carry = data.lsb();
    cpu.update_negative_zero_flags(result);
    operand.store(cpu, result);
}

pub fn sta<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    operand.store(cpu, cpu.a.get::<T>());
}

pub fn stp(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.halt = true;
}

pub fn sty<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    operand.store(cpu, cpu.y.get::<T>());
}

pub fn stx<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    operand.store(cpu, cpu.x.get::<T>());
}

pub fn stz<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    operand.store(cpu, T::zero());
}

pub fn inc<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let value: T = operand.load::<T>(cpu).wrapping_add(&T::one());
    cpu.bus.cycle_io();
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

pub fn inx<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    let value: T = cpu.x.get::<T>().wrapping_add(&T::one());
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn iny<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    cpu.bus.cycle_io();
    let value: T = cpu.y.get::<T>().wrapping_add(&T::one());
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn dec<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let value: T = operand.load::<T>(cpu).wrapping_sub(&T::one());
    cpu.bus.cycle_io();
    cpu.update_negative_zero_flags(value);
    operand.store(cpu, value);
}

pub fn dex<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    let value: T = cpu.x.get::<T>().wrapping_sub(&T::one());
    cpu.bus.cycle_io();
    cpu.x.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn dey<T: UInt>(cpu: &mut Cpu<impl MainBus>, _: &Operand) {
    let value: T = cpu.y.get::<T>().wrapping_sub(&T::one());
    cpu.bus.cycle_io();
    cpu.y.set(value);
    cpu.update_negative_zero_flags(value);
}

pub fn ora<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let result = cpu.a.get::<T>() | operand_value;
    cpu.a.set(result);
    cpu.update_negative_zero_flags(result);
}

pub fn eor<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let result = cpu.a.get::<T>() ^ operand_value;
    cpu.a.set(result);
    cpu.update_negative_zero_flags(result);
}

pub fn tcd(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.d = cpu.a.get();
    cpu.update_negative_zero_flags(cpu.d);
}

pub fn and<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let result = cpu.a.get::<T>() & operand_value;
    cpu.a.set(result);
    cpu.update_negative_zero_flags(result);
}

pub fn bit<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let result: T = cpu.a.get::<T>() & operand_value;
    if let Operand::Address(_, _, _) = operand {
        cpu.status.negative = operand_value.msb();
        cpu.status.overflow = operand_value.bit(T::N_BITS - 2);
    }
    cpu.status.zero = result == T::zero();
}

pub fn cpx<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let (value, overflow) = cpu.x.get::<T>().overflowing_sub(&operand_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !overflow;
}

pub fn cpy<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let (value, overflow) = cpu.y.get::<T>().overflowing_sub(&operand_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !overflow;
}

pub fn cmp<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let operand_value: T = operand.load(cpu);
    let (value, overflow) = cpu.a.get::<T>().overflowing_sub(&operand_value);
    cpu.update_negative_zero_flags(value);
    cpu.status.carry = !overflow;
}

pub fn asl<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    let data: T = operand.load(cpu);
    cpu.bus.cycle_io();
    cpu.status.carry = data.msb();
    let result = data << 1;
    cpu.update_negative_zero_flags(result);
    operand.store(cpu, result);
}

pub fn adc<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
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

pub fn sbc<T: UInt>(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
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

pub fn wdm(cpu: &mut Cpu<impl MainBus>) {
    // William D. Mensch, Jr (WDM) opcode
    // Acts like a 2-byte NOP but without reading the second byte.
    cpu.bus.cycle_io();
    cpu.pc = cpu.pc.add(1_u8, Wrap::WrapBank);
}

pub fn wai(cpu: &mut Cpu<impl MainBus>) {
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
    cpu.bus.cycle_io();
}

pub fn mvn(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if let Operand::MoveAddressPair(source_bank, destination_bank) = *operand {
        cpu.db = destination_bank;

        let value = cpu
            .bus
            .cycle_read_u8(AddressU24::new(source_bank, cpu.x.value));
        cpu.bus
            .cycle_write_u8(AddressU24::new(destination_bank, cpu.y.value), value);
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();

        cpu.a.value = cpu.a.value.wrapping_sub(1);
        cpu.x.value = cpu.x.value.wrapping_add(1);
        cpu.y.value = cpu.y.value.wrapping_add(1);

        // Keep PC on this instruction until move is complete
        if cpu.a.value != 0xFFFF {
            cpu.pc = cpu.pc.sub(3_u8, Wrap::NoWrap);
        }
    }
}

pub fn mvp(cpu: &mut Cpu<impl MainBus>, operand: &Operand) {
    if let Operand::MoveAddressPair(source_bank, destination_bank) = *operand {
        cpu.db = destination_bank;

        let value = cpu
            .bus
            .cycle_read_u8(AddressU24::new(source_bank, cpu.x.value));
        cpu.bus
            .cycle_write_u8(AddressU24::new(destination_bank, cpu.y.value), value);
        cpu.bus.cycle_io();
        cpu.bus.cycle_io();

        cpu.a.value = cpu.a.value.wrapping_sub(1);
        cpu.x.value = cpu.x.value.wrapping_sub(1);
        cpu.y.value = cpu.y.value.wrapping_sub(1);

        // Keep PC on this instruction until move is complete
        if cpu.a.value != 0xFFFF {
            cpu.pc = cpu.pc.sub(3_u8, Wrap::NoWrap);
        }
    }
}
