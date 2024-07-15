//! Implementation of the OAM containing sprite data.
use std::fmt::Display;
use std::fmt::Formatter;

use bitcode::Decode;
use bitcode::Encode;
use intbits::Bits;

use crate::common::address::AddressU15;

#[derive(Encode, Decode)]
pub struct Oam {
    memory: Vec<u8>,
    /// Contains the currently selected OAM address set via the OAMADD register.
    current_addr: OamAddr,
    /// Represents the write latch. Contains the previous written value or None if the latch is
    /// not set.
    latch: Option<u8>,
    /// Sprite sizes set by OBJSEL.
    /// Each sprite can select one of these sizese in the OAM attributes.
    sprite_sizes: (SpriteSize, SpriteSize),
    /// Base address for both nametables used by sprites.
    pub nametables: (AddressU15, AddressU15),

    pub main_enabled: bool,
    pub sub_enabled: bool,
    pub color_math_enabled: bool,
}

impl Oam {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            memory: vec![0; 544],
            current_addr: OamAddr(0),
            latch: None,
            sprite_sizes: (SpriteSize::Size8x8, SpriteSize::Size16x16),
            nametables: (AddressU15(0), AddressU15(0)),
            main_enabled: false,
            sub_enabled: false,
            color_math_enabled: false,
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

    /// 7  bit  0
    /// ---- ----
    /// SSSN NbBB
    /// |||| ||||
    /// |||| |+++- Name base address (word address = bBB << 13)
    /// |||+-+---- Name select (word offset = (NN+1) << 12)
    /// +++------- Object size:
    ///             0:  8x8  and 16x16
    ///             1:  8x8  and 32x32
    ///             2:  8x8  and 64x64
    ///             3: 16x16 and 32x32
    ///             4: 16x16 and 64x64
    ///             5: 32x32 and 64x64
    ///             6: 16x32 and 32x64
    ///             7: 16x32 and 32x32
    pub fn write_objsel(&mut self, value: u8) {
        self.nametables.0 = AddressU15((value.bits(0..=1) as u16) << 13);
        self.nametables.1 = self.nametables.0 + ((value.bits(3..=4) as u16 + 1) << 12);
        use SpriteSize::*;
        self.sprite_sizes = match value.bits(5..=7) {
            0 => (Size8x8, Size16x16),
            1 => (Size8x8, Size32x32),
            2 => (Size8x8, Size64x64),
            3 => (Size16x16, Size32x32),
            4 => (Size16x16, Size64x64),
            5 => (Size32x32, Size64x64),
            6 => (Size16x32, Size32x64),
            7 => (Size16x32, Size32x32),
            _ => unreachable!(),
        };
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

    pub fn get_sprites_on_scanline(&self, scanline: u32, priority: u32) -> Vec<(Sprite, u32)> {
        let mut sprites = Vec::new();
        for sprite_id in 0..128 {
            let sprite = self.get_sprite(sprite_id);
            if sprite.priority != priority as u8 {
                continue;
            }

            let y = sprite.y;
            let overdraw_scanline = scanline + 256;
            if (y..(y + sprite.height())).contains(&scanline) {
                sprites.push((sprite, scanline - y));
            } else if (y..(y + sprite.height())).contains(&overdraw_scanline) {
                sprites.push((sprite, overdraw_scanline - y));
            }
            if sprites.len() > 32 {
                break;
            }
        }
        sprites
    }
    pub fn get_all_sprites_on_scanline(&self, scanline: u32) -> Vec<(Sprite, u32)> {
        let mut sprites = Vec::new();
        for sprite_id in 0..128 {
            let sprite = self.get_sprite(sprite_id);

            let y = sprite.y;
            let overdraw_scanline = scanline + 256;
            if (y..(y + sprite.height())).contains(&scanline) {
                sprites.push((sprite, scanline - y));
            } else if (y..(y + sprite.height())).contains(&overdraw_scanline) {
                sprites.push((sprite, overdraw_scanline - y));
            }
            if sprites.len() > 32 {
                break;
            }
        }
        sprites
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
            self.sprite_sizes,
            self.nametables,
        )
    }
}

#[derive(Default, Clone, Copy, Debug, Encode, Decode)]
struct OamAddr(u16);

impl OamAddr {
    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1) % 544;
    }
}

impl std::ops::Add<u16> for OamAddr {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Self((self.0 + rhs) % 544)
    }
}

impl From<OamAddr> for usize {
    fn from(addr: OamAddr) -> Self {
        addr.0 as usize
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub tile: u32,
    pub nametable: AddressU15,
    pub palette: u8,
    pub priority: u8,
    pub hflip: bool,
    pub vflip: bool,
    pub size: SpriteSize,
}

impl Sprite {
    fn new(
        id: u32,
        data: [u8; 4],
        attributes: u8,
        sprite_sizes: (SpriteSize, SpriteSize),
        nametables: (AddressU15, AddressU15),
    ) -> Self {
        Self {
            id,
            x: if attributes.bit(id % 4 * 2) {
                (data[0] as u32).wrapping_sub(256)
            } else {
                data[0] as u32
            },
            y: data[1] as u32,
            tile: data[2] as u32,
            nametable: if data[3].bit(0) {
                nametables.1
            } else {
                nametables.0
            },
            palette: data[3].bits(1..=3),
            priority: data[3].bits(4..=5),
            hflip: data[3].bit(6),
            vflip: data[3].bit(7),
            size: if attributes.bit((id % 4) * 2 + 1) {
                sprite_sizes.1
            } else {
                sprite_sizes.0
            },
        }
    }

    pub fn palette_addr(&self) -> u8 {
        128 + self.palette * 16
    }

    pub fn coarse_width(&self) -> u32 {
        match self.size {
            SpriteSize::Size8x8 => 1,
            SpriteSize::Size16x16 => 2,
            SpriteSize::Size32x32 => 4,
            SpriteSize::Size64x64 => 8,
            SpriteSize::Size16x32 => 2,
            SpriteSize::Size32x64 => 4,
        }
    }

    pub fn coarse_height(&self) -> u32 {
        match self.size {
            SpriteSize::Size8x8 => 1,
            SpriteSize::Size16x16 => 2,
            SpriteSize::Size32x32 => 4,
            SpriteSize::Size64x64 => 8,
            SpriteSize::Size16x32 => 4,
            SpriteSize::Size32x64 => 8,
        }
    }

    pub fn width(&self) -> u32 {
        self.coarse_width() * 8
    }

    pub fn height(&self) -> u32 {
        self.coarse_height() * 8
    }
}

impl Display for Sprite {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sprite {:02X}: Tile{:02X} {} from table {} at ({}, {}) Pal{} Pri{}{}{}",
            self.id,
            self.tile,
            self.size,
            self.nametable,
            self.x,
            self.y,
            self.palette,
            self.priority,
            if self.hflip { " HFlip" } else { "" },
            if self.vflip { " VFlip" } else { "" },
        )
    }
}

#[derive(Default, Clone, Copy, Debug, Encode, Decode, strum::Display)]
pub enum SpriteSize {
    #[default]
    Size8x8,
    Size16x16,
    Size32x32,
    Size64x64,
    Size16x32,
    Size32x64,
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
        // Sprite data for sprite 0
        oam.memory[0x00..0x04].copy_from_slice(&[0x77, 0xFC, 0x00, 0x30]);
        // Attribute data for sprite 0
        oam.memory[0x200] = 0x02;

        assert_eq!(
            oam.get_sprite(0).to_string(),
            "Sprite 00: Tile00 Size16x16 from table $0000 at (119, 252) Pal0 Pri3"
        )
    }
}
