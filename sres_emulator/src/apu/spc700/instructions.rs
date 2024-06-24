use intbits::Bits;

use super::operands::Operand;
use super::AddressMode;
use super::DecodedOperand;
use super::Spc700StatusFlags;
use crate::apu::spc700::Spc700;
use crate::apu::spc700::Spc700Bus;
use crate::common::address::Address;
use crate::common::address::AddressU16;
use crate::common::address::Wrap;
use crate::util::uint::U16Ext;
use crate::util::uint::U8Ext;
use crate::util::uint::UInt;

impl<BusT: Spc700Bus> Spc700<BusT> {
    ////////////////////////////////////////////////////////////////////////////////
    // Arithmetic/Logic instructions

    #[inline]
    fn alu_operation(
        &mut self,
        left_op: Operand,
        right_op: Operand,
        f: impl FnOnce(&mut Self, u8, u8) -> u8,
    ) {
        if right_op.is_address_mode(AddressMode::XIndirect)
            || right_op.is_address_mode(AddressMode::YIndirect)
        {
            self.bus.cycle_read_u8(self.pc);
        }
        let right = right_op.decode(self).load(self);
        let left_op = left_op.decode(self);
        let left = left_op.load(self);
        let result = f(self, left, right);
        self.update_negative_zero_flags(result);
        left_op.store(self, result);
    }

    pub fn adc(&mut self, left_op: Operand, right_op: Operand) {
        self.alu_operation(left_op, right_op, |cpu, left, right| {
            let (mut result, mut carry) = left.overflowing_add(right);
            if cpu.status.carry {
                let (value2, carry2) = result.overflowing_add(1);
                result = value2;
                carry = carry || carry2;
            }
            cpu.status.half_carry =
                ((right & 0x0F) + (left & 0x0F) + cpu.status.carry as u8) > 0x0F;
            cpu.status.carry = carry;
            cpu.status.overflow = ((right ^ result) & (left ^ result)).msb();
            result
        });
    }

    pub fn sbc(&mut self, left_op: Operand, right_op: Operand) {
        self.alu_operation(left_op, right_op, |cpu, left, right| {
            let (mut result, mut carry) = left.overflowing_sub(right);
            if !cpu.status.carry {
                let (value2, carry2) = result.overflowing_sub(1);
                result = value2;
                carry = carry || carry2;
            }
            cpu.status.half_carry =
                (left & 0x0F).wrapping_sub((right & 0x0F) + !cpu.status.carry as u8) & 0x10 != 0x10;
            cpu.status.carry = !carry;
            cpu.status.overflow = ((left ^ right) & (left ^ result)).msb();
            result
        });
    }

    pub fn and(&mut self, left_op: Operand, right_op: Operand) {
        self.alu_operation(left_op, right_op, |_, left, right| left & right);
    }

    pub fn or(&mut self, left_op: Operand, right_op: Operand) {
        self.alu_operation(left_op, right_op, |_, left, right| left | right);
    }

    pub fn eor(&mut self, left_op: Operand, right_op: Operand) {
        self.alu_operation(left_op, right_op, |_, left, right| left ^ right);
    }

    pub fn cmp(&mut self, left_op: Operand, right_op: Operand) {
        if right_op.is_address_mode(AddressMode::XIndirect)
            || right_op.is_address_mode(AddressMode::YIndirect)
        {
            self.bus.cycle_read_u8(self.pc);
        }
        let right = right_op.decode(self).load(self);
        let left = left_op.decode(self).load(self);
        let value = left.wrapping_sub(right);
        if left_op.is_in_memory() {
            self.bus.cycle_io();
        }
        self.update_negative_zero_flags(value);
        self.status.carry = left >= right;
    }

    pub fn inc(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let value = operand.load(self).wrapping_add(1);
        if operand.is_alu_register() {
            self.bus.cycle_read_u8(self.pc);
        }
        self.update_negative_zero_flags(value);
        operand.store(self, value);
    }

    pub fn dec(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let value = operand.load(self).wrapping_sub(1);
        if operand.is_alu_register() {
            self.bus.cycle_read_u8(self.pc);
        }
        self.update_negative_zero_flags(value);
        operand.store(self, value);
    }

    pub fn mul(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        for _ in 0..7 {
            self.bus.cycle_io();
        }
        let result = self.y as u16 * self.a as u16;
        self.a = result.low_byte();
        self.y = result.high_byte();
        self.update_negative_zero_flags(self.y);
    }

    pub fn div(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        for _ in 0..10 {
            self.bus.cycle_io();
        }

        let ya = u16::from_be_bytes([self.y, self.a]);
        let y = self.y as u16;
        let x = self.x as u16;

        self.status.half_carry = (y & 15) >= (x & 15);
        self.status.overflow = y >= x;

        // The logic for overflowing div's is weird, helpful explanation at:
        // https://helmet.kafuka.org/bboard/thread.php?id=228
        if y < (x << 1) {
            self.a = (ya / x) as u8;
            self.y = (ya % x) as u8;
        } else {
            self.a = 255_u16.wrapping_sub(ya.wrapping_sub(x << 9) / (256 - x)) as u8;
            self.y = (x + ((ya.wrapping_sub(x << 9)) % (256 - x))) as u8;
        }
        self.update_negative_zero_flags(self.a);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Shift instructions

    #[inline]
    fn shift_operation(&mut self, operand: Operand, f: impl FnOnce(u8, bool) -> (u8, bool)) {
        let operand = operand.decode(self);
        let value = operand.load(self);
        if operand.is_alu_register() {
            self.bus.cycle_read_u8(self.pc);
        }
        let (result, new_carry) = f(value, self.status.carry);
        self.status.carry = new_carry;
        self.update_negative_zero_flags(result);
        operand.store(self, result);
    }

    pub fn rol(&mut self, operand: Operand) {
        self.shift_operation(operand, |value, carry| {
            ((value << 1).with_bit(0, carry), value.bit(7))
        });
    }

    pub fn ror(&mut self, operand: Operand) {
        self.shift_operation(operand, |value, carry| {
            ((value >> 1).with_bit(7, carry), value.bit(0))
        });
    }

    pub fn asl(&mut self, operand: Operand) {
        self.shift_operation(operand, |value, _| (value << 1, value.bit(7)));
    }

    pub fn lsr(&mut self, operand: Operand) {
        self.shift_operation(operand, |value, _| (value >> 1, value.bit(0)));
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Status bit operations

    pub fn clrp(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.status.direct_page = false;
    }

    pub fn setp(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.status.direct_page = true;
    }

    pub fn setc(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.status.carry = true;
    }

    pub fn clrc(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.status.carry = false;
    }

    pub fn clrv(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.status.overflow = false;
    }

    pub fn ei(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        self.status.irq_enable = true;
    }

    pub fn di(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        self.status.irq_enable = false;
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Stack instructions

    pub fn push(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        self.bus.cycle_read_u8(self.pc);
        let value = operand.load(self);
        self.stack_push_u8(value);
        self.bus.cycle_io();
    }

    pub fn pop(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        let value = self.stack_pop_u8();
        operand.store(self, value);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Call / Jump / Break instructions

    pub fn jmp(&mut self, operand: Operand) {
        self.pc = AddressU16(operand.decode(self).load_u16(self));
    }

    pub fn brk(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.stack_push_u16(self.pc.0);
        self.stack_push_u8(self.status.into());
        self.status.irq_enable = false;
        self.status.break_command = true;
        self.bus.cycle_io();
        self.pc = AddressU16(self.bus.cycle_read_u16(AddressU16(0xffde), Wrap::NoWrap));
    }

    pub fn call(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        self.bus.cycle_io();
        self.stack_push_u16(self.pc.0);
        self.bus.cycle_io();
        self.bus.cycle_io();
        self.pc = AddressU16(operand.load_u16(self));
    }

    pub fn tcall(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        self.stack_push_u16(self.pc.0);
        self.bus.cycle_io();
        let addr = AddressU16(0xFFDE).sub(operand.load(self) * 2, Wrap::NoWrap);
        self.pc = AddressU16(self.bus.cycle_read_u16(addr, Wrap::NoWrap));
    }

    pub fn pcall(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        self.bus.cycle_io();
        self.stack_push_u16(self.pc.0);
        self.bus.cycle_io();
        self.pc = AddressU16(0xFF00).add(operand.load(self) as u16, Wrap::WrapPage);
    }

    pub fn ret(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        let pc = self.stack_pop_u16();
        self.pc = AddressU16(pc);
    }

    pub fn reti(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        self.status = Spc700StatusFlags::from(self.stack_pop_u8());
        let pc = self.stack_pop_u16();
        self.pc = AddressU16(pc);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Branch instructions

    #[inline]
    fn branch(&mut self, offset_op: Operand, condition: bool) {
        let addr = AddressU16(offset_op.decode(self).load_u16(self));
        if condition {
            self.bus.cycle_io();
            self.bus.cycle_io();
            self.pc = addr.sub(1_u16, Wrap::NoWrap);
        }
    }

    pub fn bra(&mut self, operand: Operand) {
        self.branch(operand, true);
    }

    pub fn beq(&mut self, operand: Operand) {
        self.branch(operand, self.status.zero);
    }

    pub fn bne(&mut self, operand: Operand) {
        self.branch(operand, !self.status.zero);
    }

    pub fn bpl(&mut self, operand: Operand) {
        self.branch(operand, !self.status.negative);
    }

    pub fn bmi(&mut self, operand: Operand) {
        self.branch(operand, self.status.negative);
    }

    pub fn bvc(&mut self, operand: Operand) {
        self.branch(operand, !self.status.overflow);
    }

    pub fn bvs(&mut self, operand: Operand) {
        self.branch(operand, self.status.overflow);
    }

    pub fn bcs(&mut self, operand: Operand) {
        self.branch(operand, self.status.carry);
    }

    pub fn bcc(&mut self, operand: Operand) {
        self.branch(operand, !self.status.carry);
    }

    pub fn bbs(&mut self, left_op: Operand, right_op: Operand) {
        let right_op = right_op.decode(self);
        let value = right_op.load(self).bit(right_op.bit());
        self.bus.cycle_io();
        self.branch(left_op, value);
    }

    pub fn bbc(&mut self, left_op: Operand, right_op: Operand) {
        let right_op = right_op.decode(self);
        let value = right_op.load(self).bit(right_op.bit());
        self.bus.cycle_io();
        self.branch(left_op, !value);
    }

    pub fn cbne(&mut self, left_op: Operand, right_op: Operand) {
        let value = right_op.decode(self).load(self);
        self.bus.cycle_io();
        self.branch(left_op, value != self.a);
    }

    pub fn dbnz(&mut self, left_op: Operand, right_op: Operand) {
        let left_op = left_op.decode(self);
        if left_op.is_alu_register() {
            self.bus.cycle_read_u8(self.pc);
            self.bus.cycle_io();
        }
        let value = left_op.load(self).wrapping_sub(1);
        left_op.store(self, value);
        self.branch(right_op, value != 0);
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Single-bit instructions

    pub fn set1(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let value = operand.load(self).with_bit(operand.bit(), true);
        operand.store(self, value);
    }

    pub fn clr1(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let value = operand.load(self).with_bit(operand.bit(), false);
        operand.store(self, value);
    }

    pub fn tset1(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let value = operand.load(self);
        operand.load(self); // CPU will re-read the value for another cycle
        operand.store(self, value | self.a);
        self.update_negative_zero_flags(self.a.wrapping_sub(value));
    }

    pub fn tclr1(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let value = operand.load(self);
        operand.load(self); // CPU will re-read the value for another cycle
        operand.store(self, value & !self.a);
        self.update_negative_zero_flags(self.a.wrapping_sub(value));
    }

    pub fn not1(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        if matches!(operand, DecodedOperand::Carry) {
            self.bus.cycle_read_u8(self.pc);
            self.bus.cycle_io();
        }
        let value = operand.load(self);
        let result = value.with_bit(operand.bit(), !value.bit(operand.bit()));
        operand.store(self, result);
    }

    pub fn or1(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let bit = operand.load(self).bit(operand.bit());
        self.status.carry = self.status.carry || bit;
        self.bus.cycle_io()
    }

    pub fn and1(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let bit = operand.load(self).bit(operand.bit());
        self.status.carry = self.status.carry && bit;
    }

    pub fn eor1(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let bit = operand.load(self).bit(operand.bit());
        self.bus.cycle_io();
        self.status.carry ^= bit;
    }

    pub fn mov1(&mut self, left_op: Operand, right_op: Operand) {
        let right_op = right_op.decode(self);
        let bit = right_op.load(self).bit(right_op.bit());
        let left_op = left_op.decode(self);
        let value = left_op.load(self);
        let result = value.with_bit(left_op.bit(), bit);
        left_op.store(self, result);
        if matches!(right_op, DecodedOperand::Carry) {
            self.bus.cycle_io();
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // 16-bit (wide) instructions

    pub fn addw(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let low = operand.load(self);
        self.bus.cycle_io();
        let high = operand.load_high(self);

        let right = u16::from_le_bytes([low, high]);
        let left = u16::from_le_bytes([self.a, self.y]);
        let (result, carry) = left.overflowing_add(right);
        self.update_negative_zero_flags(result);
        self.status.carry = carry;
        self.status.overflow = ((right ^ result) & (left ^ result)).msb();
        self.status.half_carry = ((right & 0x0FFF) + (left & 0x0FFF)) > 0x0FFF;
        self.a = result.low_byte();
        self.y = result.high_byte();
    }

    pub fn subw(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        let low = operand.load(self);
        self.bus.cycle_io();
        let high = operand.load_high(self);

        let right = u16::from_le_bytes([low, high]);
        let left = u16::from_le_bytes([self.a, self.y]);
        let (result, carry) = left.overflowing_sub(right);
        self.update_negative_zero_flags(result);
        self.status.carry = !carry;
        self.status.half_carry =
            (left & 0x0FFF).wrapping_sub((right & 0x0FFF) + !self.status.carry as u16) & 0x1000
                != 0x1000;
        self.status.overflow = ((left ^ right) & (left ^ result)).msb();
        self.a = result.low_byte();
        self.y = result.high_byte();
    }

    pub fn cmpw(&mut self, left_op: Operand, right_op: Operand) {
        let right = right_op.decode(self).load_u16(self);
        let left = left_op.decode(self).load_u16(self);
        let result = left.wrapping_sub(right);
        self.update_negative_zero_flags(result);
        self.status.carry = left >= right;
    }

    pub fn incw(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        // decrement low and high byte separately to match hardware read/write cycle order
        let (low, overflow) = operand.load(self).overflowing_add(1);
        operand.store(self, low);
        let high = operand.load_high(self).wrapping_add(overflow as u8);
        operand.store_high(self, high);
        self.update_negative_zero_flags(u16::from_le_bytes([low, high]));
    }

    pub fn decw(&mut self, operand: Operand) {
        let operand = operand.decode(self);
        // decrement low and high byte separately to match hardware read/write cycle order
        let (low, overflow) = operand.load(self).overflowing_sub(1);
        operand.store(self, low);
        let high = operand.load_high(self).wrapping_sub(overflow as u8);
        operand.store_high(self, high);
        self.update_negative_zero_flags(u16::from_le_bytes([low, high]));
    }

    pub fn movw(&mut self, left_op: Operand, right_op: Operand) {
        let right_op = right_op.decode(self);
        let left_op = left_op.decode(self);

        if left_op.is_alu_register() {
            // MOVW YA, dp
            // Loading the dp has an extra idle cycle between bytes.
            // Smell: Leaky abstraction
            let low = right_op.load(self);
            self.bus.cycle_io();
            let high = right_op.load_high(self);
            let right = u16::from_le_bytes([low, high]);
            self.update_negative_zero_flags(right);
            left_op.store_u16(self, right);
        } else {
            // MOVW dp, YA
            // Storing into the dp address will do an extra read from the target
            // address.
            // Smell: Leaky abstraction
            let right = right_op.load_u16(self);
            left_op.load(self);
            left_op.store_u16(self, right);
        };
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Misc instructions

    pub fn mov(&mut self, left_op: Operand, right_op: Operand) {
        let right = if right_op.is_address_mode(AddressMode::XIndirect) {
            // TODO: this stinks!
            let right_op = right_op.decode(self);
            self.bus.cycle_read_u8(self.pc);
            right_op.load(self)
        } else {
            right_op.decode(self).load(self)
        };
        let left_op = left_op.decode(self);

        if left_op.is_alu_register() {
            // Moves into registers will update the N and Z flags
            self.update_negative_zero_flags(right);
            if right_op.is_alu_register() {
                self.bus.cycle_read_u8(self.pc);
            }
        } else {
            // Moves into memory locations will do an extra read from the target address.
            if right_op.is_address_mode(AddressMode::Dp) {
                // Somehow 0xFA (MOV from a direct page address into another) is an exception.
            } else if left_op.is_address_mode(AddressMode::DpIndirectYIdx) {
                left_op.load(self);
            } else if left_op.is_address_mode(AddressMode::XIndirect) {
                self.bus.cycle_read_u8(self.pc);
                left_op.load(self);
            } else if left_op.is_address_mode(AddressMode::XIndirectAutoInc) {
                // TODO: this stinks!
                self.bus.cycle_read_u8(self.pc);
                self.bus.cycle_io();
            } else {
                left_op.load(self);
            }
        }
        left_op.store(self, right);
    }

    pub fn nop(&mut self) {
        self.bus.cycle_read_u8(self.pc);
    }

    pub fn xcn(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        self.bus.cycle_io();
        self.bus.cycle_io();
        let a = self.a;
        let a = (a >> 4) | (a << 4);
        self.update_negative_zero_flags(a);
        self.a = a;
    }

    pub fn daa(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        if self.status.carry || self.a > 0x99 {
            self.a = self.a.wrapping_add(0x60);
            self.status.carry = true;
        }
        if self.status.half_carry || self.a.low_nibble() > 0x09 {
            self.a = self.a.wrapping_add(0x06);
        }
        self.update_negative_zero_flags(self.a);
    }

    pub fn das(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        if !self.status.carry || self.a > 0x99 {
            self.a = self.a.wrapping_sub(0x60);
            self.status.carry = false;
        }
        if !self.status.half_carry || self.a.low_nibble() > 0x09 {
            self.a = self.a.wrapping_sub(0x06);
        }
        self.update_negative_zero_flags(self.a);
    }

    pub fn sleep(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
    }

    pub fn stop(&mut self) {
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
        self.bus.cycle_read_u8(self.pc);
        self.bus.cycle_io();
    }
}
