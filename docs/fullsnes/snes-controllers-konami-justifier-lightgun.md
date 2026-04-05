# SNES Controllers Konami Justifier (Lightgun)

```text
   -_______________________--
  |                :  :....: \_/\
  |________________:__:....:    /
   \________________\.:....; O  \_    <---- O = Start Button
                    |_ _____      |
                      \| ) |/\     \  <---- ) = Trigger
                       \___/  |     |
                              |     |
                              |_____|
               RJ12-socket ____/   \________ Cable (to SNES controller port)
            (for second gun)

  Blue Gun --> connects to SNES (and has 6pin RJ12 socket for second gun)
  Pink Gun --> connects to 6pin RJ12 socket of first gun
```

#### Justifier Bits

```text
  1st..12th   Unused             (always 0=High)
  13th..16th  ID Bit3-0          (MSB first, 1=Low=One, always 0Eh = 1110b)
  17th..24th  Extra ID Bit7-0    (MSB first, 1=Low=One, always 55h = 01010101b)
  25th        Gun 1 Trigger      (1=Low=Pressed?)
  26th        Gun 2 Trigger      (1=Low=Pressed?)
  27th        Gun 1 Start Button (1=Low=Pressed?)
  28th        Gun 2 Start Button (1=Low=Pressed?)
  29th        Previous Frame was H/V-latching Gun1/2 (1=Low=Gun1, 0=High=Gun2)
  30th..32th  Unused             (always 0=High)
  33th and up Unused             (always 1=Low)
```

For obtaining the H/V position & latch flag (Ports 213Ch,213Dh,213Fh), see:

> **See:** [SNES PPU Timers and Status](snes-ppu-timers-and-status.md)

Note that the 29th bit toggles even when Gun2 is not connected.

#### SNES Justifier Game(s)

```text
  Lethal Enforcers (bundled with the hardware) (1993) Konami (US) (EU) (JP)
```

IOBit is used just like for the SuperScope. However, since two guns may be plugged into one port, which gun is actually connected to IOBit changes each time Latch cycles. Also note, the Justifier does not wait for the trigger to be pulled before attempting to latch, it will latch every time it sees the electron gun. Bit 6 of $213F may be used to determine if the Justifier was pointed at the screen or not.

Data2 is presumably not connected, but this is not known for sure.

Nardz: "Actually when I was a kid, I bought the SNES Justifier Battle Clash package. The Weird thing about it is that The package had A Sega Justifier in it, but it came with this SNES/Sega Adapter, which pluged into your SNES and the Sega Justifier would plug into the back of the connector." -- But, Battle Clash is a Super Scope game, not a Justifier game???

The pinouts of the 6pin RJ12-socket are unknown.

#### Sega Version

There's also an identically looking Blue Gun for Sega (but with 9pin joystick connector). The Pink Gun can be used with both SNES and Sega Blue Gun versions.
