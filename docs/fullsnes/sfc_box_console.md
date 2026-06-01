# SFC-Box Component List (Console)

#### SFC-Box Mainboard "MAIN 0871-100A"  Section 1 (CPU/PPU) (Front-Left)

```text
  U1  100pin S-CPU B
  U2  100pin S-PPU1
  U3  100pin S-PPU2 C
  U4  28pin  LH2A256N-10PLL (Mosel-Vitelic, VRAM, 32Kx8)
  U5  28pin  LH2A256N-10PLL (Mosel-Vitelic, VRAM, 32Kx8)
  U6  64pin  S-WRAM A
  U7  24pin  S-ENC A (near APU section)
  U8         N/A ?
  U9  14pin  74HCU04
  U10 8pin   unknown (maybe audio amplifier) (near relay)
  U11 8pin   unknown (maybe audio amplifier) (near relay)
  X1  D21M4 Oscillator (21.47727MHz for S-CPU)
  TC1 Red Trimmer (for above oscillator)
```

#### Section 2 (APU) (Rear-Left)

```text
  IC1 64pin  S-SMP
  IC2 80pin  S-DSP A
  IC3 28pin  HM9453100FP (APU-RAM, 32Kx8)
  IC4 28pin  HM9453100FP (APU-RAM, 32Kx8)
  IC5 16pin  NEC uPD6376 (serial audio D/A converter)
  IC6 8pin   unknown (maybe audio amplifier)
  IC72 28pin MB90082-001 (OSD video controller) (near S-ENC A)
  X2  Blue oscillator (maybe 24.576MHz for APU?)
  X4  D143A4 oscillator (maybe separate NTSC color clock 3.579545MHz mul 4)
  TC2 Trimmer (for X4/D143A4)
  TC3 Trimmer (for IC72/OSD-Chip pin16)
```

#### Section 3 (Rear-Right)

```text
  IC30 80pin  HD64180RF6X (extended Z80 CPU)
  X3  D921B4 oscillator (9.216MHz) (ie. HD64180 clocked at PHI=4.608MHz)
  24 small logic chips (details unknown) (plenty 74HCxxx & 74LSxxx)
```

#### Section 4 (Front-Right)

```text
  18 small logic chips (details unknown)
```

#### Connectors

```text
  CN1 100pin cartridge slot (2x50pin male) (via adaptor to TWO 2x50 slots)
  CN2 44pin  daugtherboard socket (2x22pin male)
  CN3 3pin   unknown/unused (without cable?) (front-right) (coin mechanics?)
  CN4 5pin   Yellow Cable to Front Panel (FR 0871-105) (front-middle)
  CN5 7pin   Yellow Cable to 6-position Keyswitch (front-middle)
  CN6 11pin  Multi-colored Cable (to joypad connectors) (front-left)
  CN? 7pin   Yellow Cable to Modulator (rear-left)
  CN8 6pin   unknown/unused (without cable?) (front-right) (maybe RS232 ???)
      2pin   Black cable to "Nintendo AC Adapter" (Input: DC 5V 10W) (rear)
      2pin   RCA Audio Out Left (rear)
      2pin   RCA Audio Out Right (rear)
```

#### Options & Specials

```text
  SP1 3pin   unknown / jumper (near HD64180) (usually two pins bridged)
  SP2 2pin   unknown / jumper or so (near OSD chip) (usually not bridged)
  SP3 3x5pin unknown / not installed (located in center of mainboard)
  JP1,JP2,JP3,JP4,JP5 - seem to allow to disconnect Shield from GND
  TR1 3pin   unknown (big transistor or so)
  ?   8pin   OMRON G5V-2-5VDC (dual two-position relay) (rear-left)
```

#### SFC-Box Daughterboard "(unknown PCB name)" (Modulator)

Shielded box with whatever contents, plus external connectors:

```text
  2pin   RCA Audio Out Mono      ;\raw A/V (stereo is also available, via
  2pin   RCA Video Out Composite ;/the external connectors on mainboard)
  2pin   RF Out                  ;\
  2pin   ANT In                  ; RF modulated
  ?pin   Channel Select 1CH/2CH  ;/
  7pin   Yellow Cable to Mainboard
```

SFC-Box Daughterboard "Nintendo AC Adapter" (Power Supply) Remove-able metal box with whatever contents, plus external connectors:

```text
  4pin AC OUT 200W MAX (dual 2pin or so)
  2pin DC OUT 5V 5A (via short EXTERNAL cable to Mainboard)
  2pin AC IN (cable to wall socket)
  AC125 5A (Fuse?)
```

#### SFC-Box Daughterboard "PU 0871-101"

```text
  IC1  28pin  DIP 27C512 EPROM "KROM 1" (usually 28pin; 28pin/32pin possible)
  IC2  28pin  SMD SRM20257 (SRAM 32Kx8) (Work-RAM for HD64180, battery-backed)
  IC3  16pin  SMD 74HC139
  IC4  20pin  SMD 74HC273D (Philips)
  IC5  20pin  SMD 74LS541
  IC6  16pin  SMD MB3790 (Fujitsu battery controller)
  IC7  14pin  SMD S-3520CF (Seiko RTC, Real Time Clock, battery-backed)
  IC8  14pin  SMD <unknown>
  IC9  14pin  SMD <unknown>
  X1   2pin   Oscillator (for RTC) ("S441") (probably 32kHz or so)
  TC1         Osc-Adjust (for RTC)
  BAT  2pin   Battery (for IC7/RTC and IC2/SRAM)
  TM1  8pin   Massive connector (not installed) (maybe FamicomBox-style CATV?)
  CN1  44pin  DIP Female Connector 2x22pin (to Mainboard)
```

#### SFC-Box Daughterboard "GD 0871-103" (Game Cartridge Connectors)

```text
  CN?  100pin DIP Female Connector 2x22pin (to Mainboard)
  CN?  100pin DIP Male Connector 2x22pin (to Cartridge 1)
  CN?  100pin DIP Male Connector 2x22pin (to Cartridge 2)
```

#### SFC-Box Daughterboard "CC 0871-104" (Controller Connectors)

```text
  11pin  Multicolored Cable to Mainboard
  7pin   Controller 1  ;\Standard SNES Joypad connectors (for two standard
  7pin   Controller 2  ;/joypads with extra-long cables)
```

#### SFC-Box Daughterboard "FR 0871-105" (Front Panel)

```text
  TV-LED and GAME-LED
  GAME/TV-Button
  RESET-Button
  5pin Yellow Cable (to Mainboard)
```

#### SFC-Box Daughterboard "(unknown PCB name)" (Keyswitch)

```text
  10-Position Keyswitch (requires a key) (6-positions connected)
  7pin Yellow Cable to (to Mainboard) (one common pin, plus 6 switch positions)
```

The 10-position keyswitch is mechanically limited to 6 positions (9,0,1,2,3,4).

There are different keys for different purposes. For example, a "visitor" key can select only the "ON" and "OFF" positions. The right-most position does switch a relay, which does... what? Probably switch-off the SFC-Box?
