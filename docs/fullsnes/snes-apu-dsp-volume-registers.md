# SNES APU DSP Volume Registers

0Ch - MVOLL - Left channel master volume (R/W) 1Ch - MVOLR - Right channel master volume (R/W)

```text
  0-7   Volume (-127..+127) (negative = phase inverted) (sample=sample*vol/128)
```

Value -128 causes multiply overflows (-8000h*-80h=-400000h).

x0h - VxVOLL - Left volume for Voice 0..7 (R/W) x1h - VxVOLR - Right volume for Voice 0..7 (R/W)

```text
  0-7   Volume (-128..+127) (negative = phase inverted) (sample=sample*vol/128)
```

Value -128 can be safely used (unlike as for all other volume registers, there is no overflow; because ADSR/Gain does lower the incoming sample to max=sample*7FFh/800h).

#### Output Mixer

```text
  sum =       sample0*V0VOLx SAR 6      ;\
  sum = sum + sample1*V0V1Lx SAR 6      ;
  sum = sum + sample2*V0V2Lx SAR 6      ; with 16bit overflow handling
  sum = sum + sample3*V0V3Lx SAR 6      ; (after each addition)
  sum = sum + sample4*V0V4Lx SAR 6      ;
  sum = sum + sample5*V0V5Lx SAR 6      ;
  sum = sum + sample6*V0V6Lx SAR 6      ;
  sum = sum + sample7*V0V7Lx SAR 6      ;
  sum =       (sum*MVOLx SAR 7)         ;
  sum = sum + (fir_out*EVOLx SAR 7)     ;/
  if FLG.MUTE then sum = 0000h
  sum = sum XOR FFFFh  ;-final phase inversion (as done by built-in post-amp)
```
