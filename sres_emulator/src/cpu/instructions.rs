use super::Cpu;
use crate::bus::Bus;

pub struct OpCodeTableEntry<BusT: Bus> {
    pub name: &'static str,
    pub execute: fn(&mut Cpu<BusT>),
}

impl<BusT: Bus> Default for OpCodeTableEntry<BusT> {
    fn default() -> Self {
        Self {
            name: "ill",
            execute: |_| {
                panic!("Unimplemented instruction");
            },
        }
    }
}

pub fn build_opcode_table<BusT: Bus>() -> [OpCodeTableEntry<BusT>; 256] {
    macro_rules! opcode {
        ($method: ident) => {
            OpCodeTableEntry {
                name: stringify!($method),
                execute: |cpu| $method(cpu),
            }
        };
    }

    let mut table = [(); 256].map(|_| Default::default());
    table[0x78] = opcode!(sei);
    table[0x18] = opcode!(clc);
    table[0xFB] = opcode!(xce);
    table
}

fn sei<BusT: Bus>(cpu: &mut Cpu<BusT>) {
    cpu.status.irq_disable = true;
}

fn clc<BusT: Bus>(cpu: &mut Cpu<BusT>) {
    cpu.status.carry = false;
}

fn xce<BusT: Bus>(cpu: &mut Cpu<BusT>) {
    (cpu.status.carry, cpu.emulation_mode) = (cpu.emulation_mode, cpu.status.carry);
}
