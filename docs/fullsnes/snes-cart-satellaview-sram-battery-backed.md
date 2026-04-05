# SNES Cart Satellaview SRAM (Battery-backed)

The Satellaview BIOS cartridge contains 32Kbyte battery-backed SRAM, mapped to eight 4Kbyte chunks at 5000h..5FFFh in Bank 10h..17h.

#### SRAM Map

```text
  0000h 2    ID "SG" (aka 53h,47h)
  0002h 2    Checksum Complement (same as below checksum XORed with FFFFh)
  0004h 2    Checksum (bytes 0..2FFFh added together) (assume [2..5]=0,0,FF,FF)
  0006h 20   User's Name (Shift-JIS)
  001Ch 2    User's Gender (0000h=Boy, 0001h=Girl)
  001Eh 6    Money (max 00E8D4A50FFFh; aka 999,999,999,999 decimal)
  0024h 2    Number of Items (0..10h) (or temporarily up to 11h)
  0026h 44h  Item Entries (4-bytes each: Type=00/01=ROM/RAM, and 24bit pointer)
  006Ah 8F0h Custom RAM Items (8Fh bytes each) (First 2 bytes 00h = free entry)
  095Ah 2    Remaining Time on Doping Item (decreases when entering buildings)
  095Ch 2    Number of Doping Items (walk/run faster when pushing B Button)
  095Eh 2    Remaining Calls on first Telephone Card Item minus 1
  0960h 2    Number of Telephone Card Items (unlocks Phone Booth)
  0962h 2    Number of Transfer Devices (enables Teleport via menu or X-Button)
  0964h 2    Number of Fishing Poles (allows to get Money from Dead Dentist)
  0966h 2    Number of Smaller Neighbor's Home Keys (0000h=Lock, other=Unlock)
  0968h 2    GUI Cursor Shape 16bit selector (0000h..0005h) (other=crash)
  096Ah 3    GUI Border Scheme 24bit pointer (def=9498D9h) (MUST be 94xxxxh)
  096Dh 3    GUI Color Scheme 24bit pointer (initially 94A431h)
  0970h 1    Player got 500 coin from Person 38h      ;\(00h=No, 01h=Yes,
  0971h 1    Player got 1000 coin from Person 39h     ; flags stay set until
  0972h 1    Player got 5000 coin from Person 3Ah     ; that Person leaves
  0973h 1    Player picked-up Red Ball aka Person 00h ;/the town)
  0974h 18h  BIOS Boot/NMI/IRQ Hook Vectors (retf's) (mapped to 105974h and up)
  098Ch 2B0h BIOS Function Hook Vectors (jmp far's) (mapped to 10598Ch and up)
  0C3Ch 64h  BIOS Reset Function (and some zero-filled bytes)
  0CA0h 100h BIOS Interpreter Token Handlers (16bit addresses in bank 81h)
  0DA0h 100h Garbage Filled (reserved for unused Tokens number 80h..FFh)
  0EA0h 2    Garbage Filled (for impossible 8bit token number 100h)
  0EA2h 215Eh Reserved (but, mis-used for game positions by some games)
  3000h 3000h Backup Copy of 0..2FFFh
  6000h 2000h General Purpose (used for game positions by various games)
```

#### Game Positions in SRAM

```text
  0000h-0EA1h  BX-X BIOS (see above)
  1400h-14FFh  BS Super Mario USA 3 (256 bytes)
  1500h-15FFh  BS Super Mario Collection 3 (256 bytes)
  1500h-15FFh  BS Kodomo Tyosadan Mighty Pockets 3 (256 bytes)
  1600h-1626h  BS Satella Walker 2 (27h bytes)
  1600h-1626h  BS Satella2 1 (27h bytes)
  1700h-17FFh  BS Excitebike Bun Bun Mario Battle Stadium 4 (256 bytes)
  2000h-27FFh  BS Marvelous Camp Arnold Course 1 (2Kbytes)
  2800h-2F89h  BS Dragon Quest 1 (1.9Kbytes) (probably 1K, plus 1K backup copy)
  2006h-2FF5h  BS Zelda no Densetsu Remix (3.9Kbytes)
  2000h-2EFFh  BS Zelda no Densetsu Kodai no Sekiban Dai 3 Hanashi (3.75K)
  2000h-2FFFh  BS Super Famicom Wars (V1.2) (first 4K of 8Kbytes)
  3000h-5FFFh  Backup Copy of 0..2FFFh (not useable for other purposes)
  6000h-63FFh  BS Treasure Conflix (1Kbyte)
  6020h-62F9h  BS Sutte Hakkun 98 Winter Event Version (0.7Kbytes)
  6000h-7FFFh  BS Chrono Trigger - Jet Bike Special (8Kbytes)
  6800h-6FFFh  BS Super Famicom Wars (V1.2) (middle 2K of 8Kbytes)
  7500h-7529h  BS Cu-On-Pa (2Ah bytes)
  7800h-7FFFh  BS Super Famicom Wars (V1.2) (last 2K of 8Kbytes)
  7826h-7827h  BS Dr. Mario (only 2 bytes used?)
```

(7C00h-7FFFh) BS Radical Dreamers (default, if free, at 7C00h) (1Kbyte) There is no filesystem with filenames nor SRAM allocation. Most games are using hardcoded SRAM addresses, and do overwrite any other data that uses the same addresses.

One exception is BS Radical Dreamers: The game searches for a free (zerofilled) 1K block (defaults to using 7C00h-7FFFh), if there isn't any free block, then it prompts the user to select a 1K memory block (at 6000h-7FFFh) to be overwritten.

Some games are saving data in PSRAM (eg. Zelda no Densetsu: Kamigami no Triforce, in bank 70h) rather than SRAM - that kind of saving survives Reset, but gets lost on power-off.

There aren't any known BS games that save data in FLASH memory. Some BS games are using passwords instead of saving.

#### BIOS Hooks

These are 4-byte fields, usually containing a "JMP 80xxxxh" opcode (or a "RETF" opcode in a few cases), changing them to "JMP 1x5xxxh" allows to replace the normal BIOS functions by updated functions that are installed in SRAM. Unknown if any such BIOS updates do exist, and at which SRAM locations they are intended/allowed to be stored.

#### SRAM Speed

The Satellaview SRAM is mapped to a FAST memory area with 3.58MHz access time (unlike ALL other SNES RAM chips like internal WRAM or external SRAM in other cartridges).

The bad news is that this wonderful FAST memory isn't usable: It's located in Bank 10h-17h, so it isn't usable as CPU Stack or CPU Direct Page (both S and D registers can access Bank 00h only). And, the first 24Kbytes are used as reserved (and checksummed) area.
