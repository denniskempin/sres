# SNES Cart Satellaview I/O FLASH Detection (Type 1,2,3,4)

The Satellaview FLASH cartridges contain slightly customized "standard" FLASH chips; with a custom Nintendo-specific Chip Detection sequence:

#### Detection Sequence

```text
  [C00000h]=38h, [C00000h]=D0h                  ;request chip info part 1
  delay (push/pop A, three times each)          ;delay
  [C00000h]=71h                                 ;enter status mode
  repeat, X=[C00002h], until (X.bit7=1)         ;wait until ready
  [C00000h]=72h, [C00000h]=75h                  ;request chip info part 2
  FOR i=0 to 9, info[i]=BYTE[C0FF00h+i*2], NEXT ;read chip info (10 bytes)
  [C00000h]=FFh   ;somewhat bugged, see below   ;terminate status mode
```

Note: Nintendo Power flashcarts are also using very similar nonstandard FLASH commands as above (there, for reading hidden mapping info, instead of for chip detection).

BUG: For Type 2 chips, one <should> use "[C05555h]=AAh, [C02AAAh]=55h, [C05555h]=F0h" instead of "[C00000h]=FFh" (the BIOS is actually <trying> to do that, but it's doing it before having deciphered the Type bits).

#### Detection Values

```text
  info[0] - ID1 (Must be "M" aka 4Dh)
  info[1] - ID2 (Must be "P" aka 50h)
  info[2] - Flags (Must be bit7=0 and bit0=0) (other bits unknown)
  info[3] - Device Info (upper 4bit=Type, lower 4bit=Size)
  info[4..9] - Unknown/Unused (BIOS copies them to RAM, but doesn't use them)
```

Type must be 01h..04h for Type 1-4 accordingly. Size must be 07h..0Ch for 128Kbyte, 256Kbyte, 512Kbyte, 1Mbyte, 2Mbyte, 4Mbyte accordingly (ie. 1 SHL N Kbytes).

#### Rejected Values

Wrong ID1/ID2 or wrong Flag Bit7/Bit0 are rejected. Type 00h or 05h..0Fh are rejected. Size 00h..05h is rejected. Size 06h would be a half 128Kbyte block, which is rounded-down to 0 blocks by the BIOS. Size 0Dh would exceed the 32bit block allocation flags in header entry 7FD0h/FFD0h. Size 0Fh would additionally exceed the 8bit size number.

#### Special Cases

If no FLASH cartridge is inserted, then the detection does probably rely on open bus values, ie. "MOV A,[C00002h]" probably needs to return C0h (the last opcode byte) which has bit7=1, otherwise the detection wait-loop would hang forever.

There are reportedly some "write-protected" cartridges. Unknown what that means, ROM-cartridges, or FLASH-cartridges with some (or ALL) sectors being write-protected. And unknown what detection values they do return.

#### FLASH Base Address

The Satellaview BIOS always uses C00000h as Base Address when writing commands to FLASH, the MCC chip could be programmed to mirror FLASH to other locations (although unknown if they are write-able, if so, commands could be also written to that mirrors).

Game Cartridges with built-in FLASH cartridge slot may map FLASH to other locations than C00000h, details on that games are unknown. The game carts don't include MCC chips, but other mapping hardware: In at least some of them the mapping seems to be controlled by a SA-1 chip (an external 10.74MHz 65C816 CPU with on-chip I/O ports, including memory-mapping facilities), the SA-1 doesn't seem to have FLASH-specific mapping registers, so the FLASH might be mapped as secondary ROM-chip. There may be also other games without SA-1, using different FLASH mapping mechanism(s)? For some details, see:

> **See:** [SNES Cart Data Pack Slots (satellaview-like mini-cartridge slot)](snes-cart-data-pack-slots-satellaview-like-mini-cartridge-slot.md)

#### General Notes

FLASH erase sets all bytes to FFh. FLASH writes can only change bits from 1 to 0. Thus, one must normally erase before writing (exceptions are, for example, clearing the "Limited-Start" bits in Satellaview file header). Type 2 can write 128 bytes at once (when writing less bytes, the other bytes in that area are left unchanged). The status/detection values may be mirrored to various addresses; the normal FLASH memory may be unavailable during write/erase/detection (ie. don't try to access FLASH memory, or even to execute program code in it, during that operations).
