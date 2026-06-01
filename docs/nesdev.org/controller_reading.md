---
title: "Controller reading"
source_url: "https://snes.nesdev.org/wiki/Controller_reading"
pageid: 41
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The SNES has a feature that can automatically read the game controllers, so the 65c816 does not need to spend any time doing that. The controllers can also be read manually, the same way the NES does it. The automatic reading feature will only read 16 bits from the controller, so peripherals like the [[Mouse]] need to be read either completely manually or with a combination of automatic and manual reading.

If no controller is connected to a port all bits should read as 0.

## Automatic controller reading

Automatic controller reading is enabled and disabled with the `NMITIMEN` register. If the least significant bit is set, then the SNES will start reading the game controllers for approximately the first three scanlines of vblank. This happens in the background and does not take any time away from the 65c816.

Official guides advise to check if automatic reading has finished by reading the `HVBJOY` register ($4212). This can be omitted if you can guarantee the timing does not overlap the start of vblank (e.g. a full OAM DMA takes about three scanlines).

The values read from the controllers are made available via the [[MMIO registers#Auto-read results|`JOY1-JOY4` registers]] ($4218-421F). It can be a good idea to copy these values somewhere else, so that if the game logic ends up taking more than one frame to complete (and the SNES enters vblank again), then these values changing won't cause any problems. This can also help with calculating when a button is pressed that wasn't pressed the previous frame, for actions that should only happen once per press.

`JOYOUT` ($4016) bit 0 must remain at 0 during the auto-read for it to function correctly[[1]](#cite_note-1).

```
  ; ensure auto-read has finished
:
  lda HBVJOY
  and #1
  bne :-

  ; 16-bit accumulator
  lda keydown
  sta keylast
  lda JOY1L
  sta keydown ; buttons currently pressed
  eor keylast
  and keydown
  sta keynew  ; buttons newly pressed this frame (0->1)
```

Important note: There is a small window of time where vblank has started but automatic controller reading has not[[2]](#cite_note-2). During this time `HVBJOY` will correctly say that auto reading is not currently active. Normally the first few instructions at the start of an NMI handler are more than enough delay to exit this brief window, but an attempt to read immediately after the start of vblank may require additional delay.

## Manual controller reading

Controllers can be read manually through the use of registers at $4016 and $4017. These are the same addresses the NES uses, and these registers work the same way as the NES's.

SNES controllers contain a shift register. The SNES can reset the shift register, and fill it with the current state of all of the buttons. The SNES can also retrieve one bit from the shift register, which causes the next read to receive the next bit, and so on, until the controller runs out of bits to send.

Controllers are reset with [[MMIO registers#JOYOUT - Joypad output ($4016 write|JOYOUT ($4016)]] "MMIO registers"). The least significant bit in that register controls a reset signal that controllers receive, and writing a 1, then a 0 to this register will reset the controllers. This is only required if controllers are being read completely manually - if manual reading is being used to read additional bits after automatic reading, the controllers will already be ready to read out those additional bits.

Reading [[MMIO registers#JOYSER0 - Joypad serial data port 1 ($4016 read|JOYSER0 ($4016)]] "MMIO registers") or [[MMIO registers#JOYSER1 - Joypad serial data port 2 ($4017 read|JOYSER1 ($4017)]] "MMIO registers") will request bits from controller port 1, and controller port 2, respectively. Controllers actually return two bits each read, but standard controllers only use the least significant bit.

Here's example code that reads another sixteen bits from a controller, helpful for the [[Mouse]] which has 32 bits of information to read. This example uses the result as a ring counter - once the initial 1 bit gets shifted out, the loop will have run sixteen times and will stop.

```
  ; 8-bit accumulator

  ; Initialize the result variable to $0001
  lda #1
  sta result_lo
  stz result_hi

Loop:
  lda JOYSER0    ; Get one bit from the controller
  lsr            ; Put that bit into the carry flag
  rol result_lo  ; Shift the carry flag into the result
  rol result_hi
  bcc Loop       ; Stop once result_hi shifts out a 1
```

## References

1. [↑](#cite_ref-1) [Forum thread](https://forums.nesdev.org/viewtopic.php?t=24182): $4016=1 blocks auto-read, not emulated?
2. [↑](#cite_ref-2) [Forum thread](https://forums.nesdev.org/viewtopic.php?p=188011): Stupid problems with autoread on hardware
