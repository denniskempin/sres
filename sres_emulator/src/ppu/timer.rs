//! Tracking of PPU events and timing.

use crate::common::address::AddressU24;
use crate::util::uint::U16Ext;
use crate::util::uint::UInt;
use crate::util::EdgeDetector;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PpuTimer {
    pub master_clock: u64,
    pub v: u64,
    pub h_counter: u64,
    pub f: u64,
    pub dram_refresh_position: u64,
    pub vblank_detector: EdgeDetector,
    pub hv_timer_detector: EdgeDetector,

    pub timer_flag: bool,
    pub timer_interrupt: bool,
    pub h_timer_target: u16,
    pub v_timer_target: u16,
    pub timer_mode: HVTimerMode,
}

impl PpuTimer {
    pub fn from_master_clock(master_clock: u64) -> Self {
        let (f, v, h) = master_clock_to_fvh(master_clock);

        Self {
            master_clock,
            v,
            h_counter: h,
            f,
            ..Default::default()
        }
    }

    pub fn bus_peek(&self, addr: AddressU24) -> Option<u8> {
        match addr.offset {
            0x4211 => self.peek_timeup(),
            0x4212 => self.peek_hvbjoy(),
            _ => unreachable!(),
        }
    }

    pub fn bus_read(&mut self, addr: AddressU24) -> u8 {
        match addr.offset {
            0x4211 => self.read_timeup(),
            0x4212 => self.read_hvbjoy(),
            _ => unreachable!(),
        }
    }

    pub fn bus_write(&mut self, addr: AddressU24, value: u8) {
        match addr.offset {
            0x4207 => self.write_htimel(value),
            0x4208 => self.write_htimeh(value),
            0x4209 => self.write_vtimel(value),
            0x420A => self.write_vtimeh(value),
            _ => unreachable!(),
        }
    }

    pub fn advance_master_clock(&mut self, mut master_cycles: u64) {
        // Do not advance clock too far at once so we do not miss events along the way.
        // Scientifically determined to be the highest value that keeps passing tests...
        while master_cycles > 64 {
            master_cycles -= 64;
            self.tick_master_clock(64);
        }
        self.tick_master_clock(master_cycles);
    }

    pub fn consume_timer_interrupt(&mut self) -> bool {
        let timer_interrupt = self.timer_interrupt;
        self.timer_interrupt = false;
        timer_interrupt
    }

    /// TIMEUP - Timer flag ($4211 read)
    /// 7  bit  0
    /// ---- ----
    /// Txxx xxxx
    /// |||| ||||
    /// |+++-++++- (Open bus)
    /// +--------- Timer flag
    ///
    /// On power-on: TIMEUP = TIMEUP & $7F
    /// On reset:    TIMEUP = TIMEUP & $7F
    /// On read:     TIMEUP = TIMEUP & $7F
    fn peek_timeup(&self) -> Option<u8> {
        Some(if self.timer_flag { 0x80 } else { 0 })
    }

    fn read_timeup(&mut self) -> u8 {
        let value = self.peek_timeup().unwrap();
        self.timer_flag = false;
        value
    }

    /// HVBJOY - H/V blank and joypad status ($4212 read)
    /// 7  bit  0
    /// ---- ----
    /// VHxx xxxJ
    /// |||| ||||
    /// |||| |||+- Joypad auto-read in-progress flag
    /// ||++-+++-- (Open bus)
    /// |+-------- Hblank flag
    /// +--------- Vblank flag
    fn peek_hvbjoy(&self) -> Option<u8> {
        let mut value: u8 = 0;
        if self.v > 225 {
            value.set_bit(7, true);
        }
        if self.hdot() > 274 {
            value.set_bit(6, true);
        }
        Some(value)
    }

    fn read_hvbjoy(&mut self) -> u8 {
        self.peek_hvbjoy().unwrap()
    }

    /// HTIMEL, HTIMEH - H timer target ($4207, $4208 write)
    ///   HTIMEH      HTIMEL
    ///   $4208       $4207
    /// 7  bit  0   7  bit  0
    /// ---- ----   ---- ----
    /// .... ...H   LLLL LLLL
    ///         |   |||| ||||
    ///         +---++++-++++- H counter target for timer IRQ
    ///
    /// On power-on: HTIME = $1FF
    fn write_htimel(&mut self, value: u8) {
        self.h_timer_target.set_low_byte(value);
    }

    fn write_htimeh(&mut self, value: u8) {
        self.h_timer_target.set_high_byte(value);
    }

    /// VTIMEL, VTIMEH - V timer target ($4209, $420A write)
    ///   VTIMEH      VTIMEL
    ///   $420A       $4209
    /// 7  bit  0   7  bit  0
    /// ---- ----   ---- ----
    /// .... ...H   LLLL LLLL
    ///         |   |||| ||||
    ///         +---++++-++++- V counter target for timer IRQ
    ///
    /// On power-on: VTIME = $1FF
    fn write_vtimel(&mut self, value: u8) {
        self.v_timer_target.set_low_byte(value);
    }

    fn write_vtimeh(&mut self, value: u8) {
        self.v_timer_target.set_high_byte(value);
    }

    fn tick_master_clock(&mut self, master_cycles: u64) {
        self.master_clock += master_cycles;
        self.h_counter += master_cycles;

        // ~536 master cycles after the start of a scanline, the CPU will pause for 40 cycles.
        // See: https://wiki.superfamicom.org/timing#clocks-and-refresh-10
        if ((self.h_counter - master_cycles)..=(self.h_counter))
            .contains(&self.dram_refresh_position)
        {
            self.h_counter += 40;
            self.master_clock += 40;
        }

        // Check timer early as well, in case the H target is at the end of the scanline and we
        // may jump over it when jumping to the next scanline.
        self.update_timer_detector();

        // Line 240 of each odd frame is 4 cycles shorter.
        // See: https://snes.nesdev.org/wiki/Timing#Short_and_Long_Scanlines
        let h_duration = if self.v == 240 && self.f % 2 == 1 {
            1360
        } else {
            1364
        };
        if self.h_counter >= h_duration {
            self.hv_timer_detector.update_signal(false);
            self.h_counter -= h_duration;
            self.v += 1;
            self.dram_refresh_position = 538 - ((self.master_clock - self.h_counter) & 7);
        }

        if self.v >= 262 {
            self.v -= 262;
            self.f += 1;
        }

        self.vblank_detector.update_signal(self.v >= 225);
        self.update_timer_detector();
    }

    fn update_timer_detector(&mut self) {
        if self.timer_mode == HVTimerMode::Off {
            return;
        }
        let h_hit = self.hdot() >= self.h_timer_target as u64;
        let v_hit = self.v >= self.v_timer_target as u64;

        self.hv_timer_detector.update_signal(match self.timer_mode {
            HVTimerMode::TriggerH => h_hit,
            HVTimerMode::TriggerV => v_hit,
            HVTimerMode::TriggerHV => h_hit && v_hit,
            HVTimerMode::Off => unreachable!(),
        });

        if self.hv_timer_detector.consume_rise() {
            self.timer_flag = true;
            self.timer_interrupt = true;
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
            vblank_detector: EdgeDetector::new(),
            hv_timer_detector: EdgeDetector::new(),
            timer_flag: false,
            timer_interrupt: false,
            h_timer_target: 0x1FF,
            v_timer_target: 0x1FF,
            timer_mode: HVTimerMode::Off,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::Display)]
pub enum HVTimerMode {
    Off,
    TriggerH,
    TriggerV,
    TriggerHV,
}

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
    fn test_hv_increments() {
        let mut timer = PpuTimer::default();
        timer.advance_master_clock(186);
        for (v, h) in V_H_REFERENCE_LOG {
            assert_eq!(timer.v, *v);
            assert_eq!(timer.hdot(), *h);
            timer.advance_master_clock(14);
        }
    }

    #[test]
    fn test_h_timer() {
        let mut timer = PpuTimer::default();
        // Enable timer on H=64
        timer.bus_write(0x4207.into(), 0x40);
        timer.bus_write(0x4208.into(), 0x00);
        timer.timer_mode = HVTimerMode::TriggerH;

        // H=63: All flags should be false
        timer.advance_master_clock(255);
        assert_eq!(timer.hdot(), 63);
        assert_eq!(timer.bus_read(0x4211.into()), 0x00);
        assert!(!timer.consume_timer_interrupt());

        // H=64: Timer should trigger
        timer.advance_master_clock(1);
        assert_eq!(timer.hdot(), 64);
        assert_eq!(timer.bus_read(0x4211.into()), 0x80);
        assert!(timer.consume_timer_interrupt());

        // Still H=64: Flags should remain false because they have been consumed
        timer.advance_master_clock(1);
        assert_eq!(timer.hdot(), 64);
        assert_eq!(timer.bus_read(0x4211.into()), 0x00);
        assert!(!timer.consume_timer_interrupt());

        // Next scanline H=64: Timer should trigger again
        timer.advance_master_clock(1324);
        assert_eq!(timer.hdot(), 64);
        assert_eq!(timer.bus_read(0x4211.into()), 0x80);
        assert!(timer.consume_timer_interrupt());
    }

    #[test]
    fn test_v_timer() {
        let mut timer = PpuTimer::default();
        // Enable timer on V=2
        timer.bus_write(0x4209.into(), 0x02);
        timer.bus_write(0x420A.into(), 0x00);
        timer.timer_mode = HVTimerMode::TriggerV;

        // V=1: All flags should be false
        timer.advance_master_clock(1324);
        assert_eq!(timer.v, 1);
        assert_eq!(timer.bus_read(0x4211.into()), 0x00);
        assert!(!timer.consume_timer_interrupt());

        // V=2: Timer should trigger
        timer.advance_master_clock(1324);
        assert_eq!(timer.v, 2);
        assert_eq!(timer.bus_read(0x4211.into()), 0x80);
        assert!(timer.consume_timer_interrupt());

        // Still V=2: Flags should remain false because they have been consumed
        timer.advance_master_clock(100);
        assert_eq!(timer.v, 2);
        assert_eq!(timer.bus_read(0x4211.into()), 0x00);
        assert!(!timer.consume_timer_interrupt());
    }
}
