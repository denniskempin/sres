# SFC-Box Overview

#### Main Menu

```text
  Allows to select from 5 games
```

Note: If the two cartridges do contain more than 5 games in total, then the GUI is divided into two pages with 4 games, plus prev/next page option (unknown if more than 8 games are also supported).

#### Per Game Menu (after selecting a Game in Main Menu)

```text
  1. Game Start
  2. Game Instructions
  3. Game Preview
  4. Return to Main Menu
```

#### Soft-Reset Feature

```text
  Press L+R+Select+Start (on Joypad) --> Reset Current Game
  Press Reset Button (on SFC-Box Front Panel) --> Restart Boot Menu
```

#### GAME/TV Button

Allows to switch between Game & TV mode. The purpose is totally unclear... maybe it just allows to disable forwarding the Antenna-input to the RF-Out connector... but, <why> should one want to disable that?

#### SFC-Box Cartridges

The SFC-Box contains two special multi-game cartridges. There have been only 4 cartridges produced. The first cartridge MUST be always PSS61 (contains 3 games, plus the required GUI and 128Kbyte SRAM). The second cartridge can be PSS62, PSS63, or PSS64 (which contain 2 games each; these carts have no own SRAM, but they can share portions of the SRAM from the PSS61 cart).

#### SFC-Box Special ROM/EPROMs

```text
  KROM 1     EPROM 64Kbytes     (HD64180 BIOS in SFC-Box console)
  GROM1-1    EPROM 32Kbytes     (Directory) (IC1 in PSS61 cart)
  GROM2-1    EPROM 32Kbytes     (Directory) (IC1 in PSS62 cart)
  GROM3-1    EPROM 32Kbytes     (Directory) (IC1 in PSS63 cart)
  GROM4-1    EPROM 32Kbytes     (Directory) (IC1 in PSS64 cart)
  ATROM-4S-0 LoROM 512Kbytes    (GUI "Attraction" Menu) (ROM5 in PSS61 cart)
  DSP1       DSP ROM 8Kbytes    (or with padding: 10Kbytes)
  MB90082    OSD ROM 9Kbytes    (OSD-Character Set in MB90082-001 chip)
```

ATROM-4S-0 contains a regular SNES header at 7FC0h, interesting entries are:

```text
  7FC0h Title "4S ATTRACTION        "
  7FD6h Coprocessors (00h) (none, but, ATROM can communicate with the HD64180)
  7FD8h RAM Size (00h) (none, but, GROM indicates 32Kbytes allocated to ATROM)
  7FDAh Maker (B6h) (HAL)
```

Note: "GROM3-1" is dumped (but its ROM-image is "conventionally" misnamed as "GROM1-3"). There is reportedly also a "different" GUI version (not confirmed/details unknown, maybe there's just a configuration setting, in SRAM or EPROMs or so, that changes the GUI appearance).

#### SFC-Box Game ROMs

```text
  SHVC-4M-1  LoROM 2048Kbytes (Mario Collection) (ROM3 in PSS61 cart)
  SHVC-MK-0  HiROM 512Kbytes  (Mario Kart)       (ROM12 in PSS61 cart)
  SHVC-FO-1  LoROM 1024Kbytes (Starfox)          (IC20 in PSS61 cart)
  SHVC-GC-0  LoROM 1024Kbytes (WaiaraeGolf)      (ROM1 in PSS62 cart)
  SHVC-2A-1  HiROM 512Kbytes  (Mahjong)          (ROM9 in PSS62 cart)
  SHVC-8X-1  HiROM 4096Kbytes (Donkey Kong)      (ROM7 in PSS63 cart)
  SHVC-T2-1  LoROM 1024Kbytes (Tetris2/Bombliss) (ROM3 in PSS63 cart)
  SHVC-8X-1  HiROM 4096Kbytes (Donkey Kong)      (ROM7 in PSS64 cart)
  SHVC-M4-0  HiROM 1024Kbytes (Bomberman2)       (ROM9 in PSS64 cart)
```

All Game ROMs seem to be identical as in normal (japanese) cartridges, (ie. without any SFC-Box specific revisions).

#### SFC-Box ROM-Images

ROM-Images should contain all EPROMs/ROMs from the cartridge, ordered as so:

```text
  GROM + ROM0(+ROM1(+ROM2(+etc))) (+DSP1)
```

The GROM at the begin of the file does also serve as file header:

```text
  The size of the GROM (1 SHL N kbytes) is found in GROM [0001h].
  The number of ROMs is found in GROM [0000h].
  Title & Size of ROM<n> can be found at [[0008h]+n*2]*1000h.
  Physical IC Socket ID for ROM<n> can be found in GROM at [0008h]+[0000h]*2+n.
  The presence of a DSP ROM Image is indicated in GROM [0004h].Bit1.
```

With that information, one can calculate the file-offsets for each ROM.

If desired, one may merge two cartridges images in one file, eg.

```text
  GROM1+ROM0+ROM1+ROM2+ROM3+DSP + GROM2+ROM0+ROM1
```

Before merging GROM+ROMs, make sure that the ROMs are raw-images (without 512-byte copier headers), and that the DSP ROM is unpadded (8Kbytes), in little-endian format.

The additional "non-cartridge" ROMs of the SFC-Box (KROM1 and MB90082) should be located in a separate BIOS folder; not in the cartridge ROM-Image.

#### SFC-Box Crashes

Some bugged ATROM functions (7E2125h and 7E2173h) are messing up the SNES stack, causing the SNES to run into endless execution of BRK opcodes (thereby destroying lower 8K of WRAM, any enabled SRAM, and all I/O ports). Normally, SNES emulators could stop emulation in such "beyond-repair" situations -however, for the SFC-Box, emulation must be kept running (or better: crashing), since the KROM can restore normal operation by issuing a /RESET to the SNES (for example, this is happens near completion of the "**********" progress bar in the SFC-Box boot screen). Moreover, the KROM does change SNES mapping (via Port C0h/C1h), apparently without pausing/resetting the SNES CPU during that time, thus causing SNES to execute garbage code (though there are also working situations: eg. when checking the GAME headers, the SNES executes ATROM code relocated to WRAM). And, there seems to be a situation (maybe caused by above stuff) where the SNES NMI handler jumps to Open Bus regions. Note: In most or all cases, the crashing program is running into BRK opcodes (emulating BRK opcodes as "leave PC and SP unchanged" helps avoiding the more hazardous crash-effects).
