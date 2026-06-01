---
title: "APU register table/DSP global"
source_url: "https://snes.nesdev.org/wiki/APU_register_table/DSP_global"
pageid: 178
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

See: [[APU register table]]

This table lists the 2 common names for the S-DSP global registers.

| Name | | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- | --- |
| [[S-DSP registers#MVOL|MVOLL]] | MVOL (L) | $0C | VVVV VVVV | RW | Left channel main volume, signed. |
| [[S-DSP registers#MVOL|MVOLR]] | MVOL (R) | $1C | VVVV VVVV | RW | Right channel main volume, signed. |
| [[S-DSP registers#EVOL|EVOLL]] | EVOL (L) | $2C | VVVV VVVV | RW | Left channel echo volume, signed. |
| [[S-DSP registers#EVOL|EVOLR]] | EVOL (R) | $3C | VVVV VVVV | RW | Right channel echo volume, signed. |
| [[S-DSP registers#KON|KON]] |  | $4C | 7654 3210 | RW | Key on. Writing this with any bit set will start a new note for the corresponding voice. |
| [[S-DSP registers#KOFF|KOFF]] | KOF | $5C | 7654 3210 | RW | Key off. Writing this with any bit set will put the corresponding voice into its release state. |
| [[S-DSP registers#FLG|FLG]] |  | $6C | RMEN NNNN | RW | Flags: soft reset (R), mute all (M), echo disable (E), noise frequency (N). |
| [[S-DSP registers#ENDX|ENDX]] |  | $7C | 7654 3210 | R | Read for end of sample flag for each channel. |
| [[S-DSP registers#EFB|EFB]] |  | $0D | VVVV VVVV | RW | Echo feedback, signed. |
| - | - | $1D | ---- ---- | RW | Unused. |
| [[S-DSP registers#PMON|PMON]] |  | $2D | 7654 321- | RW | Enables pitch modulation for each channel, controlled by OUTX of the next lower channel. |
| [[S-DSP registers#NON|NON]] |  | $3D | 7654 3210 | RW | For each channel, replaces the sample waveform with the noise generator output. |
| [[S-DSP registers#EON|EON]] |  | $4D | 7654 3210 | RW | For each channel, sends to the echo unit. |
| [[S-DSP registers#DIR|DIR]] |  | $5D | DDDD DDDD | RW | Pointer to the sample source directory page at $DD00. |
| [[S-DSP registers#ESA|ESA]] |  | $6D | EEEE EEEE | RW | Pointer to the start of the echo memory region at $EE00. |
| [[S-DSP registers#EDL|EDL]] |  | $7D | ---- DDDD | RW | Echo delay time (D). |
| [[S-DSP registers#FIR|FIR0]] | C0 | $0F | VVVV VVVV | RW | Echo filter coefficient. |
| [[S-DSP registers#FIR|FIR1]] | C1 | $1F | VVVV VVVV | RW | Echo filter coefficient. |
| [[S-DSP registers#FIR|FIR2]] | C2 | $2F | VVVV VVVV | RW | Echo filter coefficient. |
| [[S-DSP registers#FIR|FIR3]] | C3 | $3F | VVVV VVVV | RW | Echo filter coefficient. |
| [[S-DSP registers#FIR|FIR4]] | C4 | $4F | VVVV VVVV | RW | Echo filter coefficient. |
| [[S-DSP registers#FIR|FIR5]] | C5 | $5F | VVVV VVVV | RW | Echo filter coefficient. |
| [[S-DSP registers#FIR|FIR6]] | C6 | $6F | VVVV VVVV | RW | Echo filter coefficient. |
| [[S-DSP registers#FIR|FIR7]] | C7 | $7F | VVVV VVVV | RW | Echo filter coefficient. |
