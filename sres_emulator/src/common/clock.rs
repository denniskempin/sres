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

        let master_clock = f_cycles + v_cycles + h_counter;

        ClockInfo {
            master_clock,
            v,
            h_counter,
            f,
        }
    }

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
}
