# SNES Cart Satellaview Chipsets

#### BSC-1A5B9P-01 (1995) (BIOS cartridge PCB)

```text
  U1  44pin  MCC-BSC LR39197 Nintendo
  U2  36pin  ROM (36pin/40pin possible)
  U3  32pin  658512LFP-85 (4Mbit PSRAM)
  U4  28pin  LH52B256NB-10PLL (256Kbit SRAM)
  U5  8pin   MM1134 (battery controller for SRAM)
  BT1 2pin   Battery
  CN1 62pin  SNES Cartridge Edge (pin 2,33 used)
  CN2 62pin  Flash Cartridge Connector (male?)
```

There's no CIC chip (either it's contained in the MCC-chip... or in the flash card, but in that case the thing won't work without flash card?)

#### MAIN-BSA-01 (1995) (receiver unit/expansion port PCB)

```text
  U1 20pin  74LS541 8-bit 3-state buffer/line driver
  U2 20pin  74LS541 8-bit 3-state buffer/line driver
  U3 20pin  74LS245 8-bit 3-state bus transceiver
  U4 8pin   SPR-BSA (unknown, might be controlled via port 2198h or 2199h?)
  U5 100pin DCD-BSA (custom Nintendo chip)
  U6 64pin  MN88821 (maybe a MN88831 variant: Satellite Audio Decoder)
  U7 18pin  AN3915S Clock Regenerator (for amplifying/stabilizing Y1 crystal)
  U8 4pin   PQ05RH1L (5V regulator with ON/OFF control)
  U9 14pin  LM324 Quad Amplifier
  Y1 2pin   18.432MHz crystal
  T1 4pin   ZJYS5102-2PT Transformator
  T2 4pin   ZJYS5102-2PT Transformator
  CN1 28pin SNES Expansion Port
  CN2 38pin Expansion Port (EXT) (believed to be for modem)
  CN3 3pin  To POWER and ACCESS LEDs on Front Panel
  CN4 7pin  Rear connector (satellite and power supply?)
```

#### BSMC-AF-01 (Memory Card PCB) (to be plugged into BIOS cartridge)

```text
  U1  56pin Sharp LH28F800SUT-ZI (or -Z1?) (1Mbyte FLASH)
  CN1 62pin Flash Cartridge Connector (female?)
```

There are no other chips on this PCB (only capacitors and resistors).

#### BSMC-CR-01 (Memory Card PCB) (to be plugged into GAME cartridges)

```text
  U1  ?pin  unknown (reportedly read-only... mask ROM?)
  CN1 62pin Flash Cartridge Connector (female?)
```

#### BSC-1A5M-01 (1995) (GAME cartridge with onboard FLASH cartridge slot)

```text
  U1  36pin  ROM
  U2  28pin  SRAM (32Kbytes)
  U3  16pin  MAD-1A
  U4  16pin  CIC D411B
  BT1 2pin   Battery CR2032
  CN1 62pin  SNES Cartridge Edge (pin 2,33 used)
  CN2 62pin  Flash Cartridge Connector (male 2x31 pins)
```

Used by "Derby Stallion 96" (and maybe other games, too).

BSC-1L3B-01 (1996) (GAME cartridge with SA1 and onboard FLASH cartridge slot)

```text
  U1  44pin  ROM
  U2  28pin  SRAM (8Kbytes)
  U3  128pin SA1
  U4  8pin   MM1026AF (battery controller for SRAM)
  BT1 2pin   Battery
  CN1 62pin  SNES Cartridge Edge (pin 2,33 used)
  CN2 62pin  Flash Cartridge Connector (male?)
```

Used by "Itoi Shigesato no Bass Tsuri No. 1" (and maybe other games, too).

#### Nintendo Power flashcarts

Theoretically, Nintendo Power flashcarts are also compatible with the BSX expansion hardware (in terms of connecting EXPAND to SYSCK via 100 ohms), unknown if any Nintendo Power titles did actually use that feature.
