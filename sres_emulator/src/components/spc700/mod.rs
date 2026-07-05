//! Implementation of the SPC700 CPU.
mod debug;
mod instructions;
mod opcode_table;
mod operands;
mod status;
mod test;

use log::info;

pub use self::debug::Spc700Debug;
pub use self::debug::Spc700Event;
pub use self::debug::Spc700State;
use self::opcode_table::InstructionDef;
use self::operands::AddressMode;
use self::operands::DecodedOperand;
use self::status::Spc700StatusFlags;
use crate::common::address::Address;
use crate::common::address::AddressU16;
use crate::common::address::Wrap;
use crate::common::bus::Bus;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::common::uint::UInt;

pub trait Spc700Bus: Bus<AddressU16> {
    fn spc_cycle(&self) -> u64;
    fn master_clock(&self) -> u64;
    fn update_master_clock(&mut self, cycles: u64);
    /// Make deferred CPUIO output-port writes visible up to the given exposed SPC cycle. Used to
    /// model the fact that a port write only becomes observable to the S-CPU once the master clock
    /// reaches the SPC cycle on which the (atomically executed) instruction performed the write.
    fn promote_channel_out_writes(&mut self, _exposed_spc_cycle: u64) {}
}

pub struct Spc700<BusT: Spc700Bus> {
    pub bus: BusT,
    debug_event_collector: DebugEventCollectorRef<Spc700Event>,
    opcode_table: [InstructionDef<BusT>; 256],
    pc: AddressU16,
    a: u8,
    y: u8,
    x: u8,
    sp: u8,
    status: Spc700StatusFlags,
}

impl<BusT: Spc700Bus> Spc700<BusT> {
    pub fn new(bus: BusT, debug_event_collector: DebugEventCollectorRef<Spc700Event>) -> Self {
        let mut cpu = Self {
            opcode_table: opcode_table::build_opcode_table(),
            bus,
            pc: AddressU16(0),
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            status: Spc700StatusFlags::default(),
            debug_event_collector,
        };
        cpu.reset();
        cpu
    }

    pub fn debug(&self) -> Spc700Debug<'_, BusT> {
        Spc700Debug(self)
    }

    pub fn reset(&mut self) {
        self.pc = AddressU16(0xFFC0);
        self.sp = 0xef;
        self.status.zero = true;
    }

    pub fn catch_up_to_master_clock(&mut self, master_cycles: u64) {
        self.bus.update_master_clock(master_cycles);
        // Match Mesen2's SPC clock calibration: the effective SPC sample rate is
        // 32040 Hz (32000 + the +40 SpcClockSpeedAdjustment default), so the SPC runs
        // at 32040 * 64 = 2,049,600 Hz, against a 21,477,270 Hz NTSC master clock.
        // See Mesen2 Spc::UpdateClockRatio / SnesConsole.cpp.
        const SPC_CLOCK_FREQUENCY: u64 = 32040 * 64;
        const MASTER_CLOCK_FREQUENCY: u64 = 21_477_270;
        let clock_ratio = SPC_CLOCK_FREQUENCY as f64 / MASTER_CLOCK_FREQUENCY as f64;
        let exposed_spc_cycle = (master_cycles as f64 * clock_ratio).floor() as u64;
        let target_spc_cycle = exposed_spc_cycle - 1;
        while self.bus.spc_cycle() < target_spc_cycle {
            self.step();
        }
        // Reveal any deferred CPUIO output-port writes whose SPC write cycle is now at or before
        // the SPC cycle exposed to the S-CPU by the current master clock. Because instructions
        // execute atomically, a write performed during an instruction is buffered until the master
        // clock actually reaches that cycle. See SRE-24.
        self.bus.promote_channel_out_writes(exposed_spc_cycle);
    }

    pub fn step(&mut self) {
        self.debug_event_collector
            .on_event(Spc700Event::Step(self.debug().state()));
        if log::log_enabled!(target: "spc700_step", log::Level::Info) {
            info!(target: "spc700_step", "{}", self.debug().state());
        }

        let opcode = self.bus.cycle_read_u8(self.pc);
        let instruction = &self.opcode_table[opcode as usize];
        (instruction.execute)(self);
    }

    fn update_negative_zero_flags<T: UInt>(&mut self, value: T) {
        self.status.negative = value.bit(T::N_BITS - 1);
        self.status.zero = value.is_zero();
    }

    fn stack_push_u8(&mut self, value: u8) {
        self.bus
            .cycle_write_u8(AddressU16::new_direct_page(1, self.sp), value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.stack_push_u8(bytes[1]);
        self.stack_push_u8(bytes[0]);
    }

    fn stack_pop_u8(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.bus
            .cycle_read_u8(AddressU16::new_direct_page(1, self.sp))
    }

    fn stack_pop_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.stack_pop_u8(), self.stack_pop_u8()])
    }

    fn direct_page_addr(&self, offset: u8) -> AddressU16 {
        AddressU16::new_direct_page(if self.status.direct_page { 1 } else { 0 }, offset)
    }

    fn fetch_program_u8(&mut self) -> u8 {
        let value = self.bus.cycle_read_u8(self.pc);
        self.pc = self.pc.add(1_u8, Wrap::NoWrap);
        value
    }

    fn fetch_program_u16(&mut self) -> u16 {
        let value = self.bus.cycle_read_u16(self.pc, Wrap::NoWrap);
        self.pc = self.pc.add(2_u8, Wrap::NoWrap);
        value
    }
}
