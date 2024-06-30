//! Implementation of the 65816 main cpu of the SNES.
mod instructions;
mod opcode_table;
mod operands;
mod status;

use intbits::Bits;

use self::opcode_table::build_opcode_table;
use self::opcode_table::Instruction;
pub use self::opcode_table::InstructionMeta;
pub use self::status::StatusFlags;
use crate::common::address::AddressU24;
use crate::common::address::Wrap;
use crate::common::uint::RegisterSize;
use crate::common::uint::UInt;
use crate::debugger::DebuggerRef;
use crate::debugger::Event;
use crate::main_bus::MainBus;
use crate::trace::CpuTraceLine;

#[derive(Default)]
pub struct VariableLengthRegister {
    pub value: u16,
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

#[derive(Clone, Copy, PartialEq, Debug, strum::Display, strum::EnumString)]
pub enum NativeVectorTable {
    #[strum(serialize = "cop")]
    Cop = 0xFFE4,
    #[strum(serialize = "break", serialize = "brk")]
    Break = 0xFFE6,
    #[strum(serialize = "nmi")]
    Nmi = 0xFFEA,
    #[strum(serialize = "irq")]
    Irq = 0xFFEE,
}

pub enum EmuVectorTable {
    Cop = 0xFFF4,
    Break = 0xFFF6,
    Nmi = 0xFFFA,
    Reset = 0xFFFC,
    Irq = 0xFFFE,
}

pub struct Cpu<BusT: MainBus> {
    pub bus: BusT,
    pub pc: AddressU24,
    pub a: VariableLengthRegister,
    pub x: VariableLengthRegister,
    pub y: VariableLengthRegister,
    pub s: u16,
    pub d: u16,
    pub db: u8,
    pub status: StatusFlags,
    pub emulation_mode: bool,
    pub master_cycle: u64,
    pub halt: bool,
    instruction_table: [Instruction<BusT>; 256],
    pub debugger: DebuggerRef,
}

const STACK_BASE: u16 = 0;

impl<BusT: MainBus> Cpu<BusT> {
    pub fn new(bus: BusT, debugger: DebuggerRef) -> Self {
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
            master_cycle: 0,
            halt: false,
            instruction_table: build_opcode_table(),
            debugger,
        };
        cpu.reset();
        cpu
    }

    /// Return the instruction meta data for the instruction at the given address
    pub fn load_instruction_meta(&self, addr: AddressU24) -> (InstructionMeta, AddressU24) {
        let opcode = self.bus.peek_u8(addr).unwrap_or_default();
        (self.instruction_table[opcode as usize].meta)(self, addr)
    }

    pub fn peek_next_operations(&self, count: usize) -> impl Iterator<Item = InstructionMeta> + '_ {
        let mut pc = self.pc;
        (0..count).map(move |_| {
            let (meta, new_pc) = self.load_instruction_meta(pc);
            pc = new_pc;
            meta
        })
    }

    fn update_register_sizes(&mut self) {
        if self.status.index_register_size_or_break {
            self.x.value = self.x.get::<u8>() as u16;
            self.y.value = self.y.get::<u8>() as u16;
        }
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
        if self.debugger.enabled {
            self.debugger
                .on_event(Event::CpuStep(CpuTraceLine::from_cpu(self)));
        }
        let opcode = self.bus.cycle_read_u8(self.pc);
        (self.instruction_table[opcode as usize].execute)(self);

        if self.bus.check_nmi_interrupt() {
            self.interrupt(NativeVectorTable::Nmi);
        }
        if !self.status.irq_disable && self.bus.consume_timer_interrupt() {
            self.interrupt(NativeVectorTable::Irq);
        }
    }

    fn interrupt(&mut self, handler: NativeVectorTable) {
        self.debugger.on_event(Event::CpuInterrupt(handler));
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

#[cfg(test)]
mod tests {

    use std::io::Write;
    use std::process::Command;
    use std::str::from_utf8;

    use tempfile::NamedTempFile;

    use super::*;
    use crate::cartridge::Cartridge;
    use crate::common::address::Address;
    use crate::common::address::AddressU24;
    use crate::common::address::Wrap;
    use crate::common::bus::Bus;
    use crate::debugger::DebuggerRef;
    use crate::main_bus::MainBusImpl;
    use crate::System;

    fn assemble(code: &str) -> Vec<u8> {
        let mut code_file = NamedTempFile::new().unwrap();
        writeln!(code_file, "{}", code).unwrap();

        let assembled = Command::new("xa")
            .args(["-w", "-o", "-"])
            .arg(code_file.path())
            .output()
            .unwrap();
        if !assembled.status.success() {
            println!("{}", from_utf8(&assembled.stderr).unwrap());
            panic!("Failed to assemble code");
        }
        assert!(assembled.status.success());
        assembled.stdout
    }

    fn cpu_with_program(code: &str) -> Cpu<MainBusImpl> {
        let assembled = assemble(code);
        let debugger = DebuggerRef::new();
        // TODO: Use a test bus instead of SresBus/System
        let mut cpu = Cpu::new(
            MainBusImpl::new(&Cartridge::with_program(&assembled), debugger.clone()),
            debugger,
        );
        cpu.pc = AddressU24::new(0, 0x8000);
        cpu
    }

    #[test]
    #[ignore = "requires xa installed"]
    pub fn test_simple_program() {
        const PROGRAM: &str = "
            lda #$12
            adc #$34
            sta $1000
        ";
        let mut cpu = cpu_with_program(PROGRAM);
        cpu.step();
        assert_eq!(cpu.a.value, 0x12);
        cpu.step();
        assert_eq!(cpu.a.value, 0x46);
        cpu.step();
        assert_eq!(cpu.bus.cycle_read_u8(0x1000.into()), 0x46);
    }

    #[test]
    pub fn test_stack_u8() {
        // TODO: Use a test bus instead of SresBus/System
        let mut cpu = System::new().cpu;
        cpu.stack_push_u8(0x12);
        assert_eq!(
            cpu.bus
                .cycle_read_u8(AddressU24::new(0, cpu.s).add(1_u8, Wrap::WrapBank)),
            0x12
        );
        assert_eq!(cpu.stack_pop_u8(), 0x12);
    }

    #[test]
    pub fn test_stack() {
        // TODO: Use a test bus instead of SresBus/System
        let mut cpu = System::new().cpu;
        cpu.stack_push_u16(0x1234);
        assert_eq!(
            cpu.bus
                .cycle_read_u8(AddressU24::new(0, cpu.s).add(1_u8, Wrap::WrapBank)),
            0x34
        );
        assert_eq!(
            cpu.bus
                .cycle_read_u8(AddressU24::new(0, cpu.s).add(2_u8, Wrap::WrapBank)),
            0x12
        );
        assert_eq!(cpu.stack_pop_u16(), 0x1234);
    }

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
