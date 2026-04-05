# SNES Cart S-DD1 (Data Decompressor) (2 games)

The S-DD1 is a 100pin Data Decompression chip, used by only two games:

```text
  Star Ocean (6MB ROM, 8KB RAM) (1996) tri-Ace/Enix (JP)
  Street Fighter Alpha 2 (4MB ROM, no RAM) (1996) Capcom (NA) (JP) (EU)
```

#### S-DD1 Decompression Algorithm

> **See:** [SNES Cart S-DD1 Decompression Algorithm](snes-cart-s-dd1-decompression-algorithm.md)

#### S-DD1 I/O Ports

```text
  4800h  DMA Enable 1 (bit0..7 = DMA 0..7) (unchanged after DMA)
  4801h  DMA Enable 2 (bit0..7 = DMA 0..7) (automatically cleared after DMA)
  4802h  Unknown   ;\set to 0000h by Star Ocean (maybe SRAM related)
  4803h  Unknown   ;/unused by Street Fighter Alpha 2
  4804h  ROM Bank for C00000h-CFFFFFh (in 1MByte units)
  4805h  ROM Bank for D00000h-DFFFFFh (in 1MByte units)
  4806h  ROM Bank for E00000h-EFFFFFh (in 1MByte units)
  4807h  ROM Bank for F00000h-FFFFFFh (in 1MByte units)
  <DMA>  DMA from ROM returns Decompressed Data (originated at DMA start addr)
```

#### S-DD1 Memory Map

```text
  ???-???          SRAM (if any)
  008000h-00FFFFh  Exception Handlers, mapped in LoROM-fashion (ROM 0..7FFFh)
  C00000h-CFFFFFh  ROM (mapped via Port 4804h) (in HiROM fashion)
  D00000h-DFFFFFh  ROM (mapped via Port 4805h) (in HiROM fashion)
  E00000h-EFFFFFh  ROM (mapped via Port 4806h) (in HiROM fashion)
  F00000h-FFFFFFh  ROM (mapped via Port 4807h) (in HiROM fashion)
```

#### S-DD1 PCBs

```text
  SHVC-1NON-01  CartSlotPin59 not connected (no C12 capacitor on PA1 pin)
  SHVC-1NON-10  Strange revision (capacitor C12 between PA1 and GND)
  SNSP-1NON-10  PAL version (S-DD1.Pin82 wired to ... VCC?) (also with C12)
  SHVC-LN3B-01  Version with additional SRAM for Star Ocean
```

The 1NON board contains only two chips (100pin D-DD1 and 44pin ROM), the CIC function is included in the S-DD1, whereas Pin82 does probably select "PAL/NTSC" CIC mode.

The LN3B-board contains five chips (two 44pin ROMs, S-DD1, 8Kx8bit SRAM, and a MM1026AF battery controller).

#### S-DD1 Pinouts

```text
  1-81   Unknown
  82     PAL/NTSC (for CIC mode)
  83-100 Unknown
```
