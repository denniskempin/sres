# SNES APU Memory and I/O Map

#### SPC700 Memory Map

```text
  0000h..00EFh  RAM (typically used for CPU pointers/variables)
  00F0h..00FFh  I/O Ports (writes are also passed to RAM)
  0100h..01FFh  RAM (typically used for CPU stack)
  0200h..FFBFh  RAM (code, data, dir-table, brr-samples, echo-buffer, etc.)
  FFC0h..FFFFh  64-byte Boot ROM or RAM (selectable via Port 00F1h)
```

#### Audio-related Registers on Main CPU

The main CPU has four write-only 8bit outputs, and (mapped to the same addresses) four read-only 8bit inputs:

```text
  2140h - APUI00  - Main CPU to Sound CPU Communication Port 0
  2141h - APUI01  - Main CPU to Sound CPU Communication Port 1
  2142h - APUI02  - Main CPU to Sound CPU Communication Port 2
  2143h - APUI03  - Main CPU to Sound CPU Communication Port 3
```

The registers are used to communicate with Port 00F4h..00F7h on the SPC700 CPU (on power-up, this is done using a 64-byte BIOS in the SPC700).

Note: All CPU-APU communications are passed through these registers by software, there are no additional communication methods (like IRQs).

#### SPC700 I/O Ports

The SPC700 CPU includes 16 memory mapper ports at address 00F0h..00FFh:

```text
  00F0h - TEST    - Testing functions (W)                                  0Ah
  00F1h - CONTROL - Timer, I/O and ROM Control (W)                         80h
  00F2h - DSPADDR - DSP Register Index (R/W)                              (FFh)
  00F3h - DSPDATA - DSP Register Data (R/W)                          (DSP[7Fh])
  00F4h - CPUIO0  - CPU Input and Output Register 0 (R and W)      R=00h,W=00h
  00F5h - CPUIO1  - CPU Input and Output Register 1 (R and W)      R=00h,W=00h
  00F6h - CPUIO2  - CPU Input and Output Register 2 (R and W)      R=00h,W=00h
  00F7h - CPUIO3  - CPU Input and Output Register 3 (R and W)      R=00h,W=00h
  00F8h - AUXIO4  - External I/O Port P4 (S-SMP Pins 34-27) (R/W) (unused) FFh
  00F9h - AUXIO5  - External I/O Port P5 (S-SMP Pins 25-18) (R/W) (unused) FFh
  00FAh - T0DIV   - Timer 0 Divider (for 8000Hz clock source) (W)         (FFh)
  00FBh - T1DIV   - Timer 1 Divider (for 8000Hz clock source) (W)         (FFh)
  00FCh - T2DIV   - Timer 2 Divider (for 64000Hz clock source) (W)        (FFh)
  00FDh - T0OUT   - Timer 0 Output (R)                                    (00h)
  00FEh - T1OUT   - Timer 1 Output (R)                                    (00h)
  00FFh - T2OUT   - Timer 2 Output (R)                                    (00h)
```

#### DSP Registers

The 128 DSP Registers are indirectly accessed via SPC700 Ports 00F2h/00F3h.

("x" can be 0..7, for selecting one of the 8 voices, or of the 8 filters).

```text
  x0h - VxVOLL   - Left volume for Voice 0..7 (R/W)
  x1h - VxVOLR   - Right volume for Voice 0..7 (R/W)
  x2h - VxPITCHL - Pitch scaler for Voice 0..7, lower 8bit (R/W)
  x3h - VxPITCHH - Pitch scaler for Voice 0..7, upper 6bit (R/W)
  x4h - VxSRCN   - Source number for Voice 0..7 (R/W)
  x5h - VxADSR1  - ADSR settings for Voice 0..7, lower 8bit (R/W)
  x6h - VxADSR2  - ADSR settings for Voice 0..7, upper 8bit (R/W)
  x7h - VxGAIN   - Gain settings for Voice 0..7 (R/W)
  x8h - VxENVX   - Current envelope value for Voice 0..7 (R)
  x9h - VxOUTX   - Current sample value for Voice 0..7 (R)
  xAh - NA       - Unused (8 bytes of general-purpose RAM) (R/W)
  xBh - NA       - Unused (8 bytes of general-purpose RAM) (R/W)
  0Ch - MVOLL    - Left channel master volume (R/W)
  1Ch - MVOLR    - Right channel master volume (R/W)
  2Ch - EVOLL    - Left channel echo volume (R/W)
  3Ch - EVOLR    - Right channel echo volume (R/W)
  4Ch - KON      - Key On Flags for Voice 0..7 (W)
  5Ch - KOFF     - Key Off Flags for Voice 0..7 (R/W)
  6Ch - FLG      - Reset, Mute, Echo-Write flags and Noise Clock (R/W)
  7Ch - ENDX     - Voice End Flags for Voice 0..7 (R) (W=Ack)
  0Dh - EFB      - Echo feedback volume (R/W)
  1Dh - NA       - Unused (1 byte of general-purpose RAM) (R/W)
  2Dh - PMON     - Pitch Modulation Enable Flags for Voice 1..7 (R/W)
  3Dh - NON      - Noise Enable Flags for Voice 0..7 (R/W)
  4Dh - EON      - Echo Enable Flags for Voice 0..7 (R/W)
  5Dh - DIR      - Sample table address (R/W)
  6Dh - ESA      - Echo ring buffer address (R/W)
  7Dh - EDL      - Echo delay (ring buffer size) (R/W)
  xEh - NA       - Unused (8 bytes of general-purpose RAM) (R/W)
  xFh - FIRx     - Echo FIR filter coefficient 0..7 (R/W)
```

Register 80h..FFh are read-only mirrors of 00h..7Fh.

Technically, the DSP registers are a RAM-like 128-byte buffer; and are copied to internal registers when needed. At least some registers (KON and FLG) seem to have changed-flags (and the buffer-values are processed only when the flag is set), ENDX seems to be a real register (not using the RAM-like buffer).

Upon Reset, FLG is internally set to E0h (but reading the buffered DSP[6Ch] value return garbage. The 17 status register are updated (a few cycles after reset) to ENDX=FFh, ENVx=00h, OUTx=00h. Aside from ENDX/ENVx/OUTx, all other DSP registers contain garbage. Upon Power-up, that garbage randomly "tends" to some patterns, but those patterns change from day to day (some examples: one day all registers were set to 43h, on other days, the eight FIRx were all FFh, and other registers had whichever values).

#### APU RAM

Upon power-up, APU RAM tends to contain a stable repeating 64-byte pattern:

32x00h, 32xFFh (that, for APUs with two Motorola MCM51L832F12 32Kx8 SRAM chips; other consoles may use different chips with different garbage/patterns). After Reset, the boot ROM changes [0000h..0001h]=Entrypoint, and [0002h..00EFh]=00h).
