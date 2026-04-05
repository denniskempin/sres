---
title: "Standard controller"
source_url: "https://snes.nesdev.org/wiki/Standard_controller"
pageid: 47
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The standard controller reports 16 bits of data.

See: [[Controller reading]]

If using the [[MMIO registers#NMITIMEN|automatic read]], the reports will be available at [[MMIO registers#JOY1|JOY1]], etc.

```
   JOY1H       JOY1L
   $4219       $4218
15  bit  8   7  bit  0
 ---- ----   ---- ----
 BYsS UDLR   AXlr 0000
 |||| ||||   |||| ||||
 |||| ||||   |||| ++++- Signature
 |||| ||||   ||++------ L/R shoulder buttons
 |||| ||||   ++-------- A/X buttons
 |||| ++++------------- D-pad
 ||++------------------ Select (s) and Start (S)
 ++-------------------- B/Y buttons
```

If manually reading through [[MMIO registers#JOYSER0|JOYSER0]], these bits are delivered starting with the most significant bit (B button).

The signature is guaranteed to be 0000 for a standard SNES controller. Finding another value here indicates that a different peripheral is plugged in.

Additional reads past the first 16 return 1s on official controllers. Third party controllers may return 0s instead.

The first 8 bits of this report are identical to the NES controller, with SNES Y and B substituted for NES B and A. This offers potential compatibility with an NES controller through an adapter, though the game would have to ignore the extra buttons and signature which would normally report as all 1s from an NES controller.

The first two controllers report through JOYSER0 ($4016) D0 and JOYSER1 ($4017) D0. A [[Multitap]] peripheral exists allowing up to 5 standard controllers to be used.

## Links

- [SNES controller](https://www.nesdev.org/wiki/SNES_controller) at NESdev Wiki
