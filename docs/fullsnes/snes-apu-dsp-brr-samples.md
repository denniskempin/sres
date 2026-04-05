# SNES APU DSP BRR Samples

#### x4h - VxSRCN - Source number for Voice 0..7 (R/W)

```text
  0-7   Instrument number (index in DIR table)
```

Points to the BRR Start & Loop addresses (via a table entry at VxSRCN*4+DIR*100h), used when voices are Keyed-ON or Looped.

#### 5Dh - DIR - Sample table address (R/W)

```text
  0-7   Sample Table Address (in 256-byte steps) (indexed via VxSRCN)
```

The table can contain up to 256 four-byte entries (max 1Kbyte). Each entry is:

```text
  Byte 0-1  BRR Start Address (used when voice is Keyed-ON)
  Byte 2-3  BRR Restart/Loop Address (used when end of BRR data reached)
```

Changing DIR or VxSRCN has no immediate effect (until/unless voices are newly Looped or Keyed-ON).

#### Bit Rate Reduction (BRR) Format

The sample data consists of 9-byte block(s). The first byte of each block is:

```text
  7-4  Shift amount   (0=Silent, 12=Loudest, 13-15=Reserved)
  3-2  Filter number  (0=None, 1..3=see below)
  1-0  Loop/End flags (0..3=see below)
```

The next 8 bytes contain two samples (or nibbles) each:

```text
  7-4  First Sample  (signed -8..+7)
  3-0  Second Sample (signed -8..+7)
```

The Loop/End bits can have following values:

```text
  Code 0 = Normal   (continue at next 9-byte block)
  Code 1 = End+Mute (jump to Loop-address, set ENDx flag, Release, Env=000h)
  Code 2 = Ignored  (same as Code 0)
  Code 3 = End+Loop (jump to Loop-address, set ENDx flag)
```

The Shift amount is used to convert the 4bit nibbles to 15bit samples:

```text
  sample = (nibble SHL shift) SAR 1
  Accordingly, shift=0 is rather useless (since it strips the low bit).
  When shift=13..15, decoding works as if shift=12 and nibble=(nibble SAR 3).
```

The Filter bits allow to select the following filter modes:

```text
  Filter 0: new = sample
  Filter 1: new = sample + old*0.9375
  Filter 2: new = sample + old*1.90625  - older*0.9375
  Filter 3: new = sample + old*1.796875 - older*0.8125
```

More precisely, the exact formulas are:

```text
  Filter 0: new = sample
  Filter 1: new = sample + old*1+((-old*1) SAR 4)
  Filter 2: new = sample + old*2+((-old*3) SAR 5)  - older+((older*1) SAR 4)
  Filter 3: new = sample + old*2+((-old*13) SAR 6) - older+((older*3) SAR 4)
```

When creating BRR data, take care that "new" does never exceed -3FFAh..+3FF8h, otherwise a number of hardware glitches will occur:

```text
  If new>+7FFFh then new=+7FFFh (but, clipped to +3FFFh below) ;\clamp 16bit
  If new<-8000h then new=-8000h (but, clipped to ZERO below)   ;/(dirt-effect)
  If new=(+4000h..+7FFFh) then new=(-4000h..-1)                ;\clip 15bit
  If new=(-8000h..-4001h) then new=(-0..-3FFFh)                ;/(lost-sign)
  If new>+3FF8h OR new<-3FFAh then overflows can occur in Gauss section
```

The resulting 15bit "new" value is then passed to the Gauss filter, and additionally re-used for the next 1-2 sample(s) as "older=old, old=new".

#### BRR Notes

The first 9-byte BRR sample block should always use Filter 0 (so it isn't disturbed by uninitialized old/older values). Same for the first block at the Loop address (unless the old/older values of the initial-pass should happen to match the ending values of the looped-passes).
