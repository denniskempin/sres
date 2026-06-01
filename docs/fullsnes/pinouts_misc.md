# SNES Pinouts Misc Chips

#### S-ENC Pin-Outs

```text
  1 (R-Y)O     5 VCC    9 YI        13 VC (clk)   17 PHA (NC)    21 AG (Green)
  2 GND        6 CO     10 (B-Y)I   14 VB (clk)   18 PDO (NC)    22 AB (Blue)
  3 PCP,/BURST 7 VO     11 (R-Y)I   15 VA (NC)    19 NTSC/PAL    23 YO
  4 SW (VCC)   8 SYNC   12 BLA      16 BFP,/BURST 20 AR (Red)    24 (B-Y)O
```

Analog RGB to composite converter. Pin19 allows to select PAL or NTSC mode (also requires the correct PAL or NTSC clock input on Pin13,14).

The "S-RGB A" has reportedly other pinouts:

```text
  1 ?          5 ?      9 ?           13 ?        17 Y (luma)    21 ?
  2 ?          6 ?      10 ?          14 ?        18 ?           22 Green
  3 ?          7 CSYNC  11 ?          15 ?        19 ?           23 ?
  4 ?          8 ?      12 C (chroma) 16 ?        20 Red         24 Blue
```

#### S-CLK Pin-Outs (PAL only)

```text
  1 17.7MHz(X1.A)   4 21.28MHz(MCK)   7 3.072MHz(Cart)  10 GND    13 Low
  2 17.7MHz(X1.B)   5 4.433MHz(PAL)   8 3.072MHz(APU)   11 Low    14 VCC
  3 VCC             6 3.072MHz(CIC)   9 3.072MHz(APU)   12 Low
```

Clock multiplier/divider for PAL consolses (none such in NTSC consoles).

Pin6-9 are two inverters (for APU-generated CIC clock) (NTSC consoles have equivalent inverters in a 74HCU04 chip).

#### NEC uPD6376 (two-channel serial 16bit D/A Converter)

```text
  1 DSSEL (GND)    5 AGND (NC)     9 RREF           13 LRCK (32000Hz)
  2 DGND (GND)     6 ROUT          10 LREF          14 LRSEL (GND)
  3 NC (VCC)       7 AVDD (VCC)    11 LOUT          15 SI (DATA)
  4 DVDD (VCC)     8 AVDD (VCC)    12 AGND (GND)    16 CLK (1.536MHz)
```

DATA changes on falling CLK and is valid on raising CLK, falling LRCK indicates that the most recent 16 bits were LEFT data, raising LRCK indicates RIGHT data.

Each 16bit sample is preceeded by 8 dummy bits (which are always zero). The 16bit values are signed without BIAS offset (0000h=silence).

```text
           _   _   _   _   _   _       _   _   _   _   _   _   _
  CLK     | |_| |_| |_| |_| |_| |_   _| |_| |_| |_| |_| |_| |_| |_  Pin 16
          __________:_____________ .. __________________:
  LRCK              :MSB    <--- LEFT SAMPLE -->     LSB|_________  Pin 13
                    :___ ___ ___ _    __ ___ ___ ___ ___:
  DATA    __________/___X___X___X_ .. __X___X___X___X___\_________  Pin 15
                    :b15 b14 b13         b3  b2  b1  b0 :
```

If the MUTE bit is set in FLG.6, then DATA is always 0, and additionally /MUTE=LOW is output to the Amplifier (so the DSP is "double-muted", and, the /MUTE signal also mutes external sound inputs like from SGB).

#### S-MIX Pin-Outs

Unknown. This chip is found on some cost-down SNES mainboards. Maybe a sound amplifier.

#### LM324 Quad Amplifier

```text
  1 LeftPostOut      8 LeftPreOut
  2 LeftPostIn-      9 LeftPreIn-
  3 LeftPostIn+      10 LeftPreIn+
  4 VS (not VCC)     11 GND
  5 RightPostIn+     12 RightPreIn+
  6 RightPostIn-     13 RightPreIn-
  7 RightPostOut     14 RightPreOut
```

The Pre-amplifiers do amplify the signal from the D/A converter. The pre-amplified signal is then mixed (via resisters and capacitors) with the other two audio sources (from cartridge and expansion port), the result is then passed through the Post-amplifier stage, eventually muted via transistors (if DSP outputs /MUTE=Low), and amplified through further transistors. The final stereo signal is output to A/V connector, and mono signals are mixed (via resistors) for Expansion Port and TV Modulator). The LM324 chip and the transistors are using the VS supply, rather than normal 5V VCC.
