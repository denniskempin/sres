# SNES Cart Cheat Devices - X-Terminator & Game Wizard

#### Pro Action Replay (PAR1) clone

Wide parts of the X-Terminator BIOS are copied 1:1 from a disassembled PAR1 BIOS. The similarities begin at the entrypoint (with some entirely useless writes to 2140h-2143h), and go as far as using the ASCII characters "W H B ." as default dummy data values for the 4 codes (the initials of the PAR1 programmer W.H.BECKETT). There are some differences to the original PAR1: The hardware and I/O ports are a custom design, the GUI does resemble the PAR2 rather than the PAR1, and english words like "relation" are translated to odd expressions like "differentship". Nonetheless, the thing was called back (in some countries at least), presumably due to all too obvious copyright violations.

#### I/O Ports & Memory Map

```text
  X-Terminator 1         X-Terminator 2
  00FFE8h.W              00FFEAh.W            ;map BIOS (by writing any value)
  00FFE9h.W              00FFEBh.W            ;map GAME (by writing any value)
  00FFEAh.R (NMI read)   00FFEAh.R (NMI read) ;map BIOS/GAME (switch-selection)
  008000h-00FFFFh        008000h-00FFFFh      ;BIOS (32Kbytes)
  N/A                    028000h-02FFFFh      ;Expansion ROM 32Kbytes
  00,02,04,06:6000-7FFF  00-1F:02C00-2FFF     ;SRAM (32Kbytes)
```

Note: Both BIOS versions are confusingly using 16bit writes to the I/O ports in some cases; the LSB-write to [addr+0] has no effect (or lasts only for 1 cpu cycle), the MSB-write to [addr+1] is the relevant part.

The uncommon SRAM mapping in EVEN banks at 6000h-7FFFh was cloned from PAR. The later mapping to 2C00h-2FFFh was probably invented for compatibility with HiROM games that use 6000h-7FFFh for their own SRAM (or possibly just to look less like a PAR clone).

Aside from NMI, the X-Terminator 2 is also using IRQ vectors (though unknown if they are used only during BIOS execution or also during GAME execution, in latter case reads from FFEEh would probably also trigger memory mapping).

#### Game Wizard (by Innovation)

The Game Wizard seems to be a rebadged X-Terminator. Unknown if the I/O addresses are same as for X-Terminator 1 or 2.

#### BIOS Versions

There are at least two versions:

```text
  X-Terminator    1993 (english)  (CRC32=243C4A53h) (no built-in codes)
  X-Terminator 2  19xx (japanese) (CRC32=5F75CE9Eh) (codes for 307 games)
```

There should be probably also a separate version for Game Wizard. And, considering that the BIOS is stored on ERPOM, there might be many further versions & revisions.

Cartridge header is FFh-filled (except for exception vectors), BIOS is 32Kbytes LoROM.

#### X-Terminator Expansion ROMs

There have been at least 5 expansion cartridges released:

```text
  Parame ROM Cassette Vol 1-5 (by Game Tech)
```

The cartridges contain 256Kbytes LoROM, and they can be used in two ways:

As normal executable (via normal ROM header at ROM-offset 7FC0h aka SNES address 00FFC0h), or as cheat-code database extension for the X-Terminator 2 (via a special ROM header at ROM-offset 10000h aka SNES address 028000h).

```text
  028000h - ID "FU O9149" (aka "UFO 1994" with each 2 bytes swapped)
  028008h - Boot callback (usually containing a RETF opcode)
  028010h - List of 16bit pointers (80xxh-FFFFh), terminated by 0000h
```

The 16bit pointers do address following structures (in ROM bank 02h):

```text
  2   checksum (MSB,LSB) taken from GAME cartridge ROM header [FFDCh]
  1   number of following 5-byte codes (N)
  5*N codes (MID,MSB,DTA,LSB,TYPE)  ;TYPE=predefined description (00h..23h)
```

Unknown how the cartridges are intended to be connected (between X-Terminator and Game cartridge... or maybe to a separate expansion slot).

#### Super UFO Copier

The Super UFO copiers are somehow closely related to X-Terminator (probably both made by the same company). X-Terminator codes are supported by various Super UFO versions. Later Super UFO versions also include/support Parame expansion ROMs:

```text
  Super UFO Pro-8 V8.8c BIOS
```

This versions seems to detect "FU O9149" IDs (ie. Parame carts), moreover, it seems to include it's own "FU O9149" ID (but, strangely, at 048000h instead of 028000h, so the X-Terminator won't find it?).

#### X-Terminator Chipset (whatever X-Terminator version)

```text
  Goldstar GM76C256ALL-70 (32Kbytes SRAM, not battery-backed)
  D27256 (32Kbytes UV-Eraseable EPROM)
  two logic chips & two PALs or so (part numbers not legible on existing photo)
  3-position switch (on PCB solder-side) (SCAN/NORMAL/ACTION)
  two cartridge slots (for PAL and NTSC cartridges or so)
```
