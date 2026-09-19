//! HDMA sequencing: `hdma_setup` at V=0 h≥12, `hdma_run` at V<225 h≥1104.
//! Table and data bytes go through `MainBusImpl::bus_read` / `bus_write`.
use intbits::Bits;

use super::dma::dma_sync_duration;
use super::dma::unit_pattern;
use super::MainBusImpl;
use crate::common::address::Address;
use crate::common::address::AddressU24;
use crate::common::address::Wrap;
use crate::common::bus::BusDeviceU24;

impl<PpuT: BusDeviceU24, ApuT: BusDeviceU24> MainBusImpl<PpuT, ApuT> {
    pub(super) fn hdma_setup(&mut self) -> u64 {
        if self.dma_controller.hdma_enabled() == 0 {
            return 0;
        }

        for channel in self.dma_controller.channels_mut() {
            channel.hdma_completed = false;
            channel.hdma_do_transfer = true;
        }

        let mut cycles = 8;
        let enabled = self.dma_controller.hdma_enabled();
        for channel_idx in 0..8 {
            if !enabled.bit(channel_idx) {
                continue;
            }
            {
                let channel = &mut self.dma_controller.channels_mut()[channel_idx];
                channel.table_address = channel.bus_a_address.offset;
                channel.line_counter = 0;
            }
            cycles += self.hdma_reload(channel_idx);
        }

        dma_sync_duration(
            self.clock.clock_info().master_clock,
            self.clock_speed,
            cycles,
        )
    }

    pub(super) fn hdma_run(&mut self) -> u64 {
        if !self.dma_controller.hdma_any_active() {
            return 0;
        }

        let mut cycles = 8;
        let enabled = self.dma_controller.hdma_enabled();

        for channel_idx in 0..8 {
            if !enabled.bit(channel_idx) {
                continue;
            }
            let (do_transfer, completed, indirect, direction, bus_b, a_bank) = {
                let channel = &self.dma_controller.channels()[channel_idx];
                (
                    channel.hdma_do_transfer,
                    channel.hdma_completed,
                    channel.parameters.indirect,
                    channel.parameters.direction,
                    channel.bus_b_address,
                    if channel.parameters.indirect {
                        channel.indirect_bank
                    } else {
                        channel.bus_a_address.bank
                    },
                )
            };
            if completed || !do_transfer {
                continue;
            }

            let pattern = unit_pattern(&self.dma_controller.channels()[channel_idx].parameters);
            for &offset in pattern {
                let a_offset = {
                    let channel = &self.dma_controller.channels()[channel_idx];
                    if indirect {
                        channel.das
                    } else {
                        channel.table_address
                    }
                };
                let a_addr = AddressU24::new(a_bank, a_offset);
                let b_addr = bus_b.add(offset, Wrap::NoWrap);

                if direction {
                    let value = self.bus_read(b_addr);
                    self.bus_write(a_addr, value);
                } else {
                    let value = self.bus_read(a_addr);
                    self.bus_write(b_addr, value);
                }

                let next = a_addr.add(1_u16, Wrap::WrapBank);
                let channel = &mut self.dma_controller.channels_mut()[channel_idx];
                if indirect {
                    channel.das = next.offset;
                } else {
                    channel.table_address = next.offset;
                }
                cycles += 8;
            }
        }

        for channel_idx in 0..8 {
            if !enabled.bit(channel_idx) {
                continue;
            }
            {
                let channel = &mut self.dma_controller.channels_mut()[channel_idx];
                if channel.hdma_completed {
                    continue;
                }
                channel.line_counter = channel.line_counter.wrapping_sub(1);
                channel.hdma_do_transfer = channel.line_counter.bit(7);
            }
            cycles += self.hdma_reload(channel_idx);
        }

        dma_sync_duration(
            self.clock.clock_info().master_clock,
            self.clock_speed,
            cycles,
        )
    }

    fn hdma_reload(&mut self, channel_idx: usize) -> u64 {
        let (bank, table_address) = {
            let channel = &self.dma_controller.channels()[channel_idx];
            (channel.bus_a_address.bank, channel.table_address)
        };
        let table_addr = AddressU24::new(bank, table_address);
        let data = self.bus_read(table_addr);
        let mut cycles = 8;

        let (should_fetch_indirect, table_after_count) = {
            let channel = &mut self.dma_controller.channels_mut()[channel_idx];
            if channel.line_counter & 0x7F != 0 {
                return cycles;
            }
            channel.line_counter = data;
            channel.table_address = table_addr.add(1_u16, Wrap::WrapBank).offset;
            channel.hdma_completed = data == 0;
            channel.hdma_do_transfer = !channel.hdma_completed;
            (
                channel.parameters.indirect && !channel.hdma_completed,
                channel.table_address,
            )
        };

        if should_fetch_indirect {
            let pointer_addr = AddressU24::new(bank, table_after_count);
            let lo = self.bus_read(pointer_addr);
            let hi = self.bus_read(pointer_addr.add(1_u16, Wrap::WrapBank));
            let channel = &mut self.dma_controller.channels_mut()[channel_idx];
            channel.das = u16::from_le_bytes([lo, hi]);
            channel.table_address = pointer_addr.add(2_u16, Wrap::WrapBank).offset;
            cycles += 16;
        }

        cycles
    }
}

#[cfg(test)]
mod tests {
    use super::MainBusImpl;
    use crate::common::address::AddressU24;
    use crate::common::bus::Bus;
    use crate::common::bus::BusDeviceU24;
    use crate::common::clock::ClockInfo;
    use crate::common::uint::U16Ext;
    use crate::components::cartridge::Cartridge;
    use crate::components::cpu::MainBus;
    use crate::debugger::Debugger;

    #[derive(Default)]
    struct RecordingDevice {
        writes: Vec<(ClockInfo, AddressU24, u8)>,
        reads: Vec<(ClockInfo, AddressU24)>,
        last_clock: ClockInfo,
        read_value: u8,
    }

    impl BusDeviceU24 for RecordingDevice {
        const NAME: &'static str = "RecordingDevice";

        fn peek(&self, _addr: AddressU24) -> Option<u8> {
            Some(self.read_value)
        }

        fn read(&mut self, addr: AddressU24) -> u8 {
            self.reads.push((self.last_clock, addr));
            self.read_value
        }

        fn write(&mut self, addr: AddressU24, value: u8) {
            self.writes.push((self.last_clock, addr, value));
        }

        fn update_clock(&mut self, new_clock: ClockInfo) {
            self.last_clock = new_clock;
        }

        fn reset(&mut self) {}
    }

    type TestBus = MainBusImpl<RecordingDevice, RecordingDevice>;

    fn test_bus(program: &[u8]) -> TestBus {
        MainBusImpl::new(
            &Cartridge::with_program(program),
            RecordingDevice::default(),
            RecordingDevice::default(),
            Debugger::new(),
        )
    }

    fn addr(offset: u16) -> AddressU24 {
        AddressU24::new(0, offset)
    }

    fn write_channel0(
        bus: &mut TestBus,
        dmap: u8,
        bbad: u8,
        a1t: u16,
        a1b: u8,
        das: u16,
        dasb: u8,
    ) {
        bus.bus_write(addr(0x4300), dmap);
        bus.bus_write(addr(0x4301), bbad);
        bus.bus_write(addr(0x4302), a1t.low_byte());
        bus.bus_write(addr(0x4303), a1t.high_byte());
        bus.bus_write(addr(0x4304), a1b);
        bus.bus_write(addr(0x4305), das.low_byte());
        bus.bus_write(addr(0x4306), das.high_byte());
        bus.bus_write(addr(0x4307), dasb);
    }

    fn advance_until_v(bus: &mut TestBus, end_v: u64) {
        while bus.clock_info().f == 0 && bus.clock_info().v <= end_v {
            bus.advance_master_clock(64);
        }
    }

    fn ppu_writes_by_v(bus: &TestBus) -> Vec<(u64, u8)> {
        bus.ppu
            .writes
            .iter()
            .map(|(clock, _, value)| (clock.v, *value))
            .collect()
    }

    fn peek_a2a(bus: &TestBus) -> u16 {
        u16::from_le_bytes([
            bus.bus_peek(addr(0x4308)).unwrap(),
            bus.bus_peek(addr(0x4309)).unwrap(),
        ])
    }

    fn peek_nltr(bus: &TestBus) -> u8 {
        bus.bus_peek(addr(0x430A)).unwrap()
    }

    fn peek_das(bus: &TestBus) -> u16 {
        u16::from_le_bytes([
            bus.bus_peek(addr(0x4305)).unwrap(),
            bus.bus_peek(addr(0x4306)).unwrap(),
        ])
    }

    #[test]
    fn test_hdma_direct_pattern0() {
        // [02 AA] [81 BB] [00]: transfer AA at V=0, none at V=1, BB at V=2.
        let mut bus = test_bus(&[0x02, 0xAA, 0x81, 0xBB, 0x00]);
        write_channel0(&mut bus, 0x00, 0x00, 0x8000, 0x00, 0, 0);
        bus.bus_write(addr(0x420C), 0x01);

        advance_until_v(&mut bus, 0);
        assert_eq!(ppu_writes_by_v(&bus), vec![(0, 0xAA)]);
        assert_eq!(peek_a2a(&bus), 0x8002);
        assert_eq!(peek_nltr(&bus), 0x01);

        advance_until_v(&mut bus, 1);
        assert_eq!(ppu_writes_by_v(&bus), vec![(0, 0xAA)]);
        assert_eq!(peek_a2a(&bus), 0x8003);
        assert_eq!(peek_nltr(&bus), 0x81);

        advance_until_v(&mut bus, 2);
        assert_eq!(ppu_writes_by_v(&bus), vec![(0, 0xAA), (2, 0xBB)]);
        assert_eq!(peek_a2a(&bus), 0x8005);
        assert_eq!(peek_nltr(&bus), 0x00);

        advance_until_v(&mut bus, 5);
        assert_eq!(ppu_writes_by_v(&bus), vec![(0, 0xAA), (2, 0xBB)]);
    }

    #[test]
    fn test_hdma_repeat_pattern3() {
        // Repeat 2 lines, pattern 3: four bytes to +0,+0,+1,+1 each line.
        let mut table = vec![0x82];
        table.extend_from_slice(&[0x10, 0x11, 0x12, 0x13, 0x20, 0x21, 0x22, 0x23, 0x00]);
        let mut bus = test_bus(&table);
        write_channel0(&mut bus, 0x03, 0x00, 0x8000, 0x00, 0, 0);
        bus.bus_write(addr(0x420C), 0x01);

        advance_until_v(&mut bus, 3);
        let writes: Vec<(u64, u16, u8)> = bus
            .ppu
            .writes
            .iter()
            .map(|(clock, addr, value)| (clock.v, addr.offset, *value))
            .collect();
        assert_eq!(
            writes,
            vec![
                (0, 0x2100, 0x10),
                (0, 0x2100, 0x11),
                (0, 0x2101, 0x12),
                (0, 0x2101, 0x13),
                (1, 0x2100, 0x20),
                (1, 0x2100, 0x21),
                (1, 0x2101, 0x22),
                (1, 0x2101, 0x23),
            ]
        );
    }

    #[test]
    fn test_hdma_indirect() {
        let mut bus = test_bus(&[0x01, 0x00, 0x02, 0x00]);
        write_channel0(&mut bus, 0x40, 0x00, 0x8000, 0x00, 0, 0x00);
        bus.bus_write(addr(0x0200), 0xCC);
        bus.bus_write(addr(0x420C), 0x01);

        advance_until_v(&mut bus, 1);
        assert_eq!(ppu_writes_by_v(&bus), vec![(0, 0xCC)]);
        assert_eq!(peek_das(&bus), 0x0201);
    }

    #[test]
    fn test_hdma_direction_b_to_a() {
        let mut bus = test_bus(&[0x01, 0x00, 0x00, 0x00]);
        bus.ppu.read_value = 0x42;
        // Indirect B→A: table in ROM, data address $7E:0000.
        write_channel0(&mut bus, 0xC0, 0x00, 0x8000, 0x00, 0, 0x7E);
        bus.bus_write(addr(0x420C), 0x01);

        advance_until_v(&mut bus, 1);
        assert_eq!(bus.bus_peek(AddressU24::new(0x7E, 0x0000)), Some(0x42));
        assert!(!bus.ppu.reads.is_empty());
        assert!(bus.ppu.writes.is_empty());
    }

    #[test]
    fn test_hdma_mid_frame_enable() {
        // Dummy table that would transfer 0xFF if V=0 setup ran.
        let mut program = vec![0x01, 0xFF, 0x00];
        program.resize(0x20, 0);
        program[0x10] = 0xAA;
        let mut bus = test_bus(&program);
        write_channel0(&mut bus, 0x00, 0x00, 0x8000, 0x00, 0, 0);
        bus.bus_write(addr(0x4308), 0x10);
        bus.bus_write(addr(0x4309), 0x80);
        bus.bus_write(addr(0x430A), 0x02);

        while !(bus.clock_info().v == 5
            && bus.clock_info().h_counter >= 100
            && bus.clock_info().h_counter < 500)
        {
            bus.advance_master_clock(64);
        }
        bus.bus_write(addr(0x420C), 0x01);

        advance_until_v(&mut bus, 5);
        assert_eq!(ppu_writes_by_v(&bus), vec![(5, 0xAA)]);
        assert_eq!(peek_a2a(&bus), 0x8011);
        assert_eq!(peek_nltr(&bus), 0x01);
    }

    #[test]
    fn test_hdma_idle_zero_cost() {
        let mut clock = crate::components::clock::Clock::default();
        clock.advance_master_clock(50_000);

        let mut bus = test_bus(&[]);
        bus.advance_master_clock(50_000);
        assert_eq!(
            bus.clock_info().master_clock,
            clock.clock_info().master_clock
        );
    }

    #[test]
    fn test_hdma_reset_clears_enable() {
        let mut bus = test_bus(&[]);
        bus.bus_write(addr(0x420C), 0xFF);
        assert_eq!(bus.dma_controller.hdma_enabled(), 0xFF);
        bus.reset();
        assert_eq!(bus.dma_controller.hdma_enabled(), 0);
    }
}
