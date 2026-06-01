# SNES Add-On Voice-Kun (IR-transmitter/receiver for use with CD Players)

The Voice-Kun (sometimes called Voicer-Kun) from Koei is an Infrared transmitter/receiver. The transmitter part is used for controlling Audio CD Players (ie. to select and play tracks from Audio CDs that are included with supported games). The receiver part is used to "learn" IR-signals from different Remote Control manufacturers.

#### Controller Bits

The existing games expect the IR-unit connected to Port 2 (and a Joypad or Mouse in Port 1). The controller ID can be read via 4017h.Bit0 (serially, with STB/CLK signals). The actual IR-data is transferred via 4017h.Bit1/4201h.Bit7 (directly, without STB/CLK signals).

4017h.Bit0:

```text
  1st-12th    Unknown/unused (probably always 0=High?)
  13th-16th   ID Bits 3-0 (MSB First, 0=High=Zero) (always 0Dh)
  17th and up Unknown/unused (probably always whatever?)
```

4017h.Bit1:

```text
  any bits    Infrared level (receiver) (0=High=Off, 1=Low=On)
```

4201h.Bit7:

```text
  any bits    Infrared level (transmit) (0=Low=Off, 1=High=On)
```

#### Required Buttons

```text
  [  ]  Stop
  [|>}  Play
  [||]  Pause
  [<<]  Previous Track
  [>>]  Next Track
  0..9  Numeric Digits
  +10   Plus 10          ;\alternately either one of these
  >10   Two-Digit-Input  ;/can be selected during configuration
```

The sequence for entering 2-digit Track numbers may vary greatly (+10, or >10 or -/-- buttons, to be pressed before or after the low-digits), and, unknown if the Phillips "Toggle-Bit" is supported (needed for selecting track 11,22,33,etc. So far, one may expect problems with some CD players (though, as workaround, maybe the japanese GUI of the Voice-Kun games allows to select the starting track manually?).

#### Low Level Signals

```text
  Logical  ________--------------------________--------------------________
  Physical ________||||||||||||||||||||________||||||||||||||||||||________
```

The Voice-Kun hardware is automatically modulating/demodulating the transmitted/received signals, so the SNES software does only need to deal with "Logical" signals.

#### Voice-Kun Games

```text
  Angelique Voice Fantasy (29 Mar 1996)
  EMIT Vol. 1 - Toki no Maigo (25 Mar 1995)
  EMIT Vol. 2 - Inochigake no Tabi (25 Mar 1995)
  EMIT Vol. 3 - Watashi ni Sayonara wo (25 Mar 1995)
  EMIT Value Set (EMIT Vol. 1-3) (15 Dec 1995)
```

Note - Soundware

Koei also made a number of "Soundware" games (mostly for other consoles/computers), which did also include Audio CDs or Audio Tapes.

For the SNES, Koei did release "Super Sangokushi II", originally on 15/09/1991, and re-released on 30 Mar 1995, at least one of that two releases (unclear which one) has been reportedly available with "Soundware" - unknown if that soundware version is Voice-Kun compatible, or (if it isn't compatible) unknown how else it was intended to be used...?)
