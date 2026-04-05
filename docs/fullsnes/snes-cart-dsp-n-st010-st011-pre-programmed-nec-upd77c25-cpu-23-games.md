# SNES Cart DSP-n/ST010/ST011 (pre-programmed NEC uPD77C25 CPU) (23 games)

#### Nintendo DSP-n Chips

The DSP-n chips are 28pin NEC uPD77C25 CPUs with internal ROM/RAM. There are six versions:

```text
  DSP-1, DSP-1A, DSP-1B, DSP-2, DSP-3, DSP-4
```

DSP-1 and DSP-1A contain exactly the same Program/Data ROM. DSP-1B contains a bug-fixed DSP1/1A version. DSP2/3/4 contain custom ROMs.

#### Seta ST010/ST011 Chips

These are 64pin chips, containing a slightly extended NEC uPD77C25 with more ROM and RAM, faster CPU clock.

```text
  64pin  SETA ST010 D96050CW-012 (PCB SHVC-1DS0B-01)
  64pin  SETA ST011 D96050CW-013 (PCB SHVC-1DS0B-10; with extra transistor)
```

The onchip RAM is battery-backed and is accessible directly via SNES address bus.

#### NEC uPD77C25 Specs

> **See:** [SNES Cart DSP-n/ST010/ST011 - NEC uPD77C25 - Registers & Flags & Overview](snes-cart-dsp-n-st010-st011-nec-upd77c25-registers-flags-overview.md)
> **See:** [SNES Cart DSP-n/ST010/ST011 - NEC uPD77C25 - ALU and LD Instructions](snes-cart-dsp-n-st010-st011-nec-upd77c25-alu-and-ld-instructions.md)
> **See:** [SNES Cart DSP-n/ST010/ST011 - NEC uPD77C25 - JP Instructions](snes-cart-dsp-n-st010-st011-nec-upd77c25-jp-instructions.md)

#### Game specific info

> **See:** [SNES Cart DSP-n/ST010/ST011 - List of Games using that chips](snes-cart-dsp-n-st010-st011-list-of-games-using-that-chips.md)
> **See:** [SNES Cart DSP-n/ST010/ST011 - BIOS Functions](snes-cart-dsp-n-st010-st011-bios-functions.md)

#### DSPn/ST010/ST011 Cartridge Header

For DSPn Cartridges:

```text
  [FFD6h]=03h..05h   Chipset = DSPn (plus battery present/absent info)
```

For ST010/ST011 Cartridges:

```text
  [FFD6h]=F6h   Chipset = Custom (plus battery; for the on-chip RAM)
  [FFD4h]=00h   Last byte of Title=00h (indicate early extended header)
  [FFBFh]=01h   Chipset Sub Type = ST010/ST011
```

Note: The uPD77C25's ROM/RAM aren't counted in the ROM Size, ROM Checksum, SRAM Size (nor Expansion RAM Size) entries. The header (nor extended header) includes no info whether a DSPn game uses a DSP1, DSP2, DSP3, or DSP4, and no info if a ST010/ST011 game uses ST010 or ST011. Ideally, the uPD77C25 ROM-Image should be appended at the end of the SNES ROM-Image. In practice, it's often not there, so there's no way to detect if the game uses this or that uPD77C25 ROM (except for using a list of known Titles or Checksums).
