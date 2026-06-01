# SNES APU DSP BRR Pitch

x2h - VxPITCHL - Pitch scaler for Voice 0..7, lower 8bit (R/W) x3h - VxPITCHH - Pitch scaler for Voice 0..7, upper 6bit (R/W)

```text
  0-13  Sample rate (0=stop, 3FFFh=fastest) (1000h = 32000Hz)
  14-15 Not used (read/write-able)
```

Defines the BRR sample rate. This register (and PMON) does affect only the BRR sample frequency (but not on the Noise frequency, which is defined - and shared for all voices - in the FLG register).

2Dh - PMON - Pitch Modulation Enable Flags for Voice 1..7 (R/W) Pitch modulation allows to generate "Frequency Sweep" effects by mis-using the amplitude from channel (x-1) as pitch factor for channel (x).

```text
  0    Not used
  1-7  Flags for Voice 1..7 (0=Normal, 1=Modulate by Voice 0..6)
```

For example, output a very loud 1Hz sine-wave on channel 4 (with Direct Gain=40h, and with Left/Right volume=0; unless you actually want to output it to the speaker). Then additionally output a 2kHz sine wave on channel 5 with PMON.Bit5 set. The "2kHz" sound should then repeatedly sweep within 1kHz..3kHz range (or, for a more decent sweep in 1.8kHz..2.2kHz range, drop the Gain level of channel 4).

#### Pitch Counter

The pitch counter is adjusted at 32000Hz rate as follows:

```text
  Step = VxPitch                   ;range 0..3FFFh (0..128 kHz)
  IF PMON.Bit(x)=1 AND (x>0)       ;pitch modulation enable
    Factor = VxOUTX(x-1)           ;range -4000h..+3FFFh (prev voice amplitude)
    Factor = (Factor SAR 4)+400h   ;range +000h..+7FFh (factor = 0.00 .. 1.99)
    Step = (Step * Factor) SAR 10  ;range 0..7FEEh (0..256 kHz)
```

XXX somewhere here, STEP (or the COUNTER-RESULT) is cropped to 128kHz max) XX?

```text
  Counter = Counter + Step         ;range 0..FFFFh, carry=next BRR block
```

Counter.Bit15-12 indicates the current sample (within a BRR block).

Counter.Bit11-3 are used as gaussian interpolation index.

#### Maximum Sound Frequency

The pitch counter generates sample rates up to 128kHz. However, the Mixer and DAC are clocked at 32kHz, so the higher rates will skip (or interpolate) samples rather than outputting all samples.

The 32kHz output rate means that one can produce max 16kHz tones.

#### 4-Point Gaussian Interpolation

Interpolation is applied on the 4 most recent 15bit BRR samples (new,old,older,oldest), using bit4-11 of the pitch counter as interpolation index (i=00h..FFh):

```text
  out =       ((gauss[0FFh-i] * oldest) SAR 10) ;-initial 16bit value
  out = out + ((gauss[1FFh-i] * older)  SAR 10) ;-no 16bit overflow handling
  out = out + ((gauss[100h+i] * old)    SAR 10) ;-no 16bit overflow handling
  out = out + ((gauss[000h+i] * new)    SAR 10) ;-with 16bit overflow handling
  out = out SAR 1                               ;-convert 16bit result to 15bit
```

The Gauss table contains the following values (in hex):

```text
  000,000,000,000,000,000,000,000,000,000,000,000,000,000,000,000  ;\
  001,001,001,001,001,001,001,001,001,001,001,002,002,002,002,002  ;
  002,002,003,003,003,003,003,004,004,004,004,004,005,005,005,005  ;
  006,006,006,006,007,007,007,008,008,008,009,009,009,00A,00A,00A  ;
  00B,00B,00B,00C,00C,00D,00D,00E,00E,00F,00F,00F,010,010,011,011  ;
  012,013,013,014,014,015,015,016,017,017,018,018,019,01A,01B,01B  ; entry
  01C,01D,01D,01E,01F,020,020,021,022,023,024,024,025,026,027,028  ; 000h..0FFh
  029,02A,02B,02C,02D,02E,02F,030,031,032,033,034,035,036,037,038  ;
  03A,03B,03C,03D,03E,040,041,042,043,045,046,047,049,04A,04C,04D  ;
  04E,050,051,053,054,056,057,059,05A,05C,05E,05F,061,063,064,066  ;
  068,06A,06B,06D,06F,071,073,075,076,078,07A,07C,07E,080,082,084  ;
  086,089,08B,08D,08F,091,093,096,098,09A,09C,09F,0A1,0A3,0A6,0A8  ;
  0AB,0AD,0AF,0B2,0B4,0B7,0BA,0BC,0BF,0C1,0C4,0C7,0C9,0CC,0CF,0D2  ;
  0D4,0D7,0DA,0DD,0E0,0E3,0E6,0E9,0EC,0EF,0F2,0F5,0F8,0FB,0FE,101  ;
  104,107,10B,10E,111,114,118,11B,11E,122,125,129,12C,130,133,137  ;
  13A,13E,141,145,148,14C,150,153,157,15B,15F,162,166,16A,16E,172  ;/
  176,17A,17D,181,185,189,18D,191,195,19A,19E,1A2,1A6,1AA,1AE,1B2  ;\
  1B7,1BB,1BF,1C3,1C8,1CC,1D0,1D5,1D9,1DD,1E2,1E6,1EB,1EF,1F3,1F8  ;
  1FC,201,205,20A,20F,213,218,21C,221,226,22A,22F,233,238,23D,241  ;
  246,24B,250,254,259,25E,263,267,26C,271,276,27B,280,284,289,28E  ;
  293,298,29D,2A2,2A6,2AB,2B0,2B5,2BA,2BF,2C4,2C9,2CE,2D3,2D8,2DC  ;
  2E1,2E6,2EB,2F0,2F5,2FA,2FF,304,309,30E,313,318,31D,322,326,32B  ; entry
  330,335,33A,33F,344,349,34E,353,357,35C,361,366,36B,370,374,379  ; 100h..1FFh
  37E,383,388,38C,391,396,39B,39F,3A4,3A9,3AD,3B2,3B7,3BB,3C0,3C5  ;
  3C9,3CE,3D2,3D7,3DC,3E0,3E5,3E9,3ED,3F2,3F6,3FB,3FF,403,408,40C  ;
  410,415,419,41D,421,425,42A,42E,432,436,43A,43E,442,446,44A,44E  ;
  452,455,459,45D,461,465,468,46C,470,473,477,47A,47E,481,485,488  ;
  48C,48F,492,496,499,49C,49F,4A2,4A6,4A9,4AC,4AF,4B2,4B5,4B7,4BA  ;
  4BD,4C0,4C3,4C5,4C8,4CB,4CD,4D0,4D2,4D5,4D7,4D9,4DC,4DE,4E0,4E3  ;
  4E5,4E7,4E9,4EB,4ED,4EF,4F1,4F3,4F5,4F6,4F8,4FA,4FB,4FD,4FF,500  ;
  502,503,504,506,507,508,50A,50B,50C,50D,50E,50F,510,511,511,512  ;
  513,514,514,515,516,516,517,517,517,518,518,518,518,518,519,519  ;/
```

The gauss table is slightly bugged: Theoretically, each four values (gauss[000h+i], gauss[0FFh-i], gauss[100h+i], gauss[1FFh-i]) should sum up to

```text
800h, but in practice they do sum up to 7FFh..801h. Of which, 801h can cause
```

math overflows. For example, when outputting three or more "-8 SHL 12" BRR samples with Filter 0, some interpolation results will be +3FF8h (instead of -4000h).

When adding the four new,old,older,oldest values there's some partial overflow handling: The 1st addition can't overflow, the 2nd addition can overflow (when i=0..1Fh), the 3rd addition can overflow (when i=20h..FFh). Of which, the 2nd one bugs, the 3rd one is saturated to Min=-8000h/Max=+7FFFh (giving -4000h/+3FFFh after the final SAR 1).

#### Waveform Examples

```text
  Incoming BRR Data ---> Interpolated Data
   _   _   _   _
  | | | | | | | |         .   .   .   .    Nibbles=79797979, Shift=12, Filter=0
  | | | | | | | |   ---> / \ / \ / \ / \   HALF-volume ZIGZAG-wave
  | |_| |_| |_| |_          '   '   '   '
   ___     ___
  |   |   |   |            .'.     .'.     Nibbles=77997799, Shift=12, Filter=0
  |   |   |   |     --->  /   \   /   \    FULL-volume SINE-wave
  |   |___|   |___       '     '.'     '.
   _______                   ___
  |       |                .'   '.         Nibbles=77779999, Shift=12, Filter=0
  |       |         --->  /       \        SQUARE wave (with rounded edges)
  |       |_______       '         '.____
   _______                   ___
  |       |                .'   '.   | |   Nibbles=77778888, Shift=12, Filter=0
  |       |         --->  /       \  | |   OVERFLOW glitch on -4000h*801h
  |       |_______       '         '.|_|_
   _____         _           __
  |     |_     _|          .'  ''.    .'   Nibbles=7777CC44, Shift=12, Filter=0
  |       |___|     --->  /       '..'     CUSTOM wave-form
  |                      '
   ___     __
  |   |___|  |    _       \ ! /  .  \ ! /  Nibbles=77DE9HZK, Shift=3M, Filter=V
  |_     ____|  _|  --->  - + -  +  - + -  SOLAR STORM wave-form
  __|   |______|___       / ! \  '  / ! \
```
