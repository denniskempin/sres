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
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub s: u16,
    pub d: u16,
    pub db: u8,
    pub status: StatusFlags,
    pub emulation_mode: bool,
    instruction_table: [Instruction<BusT>; 256],
}

impl<BusT: Bus> Cpu<BusT> {
    const STACK_BASE: u32 = 0x100;

    pub fn new(bus: BusT) -> Self {
        Self {
            bus,
            a: 0,
            x: 0,
            y: 0,
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
        let opcode = self.bus.read(addr);
        (self.instruction_table[opcode as usize].meta)(self, addr)
    }

    pub fn reset(&mut self) {
        self.pc = Address {
            bank: 0,
            offset: u16::from_le_bytes([
                self.bus.read(EmuVectorTable::Reset as u32),
                self.bus.read(EmuVectorTable::Reset as u32 + 1),
            ]),
        };
    }

    pub fn step(&mut self) {
        let opcode = self.bus.read(self.pc);
        (self.instruction_table[opcode as usize].execute)(self)
    }

    fn stack_push(&mut self, value: u8) {
        self.bus.write(Self::STACK_BASE + self.s as u32, value);
        if self.s == 0 {
            return;
        }
        self.s -= 1;
    }

    fn stack_push_u16(&mut self, value: u16) {
        let [low, high] = value.to_le_bytes();
        self.stack_push(high);
        self.stack_push(low);
    }

    fn stack_pop(&mut self) -> u8 {
        if self.s == 0xFF {
            return 0;
        }
        self.s += 1;
        self.bus.read(Self::STACK_BASE + self.s as u32)
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let low = self.stack_pop();
        let high = self.stack_pop();
        u16::from_le_bytes([low, high])
    }

    fn update_negative_zero_flags(&mut self, value: u8) {
        self.status.negative = value.bit(7);
        self.status.zero = value == 0;
    }

    fn update_negative_zero_flags_u16(&mut self, value: u16) {
        self.status.negative = value.bit(15);
        self.status.zero = value == 0;
    }

    pub fn trace(&mut self) -> Trace {
        let (instruction, _) = self.load_instruction_meta(self.pc.to_address());
        Trace {
            pc: self.pc,
            instruction: instruction.operation.to_string(),
            operand: instruction.operand_str.unwrap_or_default(),
            operand_addr: instruction.operand_addr,
            a: self.a,
            x: self.x,
            y: self.y,
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
}
