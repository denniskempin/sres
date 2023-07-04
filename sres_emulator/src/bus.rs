use std::path::Path;

use anyhow::Result;
use intbits::Bits;

use crate::cartridge::Cartridge;
use crate::memory::Address;
use crate::memory::Memory;
use crate::memory::ToAddress;

pub fn master_clock_to_fvh(master_clock: u64) -> (u64, u64, u64) {
    let double_frame_length = 357368 + 357364;
    let double_frames = master_clock / double_frame_length;
    let mut f_remainder = master_clock % double_frame_length;
    let mut f = double_frames * 2;
    let odd_frame = f_remainder >= 357368;
    if odd_frame {
        f += 1;
        f_remainder -= 357368;
    }

    let v = if odd_frame && f_remainder >= 1364 * 240 {
        (f_remainder + 4) / 1364
    } else {
        f_remainder / 1364
    };

    let h_counter = if odd_frame && f_remainder >= 1364 * 240 + 1360 {
        (f_remainder + 4) % 1364
    } else {
        f_remainder % 1364
    };
    (f, v, h_counter)
}

pub fn fvh_to_master_clock(f: u64, v: u64, h: u64) -> u64 {
    let f_cycles = if f % 2 == 0 {
        f * 357366
    } else {
        f * 357366 + 2
    };

    let odd_frame = f % 2 == 1;
    let v_cycles = if odd_frame && v > 240 {
        v * 1364 - 4
    } else {
        v * 1364
    };

    f_cycles + v_cycles + h
}

/// Memory access speed as per memory map. See:
/// https://wiki.superfamicom.org/memory-mapping#memory-map-67
fn memory_access_speed(addr: Address) -> u64 {
    static FAST: u64 = 6;
    static SLOW: u64 = 8;
    static XSLOW: u64 = 12;

    match addr.bank {
        0x00..=0x3F => match addr.offset {
            0x0000..=0x1FFF => SLOW,
            0x2000..=0x3FFF => FAST,
            0x4000..=0x41FF => XSLOW,
            0x4200..=0x5FFF => FAST,
            0x6000..=0xFFFF => SLOW,
        },
        0x40..=0x7F => SLOW,
        0x80..=0xBF => {
            match addr.offset {
                0x0000..=0x1FFF => SLOW,
                0x2000..=0x3FFF => FAST,
                0x4000..=0x41FF => XSLOW,
                0x4200..=0x5FFF => FAST,
                0x6000..=0xFFFF => SLOW, // TODO fastrom support
            }
        }
        0xC0..=0xFF => SLOW, // TODO fastrom support
    }
}

pub trait Bus: Memory {
    fn reset(&mut self);
    fn ppu_timer(&self) -> PpuTimer;
    fn internal_operation_cycle(&mut self);
    fn advance_master_clock(&mut self, cycles: u64);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct DmaChannel {
    a1t: Address,
    das: u16,
}

pub struct TestBus {
    pub memory: Vec<u8>,
    pub ppu_timer: PpuTimer,
    pub dma_channels: [DmaChannel; 8],
    pub dma_pending: u8,
    pub dma_active: bool,
    pub clock_speed: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PpuTimer {
    pub master_clock: u64,
    pub v: u64,
    pub h_counter: u64,
    pub f: u64,
    pub dram_refresh_position: u64,
    pub nmi_flag: bool,
}

impl PpuTimer {
    pub fn from_master_clock(master_clock: u64) -> Self {
        let (f, v, h) = master_clock_to_fvh(master_clock);

        Self {
            master_clock,
            v,
            h_counter: h,
            f,
            dram_refresh_position: 538,
            nmi_flag: false,
        }
    }

    pub fn advance_master_clock(&mut self, master_cycles: u64) {
        for _ in 0..master_cycles {
            self.tick_master_clock();
        }
    }

    fn tick_master_clock(&mut self) {
        self.master_clock += 1;
        self.h_counter += 1;

        // 536 master cycles after the start of a scanline, the CPU will pause for 40 cycles.
        // See: https://wiki.superfamicom.org/timing#clocks-and-refresh-10
        if self.h_counter == self.dram_refresh_position {
            self.h_counter += 40;
            self.master_clock += 40;
        }

        // Line 240 of each odd frame is 4 cycles shorter.
        // See: https://snes.nesdev.org/wiki/Timing#Short_and_Long_Scanlines
        let h_duration = if self.v == 240 && self.f % 2 == 1 {
            1360
        } else {
            1364
        };
        if self.h_counter >= h_duration {
            self.h_counter -= h_duration;
            self.v += 1;
            if self.v == 225 {
                self.nmi_flag = true;
                //println!("nmi = true");
            }
            self.dram_refresh_position = 538 - (self.master_clock & 7);
        }

        if self.v >= 262 {
            self.v -= 262;
            self.f += 1;
            self.nmi_flag = false;
            //println!("nmi = false");
        }
    }

    pub fn hdot(&self) -> u64 {
        let mut counter = self.h_counter;
        if self.f % 2 == 0 || self.v != 240 {
            // Dot 323 and 327 take 6 cycles on non-short scanlines.
            if self.h_counter > 1292 {
                counter -= 2;
            }
            if self.h_counter > 1310 {
                counter -= 2;
            }
        }
        counter / 4
    }
}

impl Default for PpuTimer {
    fn default() -> Self {
        Self {
            master_clock: 0,
            v: 0,
            h_counter: 0,
            f: 0,
            dram_refresh_position: 538,
            nmi_flag: false,
        }
    }
}

impl TestBus {
    pub fn with_sfc(rom_path: &Path) -> Result<Self> {
        let mut bus = Self::default();
        // Load cartridge data into memory
        let mut cartridge = Cartridge::new();
        cartridge.load_sfc(rom_path)?;
        for (i, byte) in cartridge.rom.iter().enumerate() {
            bus.memory[0x8000 + i] = *byte;
        }
        Ok(bus)
    }

    pub fn with_sfc_data(rom: &[u8]) -> Result<Self> {
        let mut bus = Self::default();
        // Load cartridge data into memory
        let mut cartridge = Cartridge::new();
        cartridge.load_sfc_data(rom)?;
        for (i, byte) in cartridge.rom.iter().enumerate() {
            bus.memory[0x8000 + i] = *byte;
        }
        Ok(bus)
    }
    pub fn with_program(program: &[u8]) -> Self {
        let mut bus = Self::default();
        for (i, byte) in program.iter().enumerate() {
            bus.memory[i] = *byte;
        }
        bus
    }
}

impl Default for TestBus {
    fn default() -> Self {
        Self {
            memory: vec![0; 0x1000000],
            ppu_timer: PpuTimer::default(),
            dma_channels: Default::default(),
            dma_pending: 0,
            dma_active: false,
            clock_speed: 8,
        }
    }
}

impl Memory for TestBus {
    fn peek_u8(&self, addr: impl ToAddress) -> Option<u8> {
        let addr = addr.to_address();
        Some(self.memory[u32::from(addr) as usize])
    }

    fn read_u8(&mut self, addr: impl ToAddress) -> u8 {
        let addr = addr.to_address();
        self.clock_speed = memory_access_speed(addr);
        //println!("  read_u8({addr}) ({} cycles)", self.clock_speed);
        self.ppu_timer.advance_master_clock(self.clock_speed - 6);
        let value = if u32::from(addr) == 0x004210 {
            let override_value = self.peek_u8(addr).unwrap_or(0);
            if override_value > 0 {
                return override_value;
            }
            if self.ppu_timer.nmi_flag {
                // Fake NMI hold, do not reset nmi flag for the first 2 cyles.
                if !(self.ppu_timer.v == 225 && self.ppu_timer.h_counter <= 2) {
                    self.ppu_timer.nmi_flag = false;
                }
                0b1111_0010
            } else {
                0b0111_0010
            }
        } else {
            self.peek_u8(addr).unwrap_or(0)
        };
        self.advance_master_clock(6);
        value
    }

    #[allow(clippy::single_match)]
    fn write_u8(&mut self, addr: impl ToAddress, val: u8) {
        let addr = addr.to_address();
        self.clock_speed = memory_access_speed(addr);
        self.advance_master_clock(self.clock_speed);
        /* println!(
            "  write_u8({addr}) = {val:02x} ({} cycles)",
            self.clock_speed
        );*/
        match addr.bank {
            0x00..=0x1F => match addr.offset.bits(8..16) {
                0x42 => match addr.offset.bits(0..8) {
                    0x0B => {
                        self.dma_pending = val;
                    }
                    _ => {
                        self.memory[u32::from(addr) as usize] = val;
                    }
                },
                0x43 => {
                    let channel = addr.offset.bits(4..8);
                    match addr.offset.bits(0..4) {
                        0x5 => {
                            self.dma_channels[channel as usize]
                                .das
                                .set_bits(0..8, val as u16);
                        }
                        0x6 => {
                            self.dma_channels[channel as usize]
                                .das
                                .set_bits(8..16, val as u16);
                        }
                        _ => {
                            self.memory[u32::from(addr) as usize] = val;
                        }
                    }
                }
                _ => {
                    self.memory[u32::from(addr) as usize] = val;
                }
            },
            _ => {
                unimplemented!("Banks > 0x1F are not implemented yet.");
            }
        }
    }
}

impl Bus for TestBus {
    fn internal_operation_cycle(&mut self) {
        // println!("  io_cycle: (6 cycles)");
        self.clock_speed = 6;
        self.advance_master_clock(self.clock_speed);
    }

    fn advance_master_clock(&mut self, cycles: u64) {
        self.ppu_timer.advance_master_clock(cycles);

        if self.dma_pending > 0 {
            if self.dma_active {
                let dma_counter = 8 - self.ppu_timer.master_clock % 8;
                //println!("dma: c {}, speed {}", dma_counter, self.clock_speed);
                self.ppu_timer.advance_master_clock(dma_counter + 8);
                for channel in 0..8 {
                    if self.dma_pending.bit(channel) {
                        let mut length = self.dma_channels[channel as usize].das as u64;
                        if length == 0 {
                            length = 0x10000;
                        }
                        self.ppu_timer.advance_master_clock(8 + 8 * length);
                    }
                }
                self.ppu_timer
                    .advance_master_clock(self.clock_speed - dma_counter % self.clock_speed);
                self.dma_pending = 0;
                self.dma_active = false;
            } else {
                self.dma_active = true;
            }
        }
    }

    fn ppu_timer(&self) -> PpuTimer {
        self.ppu_timer
    }

    fn reset(&mut self) {
        self.ppu_timer = PpuTimer::default();
        self.advance_master_clock(186);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fvh_master_clock_conversion() {
        for master_clock in 0..=10000000 {
            let (f, v, h) = master_clock_to_fvh(master_clock);
            let actual_master_clock = fvh_to_master_clock(f, v, h);
            assert_eq!(master_clock, actual_master_clock);
        }
    }

    /// Log of (v, h) from bsnes executing nop's. 14 master cycles between each step.
    #[rustfmt::skip]
    const V_H_REFERENCE_LOG: &[(u64, u64)] = &[
        (0,46), (0,50), (0,53), (0,57), (0,60), (0,64), (0,67), (0,71), (0,74), (0,78), (0,81),
        (0,85), (0,88), (0,92), (0,95), (0,99), (0,102), (0,106), (0,109), (0,113), (0,116),
        (0,120), (0,123), (0,127), (0,130), (0,134), (0,147), (0,151), (0,154), (0,158), (0,161),
        (0,165), (0,168), (0,172), (0,175), (0,179), (0,182), (0,186), (0,189), (0,193), (0,196),
        (0,200), (0,203), (0,207), (0,210), (0,214), (0,217), (0,221), (0,224), (0,228), (0,231),
        (0,235), (0,238), (0,242), (0,245), (0,249), (0,252), (0,256), (0,259), (0,263), (0,266),
        (0,270), (0,273), (0,277), (0,280), (0,284), (0,287), (0,291), (0,294), (0,298), (0,301),
        (0,305), (0,308), (0,312), (0,315), (0,319), (0,322), (0,325), (0,328), (0,332), (0,335),
        (0,339), (1,2), (1,6), (1,9), (1,13), (1,16), (1,20), (1,23), (1,27), (1,30), (1,34),
        (1,37), (1,41), (1,44), (1,48), (1,51), (1,55), (1,58), (1,62), (1,65), (1,69), (1,72),
        (1,76), (1,79), (1,83), (1,86), (1,90), (1,93), (1,97), (1,100), (1,104), (1,107), (1,111),
        (1,114), (1,118), (1,121), (1,125), (1,128), (1,132), (1,145), (1,149), (1,152), (1,156),
        (1,159), (1,163), (1,166), (1,170), (1,173), (1,177), (1,180), (1,184), (1,187), (1,191),
        (1,194), (1,198), (1,201), (1,205), (1,208), (1,212), (1,215), (1,219), (1,222), (1,226),
        (1,229), (1,233), (1,236), (1,240), (1,243), (1,247), (1,250), (1,254), (1,257), (1,261),
        (1,264), (1,268), (1,271), (1,275), (1,278), (1,282), (1,285), (1,289), (1,292), (1,296),
        (1,299), (1,303), (1,306), (1,310), (1,313), (1,317), (1,320), (1,323), (1,327), (1,330),
        (1,333), (1,337), (2,0), (2,4), (2,7), (2,11), (2,14), (2,18), (2,21), (2,25), (2,28),
        (2,32), (2,35), (2,39), (2,42), (2,46), (2,49), (2,53), (2,56), (2,60), (2,63), (2,67),
        (2,70), (2,74), (2,77), (2,81), (2,84), (2,88), (2,91), (2,95), (2,98), (2,102), (2,105),
        (2,109), (2,112), (2,116), (2,119), (2,123), (2,126), (2,130), (2,133), (2,147), (2,150),
        (2,154), (2,157), (2,161), (2,164), (2,168), (2,171), (2,175), (2,178), (2,182), (2,185),
        (2,189), (2,192), (2,196), (2,199), (2,203), (2,206), (2,210), (2,213), (2,217), (2,220),
        (2,224), (2,227), (2,231), (2,234), (2,238), (2,241), (2,245), (2,248), (2,252), (2,255),
        (2,259), (2,262), (2,266), (2,269), (2,273), (2,276), (2,280), (2,283), (2,287), (2,290),
        (2,294), (2,297), (2,301), (2,304), (2,308), (2,311), (2,315), (2,318), (2,322), (2,325),
        (2,328), (2,331), (2,335), (2,338), (3,2), (3,5), (3,9), (3,12), (3,16), (3,19), (3,23),
        (3,26), (3,30), (3,33), (3,37), (3,40), (3,44), (3,47), (3,51), (3,54), (3,58), (3,61),
        (3,65), (3,68), (3,72), (3,75), (3,79), (3,82), (3,86), (3,89), (3,93), (3,96), (3,100),
        (3,103), (3,107), (3,110), (3,114), (3,117), (3,121), (3,124), (3,128), (3,131), (3,145),
        (3,148), (3,152), (3,155), (3,159), (3,162), (3,166), (3,169), (3,173), (3,176), (3,180),
        (3,183), (3,187), (3,190), (3,194), (3,197), (3,201), (3,204), (3,208), (3,211), (3,215),
        (3,218), (3,222), (3,225), (3,229), (3,232), (3,236), (3,239), (3,243), (3,246), (3,250),
        (3,253), (3,257), (3,260), (3,264), (3,267), (3,271), (3,274), (3,278), (3,281), (3,285),
        (3,288), (3,292), (3,295), (3,299), (3,302), (3,306), (3,309), (3,313), (3,316), (3,320),
        (3,323), (3,326), (3,329), (3,333), (3,336), (4,0), (4,3), (4,7), (4,10), (4,14), (4,17),
        (4,21), (4,24), (4,28), (4,31), (4,35), (4,38), (4,42), (4,45), (4,49), (4,52), (4,56),
        (4,59), (4,63), (4,66), (4,70), (4,73), (4,77), (4,80), (4,84), (4,87), (4,91), (4,94),
        (4,98), (4,101), (4,105), (4,108), (4,112), (4,115), (4,119), (4,122), (4,126), (4,129),
        (4,133), (4,146), (4,150), (4,153), (4,157), (4,160), (4,164), (4,167), (4,171), (4,174),
        (4,178), (4,181), (4,185), (4,188), (4,192), (4,195), (4,199), (4,202), (4,206), (4,209),
        (4,213), (4,216), (4,220), (4,223), (4,227), (4,230), (4,234), (4,237), (4,241), (4,244),
        (4,248), (4,251), (4,255), (4,258), (4,262), (4,265), (4,269), (4,272), (4,276), (4,279),
        (4,283), (4,286), (4,290), (4,293), (4,297), (4,300), (4,304), (4,307), (4,311), (4,314),
        (4,318), (4,321), (4,324), (4,327), (4,331), (4,334), (4,338), (5,1), (5,5), (5,8), (5,12),
        (5,15), (5,19), (5,22), (5,26), (5,29), (5,33), (5,36), (5,40), (5,43), (5,47), (5,50),
        (5,54), (5,57), (5,61), (5,64), (5,68), (5,71), (5,75), (5,78), (5,82), (5,85), (5,89),
        (5,92), (5,96), (5,99), (5,103), (5,106), (5,110), (5,113), (5,117), (5,120), (5,124),
        (5,127), (5,131), (5,144), (5,148),
    ];

    #[test]
    fn test_ppu_timer() {
        let mut timer = PpuTimer::default();
        timer.advance_master_clock(186);
        for (v, h) in V_H_REFERENCE_LOG {
            assert_eq!(timer.v, *v);
            assert_eq!(timer.hdot(), *h);
            timer.advance_master_clock(14);
        }
    }
}
