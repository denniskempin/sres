# SNES Cart Cheat Devices - Theory

#### ROM Patches

There are two possible ways for patching ROMs or ROM-images:

```text
  1) rewrite ROM-image in RAM once before game starts        (GF/FFE/emulators)
  2) patch on ROM reading (by watching address bus)          (GG and PAR2-3)
```

Both are basically having same results, there may be some variations concerning memory mirrors (depending on how the ROM-image in RAM is mirrored, or on how the GG/PAR2-3 do decode the ROM address).

#### WRAM Patches

Implemented by rewriting WRAM upon NMI, variations would involve mirrors:

```text
  1) allow WRAM addresses 7E0000-7FFFFF                      (PAR1-3, XT1-2)
  2) allow WRAM addresses 7E0000-7FFFFF and nn0000-nn1FFF    (N/A)
```

#### SRAM Patches

There are three possible ways for patching battery-backed SRAM:

```text
  1) rewrite once before game starts                         (GF)
  2) patch on SRAM reading (like hardware based ROM patches) (GG and PAR2-3)
  3) rewrite repeatedly on NMI execution (like WRAM patches) (N/A)
```

SRAM is usually checksummed, so SRAM patches need to be usually combined with ROM patches which do disable the checksum verification. Some devices (like Game Genie) rely on the /ROMSEL signal, and thus probably can only patch SRAM in the ROM area at 70xxxxh (but not in the Expansion area at 306xxxh).

#### Slow Motion Feature

Implemented by inserting delays in Vblank NMI handler. The feature can be usually configured in the BIOS menu, and/or controlled via joypad button combinations from within NMI handler.

Cheat Finders (for WRAM Patches) (eventually also for SRAM patches) Implemented by searching selected values from within Vblank NMI handler, or more simple: from within BIOS RESET handler. The search can be enabled/disabled mechanically via switch, or in some cases, via joypad button combinations. The searched value can be configured on RESET, or in some cases, via joypad button combinations.

#### Game Saver (Nakitek)

Allows to save a copy of WRAM/VRAM and I/O ports (but not APU memory) in DRAM, done upon joypad button-combinations sensed within BRK/NMI exception handlers.
