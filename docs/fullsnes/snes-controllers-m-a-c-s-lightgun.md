# SNES Controllers M.A.C.S. (Lightgun)

#### Multi-Purpose Arcade Combat Simulator (M.A.C.S.)

This lightgun was used by the US Army to introduce beginners how to kill real people. It was also shown on career days at high schools to win new recruits.

The hardware consists of a small lightpen attached to a M16 rifle. Software cartridges exist for C64 and SNES.

```text
  Lightpen
    _____                                     ____....-----\  _
   |_____| #\\            ________________   |______________\//______________
  ___"__"__# -------""""""                |--|                               |
```

#### |___#__#__#_                             |  |       Trigger  __             |

```text
     "  "  # |_|------..._________________|--|_         ___  |  '--.__       |
                                               |       | \ \  \       '--.___|
                                               |_ _ _ _|_/_/   \
                                                 |     |   \    \
                                                 |_____|    \    \
                                                             |___/
```

#### I/O Ports

The lightgun connects to the lightpen input on 2nd controller port. Aside from the HV-Latches, it uses only one I/O signal:

```text
  4017h.Bit0 Trigger Button (1=LOW=Pressed)
```

That is, only a single bit (no serial data transfer with CLK/STB signals). A standard joypad attached to 1st controller port allows to calibrate the lightpen (via Select button).

For obtaining the H/V position & latch flag (Ports 213Ch,213Dh,213Fh), see:

> **See:** [SNES PPU Timers and Status](snes-ppu-timers-and-status.md)

#### SNES Software

```text
  MACS Basic Rifle Marksmanship v1.1e (v1.2a) (1993) Sculptured Software (US)
  MACS Basic Rifle Marksmanship v1994.0 (1994?)
  MACS Moving Target Simulator (?) (1993) Sculptured Software (US)
```

Note: Version "1.1e" is displayed in the title screen, whilst the version string at ROM offset 05819h identifies it as version "1.2a". The program code looks crude and amateurish, and (as indicated by the corrupted ROM header) it never passed through Nintendo's Seal of Quality process.
