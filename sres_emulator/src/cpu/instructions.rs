use super::Cpu;
use crate::bus::Bus;

pub struct InstructionTableEntry<BusT: Bus> {
    pub name: &'static str,
    pub execute: fn(&mut Cpu<BusT>),
}

impl<BusT: Bus> Default for InstructionTableEntry<BusT> {
    fn default() -> Self {
        Self {
            name: "ill",
            execute: |_| {
                panic!("Unimplemented instruction");
            },
        }
    }
}

pub fn build_instruction_table<BusT: Bus>() -> [InstructionTableEntry<BusT>; 256] {
    let mut table = [(); 256].map(|_| InstructionTableEntry::default());
    table[0x78] = InstructionTableEntry {
        name: "sei",
        execute: |cpu| {
            cpu.status.irq_disable = true;
        },
    };
    table[0x18] = InstructionTableEntry {
        name: "clc",
        execute: |cpu| {
            cpu.status.carry = false;
        },
    };
    table
}
