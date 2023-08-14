use std::fmt::Display;

use intbits::Bits;
use log::error;
use log::info;
use packed_struct::prelude::*;

use crate::memory::Address;
use crate::memory::Wrap;
use crate::uint::U16Ext;
use crate::uint::U8Ext;

#[derive(Default)]
pub struct DmaController {
    dma_channels: [DmaChannel; 8],
    dma_pending: u8,
    dma_active: bool,
}

impl DmaController {
    /// Writes to 0x43XX set DMA parameters
    pub fn write_43xx_parameter(&mut self, addr: u8, value: u8) {
        let channel = addr.high_nibble() % 8;
        match addr.low_nibble() {
            0x0 => {
                self.dma_channels[channel as usize].parameters =
                    DmaParameters::unpack_from_slice(&[value]).unwrap()
            }
            0x1 => {
                self.dma_channels[channel as usize]
                    .bus_b_address
                    .offset
                    .set_low_byte(value);
            }
            0x2 => {
                self.dma_channels[channel as usize]
                    .bus_a_address
                    .offset
                    .set_low_byte(value);
            }
            0x3 => {
                self.dma_channels[channel as usize]
                    .bus_a_address
                    .offset
                    .set_high_byte(value);
            }
            0x4 => {
                self.dma_channels[channel as usize].bus_a_address.bank = value;
            }
            0x5 => {
                self.dma_channels[channel as usize]
                    .byte_count
                    .set_low_byte(value);
            }
            0x6 => {
                self.dma_channels[channel as usize]
                    .byte_count
                    .set_high_byte(value);
            }
            _ => {
                error!("Unimplemented register: 0x43{:02X}", addr)
            }
        }
    }

    /// Reads back the dma parameters at address 0x43xx
    pub fn read_43xx_parameter(&mut self, addr: u8) -> u8 {
        let channel = addr.high_nibble() % 8;
        match addr.low_nibble() {
            0x0 => self.dma_channels[channel as usize]
                .parameters
                .pack()
                .unwrap()[0],
            0x1 => self.dma_channels[channel as usize]
                .bus_b_address
                .offset
                .low_byte(),
            0x2 => self.dma_channels[channel as usize]
                .bus_a_address
                .offset
                .low_byte(),
            0x3 => self.dma_channels[channel as usize]
                .bus_a_address
                .offset
                .high_byte(),
            0x4 => self.dma_channels[channel as usize].bus_a_address.bank,
            0x5 => self.dma_channels[channel as usize].byte_count.low_byte(),
            0x6 => self.dma_channels[channel as usize].byte_count.high_byte(),
            _ => {
                error!("Unimplemented register: 0x43{:02X}", addr);
                0
            }
        }
    }

    pub fn write_420b_dma_enable(&mut self, value: u8) {
        self.dma_pending = value;
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
    ) -> Option<(Vec<(Address, Address)>, u64)> {
        if !self.dma_active {
            return None;
        }

        let mut duration = 16 - master_clock % 8;
        let mut transfers: Vec<(Address, Address)> = Vec::new();
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

                let mut bus_a_address = channel.bus_a_address;
                for idx in 0..length {
                    let bus_b_address = channel
                        .bus_b_address
                        .add(bus_b_pattern[idx % bus_b_pattern.len()], Wrap::NoWrap);

                    if channel.parameters.direction {
                        transfers.push((bus_b_address, bus_a_address));
                    } else {
                        transfers.push((bus_a_address, bus_b_address));
                    }

                    let increment = if channel.parameters.fixed {
                        0
                    } else if channel.parameters.decrement {
                        -1
                    } else {
                        1
                    };
                    bus_a_address = bus_a_address.add_signed(increment, Wrap::NoWrap);
                }

                duration += 8 + 8 * length as u64;
            }
        }
        duration += clock_speed - duration % clock_speed;

        Some((transfers, duration))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DmaChannel {
    parameters: DmaParameters,
    bus_a_address: Address,
    bus_b_address: Address,
    byte_count: u16,
}

impl Default for DmaChannel {
    fn default() -> Self {
        Self {
            parameters: DmaParameters::default(),
            bus_a_address: Address::default(),
            bus_b_address: Address::new(0, 0x21FF),
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
            "{} ({})  {}  {} +[{}] ({:06X} bytes)",
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
