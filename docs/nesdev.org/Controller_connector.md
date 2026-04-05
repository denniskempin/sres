---
title: "Controller connector"
source_url: "https://snes.nesdev.org/wiki/Controller_connector"
pageid: 92
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## Pinout

```
Port 1
  _
 / \
| 7 | -- GND
| 6 | <> joypad IO D6
| 5 | <- joypad 1 D1
|---|
| 4 | <- joypad 1 D0
| 3 | -> OUT0
| 2 | -> joypad 1 /OE
| 1 | -- +5V 
|___|

Port 2
 ___
|   |
| 1 | -- +5V
| 2 | -> joypad 2 /OE
| 3 | -> OUT0
| 4 | <- joypad 2 D0
|---|
| 5 | <- joypad 2 D1
| 6 | <> joypad IO D7
| 7 | -- GND
 \_/
```

### Signal descriptions

- **OUT0**: Output to the controller, used on standard controllers to latch the current button state. Written manually via [[MMIO registers#JOYOUT|JOYOUT]] D0 or automatically during autoread.
- **joypad 1 /OE**, **joypad 2 /OE**: Clocks the joypad. Asserted when reading [[MMIO registers#JOYSER0|JOYSER0]] (joypad 1) or [[MMIO registers#JOYSER1|JOYSER1]] (joypad 2) and during autoread.
- **joypad 1 D1..0**: Controller input read manually via [[MMIO registers#JOYSER0|JOYSER0]] D1..0 or automatically via [[MMIO registers#JOY1|JOY1]] (D0) and [[MMIO registers#JOY3|JOY3]] (D1).
- **joypad 2 D1..0**: Controller input read manually via [[MMIO registers#JOYSER1|JOYSER1]] D1..0 or automatically via [[MMIO registers#JOY2|JOY2]] (D0) and [[MMIO registers#JOY4|JOY4]] (D1).
- **joypad IO D7..6**: Bidirectional IO bits accessed through [[MMIO registers#RDIO|RDIO]] and [[MMIO registers#WRIO|WRIO]] D7..6. D7 also functions specially as a light pen input via [[PPU registers#OPHCT|OPHCT]] and [[PPU registers#OPVCT|OPVCT]].

## Standard controller wiring

| Color | Pin | Name |
| --- | --- | --- |
| White | 1 | +5V |
| Yellow | 2 | joypad /OE |
| Orange | 3 | OUT0 |
| Red | 4 | joypad D0 |
| Brown | 7 | GND |
