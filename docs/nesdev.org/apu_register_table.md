---
title: "APU register table"
source_url: "https://snes.nesdev.org/wiki/APU_register_table"
pageid: 179
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## S-SMP registers

S-SMP register summary

| S-SMP registers ([[APU register table/SMP|table source]]) | | | | | |
| --- | --- | --- | --- | --- | --- |
| Name | | Address | Bits | Type | Notes |
| [[S-SMP#TEST|TEST]] | Test | $F0 | IIEE TRWH | W8 | Undocumented test register. |
| [[S-SMP#CONTROL|CONTROL]] | Control | $F1 | I.CC .210 | W8 | Enable IPL ROM (I), Clear data ports (C), timer enable (2,1,0). |
| [[S-SMP#DSPADDR|DSPADDR]] | Register Address | $F2 | RAAA AAAA | RW8 | Selects a DSP register address. |
| [[S-SMP#DSPDATA|DSPDATA]] | Register Data | $F3 | DDDD DDDD | RW8 | Reads or writes data to the selected DSP address. |
| [[S-SMP#CPUIO|CPUIO0]] | Port 0 | $F4 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO0|APUIO0]]. |
| [[S-SMP#CPUIO|CPUIO1]] | Port 1 | $F5 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO1|APUIO1]]. |
| [[S-SMP#CPUIO|CPUIO2]] | Port 2 | $F6 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO2|APUIO2]]. |
| [[S-SMP#CPUIO|CPUIO3]] | Port 3 | $F7 | DDDD DDDD | RW8 | Reads or writes data to [[MMIO registers#APUIO3|APUIO3]]. |
|  | --- | $F8 | .... .... | RW8 | Unused (normal RAM). |
|  | --- | $F9 | .... .... | RW8 | Unused (normal RAM). |
| [[S-SMP#TxTARGET|T0TARGET]] | Timer 0 | $FA | TTTT TTTT | W8 | 8KHz timer 0 interval. |
| [[S-SMP#TxTARGET|T1TARGET]] | Timer 1 | $FB | TTTT TTTT | W8 | 8KHz timer 1 interval. |
| [[S-SMP#TxTARGET|T2TARGET]] | Timer 2 | $FC | TTTT TTTT | W8 | 64KHz timer 2 interval. |
| [[S-SMP#TxOUT|T0OUT]] | Counter 0 | $FD | 0000 CCCC | R8 | Timer 0 count-up. |
| [[S-SMP#TxOUT|T1OUT]] | Counter 1 | $FE | 0000 CCCC | R8 | Timer 1 count-up. |
| [[S-SMP#TxOUT|T2OUT]] | Counter 2 | $FF | 0000 CCCC | R8 | Timer 2 count-up. |

Register types:

- **RW** - Readable and Writable
- **R** - Readable
- **W** - Write only (reading will read back $00)

## S-DSP registers

S-DSP register summary

| S-DSP global registers ([[APU register table/DSP global|table source]]) | | | | | |
| --- | --- | --- | --- | --- | --- |
| Name | | Address | Bits | Type | Notes |
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
| S-DSP voice registers ([[APU register table/DSP global|table source]]) | | | | | |
| Name | | Address | Bits | Type | Notes |
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

Register types:

- **RW** - Readable and Writable
- **R** - Readable (technically writable, but not intended to be written to)

## References

- Anomie's SPC700 Doc
- Anomie's S-DSP Doc
- ares source code, [ares/sfc/smp](https://github.com/ares-emulator/ares/tree/master/ares/sfc/smp) directory, by Near and ares team
- ares source code, [ares/sfc/dsp](https://github.com/ares-emulator/ares/tree/master/ares/sfc/dsp) directory, by Near and ares team
