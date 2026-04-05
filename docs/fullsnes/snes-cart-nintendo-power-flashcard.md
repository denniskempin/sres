# SNES Cart Nintendo Power (flashcard)

Nintendo Power cartridges are official FLASH cartridges from Nintendo (released only in Japan). Unlike the older Satellaview FLASH cartridges, they do connect directly to the SNES cartridge slot. The capacity is 4MByte FLASH and 32KByte battery-backed SRAM.

#### FLASH (512Kbyte blocks)

The FLASH is divided into eight 512Kbyte blocks. The first block does usually contain a Game Selection Menu, the other blocks can contain up to seven 512KByte games, or other combinations like one 3MByte game and one 512KByte game. Alternately, the cartridge can contain a single 4MByte game (in that case, without the Menu).

#### SRAM (2Kbyte blocks) (battery-backed)

The SRAM is divided into sixteen 2Kbyte blocks for storing game positions.

Games can use one or more (or all) of these blocks (the menu doesn't use any of that memory).

> **See:** [SNES Cart Nintendo Power - New Stuff](snes-cart-nintendo-power-new-stuff.md)
> **See:** [SNES Cart Nintendo Power - I/O Ports](snes-cart-nintendo-power-i-o-ports.md)
> **See:** [SNES Cart Nintendo Power - FLASH Commands](snes-cart-nintendo-power-flash-commands.md)
> **See:** [SNES Cart Nintendo Power - Directory](snes-cart-nintendo-power-directory.md)
> **See:** [SNES Pinouts Nintendo Power Flashcarts](snes-pinouts-nintendo-power-flashcarts.md)

#### Nintendo Power Games

Games have been available at kiosks with FLASH Programming Stations. There are around 150 Nintendo Power games: around 21 games exclusively released only for Nintendo Power users, and around 130 games which have been previously released as normal ROM cartridges.

#### Nintendo Power PCB "SHVC-MMS-X1" or "SHVC-MMS-02" (1997) Chipset (SNES)

```text
  U1  18pin CIC       ("F411B Nintendo")
  U2 100pin MX15001   ("Mega Chips MX15001TFC")
  U3  44pin 16M FLASH ("MX 29F1601MC-11C3") (2Mbyte FLASH, plus hidden sector)
  U4  44pin 16M FLASH ("MX 29F1601MC-11C3") (2Mbyte FLASH, plus hidden sector)
  U5  44pin 16M FLASH (N/A, not installed)
  U6  28pin SRAM      ("SEC KM62256CLG-7L") (32Kbyte SRAM)
  U7   8pin MM1134    ("M 707 134B") (battery controller)
  BAT1 2pin Battery   ("Panasonic CR2032 +3V")
```

#### Nintendo Power PCB "DMG-A20-01" (199x) Chipset (Gameboy version)

```text
  U1  80pin G-MMC1    ("MegaChips MX15002UCA"
  U2  40pin 8M FLASH  ("MX29F008ATC-14") (plus hidden sector)
  U3  32pin 1M SRAM   ("UT621024SC-70LL")
  X1   3pin N/A       (oscillator? not installed)
  BAT1 2pin Battery   ("Panasonic CR2025")
```

#### Nintendo Power Menu SNES Cartridge Header

```text
  Gamecode:        "MENU" (this somewhat indicates the "MX15001" chip)
  ROM Size:        512K (the menu size, not including the other FLASH blocks)
  SRAM Size:       0K (though there is 32Kbyte SRAM for use by the games)
  Battery Present: Yes
  Checksum:        Across 512Kbyte menu, with Directory assumed to be
                   FFh-filled (except for the "MULTICASSETTE 32" part)
```

The PCB doesn't contain a ROM (the Menu is stored in FLASH, too).

#### Nintendo Power Menu Content

```text
  ROM Offset  SNES Address Size   Content
  000000h     808000h      4xxxh  Menu Code (around 16K, depending on version)
  004xxxh     80xxxxh      3xxxh  Unused (FFh-filled)
  007FB0h     80FFB0h      50h    Cartridge Header
  008000h     818000h      40000h Unused (FFh-filled)
  048000h     898000h      372Bh  Something (APU code/data or so)
  04B72Bh     8xxxxxh      47D5h  Unused (FFh-filled)
  050000h     8A8000h      8665h  Something (VRAM data or so)
  058665h     8Bxxxxh      798Bh  Unused (FFh-filled)
  060000h     8C8000h      10000h Directory (File 0..7) (2000h bytes/entry)
  070000h     8E8000h      10000h Unused (FFh-filled)
```

#### Note

Nintendo has used the name "Nintendo Power" for various different things:

```text
  Super Famicom Flashcards (in Japan)
  Gameboy Color Flashcards (in Japan)
  Super Famicom Magazine (online via Satellaview BS-X) (in Japan)
  Official SNES Magazine (printout) (in USA)
```
