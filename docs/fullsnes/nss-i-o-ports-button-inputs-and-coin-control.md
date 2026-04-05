# NSS I/O Ports - Button Inputs and Coin Control

#### Port 00h.R - Joypad Buttons (IC46/74LS540)

```text
  7   SNES Watchdog (0=SNES did read Joypads, 1=Didn't do so) (ack via 07h.W)
  6   Vsync (from OSD or SNES ?)  (0=Vsync, 1=No) (zero for ca. 3 scanlines)
  5   Button "Joypad Button B?"   (0=Released, 1=Pressed)
  4   Button "Joypad Button A"    (0=Released, 1=Pressed)
  3   Button "Joypad Down"        (0=Released, 1=Pressed)
  2   Button "Joypad Up"          (0=Released, 1=Pressed)
  1   Button "Joypad Left"        (0=Released, 1=Pressed)
  0   Button "Joypad Right"       (0=Released, 1=Pressed)
```

#### Port 01h.R - Front-Panel Buttons & Game Over Flag (IC38/74LS540)

```text
  7   From SNES Port 4016h.W.Bit2 (0=Game Over Flag, 1=Normal) (Inverted!)
  6   Button "Restart"            (0=Released, 1=Pressed) ;-also resets SNES?
  5   Button "Page Up"            (0=Released, 1=Pressed)
  4   Button "Page Down"          (0=Released, 1=Pressed)
  3   Button "Instructions"       (0=Released, 1=Pressed)
  2   Button "Game 3"             (0=Released, 1=Pressed) ;\if present (single
  1   Button "Game 2"             (0=Released, 1=Pressed) ; cartridge mode does
  0   Button "Game 1"             (0=Released, 1=Pressed) ;/work without them)
```

#### Port 02h.R - Coin and Service Buttons Inputs (IC32/74LS540)

```text
  7-3 External 5bit input (usually CN5 isn't connected: always 0=High)
  2   Service Button (1=Pressed: Add Credit; with INST button: Config)
  1   Coin Input 2   (1=Coin inserted in coin-slot 2)
  0   Coin Input 1   (1=Coin inserted in coin-slot 1)
```

#### Port 84h.W - Coin Counter Outputs (IC25/74HC161)

```text
  7-4 Unknown/unused (should be always 0) (probably not connected anywhere)
  3-2 Unknown/unused (should be always 0) (probably wired to 74HC161)
  1   Coin Counter 2 (0=No change, 1=Increment external counter)
  0   Coin Counter 1 (0=No change, 1=Increment external counter)
```

Accessed only as "Port 84h". To increase a counter, the bit should be set for around 4 frames, and cleared for at least 3 frames (before sending a second pulse).
