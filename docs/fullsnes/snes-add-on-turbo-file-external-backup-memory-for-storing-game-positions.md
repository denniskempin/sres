# SNES Add-On Turbo File (external backup memory for storing game positions)

The Turbo File add-ons are an external battery-backed RAM-Disks made by ASCII.

Turbo File hardware has been produced for NES, SNES, 8bit Gameboy, and Gameboy Advance. It's been sold only in japan, and it's mainly supported by ASCII's own games. The SNES related hardware versions are:

```text
  SNES Turbo File Twin (160K) (128K in STF mode, 4x8K in TFII mode)
  SNES Turbo File Adapter (SNES adapter for NES Turbo File & Turbo File II)
```

TFII Mode (old NES mode, 4x8Kbyte)

> **See:** [SNES Add-On Turbo File - TFII Mode Transmission Protocol](snes-add-on-turbo-file-tfii-mode-transmission-protocol.md)
> **See:** [SNES Add-On Turbo File - TFII Mode Filesystem](snes-add-on-turbo-file-tfii-mode-filesystem.md)

STF Mode (native SNES mode, 128Kbyte)

> **See:** [SNES Add-On Turbo File - STF Mode Transmission Protocol](snes-add-on-turbo-file-stf-mode-transmission-protocol.md)
> **See:** [SNES Add-On Turbo File - STF Mode Filesystem](snes-add-on-turbo-file-stf-mode-filesystem.md)

#### Compatible Games

> **See:** [SNES Add-On Turbo File - Games](snes-add-on-turbo-file-games.md)

#### NES Turbofile (AS-TF02)

Original NES version, contains 8Kbytes battery backed RAM, and a 2-position PROTECT switch, plus a LED (unknown purpose).

#### NES Turbo File II (TFII)

Newer NES version, same as above, but contains 32Kbytes RAM, divided into four 8Kbyte slots, which can be selected with a 4-position SELECT switch.

#### SNES Turbo File Adapter

Allows to connect a Turbo File or Turbo File II to SNES consoles. Aside from the pin conversion (15pin NES to 7pin SNES), it does additionally contain some electronics (for generating a SNES controller ID, and a more complicated protocol for entering the data-transfer phase). Aside from storing SNES game positions, this can be also used to import NES files to SNES games.

#### SNES Turbo File Twin

SNES version with 160Kbyte SRAM, and with 5-position mode SELECT switch. 128K used in STF mode ("SNES Super Turbo File"), and 4x8K used in TFII modes 1/2/3/4 (equivalent to NES Turbo File II with SNES Turbo File Adapter).

Small square box that connects via cable to controller port.

#### Two position PROTECT switch (off/on)

Five position SELECT switch (STF, and "TFII" 1,2,3,4) There is a red LED. And two 1.5V batteries?

#### Hardware Versions

```text
  Name                Capacity                  Connection
  Turbofile (AS-TF02) 1x8Kbyte                  NES-to-SNES Adapter
  Turbo File II       4x8Kbyte                  NES-to-SNES Adapter
  Turbo File Twin     4x8Kbyte plus 128Kbyte    Direct SNES Connection
  Gameboy version     ?                         N/A ?
  Gameboy Advance     ?                         N/A ?
```
