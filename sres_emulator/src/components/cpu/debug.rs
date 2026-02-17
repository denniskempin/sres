use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;

use super::Cpu;
use super::MainBus;
use super::NativeVectorTable;
use super::StatusFlags;
use crate::common::address::AddressU24;
use crate::common::address::InstructionMeta;
use crate::common::address::VariableLengthUInt;
use crate::common::clock::ClockInfo;

#[derive(Debug, Clone, PartialEq)]
pub enum CpuEvent {
    Step(CpuState),
    Interrupt(NativeVectorTable),
}

pub struct CpuDebug<'a, BusT: MainBus>(pub &'a Cpu<BusT>);

impl<BusT: MainBus> CpuDebug<'_, BusT> {
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
            status: StatusFlags {
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

/// Represents a snapshot of the Cpu state for debugging purposes
/// It formats into a string compatible with MESEN using the following custom format string:
/// [Disassembly][EffectiveAddress] [MemoryValue,h][Align,38] A:[A,4h] X:[X,4h] Y:[Y,4h] S:[SP,4h] D:[D,4h] DB:[DB,2h] P:[P,8] V:[Scanline,3] H:[HClock,4] F:[FrameCount]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CpuState {
    pub instruction: InstructionMeta<AddressU24>,
    pub a: u16,
    pub x: u16,
    pub y: u16,
    pub s: u16,
    pub d: u16,
    pub db: u8,
    pub status: StatusFlags,
    pub clock: ClockInfo,
}

impl CpuState {
    /// Parse a Mesen trace line into a CpuState object
    pub fn parse_mesen_trace(s: &str) -> Result<Self> {
        // The trace format has a fixed width, which allows us to use direct indexing to parse
        // instead of much slower regex or nom parsing.
        //
        // Example with indices of the start of each item:
        //n
        // 00e811  BPL $E80E                      A:9901 X:0100 Y:0000 S:1FF3 D:0000 DB:00 P:nVMxdIZC V:123 H:1234 F:1
        // 0       8   12                           41     48     55     62     69      77   82         93    99     105

        if &s[39..=40] != "A:" {
            bail!("Invalid Mesen trace format - missing A: field");
        }
        if &s[80..=81] != "P:" {
            bail!("Invalid Mesen trace format - missing P: field");
        }
        if &s[91..=92] != "V:" {
            bail!("Invalid Mesen trace format - missing V: field");
        }
        if &s[97..=98] != "H:" {
            bail!("Invalid Mesen trace format - missing H: field");
        }
        if &s[104..=105] != "F:" {
            print!("{}", &s[104..=105]);
            bail!("Invalid Mesen trace format - missing F: field");
        }

        let (operation, operand_str, effective_addr_and_value) =
            Self::parse_disassembly(s[8..39].trim())?;
        Ok(CpuState {
            instruction: InstructionMeta {
                address: u32::from_str_radix(&s[0..6], 16)
                    .with_context(|| "pc")?
                    .into(),
                operation,
                operand_str,
                effective_addr_and_value,
            },
            a: u16::from_str_radix(&s[41..45], 16).with_context(|| "a")?,
            x: u16::from_str_radix(&s[48..52], 16).with_context(|| "x")?,
            y: u16::from_str_radix(&s[55..59], 16).with_context(|| "y")?,
            s: u16::from_str_radix(&s[62..66], 16).with_context(|| "s")?,
            d: u16::from_str_radix(&s[69..73], 16).with_context(|| "d")?,
            db: u8::from_str_radix(&s[77..79], 16).with_context(|| "db")?,
            status: {
                let status_str = &s[82..90];
                if status_str.len() != 8 {
                    bail!("StatusFlags string must be 8 characters long");
                }
                let mut chars = status_str.chars();
                StatusFlags {
                    negative: chars.next().unwrap().is_uppercase(),
                    overflow: chars.next().unwrap().is_uppercase(),
                    accumulator_register_size: chars.next().unwrap().is_uppercase(),
                    index_register_size_or_break: chars.next().unwrap().is_uppercase(),
                    decimal: chars.next().unwrap().is_uppercase(),
                    irq_disable: chars.next().unwrap().is_uppercase(),
                    zero: chars.next().unwrap().is_uppercase(),
                    carry: chars.next().unwrap().is_uppercase(),
                }
            },
            clock: ClockInfo::from_master_clock(Self::mesen_vhf_to_master_clock(
                u64::from_str(&s[93..96].trim()).with_context(|| "v")?,
                u64::from_str(&s[99..103].trim()).with_context(|| "h")?,
                u64::from_str(&s[106..].trim()).with_context(|| "f")?,
            )),
        })
    }

    // Mesen increments the frame number at the start of vblank (v=225), which complicates the
    // logic to determine which master clock we are at.
    pub fn mesen_vhf_to_master_clock(v: u64, h_counter: u64, f: u64) -> u64 {
        // Short scanline at v=240:
        // - Odd frames (1,3,5,...) have vblank from original even frames: NO short scanline
        // - Even frames (2,4,6,...) have vblank from original odd frames: HAS short scanline

        if f == 0 {
            // Frame 0: only active display v=0-224
            return v * 1364 + h_counter;
        }

        // Calculate cycles before frame f starts
        // Frame 0: 225 * 1364 = 306900 cycles
        // Frame 1: 262 * 1364 = 357368 cycles (no short scanline)
        // Frame 2: 262 * 1364 - 4 = 357364 cycles (has short scanline)
        // Pattern repeats: odd frames 357368, even frames 357364
        let frame0_length: u64 = 225 * 1364;
        let odd_frame_length: u64 = 357368;
        let even_frame_length: u64 = 357364;
        let frame_pair_length: u64 = odd_frame_length + even_frame_length;

        let pairs = (f - 1) / 2;
        let mut f_cycles = frame0_length + pairs * frame_pair_length;
        if f % 2 == 0 {
            f_cycles += odd_frame_length;
        }

        // Calculate cycles within frame f
        // If v >= 225: in vblank portion
        // If v < 225: in active portion (after 37 vblank scanlines)
        let has_short_scanline = f % 2 == 0; // even frames have short scanline

        let v_cycles = if v >= 225 {
            // In vblank portion (v=225-261 maps to offset 0-36)
            let vblank_v = v - 225;
            // Short scanline is at v=240 (vblank_v = 15)
            if has_short_scanline && vblank_v > 15 {
                vblank_v * 1364 - 4
            } else {
                vblank_v * 1364
            }
        } else {
            // In active portion, after 37 vblank scanlines
            let vblank_cycles: u64 = if has_short_scanline {
                37 * 1364 - 4
            } else {
                37 * 1364
            };
            vblank_cycles + v * 1364
        };

        // The master clock lags behind 8 cycles in mesen. Unsure why exactly, but it can be
        // observed in the first cycle executed in the trace, master_clock will be 186 and h_counter will be 194.
        f_cycles + v_cycles + h_counter
    }

    pub fn parse_disassembly(
        disassembly: &str,
    ) -> Result<(
        String,
        Option<String>,
        Option<(AddressU24, VariableLengthUInt)>,
    )> {
        let pieces = disassembly.trim().split_ascii_whitespace().collect_vec();
        match pieces.len() {
            // e.g. "NMI"
            1 => Ok((pieces[0].to_string(), None, None)),
            // e.g. "BPL $1234"
            2 => Ok((pieces[0].to_string(), Some(pieces[1].to_string()), None)),
            // e.g. LDA $1234 [001234]
            3 => {
                let effective_addr_str = pieces[2].trim_matches(&['[', ']', '$']);
                let effective_addr = if let Ok(addr) = u32::from_str_radix(effective_addr_str, 16) {
                    AddressU24::from(addr)
                } else {
                    if ADDR_ANNOTATIONS_REVERSE.contains_key(effective_addr_str) {
                        AddressU24::from(ADDR_ANNOTATIONS_REVERSE[effective_addr_str])
                    } else {
                        bail!("Invalid address `{effective_addr_str}` in `{disassembly}`")
                    }
                };

                Ok((
                    pieces[0].to_string(),
                    Some(pieces[1].to_string()),
                    Some((effective_addr, VariableLengthUInt::U8(0))),
                ))
            }
            // e.g. LDA $1234 [001234] = 42
            5 => {
                let effective_addr_str = pieces[2].trim_matches(&['[', ']']).trim_matches('$');
                let effective_addr = if let Ok(addr) = u32::from_str_radix(effective_addr_str, 16) {
                    AddressU24::from(addr)
                } else {
                    if ADDR_ANNOTATIONS_REVERSE.contains_key(effective_addr_str) {
                        AddressU24::from(ADDR_ANNOTATIONS_REVERSE[effective_addr_str])
                    } else {
                        bail!("Invalid address `{effective_addr_str}`")
                    }
                };

                let value_str = pieces[4].trim_matches(&['$']).trim();
                let effective_addr_and_value = match value_str.len() {
                    2 => {
                        let value = u8::from_str_radix(value_str, 16).with_context(|| "value")?;
                        Some((effective_addr, VariableLengthUInt::U8(value)))
                    }
                    4 => {
                        let value = u16::from_str_radix(value_str, 16).with_context(|| "value")?;
                        Some((effective_addr, VariableLengthUInt::U16(value)))
                    }
                    _ => bail!("Invalid memory value `{value_str}`"),
                };
                Ok((
                    pieces[0].to_string(),
                    Some(pieces[1].to_string()),
                    effective_addr_and_value,
                ))
            }
            _ => {
                bail!("Invalid disassembly format `{disassembly}`")
            }
        }
    }

    pub fn format_disassembly(
        operation: &str,
        operand_str: &Option<String>,
        effective_addr_and_value: &Option<(AddressU24, VariableLengthUInt)>,
    ) -> String {
        match (operand_str, effective_addr_and_value) {
            (None, None) => operation.to_string(),
            (Some(operand_str), None) => format!("{operation} {operand_str}"),
            (Some(operand_str), Some((addr, value))) => {
                let addr: u32 = (*addr).into();
                let value_str = match value {
                    VariableLengthUInt::U8(v) => format!("${v:02X}"),
                    VariableLengthUInt::U16(v) => format!("${v:04X}"),
                };
                format!("{operation} {operand_str} [{addr:06X}] = {value_str}")
            }
            (None, Some(_)) => panic!("Cannot format disassembly: No operand but IO info"),
        }
    }
}

impl Display for CpuState {
    /// Format a trace object into a BSNES trace line
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "{:08} [{:06x}]  {:<30} A:{:04X} X:{:04X} Y:{:04X} S:{:04X} D:{:04X} DB:{:02X} P:{} V:{:3} H:{:4} F:{}",
            self.clock.master_clock,
            u32::from(self.instruction.address),
            Self::format_disassembly(
                &self.instruction.operation,
                &self.instruction.operand_str,
                &self.instruction.effective_addr_and_value
            ),
            self.a,
            self.x,
            self.y,
            self.s,
            self.d,
            self.db,
            format!(
                "{}{}{}{}{}{}{}{}",
                if self.status.negative { 'N' } else { 'n' },
                if self.status.overflow { 'V' } else { 'v' },
                if self.status.accumulator_register_size {
                    'M'
                } else {
                    'm'
                },
                if self.status.index_register_size_or_break {
                    'X'
                } else {
                    'x'
                },
                if self.status.decimal { 'D' } else { 'd' },
                if self.status.irq_disable { 'I' } else { 'i' },
                if self.status.zero { 'Z' } else { 'z' },
                if self.status.carry { 'C' } else { 'c' }
            ),
            self.clock.v,
            self.clock.h_counter,
            self.clock.f,
        )
    }
}

impl FromStr for StatusFlags {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            bail!("StatusFlags string must be 8 characters long");
        }
        let mut chars = s.chars();
        Ok(StatusFlags {
            negative: chars.next().unwrap().is_ascii_uppercase(),
            overflow: chars.next().unwrap().is_ascii_uppercase(),
            accumulator_register_size: chars.next().unwrap().is_ascii_uppercase(),
            index_register_size_or_break: chars.next().unwrap().is_ascii_uppercase(),
            decimal: chars.next().unwrap().is_ascii_uppercase(),
            irq_disable: chars.next().unwrap().is_ascii_uppercase(),
            zero: chars.next().unwrap().is_ascii_uppercase(),
            carry: chars.next().unwrap().is_ascii_uppercase(),
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_mesen_vhf_to_master_clock() {
        // Frame 0, start
        assert_eq!(CpuState::mesen_vhf_to_master_clock(0, 0, 0), 0);

        // Frame 0, h=194. First cycle after bootup.
        assert_eq!(CpuState::mesen_vhf_to_master_clock(0, 194, 0), 194);

        // Frame 0, at v=224 (just before vblank)
        assert_eq!(CpuState::mesen_vhf_to_master_clock(224, 0, 0), 305536);

        // Frame 0, at v=225 (vblank start). In mesen, this will be frame 1.
        assert_eq!(CpuState::mesen_vhf_to_master_clock(225, 0, 1), 306900);

        // Frame 0, at v=261 (last scanline of frame 0)
        assert_eq!(CpuState::mesen_vhf_to_master_clock(261, 0, 1), 356004);

        // Frame 0, end of last scanline
        assert_eq!(CpuState::mesen_vhf_to_master_clock(261, 1363, 1), 357367);

        // Frame 1, start (v=0, h=0, f=1)
        assert_eq!(CpuState::mesen_vhf_to_master_clock(0, 0, 1), 357368);
    }

    static EXAMPLE_MESEN_TRACE: &str = r"00e811  BPL $E80E                      A:9901 X:0100 Y:0000 S:1FF3 D:0000 DB:00 P:nVMxdIZC V:123 H:1226 F:1";
    static EXAMPLE_SRES_TRACE: &str = r"00526366: 00e811  BPL $E80E                      A:9901 X:0100 Y:0000 S:1FF3 D:0000 DB:00 P:nVMxdIZC V:123 H:1226 F:1";

    fn example_trace() -> CpuState {
        CpuState {
            instruction: InstructionMeta {
                address: AddressU24 {
                    bank: 0,
                    offset: 0xe811,
                },
                operation: "BPL".to_string(),
                operand_str: Some("$E80E".to_string()),
                effective_addr_and_value: None,
            },
            a: 0x9901,
            x: 0x0100,
            y: 0x0000,
            s: 0x1ff3,
            d: 0x0000,
            db: 0x00,
            status: StatusFlags {
                negative: false,
                overflow: true,
                accumulator_register_size: true,
                index_register_size_or_break: false,
                decimal: false,
                irq_disable: true,
                zero: true,
                carry: true,
            },
            clock: ClockInfo {
                master_clock: 526366,
                v: 123,
                h_counter: 1226,
                f: 1,
            },
        }
    }

    #[test]
    pub fn test_from_mesen_trace() {
        assert_eq!(
            CpuState::parse_mesen_trace(&EXAMPLE_MESEN_TRACE).unwrap(),
            example_trace()
        );
    }

    #[test]
    pub fn test_to_str() {
        assert_eq!(format!("{}", example_trace()), EXAMPLE_SRES_TRACE);
    }

    #[test]
    pub fn test_disassembly_roundtrip() {
        let cases = [
            "CLC",
            "BPL $1234",
            "LDA $1234 [001234] = $5678",
            "LDA $1234 [001234] = $56",
        ];
        for case in cases {
            let (operation, operand_str, io) = CpuState::parse_disassembly(case).unwrap();
            let formatted = CpuState::format_disassembly(&operation, &operand_str, &io);
            assert_eq!(formatted, case);
        }
    }
}

lazy_static! {
    pub static ref ADDR_ANNOTATIONS: HashMap<u32, &'static str> = {
        [
            // PPU Display Registers ($2100-$2114)
            (0x2100, "INIDISP"),  // Screen Display
            (0x2101, "OBSEL"),    // Object Size and Character Size
            (0x2102, "OAMADDL"),  // OAM Address Low
            (0x2103, "OAMADDH"),  // OAM Address High
            (0x2104, "OAMDATA"),  // OAM Data Write
            (0x2105, "BGMODE"),   // BG Mode and Character Size
            (0x2106, "MOSAIC"),   // Mosaic Size and Enable
            (0x2107, "BG1SC"),    // BG1 Screen Base and Size
            (0x2108, "BG2SC"),    // BG2 Screen Base and Size
            (0x2109, "BG3SC"),    // BG3 Screen Base and Size
            (0x210A, "BG4SC"),    // BG4 Screen Base and Size
            (0x210B, "BG12NBA"),  // BG1/2 Character Data Area Designation
            (0x210C, "BG34NBA"),  // BG3/4 Character Data Area Designation

            // PPU Scroll Registers ($2110-$2114)
            (0x210D, "BG1HOFS"),  // BG1 Horizontal Scroll
            (0x210E, "BG1VOFS"),  // BG1 Vertical Scroll
            (0x210F, "BG2HOFS"),  // BG2 Horizontal Scroll
            (0x2110, "BG2VOFS"),  // BG2 Vertical Scroll
            (0x2111, "BG3HOFS"),  // BG3 Horizontal Scroll
            (0x2112, "BG3VOFS"),  // BG3 Vertical Scroll
            (0x2113, "BG4HOFS"),  // BG4 Horizontal Scroll
            (0x2114, "BG4VOFS"),  // BG4 Vertical Scroll

            // PPU VRAM Registers ($2115-$2119)
            (0x2115, "VMAIN"),    // VRAM Address Increment
            (0x2116, "VMADDL"),   // VRAM Address Low
            (0x2117, "VMADDH"),   // VRAM Address High
            (0x2118, "VMDATAL"),  // VRAM Data Write Low
            (0x2119, "VMDATAH"),  // VRAM Data Write High

            // PPU Mode 7 Registers ($211A-$2120)
            (0x211A, "M7SEL"),    // Mode 7 Settings
            (0x211B, "M7A"),      // Mode 7 Matrix A
            (0x211C, "M7B"),      // Mode 7 Matrix B
            (0x211D, "M7C"),      // Mode 7 Matrix C
            (0x211E, "M7D"),      // Mode 7 Matrix D
            (0x211F, "M7X"),      // Mode 7 Center X
            (0x2120, "M7Y"),      // Mode 7 Center Y

            // PPU CGRAM Registers ($2121-$2122)
            (0x2121, "CGADD"),    // CGRAM Address
            (0x2122, "CGDATA"),   // CGRAM Data Write

            // PPU Window Registers ($2123-$212F)
            (0x2123, "W12SEL"),   // Window 1/2 Mask Settings for BG1/BG2
            (0x2124, "W34SEL"),   // Window 1/2 Mask Settings for BG3/BG4
            (0x2125, "WOBJSEL"),  // Window 1/2 Mask Settings for OBJ/MATH
            (0x2126, "WH0"),      // Window 1 Left Position
            (0x2127, "WH1"),      // Window 1 Right Position
            (0x2128, "WH2"),      // Window 2 Left Position
            (0x2129, "WH3"),      // Window 2 Right Position
            (0x212A, "WBGLOG"),   // Window Mask Logic for BG
            (0x212B, "WOBJLOG"),  // Window Mask Logic for OBJ
            (0x212C, "TM"),       // Main Screen Designation
            (0x212D, "TS"),       // Sub Screen Designation
            (0x212E, "TMW"),      // Window Mask Designation for Main Screen
            (0x212F, "TSW"),      // Window Mask Designation for Sub Screen

            // PPU Color Math Registers ($2130-$2132)
            (0x2130, "CGWSEL"),   // Color Math Control Register A
            (0x2131, "CGADSUB"),  // Color Math Control Register B
            (0x2132, "COLDATA"),  // Color Math Sub Screen Backdrop Color

            // PPU Status and Mode Registers ($2133-$213F)
            (0x2133, "SETINI"),   // Display Control 2
            (0x2134, "MPYL"),     // Multiplication Result Low
            (0x2135, "MPYM"),     // Multiplication Result Middle
            (0x2136, "MPYH"),     // Multiplication Result High
            (0x2137, "SLHV"),     // Software Latch for H/V Counter
            (0x2138, "OAMDATAREAD"), // OAM Data Read
            (0x2139, "VMDATALREAD"), // VRAM Data Read Low
            (0x213A, "VMDATAHREAD"), // VRAM Data Read High
            (0x213B, "CGDATAREAD"),  // CGRAM Data Read
            (0x213C, "OPHCT"),    // H Counter Read
            (0x213D, "OPVCT"),    // V Counter Read
            (0x213E, "STAT77"),   // PPU Status Flag and Version
            (0x213F, "STAT78"),   // PPU Status Flag and Version

            // APU Communication Registers ($2140-$2143)
            (0x2140, "APUIO0"),   // APU IO Port 0
            (0x2141, "APUIO1"),   // APU IO Port 1
            (0x2142, "APUIO2"),   // APU IO Port 2
            (0x2143, "APUIO3"),   // APU IO Port 3

            // WRAM Access Registers ($2180-$2183)
            (0x2180, "WMDATA"),   // WRAM Data Read/Write
            (0x2181, "WMADDL"),   // WRAM Address Low
            (0x2182, "WMADDM"),   // WRAM Address Middle
            (0x2183, "WMADDH"),   // WRAM Address High

            // CPU Control Registers ($4200-$420D)
            (0x4200, "NMITIMEN"), // Interrupt Enable and Joypad Request
            (0x4201, "WRIO"),     // Programmable I/O Port (Out)
            (0x4202, "WRMPYA"),   // Multiplicand
            (0x4203, "WRMPYB"),   // Multiplier
            (0x4204, "WRDIVL"),   // Dividend Low
            (0x4205, "WRDIVH"),   // Dividend High
            (0x4206, "WRDIVB"),   // Divisor
            (0x4207, "HTIMEL"),   // IRQ Timer Horizontal Counter Low
            (0x4208, "HTIMEH"),   // IRQ Timer Horizontal Counter High
            (0x4209, "VTIMEL"),   // IRQ Timer Vertical Counter Low
            (0x420A, "VTIMEH"),   // IRQ Timer Vertical Counter High
            (0x420B, "MDMAEN"),   // DMA Enable
            (0x420C, "HDMAEN"),   // HDMA Enable
            (0x420D, "MEMSEL"),   // ROM Access Speed
            (0x4016, "JOYSER0"),  // Joypad 0 Serial Data
            (0x4017, "JOYSER1"),  // Joypad 1 Serial Data

            // CPU Status Registers ($4210-$421F)
            (0x4210, "RDNMI"),    // NMI Flag and 5A22 Version
            (0x4211, "TIMEUP"),   // IRQ Flag
            (0x4212, "HVBJOY"),   // PPU Status
            (0x4213, "RDIO"),     // Programmable I/O Port (In)
            (0x4214, "RDDIVL"),   // Divide Result Low
            (0x4215, "RDDIVH"),   // Divide Result High
            (0x4216, "RDMPYL"),   // Multiply Result Low
            (0x4217, "RDMPYH"),   // Multiply Result High
            (0x4218, "JOY1L"),    // Controller Port 1 Data Low
            (0x4219, "JOY1H"),    // Controller Port 1 Data High
            (0x421A, "JOY2L"),    // Controller Port 2 Data Low
            (0x421B, "JOY2H"),    // Controller Port 2 Data High
            (0x421C, "JOY3L"),    // Controller Port 3 Data Low
            (0x421D, "JOY3H"),    // Controller Port 3 Data High
            (0x421E, "JOY4L"),    // Controller Port 4 Data Low
            (0x421F, "JOY4H"),    // Controller Port 4 Data High

            // DMA Control Registers ($43x0-$43xF, x=0-7)
            (0x4300, "DMAP0"),    // DMA0 Control
            (0x4301, "BBAD0"),    // DMA0 Destination
            (0x4302, "A1T0L"),    // DMA0 Source Address Low
            (0x4303, "A1T0H"),    // DMA0 Source Address High
            (0x4304, "A1B0"),     // DMA0 Source Bank
            (0x4305, "DAS0L"),    // DMA0 Size Low
            (0x4306, "DAS0H"),    // DMA0 Size High
            (0x4307, "DASB0"),    // DMA0 Bank for HDMA
            (0x4308, "A2A0L"),    // DMA0 HDMA Table Address Low
            (0x4309, "A2A0H"),    // DMA0 HDMA Table Address High
            (0x430A, "NTLR0"),    // DMA0 HDMA Line Counter
        ]
        .iter()
        .cloned()
        .collect()
    };
}

lazy_static! {
    pub static ref ADDR_ANNOTATIONS_REVERSE: HashMap<&'static str, u32> = {
        ADDR_ANNOTATIONS
            .iter()
            .map(|(&addr, &name)| (name, addr))
            .collect()
    };
}
