# SNES Cart Pirate X-in-1 Multicarts (2)

There's at least one korean multicart (called "C20H" or "super20hab" or so), with 20 small games stored on a relative small 1Mbyte ROM. The games are NES games ported to work on SNES, some with typical pirate mods (like removing copyright strings, or renaming the game to bizarre names).

#### I/O Ports

```text
  20xxh NES PPU left-overs (written to, but ignored by the SNES)
  40xxh NES APU left-overs (written to, but ignored by the SNES)
  8000h ROM Bank Size/Base
```

Port 8000h works around as so:

```text
  0-4  ROM Base Offset (in 32Kbyte units)
  5    Unknown/unused (always zero)
  6-7  ROM Bank Size (0=Used/unknown, 1=Unused/Unknown, 2=1x32K, 3=2x32K)
```

The ROM is mapped in LoROM fashion (with 1 or 2 banks of 32Kbyte).

SRAM might also exist (the photo shows some unidentified 24pin chip).

#### Component List

```text
  PCB Name: Unknown (it has one, but isn't legible on lousy photo)
  32pin C20H (1Mbyte ROM)
  24pin Unknown (maybe SRAM) (there is no battery visible on PCB front side)
  20pin Unknown (looks like a sanded chip; presumably memory mapper)
  16pin CIVIC CTxxxx? (CIC clone)
  46pin Cartridge Edge Connector
```
