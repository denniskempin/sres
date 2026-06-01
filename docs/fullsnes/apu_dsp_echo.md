# SNES APU DSP Echo Registers

2Ch - EVOLL - Left channel echo volume (R/W) 3Ch - EVOLR - Right channel echo volume (R/W)

```text
  0-7   Volume (-128..+127) (negative = phase inverted) (sample=sample*vol/128)
```

This is the adjustment applied to the FIR filter outputs before mixing with the main signal (after master volume adjustment).

0Dh - EFB - Echo feedback volume (R/W) Specifies the feedback volume (the volume at which the echo-output is mixed back to the echo-input).

```text
  0-7   Volume (-128..+127) (negative = phase inverted) (sample=sample*vol/128)
```

Medium values (like 40h) produce "normal" echos (with decreasing volume on each repetition). Value 00h would produce a one-shot echo, value 7Fh would repeat the echo almost infinitely (assuming that FIRx leaves the overall volume unchanged).

4Dh - EON - Echo Enable Flags for Voice 0..7 (R/W)

```text
  0-7  Flags for Voice 0..7 (0=Direct Output, 1=Echo (and Direct) Output)
```

When the bit is set and echo buffer write is enabled, this voice will be mixed into the sample to be written to the echo buffer for later echo processing.

#### 6Dh - ESA - Echo ring buffer address (R/W)

```text
  0-7   Echo Buffer Base Address (in 256-byte steps)
```

The echo buffer consists of one or more 4-byte entries:

```text
  Byte 0: Lower 7bit of Left sample  (stored in bit1-7) (bit0=unused/zero)
  Byte 1: Upper 8bit of Left sample  (stored in bit0-7)
  Byte 2: Lower 7bit of Right sample (stored in bit1-7) (bit0=unused/zero)
  Byte 3: Upper 8bit of Right sample (stored in bit0-7)
```

At 32000Hz rate, the oldest 4-byte entry is removed (and passed on to the FIR filter), and, if echo-writes are enabled in FLG register, the removed entry is then overwritten by new values (from the Echo Mixer; consisting of all Voices enabled in EON, plus echo feedback).

Changes to the ESA register are are realized (almost) immediately: After changing ESA, the hardware may keep access 1-2 samples at [OldBase+index], and does then continue at [NewBase+index] (with index being increased as usually, or reset to 0 when Echo Size has ellapsed).

7Dh - EDL - Echo delay (ring buffer size) (R/W) Specifies the size of the Echo RAM buffer (and thereby the echo delay).

Additionally to that RAM buffer, there is a 8x4-byte buffer in the FIR unit.

```text
  0-3  Echo Buffer Size (0=4 bytes, or 1..15=Size in 2K-byte steps) (max=30K)
  4-7  Not used (read/write-able)                (ie. 16 ms steps) (max=240ms)
```

Caution: It may take up to about 240ms until the hardware realizes changes to the EDL register; ie. when re-allocating the buffer size, the hardware may keep writing up to 30Kbytes according to the old size. See Echo Buffer Notes below.

#### Echo Buffer Notes

```text
        Note that the ESA register is accessed 32 cycles before the value is
        used for a write; at a sample level, this causes writes to appear to be
        delayed by at least a full sample before taking effect.

        The EDL register value is only used under certain conditions:
         * Write the echo buffer at sample 'idx' (cycles 29 and 30)
         * If idx==0, set idx_max = EDL<<9       (cycle 30-ish)
         * Increment idx. If idx>=idx_max, idx=0 (cycle 30-ish)
        This means that it can take up to .25s for a newly written value to
        actually take effect, if the old value was 0x0f and the new value is
        written just after the cycle 30 in which buffer index 0 was written.
```

#### xFh - FIRx - Echo FIR filter coefficient 0..7 (R/W)

```text
  0-7  Echo Coefficient for 8-tap FIR filter (-80h..+7Fh)
```

Value -128 should not be used for any of the FIRx registers (to avoid multiply overflows). To avoid addition overflows: The sum of POSITIVE values in first seven registers (FIR0..FIR6) should not exceed +7Fh, and the sum of NEGATIVE values should not exceed -7Fh.

The sum of all eight registers (FIR0..FIR7) should be usually around +80h (for leaving the overall output volume unchanged by the FIR unit; instead, echo volumes are usually adjusted via EFB/EVOLx registers).

The FIR formula is:

```text
  addr = (ESA*100h+ram_index*4) AND FFFFh           ;-Echo RAM read/write addr
  buf[(i-0) AND 7] = EchoRAM[addr] SAR 1            ;-input 15bit from Echo RAM
  sum =       buf[(i-7) AND 7]*FIR0 SAR 6  ;oldest  ;\
  sum = sum + buf[(i-6) AND 7]*FIR1 SAR 6           ; calculate 16bit sum of
  sum = sum + buf[(i-5) AND 7]*FIR2 SAR 6           ; oldest 7 values, these
  sum = sum + buf[(i-4) AND 7]*FIR3 SAR 6           ; additions are done
  sum = sum + buf[(i-3) AND 7]*FIR4 SAR 6           ; without overflow
  sum = sum + buf[(i-2) AND 7]*FIR5 SAR 6           ; handling
  sum = sum + buf[(i-1) AND 7]*FIR6 SAR 6           ;/
  sum = sum + buf[(i-0) AND 7]*FIR7 SAR 6  ;newest  ;-with overflow handling
  if overflow occurred in LAST addition: saturate to min/max=-8000h/+7FFFh
  audio_output=NormalVoices+((sum*EVOLx) SAR 7)     ;-output to speakers
  echo_input=EchoVoices+((sum*EFB) SAR 7)           ;-feedback to echo RAM
  echo_input=echo_input AND FFFEh                   ;-isolate 15bit/bit0=0
  if echo write enabled: EchoRAM[addr]=echo_input   ;-write (if enabled in FLG)
  i = i + 1                                         ;-FIR index for next sample
  ram_index=ram_index+1                             ;-RAM index for next sample
  decrease remain, if remain=0, reload remain from EDL and set ram_index=0
```

Note that the left and right stereo channels are filtered separately (no crosstalk), but with identical coefficients.

#### Filter Examples

```text
  FIR0 FIR1 FIR2 FIR3 FIR4 FIR5 FIR6 FIR7  EFB
  FF   08   17   24   24   17   08   FF    40  Echo with Low-pass (bugged)
  7F   00   00   00   00   00   00   00    7F  Echo (nearly endlessly repeated)
  7F   00   00   00   00   00   00   00    40  Echo (repeat w. decreasing vol)
  7F   00   00   00   00   00   00   00    00  Echo (one-shot)
```

Note: The "bugged" example (with the sum of FIR0..FIR6 exceeding +7Fh) is from official (!) specs (and thus possibly actually used by some games(?)).

#### Echo Overflows

Setting FIRx, EFB, or EVOLx to -128 does probably cause multiply overflows?
