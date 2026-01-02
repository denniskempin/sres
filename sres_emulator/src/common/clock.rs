use bitcode::Decode;
use bitcode::Encode;

#[derive(Default, Clone, Copy, PartialEq, Debug, Encode, Decode)]
pub struct ClockInfo {
    pub master_clock: u64,
    pub v: u64,
    pub h_counter: u64,
    pub f: u64,
}

impl ClockInfo {
    pub fn from_vhf(v: u64, h_counter: u64, f: u64) -> Self {
        // Frame boundary is at v=225 (vblank start)
        // Frame 0: v=0-224 only (active display before first vblank)
        // Frame n (n>=1): v=225-261 (vblank) + v=0-224 (active)
        //
        // Short scanline at v=240:
        // - Odd frames (1,3,5,...) have vblank from original even frames: NO short scanline
        // - Even frames (2,4,6,...) have vblank from original odd frames: HAS short scanline

        if f == 0 {
            // Frame 0: only active display v=0-224
            let master_clock = v * 1364 + h_counter;
            return ClockInfo {
                master_clock,
                v,
                h_counter,
                f,
            };
        }

        // Calculate cycles before frame f starts
        // Frame 0: 225 * 1364 = 306900 cycles
        // Frame 1: 262 * 1364 = 357368 cycles (no short scanline)
        // Frame 2: 262 * 1364 - 4 = 357364 cycles (has short scanline)
        // Pattern repeats: odd frames 357368, even frames 357364
        let frame0_length: u64 = 225 * 1364;
        let pair_length: u64 = 357368 + 357364; // 714732

        let f_minus_1 = f - 1;
        let pairs = f_minus_1 / 2;
        let mut f_cycles = frame0_length + pairs * pair_length;
        if f_minus_1 % 2 == 1 {
            // f is even (2,4,6,...), add frame for odd f-1
            f_cycles += 357368;
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

        let master_clock = f_cycles + v_cycles + h_counter;

        ClockInfo {
            master_clock,
            v,
            h_counter,
            f,
        }
    }

    pub fn from_master_clock(master_clock: u64) -> Self {
        // Frame boundary is at v=225 (vblank start)
        // Frame 0: 225 * 1364 = 306900 cycles
        // Frame 1+: 262 scanlines each
        //   - Odd frames (1,3,5): 357368 cycles (no short scanline)
        //   - Even frames (2,4,6): 357364 cycles (has short scanline)

        let frame0_length: u64 = 225 * 1364; // 306900

        if master_clock < frame0_length {
            // In frame 0 (active display only)
            let v = master_clock / 1364;
            let h_counter = master_clock % 1364;
            return ClockInfo {
                master_clock,
                v,
                h_counter,
                f: 0,
            };
        }

        // Past frame 0, find which subsequent frame
        let after_frame0 = master_clock - frame0_length;
        let pair_length: u64 = 357368 + 357364; // 714732

        let pairs = after_frame0 / pair_length;
        let pair_remainder = after_frame0 % pair_length;

        let (f, f_remainder) = if pair_remainder < 357368 {
            // In odd frame of the pair (1, 3, 5, ...)
            (pairs * 2 + 1, pair_remainder)
        } else {
            // In even frame of the pair (2, 4, 6, ...)
            (pairs * 2 + 2, pair_remainder - 357368)
        };

        let has_short_scanline = f % 2 == 0;
        let vblank_length: u64 = if has_short_scanline {
            37 * 1364 - 4 // 50464
        } else {
            37 * 1364 // 50468
        };

        if f_remainder < vblank_length {
            // In vblank portion (v=225-261)
            // Handle short scanline at v=240 (15th scanline of vblank)
            let (v, h_counter) = if has_short_scanline && f_remainder >= 15 * 1364 + 1360 {
                // After short scanline at v=240
                let adjusted = f_remainder + 4;
                (225 + adjusted / 1364, adjusted % 1364)
            } else {
                (225 + f_remainder / 1364, f_remainder % 1364)
            };
            ClockInfo {
                master_clock,
                v,
                h_counter,
                f,
            }
        } else {
            // In active portion (v=0-224)
            let active_offset = f_remainder - vblank_length;
            let v = active_offset / 1364;
            let h_counter = active_offset % 1364;
            ClockInfo {
                master_clock,
                v,
                h_counter,
                f,
            }
        }
    }

    pub fn hdot(&self) -> u64 {
        let mut counter = self.h_counter;
        // Short scanline at v=240 now occurs on even frames (2,4,6,...) in vblank portion.
        // On non-short scanlines, dots 323 and 327 take 6 cycles instead of 4.
        // (f=0 has no v=240, odd frames have no short scanline in their vblank)
        if self.f % 2 == 1 || self.v != 240 {
            // Non-short scanline: Dot 323 and 327 take 6 cycles.
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
}

#[cfg(test)]
mod test {
    use crate::common::clock::ClockInfo;

    #[test]
    fn test_vhf_master_clock_conversion() {
        for master_clock in 0..=10000000 {
            let clock = ClockInfo::from_master_clock(master_clock);
            let from_vhf = ClockInfo::from_vhf(clock.v, clock.h_counter, clock.f);
            assert_eq!(master_clock, from_vhf.master_clock);
        }
    }

    #[test]
    fn test_frame_boundary_positions() {
        // Test specific positions around frame boundaries
        // Current behavior: frame increments when v resets to 0

        // Frame 0, start
        let clock = ClockInfo::from_vhf(0, 0, 0);
        assert_eq!(clock.master_clock, 0);
        assert_eq!(clock.f, 0);

        // Frame 0, at v=224 (just before vblank)
        let clock = ClockInfo::from_vhf(224, 0, 0);
        assert_eq!(clock.f, 0);
        assert_eq!(clock.v, 224);

        // Frame 0, at v=225 (vblank start)
        let clock = ClockInfo::from_vhf(225, 0, 0);
        assert_eq!(clock.f, 0);
        assert_eq!(clock.v, 225);

        // Frame 0, at v=261 (last scanline of frame 0)
        let clock = ClockInfo::from_vhf(261, 0, 0);
        assert_eq!(clock.f, 0);
        assert_eq!(clock.v, 261);

        // Frame 0, end of last scanline
        let clock = ClockInfo::from_vhf(261, 1363, 0);
        assert_eq!(clock.f, 0);

        // Frame 1, start (v=0, h=0, f=1)
        let clock = ClockInfo::from_vhf(0, 0, 1);
        assert_eq!(clock.f, 1);
        assert_eq!(clock.v, 0);
    }

    #[test]
    fn test_master_clock_to_frame_boundary() {
        // Verify frame transitions via master_clock conversion
        // Frame 0: 306900 cycles (225 scanlines of active display)
        // Frame 1: 357368 cycles (no short scanline in vblank)
        // Frame 2: 357364 cycles (has short scanline in vblank)
        let frame0_length = 225 * 1364; // 306900

        // Last cycle of frame 0
        let clock = ClockInfo::from_master_clock(frame0_length - 1);
        assert_eq!(clock.f, 0);
        assert_eq!(clock.v, 224);

        // First cycle of frame 1 (vblank start)
        let clock = ClockInfo::from_master_clock(frame0_length);
        assert_eq!(clock.f, 1);
        assert_eq!(clock.v, 225);
        assert_eq!(clock.h_counter, 0);

        // Last cycle of frame 1
        let clock = ClockInfo::from_master_clock(frame0_length + 357368 - 1);
        assert_eq!(clock.f, 1);

        // First cycle of frame 2
        let clock = ClockInfo::from_master_clock(frame0_length + 357368);
        assert_eq!(clock.f, 2);
        assert_eq!(clock.v, 225);
        assert_eq!(clock.h_counter, 0);

        // Last cycle of frame 2
        let clock = ClockInfo::from_master_clock(frame0_length + 357368 + 357364 - 1);
        assert_eq!(clock.f, 2);

        // First cycle of frame 3
        let clock = ClockInfo::from_master_clock(frame0_length + 357368 + 357364);
        assert_eq!(clock.f, 3);
        assert_eq!(clock.v, 225);
        assert_eq!(clock.h_counter, 0);
    }

    #[test]
    fn test_vblank_detection() {
        // vblank starts at v=225
        let clock = ClockInfo::from_vhf(224, 0, 0);
        assert!(!clock.vblank());

        let clock = ClockInfo::from_vhf(225, 0, 0);
        assert!(clock.vblank());

        let clock = ClockInfo::from_vhf(261, 1363, 0);
        assert!(clock.vblank());

        // New frame starts, vblank ends
        let clock = ClockInfo::from_vhf(0, 0, 1);
        assert!(!clock.vblank());
    }

    // Tests for new frame boundary at v=225 (vblank start)
    // After changes, these should pass:

    #[test]
    fn test_new_frame_boundary_at_vblank() {
        // Frame 0 includes only active display: v=0-224
        // Frame increments when v reaches 225

        // Last cycle of frame 0 is v=224, h=1363
        let end_of_frame0 = ClockInfo::from_vhf(224, 1363, 0);
        assert_eq!(end_of_frame0.f, 0);
        assert_eq!(end_of_frame0.master_clock, 224 * 1364 + 1363);

        // First cycle of frame 1 is v=225, h=0
        let start_of_frame1 = ClockInfo::from_vhf(225, 0, 1);
        assert_eq!(start_of_frame1.f, 1);
        // Should be exactly one cycle after end of frame 0
        assert_eq!(start_of_frame1.master_clock, end_of_frame0.master_clock + 1);

        // Frame 1 continues through vblank (v=225-261) then active (v=0-224)
        let frame1_vblank_end = ClockInfo::from_vhf(261, 1363, 1);
        assert_eq!(frame1_vblank_end.f, 1);

        let frame1_active_start = ClockInfo::from_vhf(0, 0, 1);
        assert_eq!(frame1_active_start.f, 1);
        // Should be exactly one cycle after vblank end
        assert_eq!(frame1_active_start.master_clock, frame1_vblank_end.master_clock + 1);

        let frame1_active_end = ClockInfo::from_vhf(224, 1363, 1);
        assert_eq!(frame1_active_end.f, 1);

        // Frame 2 starts at v=225
        let start_of_frame2 = ClockInfo::from_vhf(225, 0, 2);
        assert_eq!(start_of_frame2.f, 2);
        assert_eq!(start_of_frame2.master_clock, frame1_active_end.master_clock + 1);
    }

    #[test]
    fn test_master_clock_to_new_frame_boundary() {
        // Frame 0 is 225 scanlines × 1364 cycles = 306900 cycles
        let frame0_length = 225 * 1364;

        // Last cycle of frame 0
        let clock = ClockInfo::from_master_clock(frame0_length - 1);
        assert_eq!(clock.f, 0);
        assert_eq!(clock.v, 224);
        assert_eq!(clock.h_counter, 1363);

        // First cycle of frame 1 (vblank start)
        let clock = ClockInfo::from_master_clock(frame0_length);
        assert_eq!(clock.f, 1);
        assert_eq!(clock.v, 225);
        assert_eq!(clock.h_counter, 0);

        // Frame 1 is 262 scanlines (37 vblank + 225 active)
        // Frame 1's vblank comes from original even frame, no short scanline
        // Frame 1 = 37 * 1364 + 225 * 1364 = 262 * 1364 = 357368 cycles
        let frame1_length = 262 * 1364;

        // Last cycle of frame 1
        let clock = ClockInfo::from_master_clock(frame0_length + frame1_length - 1);
        assert_eq!(clock.f, 1);
        assert_eq!(clock.v, 224);
        assert_eq!(clock.h_counter, 1363);

        // First cycle of frame 2
        let clock = ClockInfo::from_master_clock(frame0_length + frame1_length);
        assert_eq!(clock.f, 2);
        assert_eq!(clock.v, 225);
        assert_eq!(clock.h_counter, 0);
    }

    #[test]
    fn test_short_scanline_with_new_frame_boundary() {
        // The short scanline (v=240) alternates between original even/odd frames
        // In new scheme:
        // - Frame 1's vblank is from original frame 0 (even): NO short scanline
        // - Frame 2's vblank is from original frame 1 (odd): HAS short scanline at v=240

        // Frame 2's vblank should have short scanline
        // v=240 in frame 2 should have 1360 cycles instead of 1364
        let v240_start = ClockInfo::from_vhf(240, 0, 2);
        let v241_start = ClockInfo::from_vhf(241, 0, 2);
        // Short scanline at v=240 is 1360 cycles
        assert_eq!(v241_start.master_clock - v240_start.master_clock, 1360);

        // Frame 1's v=240 should be normal length
        let v240_start = ClockInfo::from_vhf(240, 0, 1);
        let v241_start = ClockInfo::from_vhf(241, 0, 1);
        assert_eq!(v241_start.master_clock - v240_start.master_clock, 1364);

        // Frame 3's v=240 should be normal (like frame 1)
        let v240_start = ClockInfo::from_vhf(240, 0, 3);
        let v241_start = ClockInfo::from_vhf(241, 0, 3);
        assert_eq!(v241_start.master_clock - v240_start.master_clock, 1364);

        // Frame 4's v=240 should be short (like frame 2)
        let v240_start = ClockInfo::from_vhf(240, 0, 4);
        let v241_start = ClockInfo::from_vhf(241, 0, 4);
        assert_eq!(v241_start.master_clock - v240_start.master_clock, 1360);
    }
}
