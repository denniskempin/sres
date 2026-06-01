---
title: "S-DSP registers"
source_url: "https://snes.nesdev.org/wiki/S-DSP_registers"
pageid: 180
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The S-DSP registers are accessed via the [[S-SMP#$F2-F3 DSP|DSPADDR and DSPDATA]] S-SMP registers.

S-DSP global register summary

| Name | | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- | --- |
| [MVOLL](#MVOL) | MVOL (L) | $0C | VVVV VVVV | RW | Left channel main volume, signed. |
| [MVOLR](#MVOL) | MVOL (R) | $1C | VVVV VVVV | RW | Right channel main volume, signed. |
| [EVOLL](#EVOL) | EVOL (L) | $2C | VVVV VVVV | RW | Left channel echo volume, signed. |
| [EVOLR](#EVOL) | EVOL (R) | $3C | VVVV VVVV | RW | Right channel echo volume, signed. |
| [KON](#KON) |  | $4C | 7654 3210 | RW | Key on. Writing this with any bit set will start a new note for the corresponding voice. |
| [KOFF](#KOFF) | KOF | $5C | 7654 3210 | RW | Key off. Writing this with any bit set will put the corresponding voice into its release state. |
| [FLG](#FLG) |  | $6C | RMEN NNNN | RW | Flags: soft reset (R), mute all (M), echo disable (E), noise frequency (N). |
| [ENDX](#ENDX) |  | $7C | 7654 3210 | R | Read for end of sample flag for each channel. |
| [EFB](#EFB) |  | $0D | VVVV VVVV | RW | Echo feedback, signed. |
| - | - | $1D | ---- ---- | RW | Unused. |
| [PMON](#PMON) |  | $2D | 7654 321- | RW | Enables pitch modulation for each channel, controlled by OUTX of the next lower channel. |
| [NON](#NON) |  | $3D | 7654 3210 | RW | For each channel, replaces the sample waveform with the noise generator output. |
| [EON](#EON) |  | $4D | 7654 3210 | RW | For each channel, sends to the echo unit. |
| [DIR](#DIR) |  | $5D | DDDD DDDD | RW | Pointer to the sample source directory page at $DD00. |
| [ESA](#ESA) |  | $6D | EEEE EEEE | RW | Pointer to the start of the echo memory region at $EE00. |
| [EDL](#EDL) |  | $7D | ---- DDDD | RW | Echo delay time (D). |
| [FIR0](#FIR) | C0 | $0F | VVVV VVVV | RW | Echo filter coefficient. |
| [FIR1](#FIR) | C1 | $1F | VVVV VVVV | RW | Echo filter coefficient. |
| [FIR2](#FIR) | C2 | $2F | VVVV VVVV | RW | Echo filter coefficient. |
| [FIR3](#FIR) | C3 | $3F | VVVV VVVV | RW | Echo filter coefficient. |
| [FIR4](#FIR) | C4 | $4F | VVVV VVVV | RW | Echo filter coefficient. |
| [FIR5](#FIR) | C5 | $5F | VVVV VVVV | RW | Echo filter coefficient. |
| [FIR6](#FIR) | C6 | $6F | VVVV VVVV | RW | Echo filter coefficient. |
| [FIR7](#FIR) | C7 | $7F | VVVV VVVV | RW | Echo filter coefficient. |
| ([[APU register table/DSP global|table source]]) | | | | | |

There are 8 voices, numbered 0 to 7. Each voice X has 10 registers in the range $X0-$X9.

S-DSP voice register summary

| Name | | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- | --- |
| [VxVOLL](#VxVOL) | VOL (L) | $X0 | SVVV VVVV | RW | Left channel volume, signed. |
| [VxVOLR](#VxVOL) | VOL (R) | $X1 | SVVV VVVV | RW | Right channel volume, signed. |
| [VxPITCHL](#VxPITCH) | P (L) | $X2 | LLLL LLLL | RW | Low 8 bits of sample pitch. |
| [VxPITCHH](#VxPITCH) | P (H) | $X3 | --HH HHHH | RW | High 6 bits of sample pitch. |
| [VxSRCN](#VxSRCN) | SRCN | $X4 | SSSS SSSS | RW | Selects a sample source entry from the directory (see DIR below). |
| [VxADSR1](#VxADSR) | ADSR (1) | $X5 | EDDD AAAA | RW | ADSR enable (E), decay rate (D), attack rate (A). |
| [VxADSR2](#VxADSR) | ADSR (2) | $X6 | SSSR RRRR | RW | Sustain level (S), sustain rate (R). |
| [VxGAIN](#VxGAIN) | GAIN | $X7 | 0VVV VVVV  1MMV VVVV | RW | Mode (M), value (V). |
| [VxENVX](#VxENVX) | ENVX | $X8 | 0VVV VVVV | R | Reads current 7-bit value of ADSR/GAIN envelope. |
| [VxOUTX](#VxOUTX) | OUTX | $X9 | SVVV VVVV | R | Reads signed 8-bit value of current sample wave multiplied by ENVX, before applying VOL. |
| ([[APU register table/DSP voice|table source]]) | | | | | |

Register types:

- **RW** - Readable and Writable
- **R** - Readable (technically writable, but not intended to be written to)

## Reading and writing S-DSP registers

The S-DSP registers are accessed via the [[S-SMP#$F2-F3 DSP|DSPADDR and DSPDATA]] S-SMP registers.

Writing to the S-SMP $F2 DSPADDR register will set the selected S-DSP register address. The S-SMP $F3 DSPDATA register is then used to read or write the selected S-DSP register.

CAUTIONS:

- Writing to DSPADDR with the high bit set will disable DSPDATA writes.
- Reading DSPDATA will return the value stored in the S-DSP's 128 byte register memory. Some DSPDATA reads might not match the internal S-DSP state until the S-DSP register has been written to.
- S-DSP registers are polled by the S-DSP at different points within the 32-cycle sample loop.
  - Most S-DSP register writes do not take effect immediately.
  - Some S-DSP registers are polled every second sample.
  - Overriding a S-DSP register write too early can skip the previous S-DSP register write (with audible glitches with the KON, KOFF registers).
  - The echo buffer registers are polled at unexpected times and should be treated with care.

```
    ; Select the KON ($4c) S-DSP register
    mov  a,       #$4c
    mov  DSPADDR, a

    ; Write 1 to the KON S-DSP register
    mov  a,       #$01
    mov  DSPDATA, a
```

The SPC-700 direct page move instructions can be used to simplify S-DSP register writes.

```
    ; Write zpTmp to the MVOLL ($0c) and MVOLR ($1c) S-DSP registers

    mov  DSPADDR, #$0c              ; Select MVOLL register
    mov  DSPDATA, zpTmp             ; Write zpTmp to MVOLL

    mov  DSPADDR, #$1c              ; Select MVOLR register
    mov  DSPDATA, zpTmp             ; Write zpTmp to MVOLR
```

The movw dp,ya 16 bit write instruction can be used to write to DSPADDR and DSPDATA in a single SPC-700 instruction.
movw DSPADDR,ya will write **A** to DSPADDR and **Y** to DSPDATA.
This instruction is useful when setting multiple S-DSP registers to the same value.

```
    mov  y,       #$7f

    ; Write Y to the MVOLL ($0c) and MVOLR ($1c) S-DSP registers

    mov  a,       #$0c
    movw DSPADDR, ya                ; DSPADDR = a, DSPDATA = y

    mov  a,       #$1c
    movw DSPADDR, ya                ; DSPADDR = a, DSPDATA = y
```

Another advantage of movw DSPADDR,ya is that you can do arithmetic on A to select the next DSPADDR to write to. For example, incrementing A by 1 to select the next voice S-DSP register or adding $10 to A to select the next global S-DSP register.

```
    mov  y,       #100

    ; Write Y to the VxVOLL ($x0) and VxVOLR ($x1) S-DSP registers

    mov  a,       #(voice << 4)     ; a = $x0

    movw DSPADDR, ya                ; DSPADDR = a, DSPDATA = y
    inc  a                          ; a = $x1
    movw DSPADDR, ya                ; DSPADDR = a, DSPDATA = y
```

The DSPADDR register is readable and writable. The direct-page read-modify-write SPC-700 instructions can be applied to the DSPADDR register to advance the selected S-DSP register without modifying the A, X or Y registers.

```
    ; Copy 8 bytes from `zpPtr` to the FIR S-DSP registers

    mov  y,       #0

    ; Select the FIR0 ($0f) S-DSP register
    mov  DSPADDR, #$0f

    Loop:
        ; Write zpPtr[y] to the S-DSP
        mov  a,       [zpPtr]+y
        mov  DSPDATA, a
        inc  y

        ; Add $10 to DSPADDR and loop if DSPADDR <= $7f (FIR7 S-DSP register)
        clrc
        adc  DSPADDR, #$10
        bpl  Loop
```

Finally, the DSPDATA register is readable and writeable. Allowing modifications of S-DSP registers without shadow variables.

```
    ; Enable noise on selected voices
    ; IN: A = bitmask of voices to enable

    ; Select NON ($3d) S-DSP register
    mov  DSPADDR, #$3d

    ; DSPDATA = DSPDATA | a
    or   a,       DSPDATA
    mov  DSPDATA, a
```

```
    ; Adds 8 bit `Y` to voice `A`'s VxPITCH registers
    ; IN: A = voice to change
    ; IN: Y = amount to add to VxPITCH
    ; REQUIRES: A < 8

    ; Select VxPITCHL register
    ; DSPADDR = ((A & 7) << 4) | 2
    and  a,       #$07
    xcn  a                          ; swap high and low nibbles of A
    or   a,       #$02
    mov  DSPADDR, a

    ; A = Y
    mov  a,       y

    ; Add A to DSPDATA (VxPITCHL)
    clrc
    adc  a,       DSPDATA
    mov  DSPDATA, a

    bcc  SkipHighByte
        ; Select VxPITCHH by setting bit 0 of DSPADDR. (VxPITCHH = VxPITCHL | 1)
        set1 DSPADDR.0

        ; Increment VxPITCHH register
        inc  DSPDATA
SkipHighByte:
```

## Global registers

### MVOLL, MVOLR - Main volume ($0C, $1C)

```
7  bit  0
---- ----
VVVV VVVV
|||| ||||
++++-++++- left/right channel volume (signed)
```

### EVOLL, EVOLR - Echo volume ($2C, $3C)

```
7  bit  0
---- ----
VVVV VVVV
|||| ||||
++++-++++- left/right channel volume (signed)
```

### KON - Key on ($4C)

```
7  bit  0
---- ----
7654 3210
|||| ||||
|||| |||+- key-on voice 0
|||| ||+-- key-on voice 1
|||| |+--- key-on voice 2
|||| +---- key-on voice 3
|||+------ key-on voice 4
||+------- key-on voice 5
|+-------- key-on voice 6
+--------- key-on voice 7
```

Writing a 1 to a KON voice bit will:

- Set the voice's envelope to 0
- Change the voice's ADSR state to Attack
- Reset the voice's BRR decoder to the start of the sample (as determined by [DIR](#DIR) and [VxSRCN](#VxSRCN))

KON is polled every second sample.

The internal KON bits are cleared 63 clocks after the bit is polled.

ERRATA:[[1]](#cite_note-1)

- Clearing KON too early can cause the voice to not key-on.
- If a voice's KON and KOFF bits are both set; the key-on will be followed by a key-off, silencing the channel.

### KOFF - Key off ($5C)

```
7  bit  0
---- ----
7654 3210
|||| ||||
|||| |||+- key-off voice 0
|||| ||+-- key-off voice 1
|||| |+--- key-off voice 2
|||| +---- key-off voice 3
|||+------ key-off voice 4
||+------- key-off voice 5
|+-------- key-off voice 6
+--------- key-off voice 7
```

Setting a voice bit in KON will:

- Change the voice's envelope state to Release

KOFF is polled every second sample.

ERRATA:[[2]](#cite_note-2)

- Clearing KOFF too early can cause the voice to not key-off.
- Clearing the KOFF bits after writing to KON might key-on, key-off or silence the channel.
- If a voice's KON and KOFF bits are both set; the key-on will be followed by a key-off, silencing the channel.

### FLG - Flags register ($6C)

```
7  bit  0
---- ----
RMEN NNNN
|||| ||||
|||+-++++- Noise frequency
||+------- Disable echo write
|+-------- Mute all
+--------- Soft reset

On power-on: internal FLG = $E0
On reset:    internal FLG = $E0
```

- **Soft Reset**: Silences all voices
  - All voices will be forced to the release state with an envelope of 0
  - Echo will still be processed
- **Mute all**: Disables audio output
  - Mute-all disables the external amplifier
  - Mute-all also disables audio from the cartridge slot and expansion port
- **Disable echo write**: When set, echo buffer writes are disabled
  - Echo buffer reads are not disabled
  - The echo buffer position will continue to advance every sample
  - **Do not** clear this bit unless the [ESA](#ESA) and [EDL](#EDL) registers are setup **and the echo buffer's position has reset at least once**.
    - The *disable echo write* flag should be cleared a minimum 7680 samples (240ms @ 32000Hz) after the ESA and EDL writes.
    - See [EDL](#EDL) errata for more details.
- **Noise frequency**: Sets the noise generator frequency
  - *Noise frequency* sets the rate at which the noise generator will generate a new noise sample (using the same [[DSP envelopes#Period Table|rate as the DSP envelopes]])

Noise generator frequencies [[3]](#cite_note-3)

| NNNNN | Frequency | NNNNN | Frequency |
| --- | --- | --- | --- |
| $00 | 0 Hz | $10 | 500 Hz |
| $01 | 16 Hz | $11 | 667 Hz |
| $02 | 21 Hz | $12 | 800 Hz |
| $03 | 25 Hz | $13 | 1.0 kHz |
| $04 | 31 Hz | $14 | 1.3 kHz |
| $05 | 42 Hz | $15 | 1.6 kHz |
| $06 | 50 Hz | $16 | 2.0 kHz |
| $07 | 63 Hz | $17 | 2.7 kHz |
| $08 | 83 Hz | $18 | 3.2 kHz |
| $09 | 100 Hz | $19 | 4.0 kHz |
| $0A | 125 Hz | $1A | 5.3 kHz |
| $0B | 167 Hz | $1B | 6.4 kHz |
| $0C | 200 Hz | $1C | 8.0 kHz |
| $0D | 250 Hz | $1D | 10.7 kHz |
| $0E | 333 Hz | $1E | 16.0 kHz |
| $0F | 400 Hz | $1F | 32.0 kHz |

On reset, FLG is believed to be set to $E0; disabling voices, audio output and echo writes. If the S-SMP reads FLG before writing to it, the value read will not match the internal S-DSP state.

The [[S-SMP#IPL Boot ROM|IPL Boot ROM]] does not set the FLG register. When switching to the IPL a spc700 program should set FLG to $E0 before jumping to the IPL.

### ENDX - End of sample flags ($7C, read-only)

```
7  bit  0
---- ----
7654 3210
|||| ||||
|||| |||+- Voice 0 BRR block has end-flag set
|||| ||+-- Voice 1 BRR block has end-flag set
|||| |+--- Voice 2 BRR block has end-flag set
|||| +---- Voice 3 BRR block has end-flag set
|||+------ Voice 4 BRR block has end-flag set
||+------- Voice 5 BRR block has end-flag set
|+-------- Voice 6 BRR block has end-flag set
+--------- Voice 7 BRR block has end-flag set
```

The voice bits are set when the current BRR block has the end-flag set (not at the end of the BRR sample).

If the voice is recently keyed-on, the ENDX bits will be clear (even if the BRR sample is a single BRR block).[[4]](#cite_note-4)

ENDX is technically writable and not intended to be written to. The S-DSP updates this register 8 times per sample.

### EFB - Echo feedback ($0D)

```
7  bit  0
---- ----
VVVV VVVV
|||| ||||
++++-++++- echo feedback (signed)
```

### PMON - Pitch modulation enable ($2D)

```
7  bit  0
---- ----
7654 321.
|||| |||
|||| ||+-- Enable pitch modulation on voice 1
|||| |+--- Enable pitch modulation on voice 2
|||| +---- Enable pitch modulation on voice 3
|||+------ Enable pitch modulation on voice 4
||+------- Enable pitch modulation on voice 5
|+-------- Enable pitch modulation on voice 6
+--------- Enable pitch modulation on voice 7
```

Pitch modulation adjusts the voice's pitch every sample using the output of the previous voice.

```
pitch = VxPITCH + (OUTX[x-1] >> 5) * VxPITCH >> 10;
```

OUTX is the 16-bit output of the previous voice after the envelope has been applied to the Gaussian Interpolated BRR sample (or noise if NON[x-1] is set) and before the VxVOL channel volume multiplication.

- An OUTX of 0 will not modify VxPITCH
- A positive OUTX will linearly increment the pitch
  - A maximumly positive OUTX will nearly double VxPITCH
  - A 50% positive OUTX will set pitch to 150% of VxPITCH
- A negative OUTX will linearly decrement the pitch
  - A minimally negative OUTX will set the pitch to 0
  - A 50% negative OUTX will set pitch to 50% of VxPITCH

- Pitch-modulation does not silence the previous voice. Usually the previous voice's VxVOL is set to 0 before pitch-modulation is enabled (but not required). VxVOL has no effect no effect on pitch-modulation.
- A pitch-modulated voice can be used as a pitch-modulation source.
- Voice 0 does not support pitch modulation as there is no previous voice.
- Voice 7 cannot be used as a pitch-modulation source.

### NON - Noise enable ($3D)

```
7  bit  0
---- ----
7654 3210
|||| ||||
|||| |||+- Replace voice 0 with noise generator
|||| ||+-- Replace voice 1 with noise generator
|||| |+--- Replace voice 2 with noise generator
|||| +---- Replace voice 3 with noise generator
|||+------ Replace voice 4 with noise generator
||+------- Replace voice 5 with noise generator
|+-------- Replace voice 6 with noise generator
+--------- Replace voice 7 with noise generator
```

Enabling noise will replace the voice's Gaussian Interpolated BRR output with the output of the noise generator.

- There is only 1 noise generator
- The BRR decoder will still decode BRR samples when noise is enabled
- If the BRR decoder encounters a non-looping end BRR block, the voice will be silenced (by switching to the release state with a 0 envelope)
  - VxSRCN and VxPITCH will affect when the BRR sample ends
  - Workaround: Set VxSRCN to a looping BRR sample when enabling noise

### EON - Echo enable ($4D)

```
7  bit  0
---- ----
7654 3210
|||| ||||
|||| |||+- Enable echo on voice 0
|||| ||+-- Enable echo on voice 1
|||| |+--- Enable echo on voice 2
|||| +---- Enable echo on voice 3
|||+------ Enable echo on voice 4
||+------- Enable echo on voice 5
|+-------- Enable echo on voice 6
+--------- Enable echo on voice 7
```

### DIR - Sample directory page ($5D)

```
7  bit  0
---- ----
DDDD DDDD
|||| ||||
++++-++++- Page pointer to the sample source directory
           (directory address = DDDDDDDD << 8)
```

DIR is the high-byte of the 256-byte-aligned address of the [[BRR samples]] directory. The [VxSRCN](#VxSRCN) register selects the directory entry for each voice.

Each entry in the directory is 4 bytes, two 16-bit (little-endian) addresses pointing to BRR sample blocks:

- The first address is the start of the sample. It is used when a voice is key-on ([KON](#KON)).
- The second address is the loop address of the sample. It is used when the current BRR block ends with the *end* flag set.

The BRR directory entry address for each voice is DIR \* 0x100 + VxSRCN \* 4.

If DIR or VxSRCN is changed while a voice is playing it has no immediate effect. The next time the voice reaches a BRR end block, the new DIR and/or VxSRCN is used to fetch the loop address.

### ESA - Echo start address ($6D)

```
7  bit  0
---- ----
EEEE EEEE
|||| ||||
++++-++++- Page pointer to start of the echo buffer
```

ERRATA:

- Care must be taken when writing to the EDL and ESA echo buffer registers.
- The ESA (echo buffer address) register is accessed once per sample and ESA writes can be delayed by a single sample.[[5]](#cite_note-5)
- The echo buffer wraps around the 16-bit Audio-RAM address boundary, clobbering zeropage.
- The echo buffer will write a minimum 4 bytes to the start of ESA, unless echo writes are disabled in [FLG](#FLG).
- See [EDL](#EDL) errata for more details.

### EDL - Echo delay ($7D)

```
7  bit  0
---- ----
.... DDDD
     ||||
     ++++- Echo delay time (delay = DDDD * 512 samples)
```

EDL controls the size (DDDD \* 2048 bytes) and length (DDDD \* 512 samples or 16ms @ 32000Hz) of the echo buffer.

ERRATA:

- Care must be taken when writing to the EDL and ESA echo buffer registers.
- Setting echo delay ([EDL](#EDL), register $7D) to 0 continuously overwrites 4 bytes of ARAM at the start of the echo buffer page (selected by ESA, $6D). In particular, ESA = $00 and EDL = $00 overwrites zero page locations $0000-$0003. If not using echo, remember to set the echo write protect bit of [FLG](#FLG) ($6C bit 5) to 1.
- EDL (echo delay / echo buffer size) register writes take effect when the S-DSP reaches the end of the echo buffer.[[6]](#cite_note-6)
  - Writing to EDL can take up to 7680 samples (240ms @ 32000Hz) to take effect.
- The ESA (echo buffer address) register is accessed once per sample and ESA writes can be delayed by a single sample.[[7]](#cite_note-7)
- The echo buffer wraps around the 16-bit Audio-RAM address boundary, clobbering zeropage.
- To prevent the echo buffer from clobbering memory when initialing the echo buffer, echo buffer writes should be disabled (FLG bit 5 set) before writing to ESA and EDL. There should be a minimum 7680 sample (240ms @ 32000Hz) delay before echo buffer writes are enabled (FLG bit 5 clear).

### FIRx - Echo FIR filter coefficients ($xF)

```
7  bit  0
---- ----
VVVV VVVV
|||| ||||
++++-++++- FIR filter tap (signed)
```

FIRx are the signed coefficients of an 8-tap [Finite impulse response](https://en.wikipedia.org/wiki/Finite_impulse_response "wikipedia:Finite impulse response") (FIR) filter.

The FIR filter is applied to the 15-bit stereo output of the echo buffer.

The output of the FIR filter is multiplied by EVOL for the echo output and EFB for echo feedback.

The FIR filter must be set before echo is enabled. The FIR identity filter ($7f $00 $00 $00 $00 $00 $00 $00) will not not modify the echo buffer output.

ERRATA:

- The FIR filter uses a clipped-sum for the first 7 tap calculations and a clamped-sum for only the last tap. [[8]](#cite_note-8)
- This can cause audio clicks if the FIR filter gain exceeds 0dB.
- To prevent FIR clicks ensure the absolute sum of the FIR taps (S-DSP registers FIR0-FIR7) is <= 128.

Further reading: [SnesLab Wiki - FIR Filter](https://sneslab.net/wiki/FIR_Filter)

## Voice registers

### VxVOLL, VxVOLR - Voice volume ($x0, $x1)

```
7  bit  0
---- ----
VVVV VVVV
|||| ||||
++++-++++- left/right channel volume (signed)
```

### VxPITCHL, VxPITCHH - Voice pitch ($x2, $x3)

```
15  bit  8   7  bit  0
 ---- ----   ---- ----
 ..PP pppp   pppp pppp
   || ||||   |||| ||||
   ++-++++---++++-++++- Voice pitch (2.12 fixed point)
```

VxPITCH sets the speed of the 4-point Gaussian interpolation and controls the sample rate of the BRR sound sample.

Sample rate: VxPITCH \* 32000 Hz / $1000

A VxPITCH of $1000 will play back the sample at the SNES native sample rate of 32,000 Hz.

The pitch can go as high as $3FFF, almost two octaves above $1000. Pitches above $1000 will be subject to some aliasing from samples that are skipped over.

The pitch can go all the way down to 0, where it is halted. Pitches below $1000 will be lacking in higher frequencies, and there is not very much precision as the pitch value approaches 0.

To play a BRR sample at a specific sample rate, set VxPITCH to sample\_rate / 32000Hz \* $1000

To play a specific note for a BRR sample, set VxPITCH to note\_frequency / sample\_frequency \* $1000
(where sample\_frequency is the frequency of the BRR sample when played at 32000Hz).

### VxSRCN - Voice sample source ($x4)

```
7  bit  0
---- ----
SSSS SSSS
|||| ||||
++++-++++- DIR sample source table entry
```

The VxSRCN register selects the voice's sample or instrument from the BRR sample directory.

If DIR or VxSRCN is changed while a voice is playing it has no immediate effect. The next time the voice reaches a BRR end block, the new DIR and/or VxSRCN is used to fetch the loop address.

See: [DIR](#DIR) S-DSP register.

### VxADSR1, VxADSR2 - ADSR enable and settings ($x5, $x6)

```
 VxADSR1
7  bit  0
---- ----
EDDD AAAA
|||| ||||
|||| ++++- Attack rate (A)
|+++------ Decay rate (D)
+--------- ADSR enable
```

```
 VxADSR2
7  bit  0
---- ----
LLLR RRRR
|||| ||||
|||+-++++- Sustain rate (SR)
+++------- Sustain level (SL)
```

- If the *ADSR enable* bit is clear, the envelope is controlled by the [VxGAIN](#VxGAIN) register.
- If the *ADSR enable* bit is set, the envelope will be changed depending on the ADSR state
  - **Attack**:
    - If AAAA == 15, Linear increase +1024 at a rate of 31
    - If AAAA < 15, Linear increase +32 at a rate of AAAA1 (equivalent to VxGAIN == 110\_AAAA1)
  - **Decay**: Exponential decrease at a rate of 1DDD0 (equivalent to VxGAIN == 101\_1DDD0)
    - *Sustain Level* controls when the ADSR switches from the decay to sustain state.
    - Decay ends when the upper 3 bits of the envelope match SL
  - **Sustain**: Exponential decrease at a rate of RRRRR (equivalent to VxGAIN == 101\_RRRRR)
    - A *Sustain Rate* of 0 has no exponential decay.
  - **Release**: Linear decrease at a fixed rate of -8 every sample.

[![](https://snes.nesdev.org/w/images/snes/thumb/e/e2/Adsr_envelope.svg/567px-Adsr_envelope.svg.png)](https://snes.nesdev.org/wiki/File:Adsr_envelope.svg)

ERRATA:

- There is a race-condition when changing the ADSR/GAIN envelope mode (bit 7 of ADSR1) in the middle of a note. If the S-DSP registers are written in the order ADSR1 followed by ADSR2/GAIN, the S-DSP might read the old ADSR2/GAIN value before the ADSR2/GAIN write, potentially glitching the rest of the envelope (especially if the previous GAIN was a fixed envelope).[[9]](#cite_note-9)
  - Workaround: Write to the ADSR2/GAIN register before the ADSR1 register.
  - Workaround: Only change the ADSR/GAIN envelope mode bit when the channel is in the release state.

See: [[DSP envelopes]]

### VxGAIN - GAIN envelope settings ($x7)

```
7  bit  0
---- ----
0VVV VVVV
 ||| ||||
 +++ ++++- Fixed envelope

7  bit  0
---- ----
1MMr rrrr
 ||| ||||
 ||+ ++++- GAIN rate
 ++------- GAIN mode
```

GAIN envelope is used if the [VxADSR1](#VxADSR1) *ADSR enable* bit is clear.

GAIN modes

| Mode bits | Name | envelope change per rate |
| --- | --- | --- |
| 0?? | Fixed | envelope = VVVVVVV << 4 every sample |
| 100 | Linear decrease | -32 |
| 101 | Exponential decrease | -1 - ((envelope - 1) >> 8) |
| 110 | Linear increase | +32 |
| 111 | Bent increase | if envelope < $600 (75%) { +32 } else { +8 } |

If the voice is in the release state, the envelope is linear decrease at a fixed rate of -8 every sample.

See: [[DSP envelopes]]

### VxENVX - Voice envelope value ($x8, read-only)

```
7  bit  0
---- ----
0EEE EEEE
 ||| ||||
 +++-++++- Upper 7 bits of envelope
```

VxENVX is technically writable and not intended to be written to. The S-DSP updates this register once per sample.

### VxOUTX - Voice sample value ($x9, read-only)

```
7  bit  0
---- ----
OOOO OOOO
|||| ||||
++++-++++- Upper 8 bits of current sample (signed)
```

VxOUTX contains the upper 8 bits of the current sample after the envelope has been applied to the Gaussian Interpolation (or noise) output and before the VxVOL channel volume multiplication.

VxOUTX is technically writable and not intended to be written to. The S-DSP updates this register once per sample.

## References

- Anomie's S-DSP Doc
- ares source code, [ares/sfc/dsp](https://github.com/ares-emulator/ares/tree/master/ares/sfc/dsp) directory, by Near and ares team

1. [↑](#cite_ref-1) Anomie's S-DSP Doc: KON and KOFF registers
2. [↑](#cite_ref-2) Anomie's S-DSP Doc: KON and KOFF registers
3. [↑](#cite_ref-3) Noise generator frequency source: [Super Famicom Development Wiki - SPC700 Reference - DSP Register: FLG](https://wiki.superfamicom.org/spc700-reference#dsp-register:-flg-1318)
4. [↑](#cite_ref-4) ares source code, ares::SuperFamicom::Dsp::voice5(), by Near and ares team
5. [↑](#cite_ref-5) Anomie's S-DSP Doc: ESA register
6. [↑](#cite_ref-6) Anomie's S-DSP Doc: EDL register
7. [↑](#cite_ref-7) Anomie's S-DSP Doc: ESA register
8. [↑](#cite_ref-8) [SnesLab Wiki - FIR Filter - S-DSP Implementation](https://sneslab.net/wiki/FIR_Filter#S-DSP_Implementation)
9. [↑](#cite_ref-9) [Terrific Audio Driver - I found a race condition](https://undisbeliever.net/blog/20231231-terrific-audio-driver.html#i-found-a-race-condition)
