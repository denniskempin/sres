# SNES Pinouts BSX Connectors

FLASH Card Slot (as found on a "BSC-1A5M-01" board) There are two conflicting numbering schemes for the 62pin connector.

Pin-numbering on the black plastic connector:

```text
  Rear/Left  --> 62 ............................... 32 <-- Rear/Right
  Front/Left --> 31 ............................... 1  <-- Front/Right
```

Pin-numbering on SNES cartridge PCB text layer:

```text
  Rear/Left  --> 62 ............................... 2  <-- Rear/Right
  Front/Left --> 61 ............................... 1  <-- Front/Right
```

Below is using the PCB text layer's numbering scheme:

```text
  1 GND
  2 GND
  3 D0
  4 D4 (with cap to gnd)
  5 D1 (with cap to gnd)
  6 D5
  7 D2
  8 D6
  9 D3
  10 D7
  11 A12
  12 -
  13 A7
  14 via R2 to /RD (33 ohm)
  15 A6
  16 via R3 to /WR (33 ohm)
  17 A5
  18 VCC
  19 A4
  20 -
  21 A3
  22 via R4 to VCC (47kOhm)
  23 A2
  24 via R5 to GND (47kOhm)
  25 A1
  26 via R6 to GND (47kOhm)
  27 A0
  28 -
  29 A14
  30 VCC
  31 VCC
  32 VCC
  33 via R7 to VCC (47kOhm)
  34 3/5 (GNDed=5V)
  35 A13
  36 REFRESH    to SNES.pin.33
  37 A8
  38 A15 rom     SNES.A16 SNES.pin.41
  39 A9
  40 A16 rom     SNES.A17 SNES.pin.42
  41 A11
  42 A17 rom     SNES.A18 SNES.pin.43
  43 A10
  44 A18 rom     SNES.A19 SNES.pin.44
  45 SYSCK       SNES.pin57 (and via R1 to SNES.pin.2 EXPAND) (100 ohm)
  46 A19 rom     SNES.A20 SNES.pin.45
  47 /RESET
  48 A20 rom     SNES.A21 SNES.pin.46
  49 -
  50 A21 rom     SNES.A23 SNES.pin.48 (NOT SNES.A22 !!!)
  51 /CS (from MAD-1A.pin1)
  52 GND
  53 Dx
  54 Dx
  55 Dx
  56 Dx     ... pins here are D8-D15 (on PCBs with 16bit databus)
  57 Dx
  58 Dx
  59 Dx
  60 Dx
  61 GND
  62 GND
```

pitch: 38.1mm per 30 pins === 1.27mm per pin There are some connection variants: The Itoi cartridge with SA-1 is using 16bit databus (with extra data lines somewhere on pin53-60), pin12 seems to be connected to something, some of the the pull-up/pull-downs and VCC/GND pins on pin 14-34 and 52 may be wired differently.

SNES Cartridge Slot Usage for Satellaview BIOS and Datapack carts REFRESH (SNES.pin33) is forwarded to FLASH cart slot (for unknown reason, maybe it is ACTUALLY used for deselecting FLASH during REFRESH, or maybe it was INTENDED for unreleased DRAM cartridges).

SYSCK (SNES.pin57) is forwarded to FLASH cart slot (for unknown reason), and is also forwarded (via 100 ohms) to EXPAND (SNES.pin2) (and from there forwarded to the BSX Receiver Unit on SNES Expansion port).

#### BSX-EXT-Port Pinouts (half-way rev-engineered by byuu)

```text
  1  = +5V
  2  = +5V
  3  = +5V
  4  = +5V
  5  = GND
  6  = GND
  7  = GND
  8  = GND
  9  = GND
  10 = GND
  11 = U3.pin17 (B2) ;\
  12 = U3.pin18 (B1) ;
  13 = U3.pin15 (B4) ;
  14 = U3.pin16 (B3) ;
  15 = U3.pin13 (B6) ;
  16 = U3.pin14 (B5) ;
  17 = U3.pin11 (B8) ;
  18 = U3.pin12 (B7) ;/
  19 = U2.pin11 (Y8)
  20 = GND
  21 = U2.pin12 (Y7)
  22 = GND
  23 = ???
  24 = GND
  25 = U1.pin12 (Y7)
  26 = GND
  27 = U1.pin11 (Y8)
  28 = U1.pin13 (Y6)
  29 = ???
  30 = ???
  31 = ???
  32 = U2.pin14 (Y5)
  33 = ???
  34 = ???
  35 = GND
  36 = GND
  37 = GND
  38 = GND
```

The U3 transceiver is probably passing databus to/from SNES, the U1/U2 drivers are maybe passing some address/control signals from SNES. The indirect connection via U1/U2/U3 may be intended to amplify the bus, or to disconnect the bus (in case when the receiver power supply isn't connected).
