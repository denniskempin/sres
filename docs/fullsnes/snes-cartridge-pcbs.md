# SNES Cartridge PCBs

#### Cartridge PCB Naming (eg. SHVC-XXXX-NN)

#### Prefix

```text
  SHVC  Normal cartridge (japan, usa, europe)
  SNSP  Special PAL version (for SA1 and S-DD1 with built-in CIC)
  BSC   BIOS (or game cartridge) with external Satellaview FLASH cartridge slot
  MAXI  Majesco Sales Inc cartridge (Assembled in Mexico)
  MJSC  Majesco Sales Inc cartridge (Assembled in Mexico)
  WEI   Whatever? (Assembled in Mexico)
  EA    Electronics Arts cartridge
```

#### First Character

```text
  1   One ROM chip (usually 36pin, sometimes 32pin)
  Y   Two 4Mbit ROM chips     (controlled by 74LS00)
  2   Two 8Mbit ROM chips     (controlled by 74LS00,MAD-1,etc.)
  B   Two 16Mbit ROM chips    (controlled by 74LS00 or MAD-1)
  L   Two 32Mbit ROM chips    (controlled by SPC7110F,S-DD1 or MAD-1)
  3   Three 8Mbit ROM chips   (controlled by 74LS139) (decoder/demultiplexer)
  4   Four ROM chips (used only for 4PVnn/4QW EPROM prototype boards)
  8   Eight ROM chips (used only for 8PVnn/8Xnn EPROM prototype boards)
```

#### Second Character(s)

```text
  A   LoRom (A15 / Pin40 not connected to ROM)   (uh, 1A3B-20 ?)
  B   LoRom plus DSP-N chip
  C   LoRom plus Mario Chip 1               62pin  (and 36pin ROM) (no X1)
  CA  LoRom plus GSU-1                      62pin  (and 32pin ROM)
  CB  LoRom plus GSU-2 or GSU-2-SP1         62pin  (and 40pin ROM)
  DC  LoRom plus CX4                        62pin  (and 32pin ROMs)
  DH  HiRom plus SPC7110F                   62pin  (and 32pin+44pin ROMs)
  DE  LoRom plus ST018                      62pin  (and 32..40pin ROM possible)
  DS  LoRom plus ST010/ST011                62pin  (and 32..36pin ROM possible)
  E   LoRom plus OBC1 chip                  62pin  (and 32pin ROMs)
  J   HiRom (A15 / Pin40 is connected to ROM)
  K   HiRom plus DSP-N chip
  L   LoRom plus SA1 chip       62pin  (and 44pin ROM) (16bit data?)
  N   LoRom plus S-DD1 chip     62pin  (and 44pin ROM)
  P   LoRom with 2 prototype EPROMs (=unlike ROM A16..Ahi,/CS)
  PV  WhateverRom with 4 prototype EPROMs (=unlike ROM A16..Ahi,/CS)
  Q   WhateverRom prototype (see book2.pdf)
  QW  WhateverRom prototype (see book2.pdf)
  RA  LoRom plus GSU1A with prototype EPROMs (=unlike ROM A16..Ahi,/CS)  62pin
  X   WhateverRom prototype (see book2.pdf)
```

#### Third Character

```text
  0   No SRAM
  1   2Kx8 SRAM (usually narrow 24pin DIP, sometimes wide 24pin DIP)
  2   prototype variable size SRAM
  3   8Kx8 SRAM (usually wide 28pin DIP)
  5   32Kx8 SRAM (usually wide 28pin DIP)
  6   64Kx8 SRAM (32pin SMD, found on boards with GSU)
  8   64Kx8 SRAM (in one SA1 cart) (seems to be a 64Kx8 chip, not 256Kx8)
```

#### Forth Character

```text
  N   No battery
  B   Battery (with Transistor+Diodes or MM1026/MM1134 chip)
  M   Battery (with MAD-1 chip; or with rare MAD-R chip)
  X   Battery (with MAD-2 chip; maybe amplifies X1 oscillator for DSP1B chips)
  C   Battery and RTC-4513
  R   Battery and S-RTC
  F   FLASH Memory (instead of SRAM) (used by JRA-PAT and SPAT4)
```

Fifth Characters (only if cart contains RAM that is NOT battery-backed)

```text
  5S  32Kx8 SRAM (for use by GSU, not battery backed)
  6S  64Kx8 SRAM (for use by GSU, not battery backed)
  7S  64Kx8 or 128Kx8 SRAM (for use by GSU, not battery backed)
  9P  512Kx8 PSRAM (32pin 658512LFP-85) (for satellaview)
  (the black-blob Star Fox PCB also contains RAM, but lacks the ending "nS")
```

#### Suffix

```text
  -NN revision number (unknown if this indicates any relevant changes)
```

#### Other Cartridge PCBs (that don't follow the above naming system)

```text
  CPU2 SGB-R-10  Super Gameboy (1994)
  SHVC-MMS-X1    Nintendo Power FLASH Cartridge (1997) (older version)
  SHVC-MMS-02    Nintendo Power FLASH Cartridge (1997) (newer version)
  SHVC-MMSA-1    Nintendo Power FLASH Cartridge (19xx) ???
  SHVC-SGB2-01   Super Gameboy 2 (1998)
  SHVC-1C0N      Star Fox (black blob version) (PCB name lacks ending nS-NN)
  SHVC TURBO     Sufami Turbo BASE CASSETTE (Bandai)
  <unknown?>     Sufami Turbo game cartridges
  123-0002-16    X-Band Modem (1995 by Catapult / licensed by Nintendo)
  BSMC-AF-01     Satellaview Mini FLASH Cartridge (plugged into BIOS cartridge)
  BSMC-CR-01     Satellaview Mini FLASH Cartridge (???) (not rewriteable ?)
  GPC-RAMC-4M    SRAM Cartridge (without ROM)?
  GPC-RAMC-S1    SRAM Cartridge (without ROM)?
  GS 0871-102    Super Famicom Box multi-game cartridge
  NSS-01-ROM-A   Nintendo Super System (NSS) cartridge
  NSS-01-ROM-B   Nintendo Super System (NSS) cartridge
  NSS-01-ROM-C   Nintendo Super System (NSS) cartridge
  NSS-X1-ROM-C   Rebadged NSS-01-ROM-C board (plus battery/sram installed)
  RB-01, K-PE1-945-01   SNES CD Super Disc BIOS Cartridge (prototype)
```

#### ROM Chips used in SNES cartridges

```text
  2Mbit 256Kbyte        LH532 TC532 N-2001 (2nd chip/2A0N) (+SGB) (+Sufami)
  4Mbit 512Kbyte 23C401 LH534 TC534 HN623n4 HN623x5 23C4001 LH5S4 CAT534 CXK384
  8Mbit  1Mbyte  23C801 LH538 TC538 HN623n8 23C8001 TC23C8003 CAT548
  16Mbit 2Mbyte  23C1601 LH537 LHMN7 TC5316 M5316
  24Mbit 3Mbyte  23C2401 (seen on SHVC-1J3M board)
  32Mbit 4Mbyte  23C3201 LH535 LHMN5 M5332 23C3202/40pin/SA1 N-32000/44pin/DD1
```

#### DIP vs SMD vs Blobs

Most SNES carts are using DIP chips. SMD chips are used only in carts with coprocessors (except S-RTC, DSP-n, ST01n). Black blobs are found in several pirate carts (and in Star Fox, which contains Nintendo's Mario Chip 1, so it's apparently not a pirate cart).
