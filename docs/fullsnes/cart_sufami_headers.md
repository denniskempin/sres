# SNES Cart Sufami Turbo ROM/RAM Headers

#### Sufami Turbo BIOS ROM Header

The BIOS has a rather incomplete Nintendo-like header at ROM Offset 07FB0h (mapped to 00FFB0h):

```text
  FFB0h Maker Code "B2"                        ;\extended header, present
  FFB2h Game Code "A9PJ"                       ; even though [FFDAh]<>33h
  FFB6h Reserved (10x00h)                      ;/
  FFC0h Title "ADD-ON BASE CASSETE  " (really mis-spelled, with only one "T")
  FFD4h Mapmode (always 30h = Fast LoROM)
  FFD5h Reserved (6x00h) (no ROM/RAM size entries, no ext.header-flag, etc.)
  FFDCh Dummy "checksum" value (always FFh,FFh,00h,00h)
  FFE0h Exception Vectors (IRQ,NMI,Entrypoint,etc.)
```

And, there is a header-like data field at ROM-Offset 00000h (mapped to 808000h) (this part isn't really a header, but rather contains ID strings that are used by the BIOS, for comparing them with Game ROM/SRAM):

```text
  8000h 16 "BANDAI SFC-ADX",0,0   ;Game ROM ID
  8010h 16 "SFC-ADX BACKUP",0,0   ;Game SRAM ID
```

Sufami Turbo Game ROM Header (40h bytes) Located at ROM Offset 00000h (mapped to 208000h/408000h for Slot A/B):

```text
  00h 14 ID "BANDAI SFC-ADX" (required, compared against 14-byte ID in BIOS)
  0Eh 2  Zero-filled
  10h 14 Title, padded with spaces (can be 7bit ASCII and 8bit Japanese)
  1Eh 2  Zero-filled
  20h 2  Entrypoint (in bank 20h) ;game starts here (if it is in Slot A)
  22h 2  NMI Vector (in bank 20h) ;if RAM[000000h]=00h: use BIOS NMI handler
  24h 2  IRQ Vector (in bank 20h)
  26h 2  COP Vector (in bank 20h)
  28h 2  BRK Vector (in bank 20h)
  2Ah 2  ABT Vector (in bank 20h)
  2Ch 4  Zero-filled
  30h 3  Unique 24bit ID of a Game (or series of games) (usually 0xh,00h,0yh)
  33h 1  Index within a series (01h and up) (eg. 01h..06h for Gundam 1-6)
  34h 1  ROM Speed (00h=Slow/2.68Mhz, 01h=Fast=3.58MHz)
  35h 1  Chipset/Features (00h=Simple, 01h=SRAM or Linkable?, 03h=Special?)
  36h 1  ROM Size in 128Kbyte Units (04h=512K, 08h=1024K)
  37h 1  SRAM Size in 2Kbyte Units (00h=None, 01h=2K, 04h=8K)
  38h 8  Zero-filled
```

Some games have additional 64 header-like bytes at ROM Offset 40h..7Fh

```text
  40h 1  Program code/data in some carts, 00h or 01h in other carts
  41h 63 Program code/data in some carts, 00h-filled in other carts
```

The game cartridges don't use/need a Nintendo-like header at 7Fxxh/FFxxh, but some games like SDBATTLE SEVEN do have one.

#### Sufami Turbo SRAM File Header (30h bytes)

```text
  0000h 15 ID "SFC-ADX BACKUP",0   ;Other = begin of free memory
  000Fh 1  Zero
  0010h 14 Title (same as 0010h..001Dh in ROM Header)
  001Eh 1  Zero
  001Fh 1  Zero (except, 01h in Poi Poi Ninja)
  0020h 4  Unique ID and Index in Series (same as 0030h..0033h in ROM Header)
  0024h 1  Filesize (in 2Kbyte units)    (same as 0037h in ROM Header)
  0025h 11 Zero-filled
```

The BIOS file-functions are only reading entry 0000h (ID) and 0024h (Filesize), the BIOS doesn't write anything, all IDs and values must be filled-in by the game.

SRAM is organized so that used 2Kbyte pages are at lower addresses, free pages at higher addresses (deleting a file in the middle will relocate any pages at higher addresses). Accordingly, files are always consisting of unfragmented continous page numbers (leaving apart that there are 32Kbyte gaps in the memory map).
