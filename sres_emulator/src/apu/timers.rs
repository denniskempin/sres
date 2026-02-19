use crate::common::uint::UInt;

#[derive(Debug, Clone)]
pub struct ApuTimer {
    // Stage 1: Base frequency divider (128 cycles for T0/T1, 16 cycles for T2)
    base_counter: u32,
    base_divisor: u32, // 128 for T0/T1, 16 for T2

    // Stage 2: User-configurable divider (0-255, where 0 = 256)
    interval_counter: u8,
    interval_target: u8,

    // Stage 3: 4-bit output counter
    output_counter: u8, // Only uses lower 4 bits

    // Control state
    enabled: bool,
}

impl ApuTimer {
    pub fn new(base_divisor: u32) -> Self {
        Self {
            base_counter: 0,
            base_divisor,
            interval_counter: 0,
            interval_target: 0,
            output_counter: 0xF, // Power-on state
            enabled: false,
        }
    }

    pub fn reset(&mut self) {
        self.base_counter = 0;
        self.interval_counter = 0;
        self.output_counter = 0; // Reset state
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        if !self.enabled {
            // 0->1 transition resets internal counter and output to 0
            self.interval_counter = 0;
            self.output_counter = 0;
        }
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn set_target(&mut self, target: u8) {
        self.interval_target = target;
    }

    pub fn read_output(&mut self) -> u8 {
        let value = self.output_counter & 0x0F; // Only lower 4 bits
        self.output_counter = 0; // Reset on read
        value
    }

    pub fn peek_output(&self) -> u8 {
        self.output_counter & 0x0F // Only lower 4 bits
    }

    /// Update timer with SPC700 cycles (not master cycles)
    pub fn update(&mut self, spc_cycles: u32) {
        // Stage 1: Base frequency divider runs continuously
        self.base_counter += spc_cycles;

        while self.base_counter >= self.base_divisor {
            self.base_counter -= self.base_divisor;

            // Stage 2: Only count if enabled
            if self.enabled {
                self.interval_counter = self.interval_counter.wrapping_add(1);

                // Check if interval matches target (0 means 256)
                let target = if self.interval_target == 0 {
                    256
                } else {
                    self.interval_target as u16
                };

                if self.interval_counter as u16 >= target {
                    self.interval_counter = 0;

                    // Stage 3: Increment 4-bit output counter
                    self.output_counter = (self.output_counter + 1) & 0x0F;
                }
            }
        }
    }
}

pub struct ApuTimers {
    timers: [ApuTimer; 3],
}

impl Default for ApuTimers {
    fn default() -> Self {
        Self::new()
    }
}

impl ApuTimers {
    pub fn new() -> Self {
        Self {
            timers: [
                ApuTimer::new(128), // Timer 0: 8kHz (128 SPC cycles)
                ApuTimer::new(128), // Timer 1: 8kHz (128 SPC cycles)
                ApuTimer::new(16),  // Timer 2: 64kHz (16 SPC cycles)
            ],
        }
    }

    pub fn reset(&mut self) {
        for timer in &mut self.timers {
            timer.reset();
        }
    }

    pub fn update_timer_enable_flags(&mut self, value: u8) {
        for (i, timer) in self.timers.iter_mut().enumerate() {
            let enable_timer = value.bit(i);
            if enable_timer == timer.enabled {
                continue;
            }

            if enable_timer {
                timer.enable();
            } else {
                timer.disable();
            }
        }
    }

    pub fn write_target(&mut self, timer_id: usize, value: u8) {
        if timer_id < 3 {
            self.timers[timer_id].set_target(value);
        }
    }

    pub fn read_output(&mut self, timer_id: usize) -> u8 {
        if timer_id < 3 {
            self.timers[timer_id].read_output()
        } else {
            0
        }
    }

    pub fn peek_output(&self, timer_id: usize) -> u8 {
        if timer_id < 3 {
            self.timers[timer_id].peek_output()
        } else {
            0
        }
    }

    /// Update all timers with SPC700 cycles
    pub fn update(&mut self, spc_cycles: u32) {
        for timer in &mut self.timers {
            timer.update(spc_cycles);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_basic_functionality() {
        let mut timers = ApuTimers::new();

        // Test initial state
        assert_eq!(timers.peek_output(0), 0xF); // Power-on state
        assert_eq!(timers.peek_output(1), 0xF);
        assert_eq!(timers.peek_output(2), 0xF);

        // Test reset
        timers.reset();
        assert_eq!(timers.peek_output(0), 0); // Reset state
        assert_eq!(timers.peek_output(1), 0);
        assert_eq!(timers.peek_output(2), 0);
    }

    #[test]
    fn test_timer_target_setting() {
        let mut timers = ApuTimers::new();

        // Set timer targets
        timers.write_target(0, 100);
        timers.write_target(1, 200);
        timers.write_target(2, 50);

        // Enable timers
        timers.update_timer_enable_flags(0x07); // Enable all 3 timers

        // Test updates don't crash
        timers.update(1000);
    }

    #[test]
    fn test_timer_output_read_reset() {
        let mut timers = ApuTimers::new();
        timers.reset();

        // Set a small target and enable timer 0
        timers.write_target(0, 1);
        timers.update_timer_enable_flags(0x01);

        // Run enough cycles to trigger timer
        timers.update(200); // Should trigger multiple times

        let first_read = timers.read_output(0);
        assert!(first_read > 0); // Should have incremented

        let second_read = timers.read_output(0);
        assert_eq!(second_read, 0); // Should be reset after read
    }

    #[test]
    fn test_timer_different_rates() {
        let mut timers = ApuTimers::new();
        timers.reset();

        // Set same target for all timers
        timers.write_target(0, 1);
        timers.write_target(1, 1);
        timers.write_target(2, 1);

        // Enable all timers
        timers.update_timer_enable_flags(0x07);

        // Run cycles - timer 2 should count faster (16 cycles vs 128)
        timers.update(128);

        let timer0_count = timers.peek_output(0);
        let timer1_count = timers.peek_output(1);
        let timer2_count = timers.peek_output(2);

        // Timer 2 should have counted more than timers 0/1
        assert!(timer2_count >= timer0_count);
        assert!(timer2_count >= timer1_count);
    }
}
