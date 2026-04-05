# SNES Controllers I/O Ports - Manual Reading

#### 4016h/Write - JOYWR - Joypad Output (W)

```text
  7-3  Not used
  2    OUT2, Output on CPU Pin 39 (seems to be not connected) (1=High)
  1    OUT1, Output on CPU Pin 38 (seems to be not connected) (1=High)
  0    OUT0, Output on CPU Pin 37 (Joypad Strobe) (both gameports, pin 3)
```

Out0-2 are found on CPU Pins 37-39, of which only Out0 seems to be connected.

Note: The NSS (arcade cabinet) uses OUT2 to signalize Game Over to the Z80 coprocessor.

#### 4016h/Read - JOYA - Joypad Input Register A (R)

```text
  7-2  Not used
  1    Input on CPU Pin 33, connected to gameport 1, pin 5 (JOY3) (1=Low)
  0    Input on CPU Pin 32, connected to gameport 1, pin 4 (JOY1) (1=Low)
```

Reading from this register automatically generates a clock pulse on CPU Pin 35, which is connected to gameport 1, pin 2.

#### 4017h/Read - JOYB - Joypad Input Register B (R)

```text
  7-5  Not used
  4    Input on CPU Pin 31, connected to GND (always 1=LOW)       (1=Low)
  3    Input on CPU Pin 30, connected to GND (always 1=LOW)       (1=Low)
  2    Input on CPU Pin 29, connected to GND (always 1=LOW)       (1=Low)
  1    Input on CPU Pin 28, connected to gameport 2, pin 5 (JOY4) (1=Low)
  0    Input on CPU Pin 27, connected to gameport 2, pin 4 (JOY2) (1=Low)
```

Reading from this register automatically generates a clock pulse on CPU Pin 36, which is connected to gameport 2, pin 2.

```text
4201h - WRIO - Joypad Programmable I/O Port (Open-Collector Output) (W)
  7-0   I/O PORT  (0=Output Low, 1=HighZ/Input)
  7     Joypad 2 Pin 6 / PPU Lightgun input (should be usually always 1=Input)
  6     Joypad 1 Pin 6
  5-0   Not connected (except, used by SFC-Box; see Hotel Boxes)
```

Note: Due to the weak high-level, the raising "edge" is raising rather slowly, for sharper transitions one may need external pull-up resistors.

```text
4213h - RDIO - Joypad Programmable I/O Port (Input) (R)
  7-0   I/O PORT  (0=Low, 1=High)
```

When used as Input via 4213h, set the corresponding bits in 4201h to HighZ.

I/O Signals 0..7 are found on CPU Pins 19..26 (in that order). IO-6 connects to Pin 6 of Controller 1. IO-7 connects to Pin 6 of Controller 2, this pin is also shared for the light pen strobe.

Wires are connected to IO-0..5, but the wires disappear somewhere in the multi-layer board (and might be dead-ends), none of them is output to any connectors (not to the Controller ports, not to the cartridge slot, and not to the EXT port).
