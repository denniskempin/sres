use log::debug;

use crate::memory::Address;

pub struct Apu {
    channel_data: [u8; 4],
}

impl Apu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            channel_data: [0xAA, 0, 0, 0],
        }
    }

    pub fn bus_read(&mut self, addr: Address) -> u8 {
        self.read_apuio(addr)
    }

    pub fn bus_peek(&self, addr: Address) -> Option<u8> {
        Some(self.peek_apuio(addr))
    }

    pub fn bus_write(&mut self, addr: Address, value: u8) {
        self.write_apuio(addr, value)
    }

    /// Register 2140..2144: APUION - APU IO Channels
    fn write_apuio(&mut self, addr: Address, value: u8) {
        let channel_id = (addr.offset - 0x2140) as usize % 4;
        self.channel_data[channel_id] = value;
        debug!("APUIO[{:04X}] = {:02X}", addr.offset, value);
    }

    fn peek_apuio(&self, addr: Address) -> u8 {
        let channel_id = (addr.offset - 0x2140) as usize % 4;
        self.channel_data[channel_id]
    }

    fn read_apuio(&mut self, addr: Address) -> u8 {
        let channel_id = (addr.offset - 0x2140) as usize % 4;
        let value = self.channel_data[channel_id];
        self.channel_data[channel_id] = match channel_id {
            0 => 0xAA,
            1 => 0xBB,
            _ => 0,
        };
        debug!("APUIO[{:04X}] reads {:02X}", addr.offset, value);
        value
    }
}
