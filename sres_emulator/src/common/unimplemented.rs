//! Named SNES hardware gaps reported to the debugger.
//! Call `DebugEventCollectorRef::on_unimplemented` from execute/read/write, never from `peek_*`.

use std::fmt::Display;
use std::fmt::Formatter;

use crate::common::address::AddressU24;

/// A distinct unimplemented hardware behavior.
///
/// Catch-all variants carry the address or register so unknown MMIO is still countable.
/// Named unit variants are preferred at known sites.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnimplementedBehavior {
    RegisterRead(AddressU24),
    RegisterWrite(AddressU24),
    UnmappedRead(AddressU24),
    UnmappedWrite(AddressU24),
    SerialJoypadRead,
    DmaUnusedRegister(AddressU24),
    PpuUnhandledRead(u16),
    PpuUnhandledWrite(u16),
    PpuStat77Read,
    PpuStat78Read,
    DspUnhandledRegister(u8),
}

impl Display for UnimplementedBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegisterRead(addr) => write!(f, "unimplemented register read {addr}"),
            Self::RegisterWrite(addr) => write!(f, "unimplemented register write {addr}"),
            Self::UnmappedRead(addr) => write!(f, "unmapped memory read {addr}"),
            Self::UnmappedWrite(addr) => write!(f, "unmapped memory write {addr}"),
            Self::SerialJoypadRead => write!(f, "serial joypad read ($4016/$4017)"),
            Self::DmaUnusedRegister(addr) => write!(f, "DMA unused register {addr}"),
            Self::PpuUnhandledRead(offset) => {
                write!(f, "PPU unhandled read ${offset:04X}")
            }
            Self::PpuUnhandledWrite(offset) => {
                write!(f, "PPU unhandled write ${offset:04X}")
            }
            Self::PpuStat77Read => write!(f, "PPU STAT77 read ($213E)"),
            Self::PpuStat78Read => write!(f, "PPU STAT78 read ($213F)"),
            Self::DspUnhandledRegister(reg) => {
                write!(f, "S-DSP unhandled register ${reg:02X}")
            }
        }
    }
}
