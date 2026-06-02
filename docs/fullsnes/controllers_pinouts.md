---
summary: Reference for the SNES joypad controller port pinouts (Port 1 and Port 2), covering the 7-pin connector signals (VCC, clock, strobe, data, I/O/lightpen, GND), the internal 11-pin daughterboard connector on the mainboard, and PAL vs NTSC differences caused by the diode-based open-collector clock and strobe outputs.
keywords: joypad, controller port, pinout, strobe, clock
importance: 4
---

# SNES Controllers Pinouts

#### Joypads (2)

```text
  Pin  Dir  Port 1         Port2            ____________ _________________
  1    -    VCC +5VDC      VCC +5VDC       / 7   6   5  |  4   3   2   1  |
  2    Out  JOY-1/3 Clock  JOY-2/4 Clock  | GND IO6 IN3 | IN1 STB CK1 VCC | 1
  3    Out  JOY-STROBE     JOY-STROBE      \____________|_________________|
  4    In   JOY-1 Data     JOY-2 Data       ____________ _________________
  5    In   JOY-3 Data     JOY-4 Data      / 7  PEN  5  |  4   3   2   1  |
  6    I/O  I/O bit6       I/O bit7, Pen  | GND IO7 IN4 | IN2 STB CK2 VCC | 2
  7    -    GND            GND             \____________|_________________|
```

Pin 6 on Port 2 is shared for I/O and Lightpen input.

#### Internal Connector

The two joypad connectors (and power LED) are located on a small daughterboard, which connects to the mainboard via an 11pin connector:

```text
  1 VCC
  2 IO6       ;-pad1
  3 IO7 / pen ;\
  4 IN2       ; pad2
  5 IN4       ;/
  6 IN1       ;\pad1
  7 IN3       ;/
  8 CK1 (one short LOW pulse per JOY1/JOY3 data bit)
  9 CK2 (one short LOW pulse per JOY2/JOY4 data bit)
  10 STB (one short HIGH pulse at begin of transfer)
  11 GND
```

For PAL consoles: The daughterboard contains diodes in the CK1, CK2, STB lines, effectively making them open-collector outputs (so the joypad may require pull-up resistors for that signals).

#### SNES PAL vs NTSC Controllers

SNES PAL consoles are passing CK1, CK2, STB lines through diodes (the diodes are located on the controller connector daughterboard inside of the console, and the diodes are effectively making that lines open-collector outputs, so PAL controllers do require pull-up resistors for that signals).

SNES NTSC consoles don't have that diodes, and don't require pull-ups. Using PAL controllers on NTSC consoles should work without problems.

For using NTSC controllers on PAL consoles: Remove or shortcut the diodes inside of the SNES, or install pull-ups inside of the controller.
