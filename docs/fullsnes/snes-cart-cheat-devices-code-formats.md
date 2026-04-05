# SNES Cart Cheat Devices - Code Formats

PAR AAAAAADD - Normal Pro Action Replay Codes (Datel) The Pro Action Replay is a cheat device for the SNES produced by Datel. The original PAR only support 3 codes, but the PAR2 supports 255 and has a built-in trainer for code searcher. There is also a PAR3, but the added features are unknown.

```text
  AAAAAADD  ;-address (AAAAAA) and data (DD)
```

Address can be a ROM, SRAM, or WRAM location. Patching cartridge memory (both ROM and SRAM) is implemented by hardware (supported by PAR2-PAR3 only, not by PAR1 or X-Terminator 1-2). Patching WRAM is done by software (rewriting the values on each Vblank NMI). WRAM addresses must be specified as 7E0000h-7FFFFFh (mirrors at nn0000h-nn1FFFh aren't recognized by the BIOSes).

#### PAR 7E000000 - Do nothing

This is the most important PAR code (required as padding value, since the GUI doesn't allow to remove items from the code list):

PAR FE0000xx..FFFFFFxx - Pre-boot WRAM patch (PAR1 only) Writes xx to the corresponding WRAM address at 7E0000h..7FFFFFh, this is done only once, and it's done BEFORE starting the game (purpose unknown - if any).

PAR 00600000 - Disable Game's NMI handler (PAR3 only) Disables the game's NMI handler (executes only the NMI handler of the PAR BIOS).

PAR DEADC0DE - Special Multi-Byte Code Prefix (PAR2a/b and PAR3 only) Allows to hook program code, this feature is rarely used.

```text
  DEADC0DE  ;-prefix (often misspelled as "DEADCODE", with "O" instead "0")
  AAAAAANN  ;-address (AAAAAA) and number of following 4-byte groups (NN)
  DDEEFFGG  ;-first 4-byte group    (DD=1st byte, .. GG=4th byte)
  HHIIJJKK  ;-second 4-byte group   (HH=5th byte, .. KK=8th byte) (if any)
  ...       ;-further 4-byte groups (etc.)                        (if any)
```

The data portion (DD,EE,FF..) (max 62h*4 = 188h bytes) is relocated to SRAM (in the PAR cartridge), and the ROM address AAAAAA is patched by a 4-byte "JMP nnnnnn" opcode (doing a far jump to the address of the relocated SRAM code; this would be at 006A80h in PAR3, at 006700h in PAR2a/b, and isn't supported in PAR1). There seems to be no special action required when returning control to the game (such like disabling the SRAM - if the hardware does support that at all?) (or such actions are required only for HiROM games that have their own SRAM at 6000h?) (or games are typically accessing SRAM at 306xxxh, so there is no conflict with PAR memory at 006xxxh?).

One can use only one DEADC0DE at a time, and, when using it, there are some more restrictions: On PAR2a/b one cannot use ANY other hardware/software patches. On PAR3 one can keep using ONE hardware patch (and any number of software patches).

PAR C0DEnn00 - Whatever (X-Terminator 2 only - not an official PAR code) Somehow changes the NMI (and IRQ) handling of the X-Terminator 2, "nn" can be 00..06.

#### Game Genie Codes (Codemasters/Galoob)

```text
  DDAA-AAAA  ;-encrypted data (DD) and encrypted/shuffled address (AA-AAAA)
```

Address can be a ROM, or SRAM location (internal WRAM isn't supported). To decrypt the code, first replace the Genie Hex digits by normal Hex Digits:

```text
  Genie Hex:    D  F  4  7  0  9  1  5  6  B  C  8  A  2  3  E
  Normal Hex:   0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
```

Thereafter, DD is okay, but AAAAAA still needs to be deshuffled:

```text
  ijklqrst opabcduv wxefghmn   ;Genie Address (i=Bit23 ... n=Bit0)
  abcdefgh ijklmnop qrstuvwx   ;SNES Address (a=Bit23 ... x=Bit0)
```

Aside from being generally annoying, the encryption makes it impossible to make codes like "Start in Level NN" (instead, one would need to make separate codes for each level).

Game Genie codes can be reportedly also used with the Game Mage. And, the PAR3 includes a "CONV" button for converting Game Genie codes. When manually decrypting them, Game Genie codes would also work on PAR2 (although the PAR2 won't allow to use five ROM patches at once).

Gold Finger / Goldfinger Codes (unknown who created this format) These codes are rarely used, and there isn't much known about them. Reportedly, they have been supported by "certain copiers" (unknown which ones... Super UFO, and also copiers from Front Far East?).

```text
  AAAAADDEEFFCCW  ;-Address (AAAAA), Data (DD,EE,FF), Checksum (CC), Area (W)
```

The Address is a ROM address, not a CPU address. Data can be 1-3 bytes, when using less than 3 bytes, pad the EE,FF fields with "XX" (and treat them as zero in the checksum calculation). Checksum is calculated as so:

```text
  CC = A0h + AAAAA/10000h + AAAAA/100h + AAAAA + DD (+ EE (+ FF))
```

W tells the copier whether to replace the byte in the DRAM (ROM image) or the SRAM (Saved game static RAM) of the copier:

```text
  W=0  DRAM (ROM image) (reportedly also for W=2,8,A,C,F)
  W=1  SRAM (Saved game image)
```

The 5-digit address allows to access max 1Mbyte. The address is an offset within the ROM-image (excluding any 200h-byte header). The first LoROM byte would be at address 00000h. Some copiers are using interleaved HiROM images -unknown if any such interleave is used on code addresses - if so, first HiROM byte would be at "ROMsize/2" (in middle of ROM-image), otherwise it'd be at 00000h (at begin of ROM-image).

Note: It doesn't seem to be possible to enter "X" digits in all copiers. Double Pro Fighter Q allows to enter "X".

Front Far East Codes (Front Far East) Supported by Front Far East copiers (Super Magicom and/or Super Wild Card?).

This format is even less popular than the Gold Finger format.

```text
  NNAAAAAADD..    Number of bytes (NN), Address (AAAAAA), Data (DD..)
```

Allows to change 1..36 bytes; resulting in a code length of 10..80 digits (to avoid confusion with 14-digit Gold Finger codes, Front Far East wants Gold Finger codes to be prefixed by a "G").

AAAAAA is a 24bit offset within the ROM-image (excluding any 200h-byte header).

As far as known, Front Far East didn't use interleaved ROM-images, so the offset should be straight.
