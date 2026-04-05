---
title: "CIC Pinout"
source_url: "https://snes.nesdev.org/wiki/CIC_Pinout"
pageid: 186
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

Pinout of the CIC Lockout Chip.
Similar Mechanism to the NES's.
Comes in Various Revisions: D411, D411B, D413, D413A, D413B, F411A and F413A

This is for the Key Chip, the one Present in the Cartridge (In a DIP Package):

```
       .----\/----.
 D1 <> | 01    16 | -- VCC
 D2 <> | 02    15 | xx NC
 NC xx | 03    14 | xx NC
GND -- | 04    13 | xx NC
 NC xx | 05    12 | xx NC
CLK -> | 06    11 | xx NC
RST -> | 07    10 | xx NC
GND -- | 08    09 | xx NC
       `----------´
```

- D1: CIC Data 1 (Cart.24)
- D2: CIC Data 2 (Cart.55)
- CLK: CIC CLock (Cart.56)
- RST: Key CIC Reset (Cart.25)
- Pin 4: Maybe Key/Lock Select

This is for the Lock Chip, the one Present in the Console (In a Surface-Mount Package):

```
        .----\/----.
  D2 <> | 01    18 | -- VCC
  D1 <> | 02    17 | xx NC
SEED -> | 03    16 | xx NC
  ?? -> | 04    15 | xx NC
  NC xx | 05    14 | xx NC
  NC xx | 06    13 | xx NC
 CLK -> | 07    12 | xx NC
 RST -> | 08    11 | -> /RESET for Cart CIC
 GND -- | 09    10 | -> /RESET for Console
        `----------´
```

- RST: Reset Input from Push Button
- ??: Unknown, tied to VCC. Most likely an Input. Maybe Key/Lock Select
