# SNES APU Low Level Timings

#### Register and RAM Access Timing Chart

On every CPU cycle, the hardware can access 3 bytes from RAM (2 DSP accesses, and 1 CPU access), and 4 bytes from DSP Register Array (3 DSP accesses, and 1 CPU access), plus some special DSP Registers (ENDX/FLG). Audio is generated at 32000Hz Rate (every 32 CPU cycles):

```text
  Time <--- RAM Access --->   <---- Register Array ---->   <--Extra-->
  T0   [V0BRR.2nd.dta1]       --      ,V0VOLL    ,V2SRCN
  T1   [V1SRCN/DIR.lsb/msb]   V0VOLR  ,V1PITCHL  ,V1ADSR1  ENDX.0, Timer0,1,2
  T2   [V1BRR.1st.hdr/dta0]   V0ENVX  ,V1PITCHH  ,V1ADSR2  V1:FLG.7
  T3   [V1BRR.2nd.dta1]       V0OUTX  ,V1VOLL    ,V3SRCN
  T4   [V2SRCN/DIR.lsb/msb]   V1VOLR  ,V2PITCHL  ,V2ADSR1  ENDX.1
  T5   [V2BRR.1st.hdr/dta0]   V1ENVX  ,V2PITCHH  ,V2ADSR2  V2:FLG.7
  T6   [V2BRR.2nd.dta1]       V1OUTX  ,V2VOLL    ,V4SRCN
  T7   [V3SRCN/DIR.lsb/msb]   V2VOLR  ,V3PITCHL  ,V3ADSR1  ENDX.2
  T8   [V3BRR.1st.hdr/dta0]   V2ENVX  ,V3PITCHH  ,V3ADSR2  V3:FLG.7
  T9   [V3BRR.2nd.dta1]       V2OUTX  ,V3VOLL    ,V5SRCN
  T10  [V4SRCN/DIR.lsb/msb]   V3VOLR  ,V4PITCHL  ,V4ADSR1  ENDX.3
  T11  [V4BRR.1st.hdr/dta0]   V3ENVX  ,V4PITCHH  ,V4ADSR2  V4:FLG.7
  T12  [V4BRR.2nd.dta1]       V3OUTX  ,V4VOLL    ,V6SRCN
  T13  [V5SRCN/DIR.lsb/msb]   V4VOLR  ,V5PITCHL  ,V5ADSR1  ENDX.4
  T14  [V5BRR.1st.hdr/dta0]   V4ENVX  ,V5PITCHH  ,V5ADSR2  V5:FLG.7
  T15  [V5BRR.2nd.dta1]       V4OUTX  ,V5VOLL    ,V7SRCN
  T16  [V6SRCN/DIR.lsb/msb]   V5VOLR  ,V6PITCHL  ,V6ADSR1  ENDX.5
  T17  [V6BRR.1st.hdr/dta0]   V5ENVX  ,V6PITCHH  ,V6ADSR2  V6:FLG.7, Timer2
  T18  [V6BRR.2nd.dta1]       V5OUTX  ,V6VOLL    ,V0SRCN
  T19  [V7SRCN/DIR.lsb/msb]   V6VOLR  ,V7PITCHL  ,V7ADSR1  ENDX.6
  T20  [V7BRR.1st.hdr/dta0]   V6ENVX  ,V7PITCHH  ,V7ADSR2  V7:FLG.7
  T21  [V7BRR.2nd.dta1]       V6OUTX  ,V7VOLL    ,V1SRCN
  T22  [V0SRCN/DIR.lsb/msb]   V7VOLR  ,V0PITCHL  ,V0ADSR1  ENDX.7
  T23  [RdEchoLeft.lsb/msb]   V7ENVX  ,V0PITCHH  ,FIR0
  T24  [RdEchoRight.lsb/msb]  V7OUTX  ,FIR1      ,FIR2
  T25  --- ;SRAM./OE=no       FIR3    ,FIR4      ,FIR5
  T26  [V0BRR.1st.hdr/dta0]   FIR6    ,FIR7      ,---
  T27  --- ;SRAM./OE=yes?!    MVOLL   ,EVOLL     ,EFB
  T28  --- ;SRAM./OE=yes?!    MVOLR   ,EVOLR     ,PMON
  T29  --- ;SRAM./OE=no       NON     ,EON       ,DIR      FLG.5
  T30  [WrEchoLeft.lsb/msb]   EDL     ,ESA       ,KON?     FLG.5
  T31  [WrEchoRight.lsb/msb]  KOFF    ,FLG.LSB   ,V0ADSR2  FLG.0-4,V0:FLG.7,KON
```

Note: In Gain-Mode, above reads VxGAIN instead of VxADSR2.

KON and KOFF are processed only each 64 clk cycles (each 2nd pass).

The SPC and DSP chips are started via same /RESET and clocked via same 2.048MHz signal, causing the SPC Timers to be incremented in sync with DSP timings: at T1 and T17 (unless one pauses the SPC Timers via TEST register).

Additionally (non-RAM-array):

```text
  ENDX         8bits
  KON-changed  1bit
  FLG.MSBs     3bit (or maybe/rather it's full 8bit, including LSBs?)
```

#### Internal Signals

LRCK is HIGH during T0..T15, and LOW during T16..T31.

#### APU Signals

```text
  Time  15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31  0  1  (Tnn)
  LRCK  ---________________________________________________------ (DSP.Pin43)
  MX1   _-__-__-__-__-__-__-__-__-__-__-__-__-__-__-__-__-__-__-- (DSP.Pin3)
  MX2   __-__-__-__-__-__-__-__-__-__-__-__-__-__-__-__-__-__-__- (DSP.Pin4)
  MX3   __'__'__'__'__'__'__'__'__'__'__'__'__'__'__'__'__'__'__' (DSP.Pin5)
  /WE   --C--C--C--C--C--C--C--C--C--C--C--C--C--C--CEECEEC--C--C (SRAM.Pin27)
  /OE   DDCDDCDDCDDCDDCDDCDDCDDCEECEECddCDDCddCddCddC--C--CDDCDDC (SRAM.Pin22)
  /CE0  DDCDDCDDCDDCDDCDDCDDCDDC--C--C--CDDCddCddC--C--C--CDDCDDC (SRAM.Pin20a)
  /CE1  --C--C--C--C--C--C--C--CEECEEC--C--C--C--C--CEECEEC--C--C (SRAM.Pin20b)
```

Whereas:

```text
  "-"  High
  "_"  Low
  "'"  Very short High (near falling edges of MX2)
  "C"  CPU access           ;-(occurs when MX2=High)
  "EE" Echo access          ;\
  "DD" DSP access (DIR/BRR) ; (occurs when MX2=Low)
  "dd" DSP access (dummy)   ;/
```

For /CE0 and /CE1: Assume DIR/BRR in lower 32K, Echo in upper 32K RAM.

The "DD" and "dd" signals seem to be always going Low (even when no new BRR/DIR data is needed).
