---
title: "UPD6376 Pinout"
source_url: "https://snes.nesdev.org/wiki/UPD6376_Pinout"
pageid: 199
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Pinout of the µPD6376 Audio DAC. It takes the Digital Serial Audio data from the S-DSP (not the DSP-x coprocessor) and converts it into an Analog signal.

```
         .----\/----.
FSSEL -> | 01    16 | <- CLK
 DGND -- | 02    15 | <- DIN
   NC xx | 03    14 | <- LRSEL
 DVCC -- | 04    13 | <- LRCLK
 AGND -- | 05    12 | -- AGND
 ROUT <~ | 06    11 | ~> LOUT
 AVCC -- | 07    10 | <~ LREF
 AVCC -- | 08    09 | <~ RREF
         ˋ----------´
```

- AVCC: Analog voltage supply
- DVCC: Digital voltage supply
- AGND: Analog ground
- DGND: Digital ground
- LREF: Reference voltage for Left DAC
- RREF: Reference voltage for Right DAC
- DIN: Data input
- CLK: Bit clock
- LRCLK: Word clock
- LRSEL: Stereo reverse
- FSSEL: When Low or Floating (internally pulled low), DIN caries multiplexed data (as on SNES). When high, left is from DIN and right is from LRSEL

- Based on <https://www.alldatasheet.com/html-pdf/6972/NEC/UPD6376/164/4/UPD6376.html>
