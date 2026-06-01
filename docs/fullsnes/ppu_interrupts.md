# SNES PPU Interrupts

```text
4200h - NMITIMEN - Interrupt Enable and Joypad Request (W)
  7     VBlank NMI Enable  (0=Disable, 1=Enable) (Initially disabled on reset)
  6     Not used
  5-4   H/V IRQ (0=Disable, 1=At H=H + V=Any, 2=At V=V + H=0, 3=At H=H + V=V)
  3-1   Not used
  0     Joypad Enable    (0=Disable, 1=Enable Automatic Reading of Joypad)
```

Disabling IRQs (via bit4-5) does additionally acknowledge IRQs. There's no such effect when disabling NMIs (via bit7).

```text
4207h/4208h - HTIMEL/HTIMEH - H-Count Timer Setting (W)
  15-9  Not used
  8-0   H-Count Timer Value (0..339) (+/-1 in long/short lines) (0=leftmost)
```

The H/V-IRQ flag in Bit7 of TIMEUP, Port 4211h gets set when the H-Counter gets equal to the H-Count register value.

```text
4209h/420Ah - VTIMEL/VTIMEH - V-Count Timer Setting (W)
  15-9  Not used
  8-0   V-Count Timer Value (0..261/311, NTSC/PAL) (+1 in interlace) (0=top)
```

The H/V-IRQ flag in Bit7 of TIMEUP, Port 4211h gets set when the V-Counter gets equal to the V-Count register value.

```text
4210h - RDNMI - V-Blank NMI Flag and CPU Version Number (R) (Read/Ack)
  7     Vblank NMI Flag  (0=None, 1=Interrupt Request) (set on Begin of Vblank)
  6-4   Not used
  3-0   CPU 5A22 Version Number (version 2 exists)
```

The NMI flag gets set at begin of Vblank (this happens even if NMIs are disabled). The flag gets reset automatically at end of Vblank, and gets also reset after reading from this register.

The SNES has only one NMI source (vblank), and the NMI flag is automatically reset (on vblank end), so there's normally no need to read/acknowledge the flag, except one special case: If one does disable and re-enable NMIs, then an old NMI may be executed again; acknowledging avoids that effect.

The CPU includes another internal NMI flag, which gets set when "[4200h].7 AND [4210h].7" changes from 0-to-1, and gets cleared when the NMI gets executed (which should happen around after the next opcode) (if a DMA transfer is in progress, then it is somewhere after the DMA, in that case the NMI can get executed outside of the Vblank period, ie. at a time when [4210h].7 is no longer set).

```text
4211h - TIMEUP - H/V-Timer IRQ Flag (R) (Read/Ack)
  7     H/V-Count Timer IRQ Flag (0=None, 1=Interrupt Request)
  6-0   Not used
```

The IRQ flag is automatically reset after reading from this register (except when reading at the very time when the IRQ condition is true (which lasts for 4-8 master cycles), then the CPU receives bit7=1, but register bit7 isn't cleared). The flag is also automatically cleared when disabling IRQs (by setting 4200h.Bit5-4 to zero).

Unlike NMI handlers, IRQ handlers MUST acknowledge IRQs, otherwise the IRQ gets executed again (ie. immediately after the RTI opcode).

```text
4212h - HVBJOY - H/V-Blank flag and Joypad Busy flag (R)
  7     V-Blank Period Flag (0=No, 1=VBlank)
  6     H-Blank Period Flag (0=No, 1=HBlank)
  5-1   Not used
  0     Auto-Joypad-Read Busy Flag (1=Busy) (see 4200h, and 4218h..421Fh)
```

The Hblank flag gets toggled in ALL scanlines (including during Vblank/Vsync).

Both Vblank and Hblank are always toggling (even during Forced Blank, and no matter if IRQs or NMIs are enabled).

#### Other IRQ Sources

IRQs can be also triggered via Cartridge Slot and Expansion Port. This is done by cartridges with SA-1 and GSU chips (CX4 carts do also have an IRQ line, although the existing games don't seem to use it).
