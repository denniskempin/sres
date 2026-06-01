# SNES Controllers Multiplayer 5 - Unsupported Hardware

The Multiplayer 5 is incompatible with almost everything except normal joypads/joysticks. The only other things that do work are Twin Taps, and maybe also the NTT Data Pad (unless it exceeds 17mA, and unless games do refuse it as device with "unknown" controller ID).

#### Unsupported Hardware in MP5 controller slots (due to missing signals)

```text
  Lightguns
  Turbo File
  SFC Modem
  Voice-Kun
  X-Band Keyboard
  A second MP5 plugged into the first MP5
```

#### Prohibited Hardware

The hardware/combinations listed below are "prohibited" (and aren't supported by any offical/licensed games). Nethertheless, they should be working in practice, hopefully without immediately catching fire or blowing the fuse (but might act unstable or cause some overheating in some situations; results <might> vary depending on hardware/production variants, room temperature, used CPU/PPU/APU load, individual or official safety measures, and on type/amount of connected cartridges/controllers).

Prohibited Hardware in MP5 controller slots (due to exceeding 17mA per slot)

```text
  Mouse (requires 50mA)
  Devices with unknown controller IDs (which might exceed 17mA)
  Maybe also various unlicensed wireless/autofire joypads/joysticks
```

#### Prohibited Hardware in CARTRIDGE slot (due to overall power consumption)

```text
  Cartridges with GSU-n (programmable RISC CPU) (aka Super FX/Mario Chip)
  Maybe also things like X-Band modem and Cheat Devices
```

#### Prohibited Hardware in BOTH controller ports (unspecified reason)

```text
  Two MP5's (connected to port 1 and 2) (maybe also power consumption related)
```

#### Prohibited Hardware in FIRST controller port (just by convention)

```text
  MP5 in port 1 (instead of port 2) (would mess-up the port 2-5 numbering)
```
