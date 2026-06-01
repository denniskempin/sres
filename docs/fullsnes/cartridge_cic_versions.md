# SNES Cartridge CIC Versions

#### NES CIC Versions

```text
  3193,3193A        NES NTSC Cartridges and Consoles       ;\USA,Canada
  6113,6113A,6113B1 NES NTSC Cartridges (not consoles)     ;/(and Korea?)
  3194              Unknown/doesn't exist?
  3195,3193A        NES PAL Cartridges and Consoles "PAL-B";-Europe
  3196(A?)          NES PAL Cartridges and Consoles        ;-Hong Kong,Asia
  3197(A?)          NES PAL Cartridges and Consoles "PAL-A";-UK,Italy,Australia
  3198(A?)          FamicomBox CIC Cartridges and Consoles ;\
  3199(A?)          FamicomBox Coin Timer (not a CIC)      ; Japan
  N/A               Famicom Cartridges and Consoles        ;/
  RFC-CPU10 (?)     NES R.O.B. robot (no CIC, but maybe a 4bit Sharp CPU, too?)
```

#### SNES CIC Versions

```text
  F411,F411A,F411B   SNES NTSC Cartridges-with-SMD-Chipset and Consoles
  D411,D411A,D411B   SNES NTSC Cartridges-with-DIP-Chipset
  F413,F413A,F413B   SNES PAL Cartridges-with-SMD-Chipset and Consoles
  D413,D413A,D413B   SNES PAL Cartridges-with-DIP-Chipset
  SA-1,S-DD1,MCC-BSC SNES Cartridges (coprocessors/mappers with on-chip CIC)
```

#### NES CIC Clones

```text
  23C1033
  337002   ;Tengen's 16pin "Rabbit" CIC clone
  337006   ;Tengen's 40pin "RAMBO-1" mapper with built-in CIC clone
  4051
  7660
  KC5373B
  MX8018
  NINA
  Ciclone  ;homebrew multi-region CIC clone (based on Tengen design)
```

Aside from using cloned CICs, many unlicensed NES cartridges used a different approach: injecting "wrong" voltages to the console, and "stunning" its CIC.

#### SNES CIC Clones

```text
  10198    - CIC clone
  noname   - CIC clone (black chip without any part number)
  ST10198S - NTSC CIC clone
  ST10198P - PAL CIC clone
  265111   - maybe also a CIC clone (used in Bung Game Doctor SF6)
  D1       - maybe also a CIC clone (used in Super UFO Pro8)
  74LS112  - reportedly also a CIC clone (with fake part number) (UFO Pro6)
  CIVIC 74LS13   16pin - CIC/D411 clone (used in a 8-in-1 pirate cart)
  CIVIC CT6911   16pin - CIC      clone (used in a 7-in-1 pirate cart)
  93C26          16pin - CIC      clone (used in a 8-in-1 pirate cart)
  D1             16pin - CIC? (used in Super VG pirate)
  STS9311A 52583 16pin - CIC clone (used in Donkey King Country 3 pirate)
  black blob     16pin - CIC/D411 clone (used in Sonic the Hedgehog pirate)
```

#### CIC Chip Year/Week Date Codes

```text
  Name   YYWW-YYWW
  3193   8539-8642
  3193A  8547-8733 (in cartridges) (but should be in consoles for more years)
  3195   8627-8638
  3195A  8647-9512
  3197A  8647-9227
  6113   8734-8823
  6113A  8823-8933
  6113B1 8847-9344
```
