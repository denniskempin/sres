---
title: "DSP Pinout"
source_url: "https://snes.nesdev.org/wiki/DSP_Pinout"
pageid: 184
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

This is how most DSP-x Coprocessors are hokked up in the Cartridge (Not to be confused with the S-DSP Audio Processor).
Chips are based on the NEC µPD77C25.

```
       .---\/---.
VCC -- | 01  28 | -- VCC
VCC -- | 02  27 | <- RS
 NC -- | 03  26 | <- /CS
 NC -- | 04  25 | <- /RD
 NC -- | 05  24 | <- /WR
 D0 <> | 06  23 | -- NC
 D1 <> | 07  22 | -- NC
 D2 <> | 08  21 | -- VCC
 D3 <> | 09  20 | -- VCC
 D4 <> | 10  19 | -- VCC
 D5 <> | 11  18 | -- VCC
 D6 <> | 12  17 | -- GND
 D7 <> | 13  16 | <- +RST
GND -- | 14  15 | <- CLK
       `--------´
```

- +RST: Inverted /RESET from Cartridge
- CLK: System Clock
- RS: Register Select (A14 when in Cartridge Memory, A12 when in Expansion Memory)
