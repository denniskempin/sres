# Z80 Local Usage

#### Nintendo Super System (Z80)

Clocked at 4.000MHz.

NMIs are used for something (probably Vblank or Vsync or so). Normal interrupts seem to be unused. There is MAYBE no watchdog hardware (but the BIOS is using a software-based watchdog; namely, it's misusing the "I" register as watchdog timer; decreased by NMI handler). ALTHOUGH, like the PC10, it might ADDITIONALLY have a hardware watchdog...?

#### Super Famicom Box (HD64180)

Clocked at by a 9.216MHz oscillator, ie. the HD64180 is internally clocked at PHI=4.608MHz.
