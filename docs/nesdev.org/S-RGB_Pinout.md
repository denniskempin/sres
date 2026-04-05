---
title: "S-RGB Pinout"
source_url: "https://snes.nesdev.org/wiki/S-RGB_Pinout"
pageid: 188
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Pinout of the S-RGB Encoder. Actually a BA6596F.

```
          .---\/---.
   BIN -> | 01  24 | -> BOUT
    NC -- | 02  23 | -- NC
   GIN -> | 03  22 | -> GOUT
    NC -- | 04  21 | -- NC
   RIN -> | 05  20 | -> ROUT
   VCC -- | 06  19 | -- VCC
SYNCIN -> | 07  18 | -> SYNCOUT
   BFP -> | 08  17 | -> YOUT
REGION -> | 09  16 | -- GND
    SC -> | 10  15 | -> UOUT
   GND -- | 11  14 | -- NC
  COUT <- | 12  13 | -- GND
          `--------´
```

xIN: Analog RGB Input
xOUT: Analog Video Output (RGB = RGB, C = Composite, YV = Luma / Chroma)
SYNCIN: Composite Sync In
SYNCOUT: COmposite Sync Out
REGION: Region Select (VCC = NTSC; GND = PAL)

Based on [this Post](https://videogameperfection.com/forums/topic/schematic-for-1chip-pal-snes/)
