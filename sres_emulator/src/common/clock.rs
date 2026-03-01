use bitcode::Decode;
use bitcode::Encode;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug, Encode, Decode)]
pub struct ClockInfo {
    pub master_clock: u64,
    pub v: u64,
    pub h_counter: u64,
    pub f: u64,
}

impl ClockInfo {
    pub fn from_master_clock(master_clock: u64) -> Self {
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

        ClockInfo {
            master_clock,
            v,
            h_counter,
            f,
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

    pub fn vblank(&self) -> bool {
        self.v >= 225
    }

    // Mesen increments the frame number at the start of vblank (v=225), which complicates the
    // logic to determine which master clock we are at.
    pub fn from_mesen_vhf(v: u64, h_counter: u64, f: u64) -> ClockInfo {
        // Short scanline at v=240:
        // - Odd frames (1,3,5,...) have vblank from original even frames: NO short scanline
        // - Even frames (2,4,6,...) have vblank from original odd frames: HAS short scanline

        if f == 0 {
            // Frame 0: only active display v=0-224
            return ClockInfo::from_master_clock(v * 1364 + h_counter);
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
        ClockInfo::from_master_clock(f_cycles + v_cycles + h_counter)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::clock::ClockInfo;

    #[test]
    fn test_mesen_vhf_to_master_clock() {
        // Frame 0, start
        assert_eq!(ClockInfo::from_mesen_vhf(0, 0, 0).master_clock, 0);

        // Frame 0, h=194. First cycle after bootup.
        assert_eq!(ClockInfo::from_mesen_vhf(0, 194, 0).master_clock, 194);

        // Frame 0, at v=224 (just before vblank)
        assert_eq!(ClockInfo::from_mesen_vhf(224, 0, 0).master_clock, 305536);

        // Frame 0, at v=225 (vblank start). In mesen, this will be frame 1.
        assert_eq!(ClockInfo::from_mesen_vhf(225, 0, 1).master_clock, 306900);

        // Frame 0, at v=261 (last scanline of frame 0)
        assert_eq!(ClockInfo::from_mesen_vhf(261, 0, 1).master_clock, 356004);

        // Frame 0, end of last scanline
        assert_eq!(ClockInfo::from_mesen_vhf(261, 1363, 1).master_clock, 357367);

        // Frame 1, start (v=0, h=0, f=1)
        assert_eq!(ClockInfo::from_mesen_vhf(0, 0, 1).master_clock, 357368);
    }
}
