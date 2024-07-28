//! Implementation of the 65816 main cpu of the SNES.
mod instructions;
mod opcode_table;
mod operands;
mod status;
#[cfg(test)]
mod test;

use std::sync::atomic::Ordering;

use intbits::Bits;

use self::opcode_table::build_opcode_table;
use self::opcode_table::Instruction;
use self::status::StatusFlags;
use crate::common::address::AddressU24;
use crate::common::address::Wrap;
use crate::common::bus::Bus;
use crate::common::debug_events::CpuEvent;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::common::debug_events::DEBUG_EVENTS_ENABLED;
use crate::common::system::ClockInfo;
use crate::common::system::CpuState;
use crate::common::system::CpuStatusFlags;
use crate::common::system::InstructionMeta;
use crate::common::system::NativeVectorTable;
use crate::common::uint::RegisterSize;
use crate::common::uint::UInt;

pub struct Cpu<BusT: MainBus> {
    pub bus: BusT,
    pc: AddressU24,
    a: VariableLengthRegister,
    x: VariableLengthRegister,
    y: VariableLengthRegister,
    s: u16,
    d: u16,
    db: u8,
    status: StatusFlags,
    emulation_mode: bool,
    halt: bool,
    instruction_table: [Instruction<BusT>; 256],
    debug_event_collector: DebugEventCollectorRef<CpuEvent>,
}

const STACK_BASE: u16 = 0;

impl<BusT: MainBus> Cpu<BusT> {
    pub fn new(bus: BusT, debug_event_collector: DebugEventCollectorRef<CpuEvent>) -> Self {
        let mut cpu = Self {
            bus,
            a: Default::default(),
            x: Default::default(),
            y: Default::default(),
            s: 0x1FF,
            d: 0,
            db: 0,
            status: StatusFlags::default(),
            pc: AddressU24::default(),
            emulation_mode: true,
            halt: false,
            instruction_table: build_opcode_table(),
            debug_event_collector,
        };
        cpu.reset();
        cpu
    }

    pub fn halted(&self) -> bool {
        self.halt
    }

    pub fn reset(&mut self) {
        self.bus.reset();
        self.pc = AddressU24 {
            bank: 0,
            offset: self
                .bus
                .peek_u16(
                    AddressU24::new(0, EmuVectorTable::Reset as u16),
                    Wrap::NoWrap,
                )
                .unwrap_or_default(),
        };
    }

    pub fn step(&mut self) {
        if DEBUG_EVENTS_ENABLED.load(Ordering::Relaxed) {
            self.debug_event_collector
                .collect_event(CpuEvent::Step(self.debug().state()));
        }
        let opcode = self.bus.cycle_read_u8(self.pc);
        (self.instruction_table[opcode as usize].execute)(self);

        if self.bus.consume_nmi_interrupt() {
            self.interrupt(NativeVectorTable::Nmi);
        }
        if !self.status.irq_disable && self.bus.consume_timer_interrupt() {
            self.interrupt(NativeVectorTable::Irq);
        }
    }

    pub fn debug(&self) -> CpuDebug<'_, BusT> {
        CpuDebug(self)
    }

    fn update_register_sizes(&mut self) {
        if self.status.index_register_size_or_break {
            self.x.value = self.x.get::<u8>() as u16;
            self.y.value = self.y.get::<u8>() as u16;
        }
    }

    fn interrupt(&mut self, handler: NativeVectorTable) {
        self.debug_event_collector
            .collect_event(CpuEvent::Interrupt(handler));
        self.stack_push_u24(u32::from(self.pc));
        self.stack_push_u8(u8::from(self.status));
        self.status.irq_disable = true;
        self.status.decimal = false;
        let address = self
            .bus
            .cycle_read_u16(AddressU24::new(0, handler as u16), Wrap::NoWrap);
        self.pc = AddressU24::new(0, address);
    }

    fn stack_push_u8(&mut self, value: u8) {
        self.bus.cycle_write_u8(AddressU24::new(0, self.s), value);
        self.s = self.s.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.stack_push_u8(bytes[1]);
        self.stack_push_u8(bytes[0]);
    }

    fn stack_push_u24(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.stack_push_u8(bytes[2]);
        self.stack_push_u8(bytes[1]);
        self.stack_push_u8(bytes[0]);
    }

    fn stack_push<T: UInt>(&mut self, value: T) {
        match T::SIZE {
            RegisterSize::U8 => {
                self.stack_push_u8(value.to_u8());
            }
            RegisterSize::U16 => {
                self.stack_push_u16(value.to_u16());
            }
        }
    }

    fn stack_pop_u8(&mut self) -> u8 {
        self.s = self.s.wrapping_add(1);
        self.bus.cycle_read_u8(AddressU24::new(0, self.s))
    }

    fn stack_pop_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.stack_pop_u8(), self.stack_pop_u8()])
    }

    fn stack_pop_u24(&mut self) -> u32 {
        u32::from_le_bytes([
            self.stack_pop_u8(),
            self.stack_pop_u8(),
            self.stack_pop_u8(),
            0,
        ])
    }

    fn stack_pop<T: UInt>(&mut self) -> T {
        match T::SIZE {
            RegisterSize::U8 => T::from_u8(self.stack_pop_u8()),
            RegisterSize::U16 => T::from_u16(self.stack_pop_u16()),
        }
    }

    fn update_negative_zero_flags<T: UInt>(&mut self, value: T) {
        self.status.negative = value.bit(T::N_BITS - 1);
        self.status.zero = value.is_zero();
    }
}

pub trait MainBus: Bus<AddressU24> {
    fn consume_nmi_interrupt(&mut self) -> bool;
    fn consume_timer_interrupt(&mut self) -> bool;
    fn clock_info(&self) -> ClockInfo;
}

#[derive(Default)]
struct VariableLengthRegister {
    value: u16,
}

impl VariableLengthRegister {
    fn set<T: UInt>(&mut self, value: T) {
        match T::SIZE {
            RegisterSize::U8 => {
                self.value.set_bits(0..8, value.to_u8() as u16);
            }
            RegisterSize::U16 => {
                self.value = value.to_u16();
            }
        }
    }

    fn get<T: UInt>(&self) -> T {
        T::from_u16(self.value)
    }
}

#[allow(dead_code)]
enum EmuVectorTable {
    Cop = 0xFFF4,
    Break = 0xFFF6,
    Nmi = 0xFFFA,
    Reset = 0xFFFC,
    Irq = 0xFFFE,
}

pub struct CpuDebug<'a, BusT: MainBus>(&'a Cpu<BusT>);

impl<'a, BusT: MainBus> CpuDebug<'a, BusT> {
    pub fn state(&self) -> CpuState {
        let (instruction, _) = self.load_instruction_meta(self.0.pc);
        CpuState {
            instruction,
            a: self.0.a.value,
            x: self.0.x.value,
            y: self.0.y.value,
            s: self.0.s,
            d: self.0.d,
            db: self.0.db,
            status: CpuStatusFlags {
                negative: self.0.status.negative,
                overflow: self.0.status.overflow,
                accumulator_register_size: self.0.status.accumulator_register_size,
                index_register_size_or_break: self.0.status.index_register_size_or_break,
                decimal: self.0.status.decimal,
                irq_disable: self.0.status.irq_disable,
                zero: self.0.status.zero,
                carry: self.0.status.carry,
            },
            clock: self.0.bus.clock_info(),
        }
    }

    /// Return the instruction meta data for the instruction at the given address
    pub fn load_instruction_meta(
        &self,
        addr: AddressU24,
    ) -> (InstructionMeta<AddressU24>, AddressU24) {
        let opcode = self.0.bus.peek_u8(addr).unwrap_or_default();
        (self.0.instruction_table[opcode as usize].meta)(self.0, addr)
    }

    pub fn peek_next_operations(
        &self,
        count: usize,
    ) -> impl Iterator<Item = InstructionMeta<AddressU24>> + '_ {
        let mut pc = self.0.pc;
        (0..count).map(move |_| {
            let (meta, new_pc) = self.load_instruction_meta(pc);
            pc = new_pc;
            meta
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn variable_length_register() {
        let mut reg = VariableLengthRegister { value: 0 };
        reg.set(0x1234_u16);
        assert_eq!(reg.get::<u8>(), 0x34);
        assert_eq!(reg.get::<u16>(), 0x1234);
        // Writing the register in u8 mode, will only overwrite the low byte
        reg.set(0xFF_u8);
        assert_eq!(reg.get::<u8>(), 0xFF);
        assert_eq!(reg.get::<u16>(), 0x12FF);
    }
}
