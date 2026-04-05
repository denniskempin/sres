---
title: "S-ENC Pinout"
source_url: "https://snes.nesdev.org/wiki/S-ENC_Pinout"
pageid: 189
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Pinout of the S-ENC Encoder. It has a Similar purpose to the [[S-RGB Pinout|S-RGB]], put it has a Different Pinout.

```
           .---\/---.
R-Y Out <- | 01  24 | -> B-Y Out
    GND -- | 02  23 | -> Yout
   /PCP -> | 03  22 | <- BIN
     SW -> | 04  21 | <- GIN
    VCC -- | 05  20 | <- RIN
   Cout <- | 06  19 | <- REGION (VCC = NTSC; GND = PAL)
    VID <- | 07  18 | -> PDO
  CSYNC -> | 08  17 | <- PHA
    YIn -> | 09  16 | <- /BFP
 B-Y In -> | 10  15 | -> VA
 R-Y In -> | 11  14 | <- VB
    BLA -> | 12  13 | <- VC
           `--------´
```

- R-Y Out: ER-EY Signal Output
- /PCP: Pedestal Clamp Pulse Input
- Cout: Chroma Signal Output
- VID: Composite Video Output
- YIn: Luma Signal Input
- B-Y In: EB-EY Signal Input
- R-Y In: ER-EY Signal Input
- VC: VCXO Delay Phase Input
- VB: VCXO Input
- VA: VCXO Output
- /BFP: Burst Input
- xIN: Analog RGB Input (From PPU)
- Yout: Luma Signal Output
- B-Y Out: EB-EY Signal Output

Based on <https://wiki.console5.com/tw/images/e/e6/BA6592F.pdf>
