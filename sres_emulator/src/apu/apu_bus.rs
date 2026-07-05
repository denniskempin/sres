use intbits::Bits;
use log::trace;

use super::timers::ApuTimers;
use crate::common::address::AddressU16;
use crate::common::bus::Bus;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::components::s_dsp::SDsp;
use crate::components::spc700::Spc700Bus;

// Match Mesen2's SPC clock calibration (see Spc700::catch_up_to_master_clock).
const SPC_CLOCK_FREQUENCY: u64 = 32040 * 64;
const MASTER_CLOCK_FREQUENCY: u64 = 21_477_270;
const MASTER_TO_SPC_RATIO: f64 = SPC_CLOCK_FREQUENCY as f64 / MASTER_CLOCK_FREQUENCY as f64;

#[derive(Clone, Debug, PartialEq)]
pub enum ApuBusEvent {
    Read(AddressU16, u8),
    Write(AddressU16, u8),
}

/// Buffered SPC→CPU port write, promoted once the master clock reaches the write cycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ChannelOutPending {
    write_spc_cycle: u64,
    value: u8,
}

pub struct ApuBus {
    pub debug_event_collector: DebugEventCollectorRef<ApuBusEvent>,
    pub spc_cycle: u64,
    pub master_clock: u64,
    pub ram: [u8; 0x10000],
    pub channel_in: [u8; 4],
    pub channel_out: [u8; 4],
    channel_out_pending: [Option<ChannelOutPending>; 4],
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
            channel_out_pending: [None; 4],
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
            self.channel_out[0] = 0;
            self.channel_out[2] = 0;
            self.channel_out_pending[0] = None;
            self.channel_out_pending[2] = None;
        }
        if self.control.clear_apuio34() {
            self.channel_in[2] = 0;
            self.channel_in[3] = 0;
        }
    }

    fn write_channel_out(&mut self, channel: usize, value: u8) {
        let write_spc_cycle = self.spc_cycle;
        if self.master_clock == 0 {
            // SPC-only unit tests step without a master clock.
            self.channel_out[channel] = value;
            self.channel_out_pending[channel] = None;
            return;
        }
        self.channel_out_pending[channel] = Some(ChannelOutPending {
            write_spc_cycle,
            value,
        });
    }

    /// Promote buffered SPC port writes to CPU-visible `channel_out` once the
    /// master clock has reached the write's SPC cycle.
    pub fn promote_channel_out_writes(&mut self) {
        if self.master_clock == 0 {
            return;
        }
        let exposed_spc_cycle =
            (self.master_clock as f64 * MASTER_TO_SPC_RATIO).floor() as u64;

        for channel in 0..4 {
            let Some(pending) = self.channel_out_pending[channel] else {
                continue;
            };
            if pending.write_spc_cycle <= exposed_spc_cycle {
                self.channel_out[channel] = pending.value;
                self.channel_out_pending[channel] = None;
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

        let channel_out_write = matches!(addr.0, 0x00F4..=0x00F7);
        if channel_out_write {
            let channel = addr.0 as usize - 0x00F4;
            self.write_channel_out(channel, value);
        }

        self.spc_cycle += 2;

        if !channel_out_write {
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
                0x00FA..=0x00FC => {
                    let timer_id = addr.0 as usize - 0x00FA;
                    self.timers.write_target(timer_id, value);
                }
                0x00FD..=0x00FF => {} // Timer outputs are read-only
                _ => self.ram[addr.0 as usize] = value,
            }
        }

        // Update timers with 1 SPC cycle
        self.timers.update(1);
    }

    fn reset(&mut self) {
        self.control = ApuControlRegister::default();
        self.timers.reset();
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

    fn promote_channel_out_writes(&mut self) {
        ApuBus::promote_channel_out_writes(self);
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
