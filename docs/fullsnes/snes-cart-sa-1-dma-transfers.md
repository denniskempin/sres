# SNES Cart SA-1 DMA Transfers

#### 2230h SA-1 DCNT - DMA Control (W)

```text
  0-1 DMA Source Device      (0=ROM, 1=BW-RAM, 2=I-RAM, 3=Reserved);\for
  2   DMA Destination Device (0=I-RAM, 1=BW-RAM)                   ;/Normal DMA
  3   Not used (should be 0)
  4   DMA Char Conversion Type (0=Type 2/Semi-Automatic, 1=Type 1/Automatic)
  5   DMA Char Conversion Enable (0=Normal DMA, 1=Character Conversion DMA)
  6   DMA Priority (0=SA-1 CPU Priority, 1=DMA Priority) ;<-- for Normal DMA
  7   DMA Enable (0=Disable, 1=Enable... and Clear Parameters?)
```

Bit6 is only valid for Normal DMA between BW-RAM and I-RAM. Source and Destination may not be the same devices (ie. no I-RAM to I-RAM, or BW-RAM to BW-RAM).

#### 2231h Both CDMA - Character Conversion DMA Parameters (W)

```text
  0-1 Color Depth (0=8bit, 1=4bit, 2=2bit, 3=Reserved)
  2-4 Virtual VRAM Width (0..5 = 1,2,4,8,16,32 characters) (6..7=Reserved)
  5-6 Not used (should be 0)
  7   Terminate Character Conversion 1 (0=No change, 1=Terminate DMA)
```

2232h Both SDA - DMA Source Device Start Address Lsb (W) 2233h Both SDA - DMA Source Device Start Address Mid (W) 2234h Both SDA - DMA Source Device Start Address Msb (W)

```text
  0-23  24bit Memory Address (translated to 23bit ROM Offset via 2220h..2223h)
  0-17  18bit BW-RAM Offset
  0-10  11bit I-RAM Offset
```

Used bits are 24bit/18bit/11bit for ROM/BW-RAM/I-RAM.

2235h Both DDA - DMA Destination Device Start Address Lsb (W) 2236h Both DDA - DMA Destination Device Start Address Mid (Start/I-RAM) (W) 2237h Both DDA - DMA Destination Device Start Address Msb (Start/BW-RAM)(W)

```text
  0-17  BW-RAM Offset (transfer starts after writing 2237h)
  0-10  I-RAM Offset  (transfer starts after writing 2236h) (2237h is unused)
```

2238h SA-1 DTC - DMA Terminal Counter Lsb (W) 2239h SA-1 DTC - DMA Terminal Counter Msb (W)

```text
  0-15  DMA Transfer Length in bytes (1..65535) (0=Reserved/unknown)
```

DTC is used only for Normal DMA (whilst Character Conversion DMA lasts endless; for Type 1: as long as SNES reads "BW-RAM" / until it sets 2231h.Bit7, for Type 2: as long as SA-1 writes BRF / until it clears 2230h.Bit0).

224xh SA-1 BRF - Bit Map Register File (2240h..224Fh) (W) These 16 registers can hold two 8 pixel rows (with 2bit/4bit/8bit per pixel).

```text
  0-1  2bit pixel (bit 2-7=unused)
  0-3  4bit pixel (bit 4-7=unused)
  0-7  8bit pixel
```

Used only for (semi-automatic) Character Conversion Type 2, where the "DMA" source data is to be written pixel-by-pixel to these registers; writing to one 8 pixel row can be done while transferring the other row to the SNES.

#### Normal DMA (memory transfer within cartridge memory)

```text
  ROM    --> I-RAM     10.74MHz
  ROM    --> BW-RAM    5.37MHz
  BW-RAM --> I-RAM     5.37MHz
  I-RAM  --> BW-RAM    5.37MHz
```

For normal DMA:

```text
  Set DCNT (select source/dest/prio/enable)
  Set SDA (set source offset)
  Set DTC (set transfer length)
  Set DDA (set destination offset, and start transfer)
  If desired, wait for CFR.Bit5 (DMA completion interrupt)
```

Normal DMA is used by J. League '96, Jumpin Derby, Marvelous. For ROM, SDA should be usually C00000h and up (HiROM mapping); Jumpin Derby is unconventionally using SDA at 2x8xxxh and up (LoROM mapping).

#### Character Conversion DMA

Used to convert bitmaps or pixels to bit-planed tiles. For details, see

> **See:** [SNES Cart SA-1 Character Conversion](snes-cart-sa-1-character-conversion.md)

#### SNES DMA (via Port 43xxh)

Can be used to transfer "normal" data from ROM/BW-RAM/I-RAM to SNES memory, also used for forwarding temporary Character Conversion data from I-RAM to SNES.

#### Unknown details

Unknown if SDA/DDA are increased and if DTC is decreased (or if that operations appear only on internal registers) (MSBs of DDA are apparently NOT increased on char conversion DMAs).
