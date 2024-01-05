use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;

use itertools::Itertools;

use super::Address;

/// Implements a sparse memory HashMap with a readable display format.
#[derive(Default, PartialEq)]
pub struct SparseMemory<AddressT: Address> {
    pub memory: HashMap<AddressT, u8>,
}

impl<AddressT: Address> SparseMemory<AddressT> {
    pub fn get(&self, addr: AddressT) -> Option<u8> {
        self.memory.get(&addr).copied()
    }

    pub fn set(&mut self, addr: AddressT, value: u8) {
        self.memory.insert(addr, value);
    }
}

impl<AddressT: Address> Display for SparseMemory<AddressT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (addr, value) in self.memory.iter().sorted() {
            writeln!(f, "{}: {:02X}", addr, value)?;
        }
        Ok(())
    }
}
