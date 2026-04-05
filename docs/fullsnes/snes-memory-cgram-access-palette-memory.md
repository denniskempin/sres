# SNES Memory CGRAM Access (Palette Memory)

```text
2121h - CGADD - Palette CGRAM Address (Color Generator Memory) (W)
```

Color index (0..255). This is a WORD-address (2-byte steps), allowing to access 256 words (512 bytes). Writing to this register resets the 1st/2nd access flipflop (for 2122h/213Bh) to 1st access.

```text
2122h - CGDATA - Palette CGRAM Data Write (W)
213Bh - RDCGRAM - Palette CGRAM Data Read (R)
  1st Access: Lower 8 bits (even address)
  2nd Access: Upper 7 bits (odd address) (upper 1bit = PPU2 open bus)
```

Reads and Writes to EVEN and ODD byte-addresses work as follows:

```text
  Write to EVEN address  -->  set Cgram_Lsb = Data    ;memorize value
  Write to ODD address   -->  set WORD[addr-1] = Data*256 + Cgram_Lsb
  Read from ANY address  -->  return BYTE[addr]
```

The address is automatically incremented after every read or write access.

#### CGRAM Content (and CGRAM-less Direct Color mode)

> **See:** [SNES PPU Color Palette Memory (CGRAM) and Direct Colors](snes-ppu-color-palette-memory-cgram-and-direct-colors.md)
