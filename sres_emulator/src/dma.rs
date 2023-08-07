use intbits::Bits;
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
        println!(
            "channel {channel} 0x{:02X} = {:08b} (0x{:02X})",
            addr.low_nibble(),
            value,
            value,
        );
        match addr.low_nibble() {
            0x0 => {
                self.dma_channels[channel as usize].parameters =
                    DmaParameters::unpack_from_slice(&[value]).unwrap()
            }
            0x1 => {
                self.dma_channels[channel as usize]
                    .destination_address
                    .offset
                    .set_low_byte(value);
            }
            0x2 => {
                self.dma_channels[channel as usize]
                    .source_address
                    .offset
                    .set_low_byte(value);
            }
            0x3 => {
                self.dma_channels[channel as usize]
                    .source_address
                    .offset
                    .set_high_byte(value);
            }
            0x4 => {
                self.dma_channels[channel as usize].source_address.bank = value;
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
            _ => {}
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
                .destination_address
                .offset
                .low_byte(),
            0x2 => self.dma_channels[channel as usize]
                .source_address
                .offset
                .low_byte(),
            0x3 => self.dma_channels[channel as usize]
                .source_address
                .offset
                .high_byte(),
            0x4 => self.dma_channels[channel as usize].source_address.bank,
            0x5 => self.dma_channels[channel as usize].byte_count.low_byte(),
            0x6 => self.dma_channels[channel as usize].byte_count.high_byte(),
            _ => 0,
        }
    }

    pub fn write_420b_dma_enable(&mut self, value: u8) {
        println!("dma enable: {:08b}", value);
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
                println!("{channel_idx}: {channel:?}");
                let mut length = channel.byte_count as u64;
                if length == 0 {
                    length = 0x10000;
                }
                for idx in 0..length {
                    if channel.parameters.direction {
                        transfers.push((channel.destination_address, channel.source_address));
                    } else {
                        transfers.push((channel.source_address, channel.destination_address));
                    }

                    if idx % 2 == 0 {
                        channel.destination_address =
                            channel.destination_address.add(1_u8, Wrap::NoWrap);
                    } else {
                        channel.destination_address =
                            channel.destination_address.sub(1_u8, Wrap::NoWrap);
                    }
                    channel.source_address = channel.source_address.add(1_u8, Wrap::NoWrap);
                }

                duration += 8 + 8 * length;
            }
        }
        duration += clock_speed - duration % clock_speed;

        Some((transfers, duration))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct DmaChannel {
    parameters: DmaParameters,
    source_address: Address,
    destination_address: Address,
    byte_count: u16,
}

impl Default for DmaChannel {
    fn default() -> Self {
        Self {
            parameters: DmaParameters::default(),
            source_address: Address::default(),
            destination_address: Address::new(0, 0x21FF),
            byte_count: 0,
        }
    }
}

#[derive(PackedStruct, Clone, Debug, Copy, PartialEq, Eq, Default)]
#[packed_struct(bit_numbering = "msb0")]
pub struct DmaParameters {
    pub direction: bool,
    pub indirect: bool,
    pub _unused: bool,
    pub decrement: bool,
    pub fixed: bool,
    #[packed_field(size_bits = "3")]
    pub transfer_pattern: u8,
}
