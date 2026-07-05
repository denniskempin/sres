use std::collections::VecDeque;

use intbits::Bits;
use log::trace;

use super::timers::ApuTimers;
use crate::common::address::AddressU16;
use crate::common::bus::Bus;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::components::s_dsp::SDsp;
use crate::components::spc700::Spc700Bus;

#[derive(Clone, Debug, PartialEq)]
pub enum ApuBusEvent {
    Read(AddressU16, u8),
    Write(AddressU16, u8),
}

pub struct ApuBus {
    pub debug_event_collector: DebugEventCollectorRef<ApuBusEvent>,
    pub spc_cycle: u64,
    pub master_clock: u64,
    pub ram: [u8; 0x10000],
    pub channel_in: [u8; 4],
    pub channel_out: [u8; 4],
    /// SPC700 writes to the CPUIO out ports (`channel_out`) must not become visible to the S-CPU
    /// before the master clock actually reaches the SPC cycle on which the write happens. Because
    /// the lazy catch-up executes whole SPC instructions atomically, a write that hardware performs
    /// partway through an instruction would otherwise be observable at the instruction's *first*
    /// cycle (up to ~1 instruction too early). Each entry is `(channel, write_spc_cycle, value)`;
    /// entries are promoted into `channel_out` by `promote_channel_out` once the exposed SPC cycle
    /// (derived from the master clock) catches up. See SRE-24.
    channel_out_pending: VecDeque<(usize, u64, u8)>,
    pub timers: ApuTimers,
    pub dsp_register_select: u8,
    pub dsp_register_readonly: bool,
    pub dsp: SDsp,
    pub control: ApuControlRegister,
}

impl ApuBus {
    #[allow(clippy::new_without_default)]
    pub fn new(debug_event_collector: DebugEventCollectorRef<ApuBusEvent>) -> Self {
        Self {
            debug_event_collector: debug_event_collector.clone(),
            spc_cycle: 6,
            master_clock: 0,
            ram: [0; 0x10000],
            channel_in: [0; 4],
            channel_out: [0; 4],
            channel_out_pending: VecDeque::new(),
            timers: ApuTimers::new(),
            dsp_register_readonly: false,
            dsp_register_select: 0,
            dsp: Default::default(),
            control: ApuControlRegister::default(),
        }
    }

    fn write_control(&mut self, value: u8) {
        self.timers.update_timer_enable_flags(value.bits(0..2));
        self.control.0 = value;
        // Wiki is a little unclear on the exact behavior. It seems that for 1/2
        // both channels are cleared. For 3/4 only the input channel
        // See https://snes.nesdev.org/wiki/S-SMP#CONTROL_-_Control_register_($F1,_write-only)
        if self.control.clear_apuio12() {
            self.channel_in[0] = 0;
            self.channel_in[1] = 0;
            self.clear_channel_out(0);
            self.clear_channel_out(2);
        }
        if self.control.clear_apuio34() {
            self.channel_in[2] = 0;
            self.channel_in[3] = 0;
        }
    }

    /// Clear a `channel_out` port and drop any not-yet-visible pending writes for it, so a stale
    /// deferred write cannot resurrect the port after a control-register clear.
    fn clear_channel_out(&mut self, channel: usize) {
        self.channel_out[channel] = 0;
        self.channel_out_pending.retain(|&(ch, _, _)| ch != channel);
    }

    /// Promote deferred `channel_out` writes whose SPC write cycle is at or before the given
    /// exposed SPC cycle (derived from the current master clock). See `channel_out_pending`.
    pub fn promote_channel_out(&mut self, exposed_spc_cycle: u64) {
        while let Some(&(channel, write_cycle, value)) = self.channel_out_pending.front() {
            if write_cycle <= exposed_spc_cycle {
                self.channel_out[channel] = value;
                self.channel_out_pending.pop_front();
            } else {
                break;
            }
        }
    }
}

impl Bus<AddressU16> for ApuBus {
    fn peek_u8(&self, addr: AddressU16) -> Option<u8> {
        match addr.0 {
            0x00F1 => Some(self.control.0),
            0x00F2 => Some(self.dsp_register_select.bits(0..=6)),
            0x00F3 => Some(self.dsp.read_register(self.dsp_register_select)),
            0x00F4..=0x00F7 => Some(self.channel_in[addr.0 as usize - 0x00F4]),
            0x00FA..=0x00FC => Some(0), // Timer targets are write-only
            0x00FD..=0x00FF => Some(self.timers.peek_output(addr.0 as usize - 0x00FD)),
            0xFFC0..=0xFFFF => {
                if self.control.ipl_rom_enabled() {
                    Some(IPL_BOOT_ROM[(addr.0 - 0xFFC0) as usize])
                } else {
                    Some(self.ram[addr.0 as usize])
                }
            }
            _ => Some(self.ram[addr.0 as usize]),
        }
    }

    fn cycle_io(&mut self) {
        trace!("{:08} [SPC] io", self.master_clock);
        self.spc_cycle += 2;
        // Update timers with 1 SPC cycle
        self.timers.update(1);
    }

    fn cycle_read_u8(&mut self, addr: AddressU16) -> u8 {
        trace!("{:08} [SPC] read {addr}", self.master_clock);
        self.spc_cycle += 2;

        // Handle timer output reads specially (they reset on read)
        let value = match addr.0 {
            0x00FD..=0x00FF => {
                let timer_id = addr.0 as usize - 0x00FD;
                self.timers.read_output(timer_id)
            }
            _ => self.peek_u8(addr).unwrap_or_default(),
        };

        self.debug_event_collector
            .on_event(ApuBusEvent::Read(addr, value));

        // Update timers with 1 SPC cycle
        self.timers.update(1);

        value
    }

    fn cycle_write_u8(&mut self, addr: AddressU16, value: u8) {
        self.debug_event_collector
            .on_event(ApuBusEvent::Write(addr, value));
        trace!("{:08} [SPC] write {addr:}", self.master_clock);

        // SPC cycle at the start of this write bus-cycle; the CPUIO out ports latch (and become
        // observable to the S-CPU) from the beginning of the write cycle, not after it completes.
        let write_cycle = self.spc_cycle;
        self.spc_cycle += 2;

        match addr.0 {
            0x00F1 => self.write_control(value),
            0x00F2 => {
                self.dsp_register_readonly = value.bit(7);
                self.dsp_register_select = value.bits(0..=6);
            }
            0x00F3 => {
                if self.dsp_register_readonly {
                    return;
                }
                self.dsp.write_register(self.dsp_register_select, value);
            }
            0x00F4..=0x00F7 => {
                // Defer visibility to the S-CPU until the master clock reaches this write's SPC
                // cycle (see `channel_out_pending` and `promote_channel_out`).
                let channel = addr.0 as usize - 0x00F4;
                self.channel_out_pending
                    .push_back((channel, write_cycle, value));
            }
            0x00FA..=0x00FC => {
                let timer_id = addr.0 as usize - 0x00FA;
                self.timers.write_target(timer_id, value);
            }
            0x00FD..=0x00FF => {} // Timer outputs are read-only
            _ => self.ram[addr.0 as usize] = value,
        }

        // Update timers with 1 SPC cycle
        self.timers.update(1);
    }

    fn reset(&mut self) {
        self.control = ApuControlRegister::default();
        self.timers.reset();
        self.channel_out_pending.clear();
    }
}

impl Spc700Bus for ApuBus {
    fn spc_cycle(&self) -> u64 {
        self.spc_cycle
    }
    fn master_clock(&self) -> u64 {
        self.master_clock
    }
    fn update_master_clock(&mut self, new_master_clock: u64) {
        self.master_clock = new_master_clock;
    }
    fn promote_channel_out_writes(&mut self, exposed_spc_cycle: u64) {
        self.promote_channel_out(exposed_spc_cycle);
    }
}

pub struct ApuControlRegister(pub u8);

impl ApuControlRegister {
    pub fn ipl_rom_enabled(&self) -> bool {
        self.0.bit(7)
    }

    pub fn clear_apuio12(&self) -> bool {
        self.0.bit(4)
    }

    pub fn clear_apuio34(&self) -> bool {
        self.0.bit(5)
    }
}

impl Default for ApuControlRegister {
    fn default() -> Self {
        Self(0xB0)
    }
}

/// See https://github.com/gilligan/snesdev/blob/master/docs/spc700.txt
const IPL_BOOT_ROM: [u8; 64] = [
    0xCD, 0xEF, 0xBD, 0xE8, 0x00, 0xC6, 0x1D, 0xD0, 0xFC, 0x8F, 0xAA, 0xF4, 0x8F, 0xBB, 0xF5, 0x78,
    0xCC, 0xF4, 0xD0, 0xFB, 0x2F, 0x19, 0xEB, 0xF4, 0xD0, 0xFC, 0x7E, 0xF4, 0xD0, 0x0B, 0xE4, 0xF5,
    0xCB, 0xF4, 0xD7, 0x00, 0xFC, 0xD0, 0xF3, 0xAB, 0x01, 0x10, 0xEF, 0x7E, 0xF4, 0x10, 0xEB, 0xBA,
    0xF6, 0xDA, 0x00, 0xBA, 0xF4, 0xC4, 0xF4, 0xDD, 0x5D, 0xD0, 0xDB, 0x1F, 0x00, 0x00, 0xC0, 0xFF,
];
