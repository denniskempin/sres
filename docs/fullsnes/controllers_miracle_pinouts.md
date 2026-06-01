# SNES Controllers Miracle Pinouts and Component List

#### 25pin SUBD connector (J6)

```text
  1  PC/Amiga/Mac RS232 GND (also wired to RTS)
  2  PC/Amiga/Mac RS232 RxD
  3  PC/Amiga/Mac RS232 TxD
  7  NES/SNES/Genesis GND
  10 NES/SNES/Genesis Data
  13 NES/SNES/Genesis Strobe
  14 Sense SENSE0 (0=MIDI Output off, 1=MIDI Output on)
  15 Sense SENSE1 (0=9600 Baud; for RS232, 1=31250 Baud; for MIDI)
  19 NES/SNES/Genesis Clock
  all other pins = not connected
```

For PC/Mac RS232 wire SENSE0=GND, SENSE1=GND

#### Miracle NES and SNES Cartridges

According to the ROM Headers: The SNES cartridge contains 512Kbyte Slow/LoROM, and no SRAM (nor other storage memory). The NES cartridge contains MMC1 mapper, 256Kbyte PRG-ROM, 64Kbyte CHR-ROM, and no SRAM (nor other storage memory).

Miracle Piano Component List (Main=Mainboard Section, Snd=Sound Engine)

```text
  U1   Snd  16pin TDA7053 (stereo amplifier for internal speakers)
  U2   Snd   8pin NE5532 (dual operational amplifier)
  U3   Snd  16pin LM13700 or LM13600 (unclear in schematic) (dual amplifier)
  U4   Snd  14pin LM324 (quad audio amplifier)
  U5   Main  3pin LM78L05 (converts +10V to VLED, supply for 16 LEDs)
  U6   Main 14pin 74LS164 serial-in, parallel-out (to 8 LEDs)
  U7   Main 14pin 74LS164 serial-in, parallel-out (to another 8 LEDs)
  U8   Main  5pin LM2931CT (converts +12V to +10V, and supply for Power LED)
  U9   Main  3pin LM78L05 (converts +10V to +5REF)
  U10  Snd  14pin TL084 (JFET quad operational amplifier)
  U11  Snd  40pin J004 (sound chip, D/A converter with ROM address generator)
  U12  Snd  32pin S631001-200 (128Kx8, Sound ROM for D/A conversion)
  U13  Main  3pin LM78L05 (converts +10V to VCC, supply for CPU and logic)
  U14  Main 40pin AS0012 (ASIC) Keyboard Interface Chip (with A/D for velocity)
  U15  Main 40pin 8032 (8051-compatible CPU) (with Y1=12MHz)
  U16  Snd  40pin AS0013 (ASIC)
  U17  Main 28pin 27C256 EPROM 32Kx8 (Firmware for CPU)
  U18  Main 28pin 6264 SRAM 8Kx8 (Work RAM for CPU)
  U19  Main 16pin LT1081 Driver for RS232 voltages
  U20  Main  8pin 6N138 opto-coupler for MIDI IN signal
  S1-8 Main  2pin Push Buttons
  S9   Main  3pin Power Switch (12V/AC)
  J1   Main  3pin 12V AC Input (1 Ampere)
  J2   Main  2pin Sustain Pedal Connector (polarity is don't care)
  J3   Snd   2pin RCA Jack Right
  J4   Snd   2pin RCA Jack Left
  J5   Snd   5pin Headphone jack with stereo switch (mutes internal speakers)
  J6   Main 25pin DB25 connector (RS232 and SNES/NES/Genesis controller port)
  J7   Main  5pin MIDI Out (DIN)
  J8   Main  5pin MIDI In (DIN)
  JP1  Main 16pin Keyboard socket right connector
  JP2  Main 16pin Keyboard socket left connector
  JP3  Snd   4pin Internal stereo speakers connector
```

Note: The official original schematics are released & can be found in internet.
