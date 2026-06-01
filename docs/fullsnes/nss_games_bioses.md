# NSS Games, BIOSes and ROM-Images

#### Nintendo Super System BIOS (Nintendo)

The BIOS is stored in a 32Kx8 EPROM on the mainboard. There are at least three BIOS versions (the version number, "02" for oldest version, and "03" for the two newer versions, is shown at the top of the Selftest result screen). The "02" version is incompatible with newer games (works only with the 3 oldest titles).

```text
  NSS-v02.bin  aka NSS-C.DAT    ;CRC32: A8E202B3 (version "02" oldest)
  NSS-v03a.bin aka NSS-IC14.02  ;CRC32: E06CB58F (version "03" older)
  NSS-v03b.bin aka NSS-V3.ROM   ;CRC32: AC385B53 (version "03" newer/patch)
```

#### NSS Cartridge ROM-Images

ROM-Images should consist of following components in following order:

```text
  1. PRG-ROM (the SNES game) (usually 512Kbytes or 1024Kbytes)
  2. INST-ROM (the Z80 title & instructions) (32Kbytes)
  3. PROM (decryption key) (16 bytes)
```

Note: For the Type B/C PCBs, the PROM is 16 bytes in size. The Type A PCBs seem to be somehow different - details are still unknown; the ROM-image format may need to be changed in case that those details are discovered.

The existing cartridges don't contain any coprocessors - if somebody should make such cartridges, please insert the coprocessor ROM (eg. DSP1) between PRG-ROM and INST-ROM.

#### NSS Games

```text
  PCB Title
  C   Act Raiser (NSS) 1992 Enix (Two EPROMs+DIPSW)
  C   Addams Family, The (NSS) 1992 Ocean (Two EPROMs+DIPSW)
  C   Contra 3: The Alien Wars (NSS) 1992 Konami (Two EPROMs+SRAM+DIPSW)
  C   David Crane's Amazing Tennis (NSS) 1992 Abs.Ent.Inc. (Two EPROMs+DIPSW)
  B   F-Zero (NSS) 1991 Nintendo (ROM+SRAM)
  C   Irem Skins Game, The (NSS) 1992 Irem (Two EPROMs+DIPSW)
  C   Lethal Weapon (NSS) 1992 Ocean (Two EPROMs+DIPSW)
  -   Magic Floor (NSS) 2012 nocash (EPROM+DIPSW, works without PROM)
  C   NCAA Basketball (NSS) 1992 Sculptured Software Inc. (Two EPROMs+DIPSW)
  C   Robocop 3 (NSS) 1992 Ocean (Two EPROMs+DIPSW)
  A   Super Mario World (NSS) 1991 Nintendo (ROM)
  A   Super Soccer (NSS) 1992 Human Inc. (EPROM)
  A   Super Tennis (NSS) 1991 Nintendo (ROM)
```

Additionally, Ocean has announced Push-Over (unknown if that was ever released). And, there seems to have been a Super Copa cartridge in Mexico. And, there is somebody owning a NHL Stanley Cup prototype cartridge.

Contra 3 also appears to exist as prototype only (its INST-ROM title/instructions are just saying "New Game 1" and "To be announced").
