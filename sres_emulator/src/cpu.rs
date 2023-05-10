pub mod instructions;
mod operands;
pub mod status;

use intbits::Bits;

use self::instructions::build_opcode_table;
use self::instructions::Instruction;
use self::instructions::InstructionMeta;
use self::status::StatusFlags;
use crate::bus::Bus;
use crate::memory::Address;
use crate::memory::ToAddress;
use crate::trace::Trace;
use crate::uint::RegisterSize;
use crate::uint::UInt;

#[derive(Default)]
pub struct VariableLengthRegister {
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

pub enum NativeVectorTable {
    Cop = 0xFFE4,
    Break = 0xFFE6,
    Nmi = 0xFFEA,
    Irq = 0xFFFE,
}

pub enum EmuVectorTable {
    Cop = 0xFFF4,
    Nmi = 0xFFFA,
    Reset = 0xFFFC,
    Irq = 0xFFFE,
}

pub struct Cpu<BusT: Bus> {
    pub bus: BusT,
    pub pc: Address,
    pub a: VariableLengthRegister,
    pub x: VariableLengthRegister,
    pub y: VariableLengthRegister,
    pub s: u16,
    pub d: u16,
    pub db: u8,
    pub status: StatusFlags,
    pub emulation_mode: bool,
    instruction_table: [Instruction<BusT>; 256],
}

const STACK_BASE: u32 = 0;

impl<BusT: Bus> Cpu<BusT> {
    pub fn new(bus: BusT) -> Self {
        Self {
            bus,
            a: Default::default(),
            x: Default::default(),
            y: Default::default(),
            s: 0x1FF,
            d: 0,
            db: 0,
            status: StatusFlags::default(),
            pc: Address::default(),
            emulation_mode: true,
            instruction_table: build_opcode_table(),
        }
    }

    /// Return the instruction meta data for the instruction at the given address
    fn load_instruction_meta(&mut self, addr: Address) -> (InstructionMeta, Address) {
        let opcode = self.bus.read_u8(addr);
        (self.instruction_table[opcode as usize].meta)(self, addr)
    }

    pub fn reset(&mut self) {
        self.pc = Address {
            bank: 0,
            offset: u16::from_le_bytes([
                self.bus.read_u8(EmuVectorTable::Reset as u32),
                self.bus.read_u8(EmuVectorTable::Reset as u32 + 1),
            ]),
        };
    }

    pub fn step(&mut self) {
        let opcode = self.bus.read_u8(self.pc);
        (self.instruction_table[opcode as usize].execute)(self)
    }

    fn stack_push_u8(&mut self, value: u8) {
        if self.s == 0 {
            return;
        }
        self.bus.write_u8(self.s as u32, value);
        self.s -= 1;
    }

    fn stack_push_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
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
        if self.s == 0xFF {
            return 0;
        }
        self.s += 1;
        self.bus.read_u8(self.s as u32)
    }

    fn stack_pop_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.stack_pop_u8(), self.stack_pop_u8()])
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

    pub fn trace(&mut self) -> Trace {
        let (instruction, _) = self.load_instruction_meta(self.pc.to_address());
        Trace {
            pc: self.pc,
            instruction: instruction.operation.to_string(),
            operand: instruction.operand_str.unwrap_or_default(),
            operand_addr: instruction.operand_addr,
            a: self.a.value,
            x: self.x.value,
            y: self.y.value,
            s: self.s,
            d: self.d,
            db: self.db,
            status: self.status,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::process::Command;
    use std::str::from_utf8;

    use tempfile::NamedTempFile;

    use crate::bus::SresBus;
    use crate::cpu::VariableLengthRegister;
    use crate::memory::Memory;

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

    #[test]
    pub fn test_assembler() {
        assert_eq!(assemble("lda $1234"), [0xAD, 0x34, 0x12]);
    }

    #[test]
    pub fn test_stack_u8() {
        let mut cpu = super::Cpu::new(SresBus::new());
        cpu.stack_push_u8(0x12);
        assert_eq!(cpu.bus.read_u8(cpu.s as u32 + 1), 0x12);
        assert_eq!(cpu.stack_pop_u8(), 0x12);
    }

    #[test]
    pub fn test_stack() {
        let mut cpu = super::Cpu::new(SresBus::new());
        cpu.stack_push_u16(0x1234);
        assert_eq!(cpu.bus.read_u8(cpu.s as u32 + 1), 0x34);
        assert_eq!(cpu.bus.read_u8(cpu.s as u32 + 2), 0x12);
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
