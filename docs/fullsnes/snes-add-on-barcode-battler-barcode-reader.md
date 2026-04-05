# SNES Add-On Barcode Battler (barcode reader)

The Barcode Battler from Epoch allows to scan barcodes (either from special paper cards, or from daily-life products like food packagings), games can then use the barcode digits as Health Points, or other game attributes.

#### Standalone-Mode

The device was originally designed as stand-alone gaming console with some push buttons, a very simple LCD screen with 7-segment digits & some predefined LCD symbols, and a built-in game BIOS (ie. without external cartridge slot, and without any bitmap graphics).

#### Link-Mode

Later versions (with black case) include an "EXT" link port, allowing to link to other Barcode Battler hardware, or to Famicom/Super Famicom consoles. The EXT port is probably bi-directional, but existing Famicom/Super Famicom games seem to be using it only for reading barcodes (without accessing the LCD screen, push buttons, speaker, or EEPROM).

> **See:** [SNES Add-On Barcode Transmission I/O](snes-add-on-barcode-transmission-i-o.md)
> **See:** [SNES Add-On Barcode Battler Drawings](snes-add-on-barcode-battler-drawings.md)

#### Barcode Battler Famicom (NES) Games

```text
  Barcode World (1992) Sunsoft (JP) (includes cable with 15pin connector)
```

#### Barcode Battler Super Famicom (SNES) Games

```text
  Alice's Paint Adventure (1995)
  Amazing Spider-Man, The - Lethal Foes (19xx)
  Barcode Battler Senki Coveni Wars (1993) Epoch
  Donald Duck no Mahou no Boushi (19xx)
  Doraemon 2: Nobita's Great Adventure Toys Land (1993)
  Doraemon 3: Nobita and the Jewel of Time (1994)
  Doraemon 4 - Nobita to Tsuki no Oukoku (19xx)
  Doroman (canceled)
  Dragon Slayer - Legend of Heroes 2 (1993) Epoch
  J-League Excite Stage '94 (1994)
  J-League Excite Stage '95 (1995)
  Lupin Sansei - Densetsu no Hihou wo Oe! (19xx)
  Super Warrior Combat (19xx - does this game exist at all?)
```

#### Barcode Battler Hardware Versions

```text
  Region__Case___EXT___Barcode-Reader__Name__________________Year___
  Japan   White  None  Yes             Barcode Battler       1991
  Japan   Black  1     Yes             Barcode Battler II    1992
  Japan   Black  2     None            Barcode Battler II^2  199x
  Europe  Black  1     Yes             Barcode Battler       1992/1993
```

The versions with one EXT socket can be connected to NES/SNES, or to one or more of the "II^2" units (allowing more players to join the game).

#### Connection to SNES/NES consoles

Connection to Super Famicom or SNES requires a "BBII INTERFACE": a small box with 4 LEDs and two cables attached (with 3pin/7pin connectors), the interface has been sold separetedly, it's needed to add a SNES controller ID code to the transmission protocol.

Connection to Famicom consoles requires a simple cable (without interface box) (with 3pin/15pin connectors), the cable was shipped with the "Barcode World" Famicom cartridge, connection to NES would require to replace the 15pin Famicom connector by 7pin NES connector.

The required 3pin EXT connector is available only on newer Barcode Battlers (with black case), not on the original Barcode Battler (with white case).

```text
  Unknown if all 3 pins are actually used by NES/SNES cable/interface?
  Unknown if NES/SNES software can access LCD/buttons/speaker/EEPROM ?
```

Connectivity "Connectivity mode is accessible if you plug in a standard 3.5mm mono jack plug into the expansion port on the left hand side of the unit, hold down the R-Battle and R-Power buttons and turn the unit on, the Barcode Battler II goes into scanner mode."

#### Barcode Battler II Interface

The hardware itself was manufactured by Epoch, and licensed by Nintendo (it says so on the case).

The four lights, from left to right, indicate as follows:

```text
  "OK"    All is well, the device is operating as normal.
  "ER"    Maybe there's something wrong?
  "BBII"  The Barcode Battler is sending data to the device.
  "SFC"   The SFC/SNES is waiting for a signal from the Barcode Battler.
```

#### Component List (may be incomplete)

```text
  80pin NEC uPD75316GF (4bit CPU with on-chip 8Kx8 ROM, 512x4 RAM, LCD driver)
  8pin Seiko S2929A (Serial EEPROM, 128x16 = 2Kbit) (same/similar as S29290)
  3pin EXT socket (3.5mm "stereo" jack) (only in new versions with black case)
  LCD Screen (with 7-segment digits and some predefined words/symbols)
  Five LEDs (labelled "L/R-Battle Side")
  Seven Push Buttons (L/R-POWER, L/R-Battle, Power on/off, Select, Set)
  Speaker with sound on/off switch (both on bottom side)
  Barcode reader (requires card-edges to be pulled through a slot)
  Batteries (four 1.5V AA batteries) (6V)
```
