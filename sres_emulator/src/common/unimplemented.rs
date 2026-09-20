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
    RomWrite(AddressU24),
    SerialJoypadRead,
    JoypadAutoReadEnable,
    DmaUnusedRegister(AddressU24),
    PpuUnhandledRead(u16),
    PpuUnhandledWrite(u16),
    PpuStat77Read,
    PpuStat78Read,
    PpuMosaicWrite,
    PpuWindowWrite,
    PpuMode7SelWrite,
    PpuMode7MatrixWrite,
    PpuCgwselWrite,
    PpuSetiniWrite,
    PpuInidispBrightness,
    PpuVramRemap,
    PpuBgMode4,
    PpuBgMode6,
    PpuBgMode7,
    PpuOffsetPerTile,
    PpuHiRes,
    PpuObjColorMath,
    DspUnhandledRegister(u8),
    DspKoff,
    DspMvol,
    DspEcho,
    DspPmon,
    DspEndx,
    DspFlgMute,
    DspFlgReset,
    ApuTestRegisterRead,
    ApuTestRegisterWrite,
    ApuControlRead,
    ApuDspDataReadonlyWrite,
    ApuTimerOutputWrite,
    CpuEmulationModeNmi,
    CpuEmulationModeIrq,
    CpuEmulationModeBreakException,
    CpuEmulationModeRtiReturn,
    Spc700Sleep,
    Spc700Stop,
}

impl Display for UnimplementedBehavior {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RegisterRead(addr) => write!(f, "unimplemented register read {addr}"),
            Self::RegisterWrite(addr) => write!(f, "unimplemented register write {addr}"),
            Self::UnmappedRead(addr) => write!(f, "unmapped memory read {addr}"),
            Self::UnmappedWrite(addr) => write!(f, "unmapped memory write {addr}"),
            Self::RomWrite(addr) => write!(f, "ROM write {addr}"),
            Self::SerialJoypadRead => write!(f, "serial joypad read ($4016/$4017)"),
            Self::JoypadAutoReadEnable => write!(f, "joypad auto-read enable (NMITIMEN bit 0)"),
            Self::DmaUnusedRegister(addr) => write!(f, "DMA unused register {addr}"),
            Self::PpuUnhandledRead(offset) => write!(f, "PPU unhandled read ${offset:04X}"),
            Self::PpuUnhandledWrite(offset) => write!(f, "PPU unhandled write ${offset:04X}"),
            Self::PpuStat77Read => write!(f, "PPU STAT77 read ($213E)"),
            Self::PpuStat78Read => write!(f, "PPU STAT78 read ($213F)"),
            Self::PpuMosaicWrite => write!(f, "PPU MOSAIC write ($2106)"),
            Self::PpuWindowWrite => write!(f, "PPU window register write"),
            Self::PpuMode7SelWrite => write!(f, "PPU M7SEL write ($211A)"),
            Self::PpuMode7MatrixWrite => write!(f, "PPU Mode 7 matrix write ($211D–$2120)"),
            Self::PpuCgwselWrite => write!(f, "PPU CGWSEL write ($2130)"),
            Self::PpuSetiniWrite => write!(f, "PPU SETINI write ($2133)"),
            Self::PpuInidispBrightness => write!(f, "PPU INIDISP brightness"),
            Self::PpuVramRemap => write!(f, "PPU VMAIN address remapping"),
            Self::PpuBgMode4 => write!(f, "PPU BG mode 4 render"),
            Self::PpuBgMode6 => write!(f, "PPU BG mode 6 render"),
            Self::PpuBgMode7 => write!(f, "PPU BG mode 7 render"),
            Self::PpuOffsetPerTile => write!(f, "PPU offset-per-tile"),
            Self::PpuHiRes => write!(f, "PPU hi-res render"),
            Self::PpuObjColorMath => write!(f, "PPU OBJ color math"),
            Self::DspUnhandledRegister(reg) => write!(f, "S-DSP unhandled register ${reg:02X}"),
            Self::DspKoff => write!(f, "S-DSP KOFF write ($5C)"),
            Self::DspMvol => write!(f, "S-DSP MVOL write ($0C/$1C)"),
            Self::DspEcho => write!(f, "S-DSP echo/FIR write"),
            Self::DspPmon => write!(f, "S-DSP PMON write ($2D)"),
            Self::DspEndx => write!(f, "S-DSP ENDX ($7C)"),
            Self::DspFlgMute => write!(f, "S-DSP FLG mute ($6C bit 6)"),
            Self::DspFlgReset => write!(f, "S-DSP FLG soft reset ($6C bit 7)"),
            Self::ApuTestRegisterRead => write!(f, "APU TEST register read ($F0)"),
            Self::ApuTestRegisterWrite => write!(f, "APU TEST register write ($F0)"),
            Self::ApuControlRead => write!(f, "APU CONTROL read ($F1)"),
            Self::ApuDspDataReadonlyWrite => write!(f, "APU DSPDATA write while readonly ($F3)"),
            Self::ApuTimerOutputWrite => write!(f, "APU timer output write ($FD–$FF)"),
            Self::CpuEmulationModeNmi => write!(f, "CPU emulation-mode NMI"),
            Self::CpuEmulationModeIrq => write!(f, "CPU emulation-mode IRQ"),
            Self::CpuEmulationModeBreakException => write!(f, "CPU emulation-mode BRK/COP"),
            Self::CpuEmulationModeRtiReturn => write!(f, "CPU emulation-mode RTI"),
            Self::Spc700Sleep => write!(f, "SPC700 SLEEP ($EF)"),
            Self::Spc700Stop => write!(f, "SPC700 STOP ($FF)"),
        }
    }
}
