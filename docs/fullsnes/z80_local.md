---
summary: Describes the local Z80 usage in two rare SNES peripheries — the Nintendo Super System (uses a Z80 at 4.000MHz with NMIs) and the Super Famicom Box (uses an HD64180 at 4.608MHz).
keywords: Z80, HD64180, Nintendo Super System, Super Famicom Box
importance: 0
---

# Z80 Local Usage

#### Nintendo Super System (Z80)

Clocked at 4.000MHz.

NMIs are used for something (probably Vblank or Vsync or so). Normal interrupts seem to be unused. There is MAYBE no watchdog hardware (but the BIOS is using a software-based watchdog; namely, it's misusing the "I" register as watchdog timer; decreased by NMI handler). ALTHOUGH, like the PC10, it might ADDITIONALLY have a hardware watchdog...?

#### Super Famicom Box (HD64180)

Clocked at by a 9.216MHz oscillator, ie. the HD64180 is internally clocked at PHI=4.608MHz.
