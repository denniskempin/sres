---
title: "Cartridge connector"
source_url: "https://snes.nesdev.org/wiki/Cartridge_connector"
pageid: 2
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## Pinout

This diagram represents a view from the right side of the console, looking down into its connector. Pins 01-31 are closest to the front of the console and correspond to the front of the cartridge.

```
    SNES (front) | Cart  |  SNES (back)
                  _______
                 |       |
   SYSTEM CLK -> |01   32| <- /WRAMSEL
       EXPAND <> |02   33| <- REFRESH
          PA6 -> |03   34| <- PA7
         /PRD -> |04   35| <- /PWR
                 |_______|
                 |       |
          GND -- |05   36| -- GND
      CPU A11 -> |06   37| <- CPU A12
      CPU A10 -> |07   38| <- CPU A13
       CPU A9 -> |08   39| <- CPU A14
       CPU A8 -> |09   40| <- CPU A15
       CPU A7 -> |10   41| <- CPU A16
       CPU A6 -> |11   42| <- CPU A17
       CPU A5 -> |12   43| <- CPU A18
       CPU A4 -> |13   44| <- CPU A19
       CPU A3 -> |14   45| <- CPU A20
       CPU A2 -> |15   46| <- CPU A21          Orientation:
       CPU A1 -> |16   47| <- CPU A22          -----------------------
       CPU A0 -> |17   48| <- CPU A23           _____________________
         /IRQ <- |18   49| <- /ROMSEL          |               (----)|\
       CPU D0 <> |19   50| <> CPU D4           |                     | '
       CPU D1 <> |20   51| <> CPU D5           |                     | |
       CPU D2 <> |21   52| <> CPU D6           |_____________________| |
       CPU D3 <> |22   53| <> CPU D7           |  '               '  |\|
      CPU /RD -> |23   54| <- CPU /WR          | 32_______________62 | |
   CIC data 1 <> |24   55| <> CIC data 2       |  |__|_________|__|  | |
key CIC reset -> |25   56| -> CIC CLK          |  1 ....          31 | |
       /RESET <> |26   57| <- PHI2             |_____________________| |
          +5V -- |27   58| -- +5V              |  |_____|___|_____|  |\|
                 |_______|                     | :| --- |   | --^ |  | |
                 |       |                     |  |_____|   |_____|  | |
          PA0 -> |28   59| <- PA1              |  |     | _ |     |  | |
          PA2 -> |29   60| <- PA3              |  |     |___|     |  | |
          PA4 -> |30   61| <- PA5              |  |     |   |     |  | |
left audio in <- |31   62| -> right audio in   '--|_____|---|_____|--' |
                 |_______|                      \*_\_____\___\_____\__\|
```

## Signal descriptions

- **EXPAND**: Connects to expansion port pin 24 and is pulled high.
- **SYSTEM CLK**: 21.47727 MHz system clock.
- **PHI2**: This is the CPU clock output. When this signal is high, this means only that the CPU bus address is in a stable state. For both reads and writes, data is only guaranteed or required to be valid at the falling edge of this signal. /RD, /WR, /PRD, and /PWR are normally used instead of this.
- **CPU A23..0**: The CPU address bus, also known as the **A bus**.
  - **CPU A23..16**: Also known as bank address or **BA7..0**.
  - **CPU /RD, CPU /WR**: Read and write control lines for this bus.
- **/ROMSEL**: Asserted when accessing ROM ($00-3F,80-BF:8000-FFFF, $40-7D,C0-FF:0000-FFFF). Also known as **/CART**.
- **/WRAMSEL**: Asserted when accessing work RAM ($00-3F,80-BF:0000-1FFF, $7E-7F:0000-FFFF).
- **PA7..0**: The peripheral address bus, also known as the **B bus**.
  - **/PRD, /PWR**: Read and write control lines for this bus.
- **D7..0**: Data lines, shared between the CPU and peripheral buses.
- **CIC CLK**: Either 4 MHz or 3.072 MHz, depending on the console.
