pub mod instructions;
pub mod status;

use self::instructions::build_opcode_table;
use self::instructions::OpCodeTableEntry;
use self::status::StatusFlags;
use crate::bus::Bus;
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
    pub pc: u32,
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub s: u16,
    pub d: u16,
    pub db: u8,
    pub status: StatusFlags,
    pub emulation_mode: bool,
    instruction_table: [OpCodeTableEntry<BusT>; 256],
}

impl<BusT: Bus> Cpu<BusT> {
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
            pc: 0,
            emulation_mode: true,
            instruction_table: build_opcode_table(),
        }
    }

    pub fn reset(&mut self) {
        self.pc = u16::from_le_bytes([
            self.bus.read(EmuVectorTable::Reset as u32),
            self.bus.read(EmuVectorTable::Reset as u32 + 1),
        ]) as u32;
    }

    pub fn step(&mut self) {
        let opcode = self.bus.read(self.pc);
        self.pc += 1;
        let instruction = &self.instruction_table[opcode as usize];
        (instruction.execute)(self);
    }

    pub fn current_opcode(&mut self) -> &'static str {
        let opcode = self.bus.read(self.pc);
        self.instruction_table[opcode as usize].name
    }

    pub fn trace(&mut self) -> Trace {
        Trace {
            pc: self.pc,
            opcode: self.current_opcode().to_string(),
            operand: "".to_string(),
            operand_addr: None,
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
