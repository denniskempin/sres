pub mod bus;
pub mod cartridge;
pub mod cpu;
pub mod dma;
pub mod logging;
pub mod memory;
pub mod ppu;
pub mod timer;
pub mod trace;
pub mod uint;

use anyhow::Result;

use bus::SresBus;
use cpu::Cpu;

pub struct System {
    pub cpu: Cpu<SresBus>,
}

impl System {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        System {
            cpu: Cpu::new(SresBus::default()),
        }
    }

    #[allow(clippy::new_without_default)]
    pub fn with_sfc_bytes(sfc_data: &[u8]) -> Result<Self> {
        Ok(System {
            cpu: Cpu::new(SresBus::with_sfc_data(sfc_data)?),
        })
    }

    pub fn execute_until<F>(&mut self, should_break: F)
    where
        F: Fn(&Cpu<SresBus>) -> bool,
    {
        loop {
            if self.cpu.halt {
                return;
            }

            self.cpu.step();

            if should_break(&self.cpu) {
                return;
            }
        }
    }

    pub fn execute_until_halt(&mut self) {
        self.execute_until(|cpu| cpu.halt)
    }

    pub fn execute_one_frame(&mut self) {
        let current_frame = self.cpu.bus.ppu_timer.f;
        self.execute_until(|cpu| cpu.bus.ppu_timer.f > current_frame)
    }

    pub fn execute_for_duration(&mut self, _seconds: f64) {
        // TODO: Implement frame skip/doubling if not running at 60fps
        self.execute_one_frame()
    }
}
