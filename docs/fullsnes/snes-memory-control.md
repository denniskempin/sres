# SNES Memory Control

```text
420Dh - MEMSEL - Memory-2 Waitstate Control (W)
  7-1   Not used
  0     Access Cycle for Memory-2 Area (0=2.68MHz, 1=3.58MHz) (0 on reset)
```

Memory-2 consists of address 8000h-FFFFh in bank 80h-BFh, and address 0000h-FFFFh in bank C0h-FFh. 3.58MHz high speed memory requires 120ns or faster ROMs/EPROMs. 2.68MHz memory requires 200ns or faster ROMs/EPROMs.

```text
  2.684658 MHz = 21.47727 MHz / 8     ;same access time as WRAM
  3.579545 MHz = 21.47727 MHz / 6     ;faster access than WRAM
```

Programs that do use the 3.58MHz setting should also indicate this in the Cartridge header at [FFD5h].Bit4.

> **See:** [SNES Cartridge ROM Header](snes-cartridge-rom-header.md)

#### Forced Blank

Allows to access video memory at any time. See INIDISP Bit7, Port 2100h.
