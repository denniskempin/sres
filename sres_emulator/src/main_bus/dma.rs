//! Implementation of DMA functionality in the main bus.
use std::fmt::Display;

use intbits::Bits;
use log::info;
use log::warn;
use packed_struct::prelude::*;

use crate::common::address::Address;
use crate::common::address::AddressU24;
use crate::common::address::Wrap;
use crate::common::debug_events::DebugEvent;
use crate::common::debug_events::DebugEventCollectorRef;
use crate::common::uint::U16Ext;
use crate::common::uint::U8Ext;

pub struct DmaController {
    dma_channels: [DmaChannel; 8],
    dma_pending: u8,
    dma_active: bool,
    debug_event_collector: DebugEventCollectorRef,
}

impl DmaController {
    pub fn new(debug_event_collector: DebugEventCollectorRef) -> Self {
        Self {
            dma_channels: Default::default(),
            dma_pending: 0,
            dma_active: false,
            debug_event_collector,
        }
    }
    pub fn update_state(&mut self) {
        if self.dma_active {
            self.dma_active = false;
            self.dma_pending = 0;
        } else if self.dma_pending > 0 {
            self.dma_active = true;
        }
    }

    pub fn pending_transfers(
        &mut self,
        master_clock: u64,
        clock_speed: u64,
    ) -> Option<(Vec<(AddressU24, AddressU24)>, u64)> {
        if !self.dma_active {
            return None;
        }

        let mut duration = 16 - master_clock % 8;
        let mut transfers: Vec<(AddressU24, AddressU24)> = Vec::new();
        for channel_idx in 0..8_usize {
            if self.dma_pending.bit(channel_idx) {
                let channel = &mut self.dma_channels[channel_idx];
                info!("DMA {channel_idx}: {channel}");
                let mut length = channel.byte_count as usize;
                if length == 0 {
                    length = 0x10000;
                }

                let bus_b_pattern: Vec<u8> = match channel.parameters.transfer_pattern {
                    DmaTransferPattern::Pattern_0 => vec![0],
                    DmaTransferPattern::Pattern_0_1 => vec![0, 1],
                    DmaTransferPattern::Pattern_0_0 => vec![0, 0],
                    DmaTransferPattern::Pattern_0_0_1_1 => vec![0, 0, 1, 1],
                    DmaTransferPattern::Pattern_0_1_2_3 => vec![0, 1, 2, 3],
                    DmaTransferPattern::Undocumented_0_1_0_1 => vec![0, 1, 0, 1],
                    DmaTransferPattern::Undocumented_0_0 => vec![0, 0],
                    DmaTransferPattern::Undocumented_0_0_1_1 => vec![0, 0, 1, 1],
                };

                for idx in 0..length {
                    let bus_b_address = channel
                        .bus_b_address
                        .add(bus_b_pattern[idx % bus_b_pattern.len()], Wrap::NoWrap);

                    if channel.parameters.direction {
                        transfers.push((bus_b_address, channel.bus_a_address));
                    } else {
                        transfers.push((channel.bus_a_address, bus_b_address));
                    }

                    let increment = if channel.parameters.fixed {
                        0
                    } else if channel.parameters.decrement {
                        -1
                    } else {
                        1
                    };
                    channel.bus_a_address =
                        channel.bus_a_address.add_signed(increment, Wrap::NoWrap);
                }

                duration += 8 + 8 * length as u64;
            }
        }

        duration += clock_speed - duration % clock_speed;

        Some((transfers, duration))
    }

    pub fn bus_read(&mut self, addr: AddressU24) -> u8 {
        match self.bus_peek(addr) {
            Some(value) => value,
            None => {
                self.debug_event_collector
                    .collect_event(DebugEvent::Error(format!("Invalid read from {}", addr)));
                0
            }
        }
    }

    pub fn bus_peek(&self, addr: AddressU24) -> Option<u8> {
        match addr.offset {
            0x43..=0x43FF => {
                let low_byte = addr.offset.low_byte();
                let channel = low_byte.high_nibble() as usize % 8;
                match low_byte.low_nibble() {
                    0x0 => Some(self.peek_dmapn(channel)),
                    0x1 => Some(self.peek_bbadn(channel)),
                    0x2 => Some(self.peek_a1tnl(channel)),
                    0x3 => Some(self.peek_a1tnh(channel)),
                    0x4 => Some(self.peek_a1bn(channel)),
                    0x5 => Some(self.peek_dasnl(channel)),
                    0x6 => Some(self.peek_dasnh(channel)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn bus_write(&mut self, addr: AddressU24, value: u8) {
        match addr.offset {
            0x420B => self.write_mdmaen(value),
            0x420C => self.write_hdmaen(value),
            0x43..=0x43FF => {
                let low_byte = addr.offset.low_byte();
                let channel = low_byte.high_nibble() as usize % 8;
                match low_byte.low_nibble() {
                    0x0 => self.write_dmapn(channel, value),
                    0x1 => self.write_bbadn(channel, value),
                    0x2 => self.write_a1tnl(channel, value),
                    0x3 => self.write_a1tnh(channel, value),
                    0x4 => self.write_a1bn(channel, value),
                    0x5 => self.write_dasnl(channel, value),
                    0x6 => self.write_dasnh(channel, value),
                    0x7 => log::warn!("HDMA not implemented."),
                    _ => {
                        self.debug_event_collector
                            .collect_event(DebugEvent::Error(format!("Invalid write to {}", addr)));
                    }
                }
            }
            _ => {
                self.debug_event_collector
                    .collect_event(DebugEvent::Error(format!("Invalid write to {}", addr)));
            }
        }
    }

    /// Register 420B: MDMAEN - DMA enable
    /// 7  bit  0
    /// ---- ----
    /// 7654 3210
    /// |||| ||||
    /// |||| |||+- Channel 0 select
    /// |||| ||+-- Channel 1 select
    /// |||| |+--- Channel 2 select
    /// |||| +---- Channel 3 select
    /// |||+------ Channel 4 select
    /// ||+------- Channel 5 select
    /// |+-------- Channel 6 select
    /// +--------- Channel 7 select
    fn write_mdmaen(&mut self, value: u8) {
        self.dma_pending = value;
    }

    /// Register 420C: HDMAEN - HDMA enable
    /// 7  bit  0
    /// ---- ----
    /// 7654 3210
    /// |||| ||||
    /// |||| |||+- Channel 0 HDMA enable
    /// |||| ||+-- Channel 1 HDMA enable
    /// |||| |+--- Channel 2 HDMA enable
    /// |||| +---- Channel 3 HDMA enable
    /// |||+------ Channel 4 HDMA enable
    /// ||+------- Channel 5 HDMA enable
    /// |+-------- Channel 6 HDMA enable
    /// +--------- Channel 7 HDMA enable
    fn write_hdmaen(&mut self, value: u8) {
        warn!("HDMAEN={:02X} not implemented", value);
    }

    /// Register 43N0: DMAPn - DMA channel N control
    /// 7  bit  0
    /// ---- ----
    /// DIxA APPP
    /// |||| ||||
    /// |||| |+++- Transfer pattern (see below)
    /// |||+-+---- Address adjust mode (DMA only):
    /// |||         0:   Increment A bus address after copy
    /// |||         1/3: Fixed
    /// |||         2:   Decrement A bus address after copy
    /// ||+------- (Unused)
    /// |+-------- Indirect (HDMA only)
    /// +--------- Direction: 0=Copy from A to B, 1=Copy from B to A
    fn write_dmapn(&mut self, channel: usize, value: u8) {
        self.dma_channels[channel].parameters = DmaParameters::unpack_from_slice(&[value]).unwrap();
    }

    fn peek_dmapn(&self, channel: usize) -> u8 {
        self.dma_channels[channel].parameters.pack().unwrap()[0]
    }

    /// Register 43N1: BBADn - DMA channel N B-bus address
    fn write_bbadn(&mut self, channel: usize, value: u8) {
        self.dma_channels[channel]
            .bus_b_address
            .offset
            .set_low_byte(value);
    }

    fn peek_bbadn(&self, channel: usize) -> u8 {
        self.dma_channels[channel].bus_b_address.offset.low_byte()
    }

    /// Register 43N2: A1TnL - DMA channel N A-bus address low
    fn write_a1tnl(&mut self, channel: usize, value: u8) {
        self.dma_channels[channel]
            .bus_a_address
            .offset
            .set_low_byte(value);
    }

    fn peek_a1tnl(&self, channel: usize) -> u8 {
        self.dma_channels[channel].bus_a_address.offset.low_byte()
    }

    /// Register 43N3: A1TnH - DMA channel N A-bus address high
    fn write_a1tnh(&mut self, channel: usize, value: u8) {
        self.dma_channels[channel]
            .bus_a_address
            .offset
            .set_high_byte(value);
    }

    fn peek_a1tnh(&self, channel: usize) -> u8 {
        self.dma_channels[channel].bus_a_address.offset.high_byte()
    }

    /// Register 43N4: A1Bn - DMA channel N A-bus bank
    fn write_a1bn(&mut self, channel: usize, value: u8) {
        self.dma_channels[channel].bus_a_address.bank = value;
    }

    fn peek_a1bn(&self, channel: usize) -> u8 {
        self.dma_channels[channel].bus_a_address.bank
    }

    /// Register 43N5: DASnL - DMA channel N byte count low
    fn write_dasnl(&mut self, channel: usize, value: u8) {
        self.dma_channels[channel].byte_count.set_low_byte(value);
    }

    fn peek_dasnl(&self, channel: usize) -> u8 {
        self.dma_channels[channel].byte_count.low_byte()
    }

    /// Register 43N6: DASnH - DMA channel N byte count high
    fn write_dasnh(&mut self, channel: usize, value: u8) {
        self.dma_channels[channel].byte_count.set_high_byte(value);
    }

    fn peek_dasnh(&self, channel: usize) -> u8 {
        self.dma_channels[channel].byte_count.high_byte()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DmaChannel {
    parameters: DmaParameters,
    bus_a_address: AddressU24,
    bus_b_address: AddressU24,
    byte_count: u16,
}

impl Default for DmaChannel {
    fn default() -> Self {
        Self {
            parameters: DmaParameters::default(),
            bus_a_address: AddressU24::default(),
            bus_b_address: AddressU24::new(0, 0x21FF),
            byte_count: 0,
        }
    }
}

impl Display for DmaChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let direction = if self.parameters.direction {
            "<-"
        } else {
            "->"
        };
        let increment = if self.parameters.fixed {
            "+0"
        } else if self.parameters.decrement {
            "-1"
        } else {
            "+1"
        };
        let pattern = match self.parameters.transfer_pattern {
            DmaTransferPattern::Pattern_0 => "0",
            DmaTransferPattern::Pattern_0_1 => "0,1",
            DmaTransferPattern::Pattern_0_0 => "0,0",
            DmaTransferPattern::Pattern_0_0_1_1 => "0,0,1,1",
            DmaTransferPattern::Pattern_0_1_2_3 => "0,1,2,3",
            DmaTransferPattern::Undocumented_0_1_0_1 => "0,1,0,1",
            DmaTransferPattern::Undocumented_0_0 => "0,0",
            DmaTransferPattern::Undocumented_0_0_1_1 => "0,0,1,1",
        };
        write!(
            f,
            "{} ({})  {}  {} +[{}] (0x{:X} bytes)",
            self.bus_a_address, increment, direction, self.bus_b_address, pattern, self.byte_count
        )
    }
}

#[allow(non_camel_case_types)]
#[derive(PrimitiveEnum_u8, Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum DmaTransferPattern {
    #[default]
    Pattern_0 = 0,
    Pattern_0_1 = 1,
    Pattern_0_0 = 2,
    Pattern_0_0_1_1 = 3,
    Pattern_0_1_2_3 = 4,
    Undocumented_0_1_0_1 = 5,
    Undocumented_0_0 = 6,
    Undocumented_0_0_1_1 = 7,
}

#[derive(PackedStruct, Clone, Debug, Copy, PartialEq, Eq, Default)]
#[packed_struct(bit_numbering = "msb0")]
pub struct DmaParameters {
    // True: Transfers A -> B, False: Transfers B -> A
    pub direction: bool,
    // HDMA only
    pub indirect: bool,
    // Reserved
    pub _unused: bool,
    // True: decrement A bus address, False: increment A bus address.
    pub decrement: bool,
    // True: fixed A bus address. Overrides increment/decrement
    pub fixed: bool,
    // Pick one of 8 transfer patterns for the B bus address
    #[packed_field(size_bits = "3", ty = "enum")]
    pub transfer_pattern: DmaTransferPattern,
}
