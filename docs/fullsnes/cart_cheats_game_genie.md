# SNES Cart Cheat Devices - Game Genie

#### Game Genie BIOS Versions

There are at least three BIOS versions, named "GENSRC", "K7", and "ed":

```text
  "GENSRC"  32Kbytes LoROM, straight I/O addresses  CRC32=AC94F94Ah
  "K7"      32Kbytes LoROM, messy I/O addresses     CRC32=F8D4C303h
  "ed"      64Kbytes LoROM, messy I/O addresses     CRC32=58CBF2FEh
```

The names are stored at ROM-offset 7FE0h (aka SNES address 00FFE0h). Most of the cartridge header contains garbage (no checksum, wrong ROM size, etc.), only the 21-byte title field is (more or less) correct:

```text
  "Game Genie      ",0,0,0,0,0   ;32K versions (GENSRC & K7)
  "Game Genie         Jo"        ;64K version (ed)
```

Note: All three versions support only 5 codes to be entered.

#### Game Genie I/O Ports

The "GENSRC" version uses quite straight I/O addresses, the "K7" and "ed" versions have the addresses messed up (unused gaps between the codes, several mirrors, of which, mirrors with address bit8 and bits16-23 all zero are having address bit0 inverted).

```text
  Version   "GENSRC"          "K7" & "ed"
  Control   W:008000h         W:xx8100h, W:008001h  ;ed: xx=00, K7: xx=FF
  CodeFlags R/W:008001h       R:FF8001h, W:008000h  ;bit 0-4 = enable code 1-5
  CodeMsb   R/W:008003h+N*4   R:FF8005h+N*6, W:008004h+N*6  ;\
  CodeMid   R/W:008004h+N*4   R:FF8006h+N*6, W:008007h+N*6  ; N=0-4 for
  CodeLsb   R/W:008005h+N*4   R:FF8007h+N*6, W:008006h+N*6  ; code 1-5
  CodeData  R/W:008006h+N*4   R:FF8008h+N*6, W:008009h+N*6  ;/
```

Other (accidently) used I/O addresses are: W:004017h (bugged joypad access), and W:00FFEAh/00FFEBh (used in an attempt to install a bugged NMI handler with RET instead of RETI opcode; the hardware is hopefully ignoring that attempt).

#### Control Register

Allows to select what is mapped to memory. Used values are:

```text
  00h Select Game Genie BIOS
  02h Select Game Genie I/O Ports
  06h Select Game Cartridge
  07h Select Game Cartridge and keep it selected
```

This register is set to 00h on /RESET (warmboot & coldboot).

#### Code Flags Register

Used to enable the 5 codes:

```text
  Bit0-4  Enable Code 1..5  (0=Disable, 1=Enable)
  Bit5-7  Should be zero
```

This register is set to 00h on initial power-up (coldboot), but kept intact on /RESET (warmboot), allowing to restore the previously enabled codes.

#### CodeAddress/CodeData Registers (5*4 bytes)

Contains the 24bit address and 8bit data values (in decrypted form). The registers are kept intact on /RESET (warmboot), allowing to restore the previously entered codes - however, when restoring the codes, the "K7" and "ed" BIOS versions are ORing the LSB of of the 24bit address value with 01h, thereby destroying all codes with even addresses (unknown why that's been done).

#### Chipset

The exact chipset is unknown, there should be the ROM and some logic (and no SRAM). There is also a LED and a 2-position (?) switch (unknown function).

#### XXX

For more in-depth info see "genie.txt" from Charles MacDonald.

```text
  http://cgfm2.emuviews.com/txt/genie.txt
```

#### Game Genie 2 (prototype)

There is also an unreleased Game Genie 2 prototype, the thing includes a small LCD screen, five push buttons, four LEDs, battery backed SRAM, a 8255 PIO, a huge ACTEL chip, and some expansion connectors.
