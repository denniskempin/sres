---
title: "Mouse"
source_url: "https://snes.nesdev.org/wiki/Mouse"
pageid: 13
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The **Super NES Mouse** (SNS-016) is a peripheral for the Super NES that was originally bundled with *Mario Paint*.
The **Hyper Click Retro Style Mouse** by Hyperkin is an optical mouse mostly compatible with software for the Super NES Mouse, with some behavior quirks.

Unlike the standard controller which returns 16 bits of data, the Super NES mouse returns 32 bits. All 32 bits can be read manually via $4016 as on the NES, but an alternative is to let the SNES do its automatic controller reading to get the first 16 bits, and then read the last 16 bits manually.

## Report

The report is divided functionally into four bytes. The most significant bit is delivered first:

```
76543210  First byte
++++++++- Always zero: 00000000

76543210  Second byte
||||++++- Signature: 0001
||++----- Current sensitivity (0: low; 1: medium; 2: high)
|+------- Left button (1: pressed)
+-------- Right button (1: pressed)

76543210  Third byte
|+++++++- Vertical displacement since last read
+-------- Direction (1: up; 0: down)

76543210  Fourth byte
|+++++++- Horizontal displacement since last read
+-------- Direction (1: left; 0: right)
```

After the fourth byte, subsequent bits will read as all 1, though the Hyperkin clone mouse instead reads a single 1 then all 0s. [[1]](#cite_note-1)

The Hyper Click mouse will not give a stable report if it is read too fast. Between each read and the next, there should be at least 170 master cycles. Between the 2nd and 3rd byte (16th and 17th bit) of the report should be at least 336 master cycles. Reading faster than this will result in corrupted values.[[2]](#cite_note-2).

### Automatic read

Official documents recommend reading the first 2 bytes with the [[MMIO registers#NMITIMEN|automatic read]] system, and then reading the remaining 16 bits through the [[MMIO registers#JOYSER1|serial interface]]. Alternatively it can be read entirely through the serial interface.

Through the automatic read, the second byte of the mouse report will be the low byte of 16-bit automatic report.

```
    .a8
    .i8
    ; wait for automatic read to finish
:
    lda a:$4212
    and #1
    bne :-
    ;
    ; Note: after the end of auto-read, a delay should be placed here to accomodate
    ;       the hyperkin mouse. This might be a good time to check the signature byte,
    ;       or do other mouse-related logic. (336 clocks is safe.)
    ;
    ; now read the remaining serial report
    ldy #16
:
    lda a:$4017 ; mouse in second controller port
    lsr
    rol z:mouseread_x
    rol z:mouseread_y
    nop ; Read delay for hyperkin mouse support, loop should be at least 170 master clocks.
    nop ; Less delay is needed if using abs/far rol, and/or SlowROM.
    nop ; This loop is 188 clocks in FastROM; it has 1 excess NOP for safety.
    nop ; (Mario Paint takes 190 clocks.)
    dey
    bne :-
    rts
```

## Motion

Motion of the mouse is given as a displacement since the last mouse read, delivered in the third and fourth bytes of the report.

The displacements are in [sign-and-magnitude](https://en.wikipedia.org/wiki/Signed_number_representations#Sign-and-magnitude_method "wikipedia:Signed number representations"), not [two's complement](https://en.wikipedia.org/wiki/Signed_number_representations#Two.27s_complement "wikipedia:Signed number representations").
For example, $05 represents five mickeys (movement units) in one direction and $85 represents five mickeys in the other.
To convert these to two's complement, use [negation](https://snes.nesdev.org/w/index.php?title=Synthetic_instructions&action=edit&redlink=1 "Synthetic instructions (page does not exist)"):

```
  ; Convert to two's complement
  lda third_byte
  bpl :+
  eor #$7F
  inc
:
  sta y_velocity

  lda fourth_byte
  bpl :+
  eor #$7F
  inc
:
  sta x_velocity
```

When the magnitude of motion is 0, the reported sign will repeat the last used sign value for that coordinate.

## Sensitivity

The mouse can be set to low, medium, or high sensitivity.

On the original SNES mouse this can be changed by sending a clock while the latch ($4016.d0) is turned on. The latch is controlled by writing to $4016, and the clock is sent by reading the serial port (either $4016 or $4017).

```
    .a8
    lda #1
    sta $4016
    lda $4016 ; mouse in first port
    stz $4016
```

Note that this cannot be done while an automatic read is active. Either test $4212 before proceeding, or otherwise guarantee that an automatic read is not occurring at the same time.

Some revisions of the mouse's microcontroller power up in an unknown state and may return useless values before the sensitivity is changed for the first time.[[3]](#cite_note-fullsnes-3)

The Hyper Click mouse will not cycle its sensitivity this way. Instead it has a manual button on the underside that must be pressed by the user to cycle sensitivity. It will always report 0 for sensitivity, regardless of its manual setting. For this reason, it is not advised to use the software sensitivity cycling to automatically detect the presence of a mouse.[[4]](#cite_note-4)

On the original SNES mouse, sensitivity setting 0 responds linearly to motion, at a rate of 50 counts per inch[[5]](#cite_note-5). Values range from 0 to 63, but values higher than 25 are increasingly difficult to produce. [[6]](#cite_note-6)

Sensitivity settings 1 and 2 appear to remap the equivalent setting 0 values 0-7 to a table, and clamping at the highest value. (Rarely, however, other values may be seen in settings 1 and 2.)

| Sensitivity | Value | | | | | | | | | | |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0 | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | ... |
| 1 | 0 | 1 | 2 | 3 | 8 | 10 | 12 | 21 | 21 | 21 | ... |
| 2 | 0 | 1 | 4 | 9 | 12 | 20 | 24 | 28 | 28 | 28 | ... |

The Hyper Click's two manually selected sensitivities both scale linearly with motion speed. Low sensitivity produces 0-31, and high sensitivity produces 0-63. The magnitude of the result is not dependent on the rate of polling, so it appears to report the current speed rather than the distance travelled since the last poll. The maximum value (31/63) at either sensitivity appears to correspond roughly to a speed of 8 inches per second. (This mouse should be used on a surface with a visible texture.)[[7]](#cite_note-7)

## Hyperkin Mouse

The Hyper Click Retro Style Mouse by Hyperkin has the following quirks:

- There must be at least 170 master cycles between bit reads.
- There must be at least 336 master cycles between the 2nd and 3rd byte (bits 16th and 17th).
- After reading 32 bits of data the Hyperkin mouse outputs a single 1, then all 0s.
- The mouse sensitivity is not adjustable in software.
  - The Hyperkin mouse will ignore cycle sensitivity commands.
  - The sensitivity bits will always read 0.
  - The sensitivity can be toggled with a physical button on the bottom of the Hyperkin mouse.

## Other notes

Some documents about interfacing with the mouse recommend reading the first 16 bits at one speed, delaying a while, and reading the other 16 bits at another speed, following logic analyzer traces from a Super NES console.
However, this is simply a result of Mario Paint using the automatic controller reading feature, and the authentic mouse will give a correct report when read at any reasonable speed.
For example, a program could read 8 bits, wait a couple thousand cycles, and then read the other 24.
The Hyper Click needs a delay after the first 16 bits, though not nearly as much as these documents recommend.

## References

:   - [Super NES Mouse](https://www.nesdev.org/wiki/Super_NES_Mouse) at NESDev Wiki
    - [[SNES Development Manual]]: Book II 4-6-1 Super NES Mouse Specifications
    - [[SNES Development Manual]]: Book II 4-7-8 mouse.x65, Mouse Driver Routine
    - [Shadowrun Mouse Patch](https://rainwarrior.ca/projects/nes/shadowrun_mouse.zip) - source code example for using mouse.
    - [SNESert Golfing](https://github.com/bbbradsmith/NESertGolfing/blob/snes/snes.s#L438) - source code example for using mouse.
    - [Super NES Mouse](https://en.wikipedia.org/wiki/Super_NES_Mouse) at Wikipedia has a list of games

1. [↑](#cite_ref-1) [forum post](https://forums.nesdev.org/viewtopic.php?p=231607#p231607): Hyperkin SNES mouse investigation
2. [↑](#cite_ref-2) [forum post](https://forums.nesdev.org/viewtopic.php?p=236484#p236484): Hyperkin mouse reads have a speed limit
3. [↑](#cite_ref-fullsnes_3-0) Martin Korth. "[Fullsnes: SNES Controllers Mouse Two Button Mouse](https://problemkaputt.de/fullsnes.htm#snescontrollersmousetwobuttonmouse)".
4. [↑](#cite_ref-4) [forum post](https://forums.nesdev.org/viewtopic.php?p=231600#p231600): Hyperkin SNES Mouse cannot software-cycle sensitivity
5. [↑](#cite_ref-5) [FullSNES](http://problemkaputt.de/fullsnes.htm#snescontrollersmousetwobuttonmouse) - Nocash SNES Mouse documentation
6. [↑](#cite_ref-6) [forum post](https://forums.nesdev.org/viewtopic.php?p=232667#p232667): SNES Mouse sensitivity measurements
7. [↑](#cite_ref-7) [forum post](https://forums.nesdev.org/viewtopic.php?p=232668#p232668): Hyperkin Mouse sensitivity measurements
