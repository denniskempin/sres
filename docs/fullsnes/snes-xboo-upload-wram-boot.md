# SNES Xboo Upload (WRAM Boot)

WRAM-Boot Circuit (for WRAM-boot-compatible ROMs, max 128K bytes)

```text
                   ____
  CTR.01./STB-----|AND \_____WRAM.58./PAWR    VCC-------/cut/--CPU.81.RDY
  EXT.09./PAWR----|____/                      PA7-------/cut/--WRAM.50.PA7
  CTR.36./SELECT--|XOR \_____HIGHSEL          /PAWR-----/cut/--WRAM.58./PAWR
  EXT.20.VCC -----|____/                      /WRAMSEL--/cut/--WRAM.15./WRAMSEL
  HIGHSEL---------|OR  \_____WRAM.50.PA7      /ROMSEL---/cut/--SLT.49./ROMSEL
  EXT.08.PA7 -----|____/                      CTR.01./STB-----[10K]--EXT.20.VCC
  HIGHSEL---------|OR  \___ ____              CTR.14./AUTOLF--[10K]--EXT.20.VCC
  SLT.32./WRAMSEL-|____/   |AND \___WRAM.15.  CTR.36./SELECT--[10K]--EXT.20.VCC
  CPU.77./ROMSEL--|OR  \___|____/   /WRAMSEL  CTR.01./STB-----|10n|--EXT.23.GND
  CTR.14./AUTOLF--|____/  ________*           CTR.14./AUTOLF--|10n|--EXT.23.GND
  CTR.14./AUTOLF--|XOR \_|_ ____              CTR.36./SELECT--|10n|--EXT.23.GND
  EXT.20.VCC------|____/   |OR  \___SLT.49.   CTR.31.INIT----------RESET_BUTTON
  CPU.77./ROMSEL___________|____/   /ROMSEL   CTR.36./SELECT---------CPU.81.RDY
  CTR.19-30.GND__________________EXT.23.GND   CTR.02-09.D0-D7---EXT.11-18.D0-D7
```

Extended Circuit (for larger ROMs, and for non-WRAM-boot-compatible ROMs)

```text
                   ____
  CTR.14./AUTOLF--|AND \____(XOR)          /STB------/cut/-----------(AND)
  CTR.01./STB-----|____/                   /AUTOLF---/cut/-----------(XOR)
  CTR.01./STB-----|OR  \____(AND)          /PARD-----/cut/---WRAM.56./PARD
  CTR.36./SELECT--|____/                   ________________
  CPU.77./ROMSEL--|OR  \__________________|/CS   SRAM   CS2|__VCC (if any)
  CTR.01./STB-----|____/        A0..A14___|A0..A14   D0..D7|__D0..D7
  SLT.nn.A15------|XOR \__________________|A15          /OE|__CPU.92./RD
  SLT.nn.A(hi+1)--|____/     A16..A(hi)___|A16..A(hi)   /WE|__CPU.91./WR
  CTR.14./AUTOLF--|OR  \___ ____          |________________|
  CTR.36./SELECT--|____/   |AND \___WRAM.56./PARD
  EXT.10./PARD_____________|____/
  CTR.10./ACK_______________________CPU.39.OUT2
```

Revision Nov 2012 (/RD disable, for /ROMSEL-less carts like GSU)

```text
                 ____
  * ------------|OR  \_____ SLT.23.     CPU.92./RD----/cut/---SLT.23./RD
  CPU.92./RD ---|____/      /RD
```

#### Functional Description (WRAM Boot)

RESET/INIT and RDY/SELECT are used to reset and stop the CPU, in that state, PA0-PA7 and /WRAMSEL are all LOW (of which, PA7 and /WRAMSEL need to be changed for DMA), and WRAM address is reset to zero. Data is then DMA transferred to WRAM via databus and /STB/PAWR. Finally, WRAM is mirrored (in HiROM fashion) to the ROM-region via /AUTOLF, and entrypoint and program code are fetched from WRAM.

WRAM-Boot compatible files are identified by ID "XBOO" at FFE0h in ROM Header.

WRAM-boot files should be also bootable from normal ROM (unless written by less-than-lame people).

The WRAM upload function is found in no$sns "Utility" menu. WRAM boot works only if a regular cartride is inserted, or if the lockout chip is disabled.

Normal ROM-cartridges can be used if the cable is disconnected, or if the parallel port is set to /STB=HIGH, /AUTOLF=HIGH, /SELECT=HIGH, /INIT=LOW, and DATA=HIGH-Z.

#### Functional Description (Extended Circuit)

Data is uploaded (in 127.5K blocks) to WRAM, and is then automatically relocated (by a 0.5K stub) from WRAM to SRAM by software, /ACK is used to indicate completion of relocation, after uploading all block(s) the console gets reset with SRAM mapped to the ROM-region.

The XOR gate maps the bottom halves of the HiROM banks as additional LoROM banks. About A(hi) and A(hi+1): For example, for 128Kx8 chip (A0..A16): connect A(hi) to A16, A(hi+1) to A17. For more than 2Mx8 SRAM: connect the A(hi+1) input to GND; or leave out the XOR gate completely.

/STB and /AUTOLF are used (while /SELECT=HIGH) to select ROM, WRAM, or SRAM mapping, and as /PARD and /PAWR (while /SELECT=LOW). The /PARD pin allows to download status information and other data from WRAM.

#### Parts List

```text
  1  74LS08 Quad 2-Input AND gates
  1  74LS32 Quad 2-Input OR gates
  1  74LS86 Quad 2-Input XOR gates
  3  10K Ohm Resistors (required when cable is disconnected)
  3  10nF capacitor (to eliminate dirt on some ports)
  1  36pin centronics socket (plus standard printer cable)
```

#### Additional components for Extended Circuit

```text
  1  74LS32 Quad 2-Input OR gates
  1  nn-pin DIP Nx8 Static RAM (SRAM)
  1  40-pin DIP Socket for SRAM
```

Requires a bi-directional parallel port (very old printer ports aren't bi-directional, also some hyper-modern ports aren't bi-directional when configured to "ECP" mode, for that ports: use "EPP" mode).

Currently SRAMs of up to 512Kx8 (32pin) should be available for less than $5, also 2Mx8 chips (36pin) are manufactured, but I've no idea where to buy them, best use a 40pin DIP socket for future expansion.

#### TEST Mode

Optionally, one can replace the "XBOO" ID-string in cartridge header by "TEST", this will cause normal Cartridge ROM to be mapped (instead of mirroring WRAM to the ROM area). The advantage is that the uploaded program can examine cartridge memory (eg. for reverse-engineering purposes), the disadvantage is that it cannot set up it's own IRQ/NMI vectors.

The program should redirect the reset vector from Bank 00h to Bank 7Eh by executing a JMP 7Exxxxh immediately after reset, and then wait until WRAM is no longer mapped at 008xxxxh; thereafter, ROM is freely accessible (the switch from WRAM to ROM mapping occurs circa 100us after RESET).
