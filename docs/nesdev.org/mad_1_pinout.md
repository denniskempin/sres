---
title: "MAD-1 Pinout"
source_url: "https://snes.nesdev.org/wiki/MAD-1_Pinout"
pageid: 185
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

"MAD-1" Stands for "Memory Address Decoder 1". It is used for mapping ROMs, the DSP-n coprocessor, and cart save RAM; as well as providing battery switchover for games with save RAM.

```
           .---\/---.
/ROM2CE <- | 01  16 | -> /ROM1CE
 /RAMCE <- | 02  15 | <- A15 (mode $20); A13 (mode $21)
 /AUXCE <- | 03  14 | <- A20 (mode $20); A14 (mode $21)
 /ROMCE <- | 04  13 | <- A21 (mode $20 or $21); A23 (mode $25)
 RAMVcc <- | 05  12 | <- A22 (mode $20); A15 or A22 (mode $21)
    +5V -- | 06  11 | <- /ROMSEL
Battery -> | 07  10 | <- Map
    GND -- | 08  09 | <- /Reset
           `--------´
```

- /ROM2CE: asserted when pin 13 is high, for using two ROM chips
- /ROM1CE: asserted when pin 13 is low, for using two ROM chips
- /ROMCE: asserted (low) if either /ROM1CE or /ROM2CE are asserted
- Map: changes the five outputs (16, 1-4) to mode $20 (when grounded) or mode $21 (when tied high)
- Battery: from Battery via a 1k resitor
- RAMVcc: power supply to cart save RAM
- /RAMCE: asserted to access cart save RAM
- /AUXCE: asserted to access cart auxiliary (only ever seen used with DSP-n coprocessor)
- /Reset: reset signal from card edge
