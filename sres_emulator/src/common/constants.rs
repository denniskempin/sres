#[derive(Clone, Copy, PartialEq, Debug, strum::Display, strum::EnumString)]
pub enum NativeVectorTable {
    #[strum(serialize = "cop")]
    Cop = 0xFFE4,
    #[strum(serialize = "break", serialize = "brk")]
    Break = 0xFFE6,
    #[strum(serialize = "nmi")]
    Nmi = 0xFFEA,
    #[strum(serialize = "irq")]
    Irq = 0xFFEE,
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct ClockInfo {
    pub master_clock: u64,
    pub v: u64,
    pub h_counter: u64,
    pub f: u64,
}

impl ClockInfo {
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
