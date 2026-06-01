---
title: "APU register table/DSP voice"
source_url: "https://snes.nesdev.org/wiki/APU_register_table/DSP_voice"
pageid: 177
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

See: [[APU register table]]

This table lists the 2 common names for the S-DSP voice registers.

| Name | | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- | --- |
| [[S-DSP registers#VxVOL|VxVOLL]] | VOL (L) | $X0 | SVVV VVVV | RW | Left channel volume, signed. |
| [[S-DSP registers#VxVOL|VxVOLR]] | VOL (R) | $X1 | SVVV VVVV | RW | Right channel volume, signed. |
| [[S-DSP registers#VxPITCH|VxPITCHL]] | P (L) | $X2 | LLLL LLLL | RW | Low 8 bits of sample pitch. |
| [[S-DSP registers#VxPITCH|VxPITCHH]] | P (H) | $X3 | --HH HHHH | RW | High 6 bits of sample pitch. |
| [[S-DSP registers#VxSRCN|VxSRCN]] | SRCN | $X4 | SSSS SSSS | RW | Selects a sample source entry from the directory (see DIR below). |
| [[S-DSP registers#VxADSR|VxADSR1]] | ADSR (1) | $X5 | EDDD AAAA | RW | ADSR enable (E), decay rate (D), attack rate (A). |
| [[S-DSP registers#VxADSR|VxADSR2]] | ADSR (2) | $X6 | SSSR RRRR | RW | Sustain level (S), sustain rate (R). |
| [[S-DSP registers#VxGAIN|VxGAIN]] | GAIN | $X7 | 0VVV VVVV  1MMV VVVV | RW | Mode (M), value (V). |
| [[S-DSP registers#VxENVX|VxENVX]] | ENVX | $X8 | 0VVV VVVV | R | Reads current 7-bit value of ADSR/GAIN envelope. |
| [[S-DSP registers#VxOUTX|VxOUTX]] | OUTX | $X9 | SVVV VVVV | R | Reads signed 8-bit value of current sample wave multiplied by ENVX, before applying VOL. |
