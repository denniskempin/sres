# SNES PPU Sprites (OBJs)

```text
2101h - OBSEL - Object Size and Object Base (W)
  7-5   OBJ Size Selection  (0-5, see below) (6-7=Reserved)
         Val Small  Large
         0 = 8x8    16x16    ;Caution:
         1 = 8x8    32x32    ;In 224-lines mode, OBJs with 64-pixel height
         2 = 8x8    64x64    ;may wrap from lower to upper screen border.
         3 = 16x16  32x32    ;In 239-lines mode, the same problem applies
         4 = 16x16  64x64    ;also for OBJs with 32-pixel height.
         5 = 32x32  64x64
         6 = 16x32  32x64 (undocumented)
         7 = 16x32  32x32 (undocumented)
        (Ie. a setting of 0 means Small OBJs=8x8, Large OBJs=16x16 pixels)
        (Whether an OBJ is "small" or "large" is selected by a bit in OAM)
  4-3   Gap between OBJ 0FFh and 100h (0=None) (4K-word steps) (8K-byte steps)
  2-0   Base Address for OBJ Tiles 000h..0FFh  (8K-word steps) (16K-byte steps)
```

#### Accessing OAM

> **See:** [SNES Memory OAM Access (Sprite Attributes)](snes-memory-oam-access-sprite-attributes.md)

#### OAM (Object Attribute Memory)

Contains data for 128 OBJs. OAM Size is 512+32 Bytes. The first part (512 bytes) contains 128 4-byte entries for each OBJ:

```text
  Byte 0 - X-Coordinate (lower 8bit) (upper 1bit at end of OAM)
  Byte 1 - Y-Coordinate (all 8bits)
  Byte 2 - Tile Number  (lower 8bit) (upper 1bit within Attributes)
  Byte 3 - Attributes
```

Attributes:

```text
  Bit7    Y-Flip (0=Normal, 1=Mirror Vertically)
  Bit6    X-Flip (0=Normal, 1=Mirror Horizontally)
  Bit5-4  Priority relative to BG (0=Low..3=High)
  Bit3-1  Palette Number (0-7) (OBJ Palette 4-7 can use Color Math via CGADSUB)
  Bit0    Tile Number (upper 1bit)
```

After above 512 bytes, additional 32 bytes follow, containing 2-bits per OBJ:

```text
  Bit7    OBJ 3 OBJ Size     (0=Small, 1=Large)
  Bit6    OBJ 3 X-Coordinate (upper 1bit)
  Bit5    OBJ 2 OBJ Size     (0=Small, 1=Large)
  Bit4    OBJ 2 X-Coordinate (upper 1bit)
  Bit3    OBJ 1 OBJ Size     (0=Small, 1=Large)
  Bit2    OBJ 1 X-Coordinate (upper 1bit)
  Bit1    OBJ 0 OBJ Size     (0=Small, 1=Large)
  Bit0    OBJ 0 X-Coordinate (upper 1bit)
```

And so on, next 31 bytes with bits for OBJ4..127. Note: The meaning of the OBJ Size bit (Small/Large) can be defined in OBSEL Register (Port 2101h).
