# NSS Component Lists

#### Cartridge PCB "NSS-01-ROM-A" (1991 Nintendo)

```text
  IC1   32pin  PRG ROM (LH534J ROM or TC574000 EPROM) (512Kx8 LoROM)
  IC2   16pin  74HC367 (2bit + 4bit drivers) (unknown purpose... for PROM?)
  IC3   28pin  INST-ROM (27C256) (32Kx8 EPROM)
  IC4   8pin   Key-Chip (RP5H01 serial 72bit PROM)
  CL/SL 2pin   Jumpers (see notes)
  CN?   100pin Cartridge connector (2x50pin)
```

Used by Super Mario World (ROM), Super Tennis (ROM), and Super Soccer (EPROM).

For ROM: Short CL1-CL5, Open SL1-SL5. For EPROM: Short SL1-SL5, Open CL1-CL5.

#### Cartridge PCB "NSS-01-ROM-B" (1991 Nintendo)

```text
  IC1   28pin  SRAM (LH5168FB-10L)
  IC2   32pin  PRG ROM (LH534J ROM) (512Kx8 LoROM)
  IC3   16pin  74LS139 (demultiplexer) (for ROM vs SRAM mapping)
  IC4   16pin  74HC367 (2bit + 4bit drivers) (unknown purpose... for PROM?)
  IC5   14pin  74HC27 (3x3 NOR) (for SW1) (not installed on the F-Zero board)
  IC6   14pin  74HC10 (3x3 NAND)(for SW1) (not installed on the F-Zero board)
  IC7   20pin  74HC540 (inv.drv)(for SW1) (not installed on the F-Zero board)
  IC8   28pin  INST-ROM (27C256) (32Kx8 EPROM)
  IC9   8pin   Key-Chip (RP5H01 serial 72bit PROM)
  SW1   16pin  DIP-Switch (8 switches)    (not installed on the F-Zero board)
  AR1   9pin   Resistor network (for SW1) (not installed on the F-Zero board)
  BAT1  2pin   Battery (CR2032 3V coin) (with socket)
  CL/SL 2pin   Jumpers (see notes)
  CN?   100pin Cartridge connector (2x50pin)
```

Used only by F-Zero. For that game: Short CL1-CL7, Open SL1-SL7. Other settings might allow to use EPROM instead ROM, or to change ROM/SRAM capacity.

#### Cartridge PCB "NSS-01-ROM-C" (1992 Nintendo)

Judging from low-res photos, the PCB is basically same as NSS-01-ROM-B, but with two PRG ROM chips (for double capactity). Exact components are unknown, except for a few ones:

```text
  IC1   28pin  SRAM (6116, 2Kx8) (DIP24 in 28pin socket?) (Contra III only)
  IC2   32pin  PRG-ROM-1 (TC574000 EPROM) (512Kx8 LoROM, upper half)
  IC3   32pin  PRG-ROM-0 (TC574000 EPROM) (512Kx8 LoROM, lower half)
  IC4   16pin  74LS139 (demultiplexer) (for ROM vs SRAM mapping)
  IC5   16pin  74HC367 (2bit + 4bit drivers) (unknown purpose... for PROM?)
  IC6   14pin  74HC27 (3x3 NOR) (for SW1)
  IC7   14pin  74HC10 (3x3 NAND)(for SW1)
  IC8   28pin  INST ROM (27C256) (32Kx8 EPROM)
  IC9   20pin  74HC540 (inv.drv)(for SW1)
  IC10  8pin   Key-Chip (RP5H01 serial 72bit PROM)
  SW1?  16pin  DIP-Switch (8 switches)  (installed)
  AR1   9pin   Resistor network for SW1 (installed)
  BAT1? 2pin   Battery (CR2032 3V coin) (with socket) (Contra III only)
  CL/SL 2pin   Jumpers (see notes)
  CN?   100pin Cartridge connector (2x50pin)
```

Used by ActRaiser, Addams Family, Amazing Tennis, Irem Skins Game, Lethal Weapon, NCAA Basketball, Robocop 3 (all without SRAM), and, by Contra III (with SRAM). Default (for all those games) is reportedly: Short CL2-CL6,CL12-CL13,CL15,CL17-CL19, Open SL1,SL7-SL12,SL14,SL16,SL20-SL22.

DIP Switches are usually/always installed. Battery/SRAM is usually NOT installed, except on the Contra III cartridge (which has "NSS-01-ROM-C" PCB rebadged as "NSS-X1-ROM-C" with a sticker).

Mainboard NSS-01-CPU MADE IN JAPAN (C) 1991 Nintendo Below lists only the main chipset (not the logic chips; which are mostly located on the bottom side of the PCB).

#### Standard SNES Chipset

```text
  S-CPU 5A22-02 (QFP100)
  S-PPU1 5C77-01 (QFP100)
  S-PPU2 5C78-01 (QFP100)
  S-WRAM LH68120 (SOP64) 128Kx8 DRAM with sequential access feature (SNES WRAM)
  Fujitsu MB84256-10L 32Kx8 SRAM (SOP28) (SNES VRAM LSBs)
  Fujitsu MB84256-10L 32Kx8 SRAM (SOP28) (SNES VRAM MSBs)
```

#### NSS/Z80 Specific Components

```text
  Zilog Z84C0006FEC Z80 CPU, clock input 4.000MHz (QFP44)
  27C256 32Kx8 EPROM "NSS-C_IC14_02" (DIP28) (Z80 BIOS)
  Sharp LH5168N-10L 8Kx8 SRAM (SOP28) (Z80 WRAM)
  Mitsubishi M50458-001SP On-Screen Display (OSD) Chip (NDIP32)
  Mitsubishi M6M80011 64x16 Serial EEPROM (DIP8)
   (Pinout: 1=CS, 2=CLK, 3=DATA IN, 4=DATA OUT, 5=VSS, 6=RESET, 7=RDY, 8=VCC)
  Seiko Epson S-3520 Real Time Clock (SOIC14)
```

#### Amplifiers/Converters/Battery and so

```text
  Sharp IR3P32A (chroma/luma to RGB converter... what is that for???) (NDIP30)
  Hitachi HA13001 Dual 5.5W Power Amplifier IC
  Matsushita AN5836 DC Volume and Tone Control IC (SIL12)
  Mitsumi Monolithic MM1026BF Battery Controller (SOIC8) (on PCB bottom side)
  5.5V - 5.5 volt supercap
```

#### Oscillators

```text
  21.47724MHz SNES NTSC Master Clock <-- not 21.47727MHz, unlike NTSC (?)
  14.31818MHz (unknown purpose, maybe for OSD chip or RGB converter or so)
  4.000MHz for Z80 CPU
  32.678kHz for RTC
  <unknown clock source> for OSD Dotclock
```

#### Connectors

```text
  CN1 - 2x28 pin connector - "JAMMA" - Audio/Video/Supply/Coin/Joypad
  CN2 - 10 pin connector - 10P Connector (Extra Joypad Buttons)
  CN3 - 13 pin connector - 13P Connector (Front Panel LEDs/Buttons)
  CN4 - 8 pin connector - alternate player 2 controller (eg. lightgun) (unused)
  CN5 - 7 pin connector - external 5bit input (Port 02h.R.bit3-7) (unused)
  CN6 - 24 pin connector (to APU daughterboard)
  CN11/12/13 - 2x50 pin connectors for game cartridges
```

#### Jumpers

```text
  SL1/SL2/SL3/CL1/CL2 - Mono/stero mode (for details see PCB text layer)
  SL4 - Use Audio+ (pin 11 on edge connector)
  SL5 - Unknown purpose
  TB1 - Z80 Watchdog Disable
```

APU Daughterboard (shielded unit, plugged into CN6 on mainboard)

```text
  Nintendo S-SMP (M) SONY (C) Nintendo '89' (QFP80) (SNES SPC700 CPU)
  Nintendo S-DSP (M) (C) SONY '89' (QFP80) (SNES sound chip)
  Toshiba TC51832FL-12 32Kx8 SRAM (SOP28) (1st half of APU RAM)
  Toshiba TC51832FL-12 32Kx8 SRAM (SOP28) (2nd half of APU RAM)
  Japan Radio Co. JRC2904 Dual Low Power Op Amp (SOIC8)
  NEC D6376 Audio 2-Channel 16-Bit D/A Converter (SOIC16)
  CN1 - 24 pin connector (to CN6 on mainboard)
  <unknown clock source> for APU (probably SNES/APU standard 24.576MHz)
```
