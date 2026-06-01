# SNES Cart Cheat Devices - Pro Action Replay Memory

#### PAR1-PAR3 SRAM

All PAR versions contain 32Kbytes SRAM, divided into 8K chunks, which are unconventionally mapped to EVEN bank numbers.

```text
  00/02/04/06:6000h..7FFFh      ;-32Kbyte SRAM (four 8K banks)
```

The SRAM is used as internal workspace (stack & variables, code list, NMI handler, deadcode handler, and list of possible-matches for the code finder).

Unknown if the SRAM is battery backed (the way how it is used by the BIOS suggests that it is NOT battery backed).

Note: Many HiROM games have their own SRAM mapped to 6000h-7FFFh, unknown if/how/when the PAR can disable its SRAM for compatibility with such games (PAR1 seems to be designed for LoROM games only, but newer PAR2-PAR3 <should> have HiROM support - if so, then the hardware must somehow switch the SRAM on/off depending on whether it executes game code, or PAR code like NMI & deadcode handlers).

#### PAR1-PAR3 Switch

All PAR versions do have a 3-position switch (on the right edge of the cartridge). The way how Datel wants it to be used seems to be:

```text
  1) Boot game with switch in MIDDLE position (maybe needed only for testing)
  2) Set LOWER position & push RESET button (to enter the BIOS menu)
  3) After selecting codes/cheat finder, start game with MIDDLE position
  4) Finally, UPPER position enables codes (best in-game, AFTER intro/menu)
```

Technically, the switch seems to work like so:

```text
  UPPER Position   "Codes on"    Enable GAME and enable codes
  MIDDLE Position  "Codes off"   Enable GAME and disable codes
  LOWER Position   "Trainer On"  Enable BIOS and (maybe) enable codes
```

The "Codes off" setting may be required for booting some games (which may use WRAM for different purposes during intro & game phases). The purpose of the "Trainer" setting is unclear, GAME/BIOS mapping could be as well done via I/O ports (and at least PAR3 does actually have such a feature for reading the GAME header during BIOS execution).

There seems to be no I/O ports for sensing the switch setting, however, the "Codes off" setting can be sensed by testing if the patches (namely the patched NMI vector) are applied to memory or not.

#### PAR BIOS Versions

```text
  Pro Action Replay Mk1 v2.1  1992        32K CRC32=81A67556h
  Pro Action Replay Mk2 v1.0  1992,93     32K CRC32=83B1D39Eh
  Pro Action Replay Mk2 v1.1  1992,93,94  32K CRC32=70D6B036h
  Pro Action Replay Mk3 v1.0U 1995       128K CRC32=0D7F770Ah
```

The two Mk2 versions are 99.9% same (v1.1 is only 10 bytes bigger than v1.0, major change seems to be the copyright message).

Aside from v1.0/v1.1, there are reportedly further PAR2 BIOS versions (named v2.P, v2.T, v2.H). Moreover, there's reportedly at least one localized BIOS (a german PAR3 with unknown version number).

#### PAR Component List

The exact component list is all unknown. Some known components are:

```text
  PAR1-3  3-position switch (on right edge of the cartridge)
  PAR1-3  32Kbytes SRAM (probably not battery-backed)
  PAR1-2  32Kbytes BIOS
  PAR3    128Kbytes BIOS (with modernized GUI and built-in "PRESET" codes)
  PAR1    46pin cartridge slot (incompatible with coprocessors that use 62pins)
  PAR2-3  62pin cartridge slot
  PAR2-3? second Npin cartridge slot at rear side (for CIC from other region)
  PAR3    two LEDs (within sticker-area on front of cartridge)
  PAR1-3  whatever logic chip(s)
```
