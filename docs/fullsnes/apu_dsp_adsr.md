# SNES APU DSP ADSR/Gain Envelope

x5h - VxADSR1 - ADSR settings for Voice 0..7, lower 8bit (R/W) x6h - VxADSR2 - ADSR settings for Voice 0..7, upper 8bit (R/W)

```text
  0-3   4bit Attack rate   ;Rate=N*2+1, Step=+32 (or Step=+1024 when Rate=31)
  4-6   3bit Decay rate    ;Rate=N*2+16, Step=-(((Level-1) SAR 8)+1)
  7     ADSR/Gain Select   ;0=Use VxGAIN, 1=Use VxADSR (Attack/Decay/Sustain)
  8-12  5bit Sustain rate  ;Rate=N, Step=-(((Level-1) SAR 8)+1)
  13-15 3bit Sustain level ;Boundary=(N+1)*100h
  N/A   0bit Release rate  ;Rate=31, Step=-8 (or Step=-800h when BRR-end)
```

If a voice is Keyed-ON: Enter Attack mode, set Level=0, and increase level. At Level>=7E0h: Switch from Attack to Decay mode (and clip level to max=7FFh if level>=800h). At Level<=Boundary: Switch from Decay to Sustain mode.

If the voice is Keyed-OFF (or the BRR sample contains the Key-Off flag): Switch from Attack/Decay/Sustain/Gain mode to Release mode.

x7h - VxGAIN - Gain settings for Voice 0..7 (R/W) When VxADSR1.Bit7=1 (Attack/Decay/Sustain Mode):

```text
  0-7   Not used (instead, Attack/Decay/Sustain parameters are used)
```

When VxADSR1.Bit7=0 and VxGAIN.Bit7=0 (Direct Gain):

```text
  0-6   Fixed Volume (Envelope Level = N*16, Rate=Infinite)  ;Volume=N*16/800h
  7     Must be 0 for this mode
```

When VxADSR1.Bit7=0 and VxGAIN.Bit7=1 (Custom Gain):

```text
  0-4   Gain rate (Rate=N)
  5-6   Gain mode (see below)
  7     Must be 1 for this mode
```

In the latter case, the four Gain Modes are:

```text
  Mode 0 = Linear Decrease  ;Rate=N, Step=-32  (if Level<0 then Level=0)
  Mode 1 = Exp Decrease     ;Rate=N, Step=-(((Level-1) SAR 8)+1)
  Mode 2 = Linear Increase  ;Rate=N, Step=+32
  Mode 3 = Bent Increase    ;Rate=N, If Level<600h then Step=+32 else Step=+8
  In all cases, clip E to 0 or 0x7ff rather than wrapping.
```

The Direct/Custom Gain modes are overriding Attack/Decay/Sustain modes (when Keyed-ON), Release (when Keyed-OFF) keeps working as usually.

x8h - VxENVX - Current envelope value for Voice 0..7 (R)

```text
  0-6  Upper 7bit of the 11bit envelope volume (0..127)
  7    Not used (zero)
```

Technically, this register IS writable. But whatever value you write will be overwritten at 32000 Hz.

x9h - VxOUTX - Current sample value for Voice 0..7 (R) This returns the high byte of the current sample for this voice, after envelope volume adjustment but before VxVOL[LR] is applied.

```text
  0-7  Upper 8bit of the current 15bit sample value (-128..+127)
```

Technically, this register IS writable. But whatever value you write will be overwritten at 32000 Hz.

#### ADSR/Gain (and Noise) Rates

The various ADSR/Gain/Noise rates are defined as 5bit rate values (or 3bit/4bit values which are converted to 5bit values as described above). The meaning of these 5bit values is as follows (the table shows the number of 32000Hz sample units it takes until the next Step is applied):

```text
  00h=Stop   04h=1024  08h=384   0Ch=160   10h=64   14h=24   18h=10   1Ch=4
  01h=2048   05h=768   09h=320   0Dh=128   11h=48   15h=20   19h=8    1Dh=3
  02h=1536   06h=640   0Ah=256   0Eh=96    12h=40   16h=16   1Ah=6    1Eh=2
  03h=1280   07h=512   0Bh=192   0Fh=80    13h=32   17h=12   1Bh=5    1Fh=1
```

Note: All values are "1,3,5 SHL n". Hardware-wise they are probably implemented as 3bit counters, driven at 32kHz, 16kHz, 8kKz, etc. (depending on the shift amount); accordingly, when entering a new ADSR phase, the location of the 1st step may vary depending on when the hardware generates the next 8kHz cycles, for example.

#### Gain Notes

Even in Gain modes, the hardware does internally keep track of whether it is in Attack/Decay/Sustain phase: In attack phase it switches to Decay at Level>=7E0h. In Decay phase it switches to Sustain at Level<=Boundary (though accidently reading a garbage boundary value from VxGAIN.Bit7-5 instead of from VxADSR2.Bit7-5). Whereas, switching phases doesn't have any audible effect, it just marks the hardware as being in a new phase (and, if software disables Gain-Mode by setting VxADSR1.Bit7, then hardware will process the current phase).

Direct Gain mode can be used to set a fixed volume, or to define the starting point for a different mode (in the latter case, mind that the hardware processes I/O ports only once per sample, so, after setting Direct Gain mode: wait at least 32 cpu cycles before selecting a different mode).

#### Misc Notes

```text
  Save the new value, *clipped* to 11 bits, to determine the
  increment for GAIN Bent Increase mode next sample. Note that a
  negative value for the new value will result in the clipped
  version being greater than 0x600.
```
