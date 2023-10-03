use intbits::Bits;

pub struct Oam {
    pub memory: Vec<u8>,
    /// Contains the currently selected OAM address set via the OAMADD register.
    current_addr: OamAddr,
    /// Represents the write latch. Contains the previous written value or None if the latch is
    /// not set.
    latch: Option<u8>,
}

impl Oam {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            memory: vec![0; 544],
            current_addr: OamAddr(0),
            latch: None,
        }
    }

    /// Register 2102/2103: OAMADD - OAM address
    ///  OAMADDH     OAMADDL
    ///   $2103       $2102
    /// 7  bit  0   7  bit  0
    /// ---- ----   ---- ----
    /// P... ...B   AAAA AAAA
    /// |       |   |||| ||||
    /// |       |   ++++-++++- OAM word address
    /// |       |   ++++-+++0- OAM priority rotation index
    /// |       +------------- OAM table select (0 = 256 word table, 1 = 16 word table)
    /// +--------------------- OAM priority rotation (1 = enable)
    ///
    /// On write: Update OAMADD
    ///           internal_oamadd = (OAMADD & $1FF) << 1
    pub fn write_oamaddl(&mut self, value: u8) {
        self.current_addr.0.set_bit(0, false);
        self.current_addr.0.set_bits(1..9, value as u16);
    }

    pub fn write_oamaddh(&mut self, value: u8) {
        self.current_addr.0.set_bit(9, value.bit(0));
    }

    /// Register 2104: OAMDATA - OAM data write
    /// 7  bit  0
    /// ---- ----
    /// DDDD DDDD
    /// |||| ||||
    /// ++++-++++- OAM data
    ///
    /// On write: If (internal_oamadd & 1) == 0, oam_latch = value
    ///           If internal_oamadd < $200 and (internal_oamadd & 1) == 1:
    ///             [internal_oamadd-1] = oam_latch
    ///             [internal_oamadd] = value
    ///           If internal_oamadd >= $200, [internal_oamadd] = value
    ///           internal_oamadd = internal_oamadd + 1
    pub fn write_oamdata(&mut self, value: u8) {
        println!("${:02X}: 0x{:02X}", self.current_addr.0, value);

        if !self.current_addr.0.bit(0) {
            self.latch = Some(value);
        }
        match self.current_addr.0 {
            0..=0x1FF => {
                if self.current_addr.0.bit(0) {
                    self.memory[usize::from(self.current_addr) - 1] = self.latch.unwrap();
                    self.memory[usize::from(self.current_addr)] = value;
                }
            }
            0x200..=0x21F => {
                self.memory[usize::from(self.current_addr)] = value;
            }
            _ => {}
        }
        self.current_addr.increment();
    }

    /// Register 2138 - OAMDATAREAD - OAM data read
    /// 7  bit  0
    /// ---- ----
    /// DDDD DDDD
    /// |||| ||||
    /// ++++-++++- OAM data
    ///
    /// On read: value = [internal_oamadd]
    ///          internal_oamadd = internal_oamadd + 1
    pub fn read_oamdataread(&mut self) -> u8 {
        let value = self.memory[usize::from(self.current_addr)];
        self.current_addr.increment();
        value
    }

    pub fn peek_oamdataread(&self) -> u8 {
        self.memory[usize::from(self.current_addr)]
    }

    pub fn get_sprite(&self, sprite_id: u32) -> Sprite {
        let sprite_addr = sprite_id as usize * 4;
        let attribute_addr = 0x200 + (sprite_id as usize) / 4;
        Sprite::new(
            sprite_id,
            self.memory[sprite_addr..sprite_addr + 4]
                .try_into()
                .unwrap(),
            self.memory[attribute_addr],
        )
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct OamAddr(u16);

impl OamAddr {
    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1) & 0x1FF;
    }
}

impl std::ops::Add<u16> for OamAddr {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self((self.0 + rhs) & 0x1FF)
    }
}

impl From<OamAddr> for usize {
    fn from(addr: OamAddr) -> Self {
        addr.0 as usize
    }
}

pub struct Sprite {
    pub x: u32,
    pub y: u32,
    pub tile: u32,
    pub palette: u32,
    pub priority: u32,
    pub hflip: bool,
    pub vflip: bool,
    pub double_resolution: bool,
}

impl Sprite {
    fn new(id: u32, data: [u8; 4], attributes: u8) -> Self {
        Self {
            x: if attributes.bit(0) {
                data[0] as u32 - 256
            } else {
                data[0] as u32
            },
            y: data[1] as u32,
            tile: (data[2] as u32).with_bit(9, data[3].bit(0)),
            palette: data[3].bits(1..=3) as u32,
            priority: data[3].bits(4..=5) as u32,
            hflip: data[3].bit(6),
            vflip: data[3].bit(7),
            double_resolution: attributes.bit(id % 4 + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_cgdata() {
        let mut oam = Oam::new();
        oam.write_oamaddl(0x42);
        oam.write_oamdata(0x03);
        oam.write_oamdata(0xE0);
        assert_eq!(oam.memory[0x84], 0x03);
        assert_eq!(oam.memory[0x85], 0xE0);
    }

    #[test]
    fn test_read_cgdataread() {
        let mut oam = Oam::new();
        oam.memory[0x84] = 0x03;
        oam.memory[0x85] = 0xE0;
        oam.write_oamaddl(0x42);
        assert_eq!(oam.read_oamdataread(), 0x03);
        assert_eq!(oam.read_oamdataread(), 0xE0);
    }

    #[test]
    fn test_get_sprite() {
        let mut oam = Oam::new();
        // Example sprite data for sprite 0
        oam.memory[0x00..0x04].copy_from_slice(&[0x77, 0xFC, 0x00, 0x30]);
        oam.memory[0x200] = 0x02;

        // Verify interpreted sprite data
        let sprite = oam.get_sprite(0);
        assert_eq!(sprite.x, 0x77);
        assert_eq!(sprite.y, 0xFC);
        assert_eq!(sprite.tile, 0);
        assert_eq!(sprite.palette, 0);
        assert_eq!(sprite.priority, 3);
        assert!(!sprite.hflip);
        assert!(!sprite.vflip);
        assert!(sprite.double_resolution);
    }
}
