# SNES Cart Sufami Turbo General Notes

#### Sufami Turbo Hardware

The "adaptor" connects to the SNES cartridge socket, it contains the BIOS ROM, and two slots for "mini-carts". Slot A for the game being played, Slot B can contain another game (some games include features that allow to access game position data from other games, some may also access ROM data from other games).

#### Sufami Turbo Memory Map

```text
  00-1F:8000h-FFFFh  BIOS ROM (always 256Kbytes)             (max 1MByte)
  20-3F:8000h-FFFFh  Cartridge A ROM (usually 512Kbytes)     (max 1MByte)
  40-5F:8000h-FFFFh  Cartridge B ROM (usually 512Kbytes)     (max 1MByte)
  60-63:8000h-FFFFh  Cartridge A SRAM (usually 0/2/8 Kbytes) (max 128Kbyte)
  70-73:8000h-FFFFh  Cartridge B SRAM (usually 0/2/8 Kbytes) (max 128Kbyte)
  80-FF:8000h-FFFFh  Mirror of above banks
```

#### Memory Notes

The BIOS detects max 128Kbyte (64 pages) SRAM per slot, some games are (maybe accidently) exceeding that limit (eg. Poi Poi Ninja zerofills 256 pages). Some games (eg. SD Gundam Part 1) access SRAM slot B at 700000h rather than 708000h.

Some games (eg. SD Ultra Battle) may fail if the SRAM in slot B is uninitialized (ie. before linking games in Slot A and B, first launch them separately in Slot A).

When not using BIOS functions, one can safely destroy all WRAM locations, except for WRAM[00000h] (which MUST be nonzero to enable the Game NMI handler & disable the BIOS NMI handler).

#### Sufami Turbo ROM Images

The games are typically 512Kbyte or 1MByte in size. Existing ROM-Images are often 1.5Mbytes or 2MBytes - those files do include the 256KByte BIOS-ROM (banks 00h-07h), plus three mirrors of the BIOS (banks 08h-1Fh), followed by the actual 512Kbyte or 1MByte Game ROM (bank 20h-2Fh or 20h-3Fh).

There are also a few 3MByte ROM-images, with additional mirrors of the game (bank 30h-3Fh), followed by a second game (bank 40h-4Fh), followed by mirrors of the second game (bank 50h-5Fh).

That formats are simple (but very bloated) solutions to load the BIOS & Game(s) as a "normal" LoROM file.

#### Sufami Turbo Games

There have been only 13 games released:

```text
  Crayon Shin Chan
  Gegege No Kitarou
  Gekisou Sentai Car Ranger
  Poi Poi Ninja                    ;-link-able with itself (2-player sram)
  Sailor Moon Stars Panic 2
  SD Gundam Generations: part 1    ;\
  SD Gundam Generations: part 2    ;
  SD Gundam Generations: part 3    ; link-able with each other
  SD Gundam Generations: part 4    ;
  SD Gundam Generations: part 5    ;
  SD Gundam Generations: part 6    ;/
  SD Ultra Battle: Seven Legend    ;\link-able with each other
  SD Ultra Battle: Ultraman Legend ;/
```

All of them available only in Japan, released between June & September 1996.

Thereafter, the games may have been kept available for a while, but altogether, it doesn't seem to have been a too successful product.

#### Component List for Sufami Turbo Adaptor

PCB "SHVC TURBO, BASE CASSETTE, BANDAI, PT-923"

```text
  IC1   18pin  unknown (CIC)
  IC2   16pin  "74AC139" or so
  IC3   40pin  SUFAMI TURBO "LH5326NJ" or so (BIOS ROM) (256Kbyte)
  IC4   8pin   unknown
  CP1   unknown (flashlight? oscillator? strange capacitor?)
  CN1   62pin  SNES cartridge edge (male)
  CN2   40pin  Sufami Cartridge Slot A (Game to be played)
  CN3   40pin  Sufami Cartridge Slot B (Other game to be "linked")
  C1..4  2pin  capacitors for IC1..4
  R1..4  2pin  resistors for unknown purpose
```

Note: Of the 62pin cartridge edge, only 43 pins are actually connected (the middle 46 pins, excluding Pin 40,48,57, aka A15/A23/SYSCK).

Component Lists for Sufami Turbo Game Carts All unknown. Probably contains only ROM, and (optionally) SRAM and Battery.

Physical SRAM size(s) are unknown (ie. unknown if there is enough memory for more than one file). Cartridge slot pin-outs are unknown.
