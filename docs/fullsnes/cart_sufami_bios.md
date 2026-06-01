# SNES Cart Sufami Turbo BIOS Functions & Charset

#### Sufami Turbo BIOS Function Summary

BIOS Function vectors (jmp 80xxxxh opcodes) are located at 80FF00h..80FF3Bh, (the first 12 (of the 15) functions are also duplicated at 80FF80h..80FFAFh).

```text
  80FF00  FillSramPages  ;in: AL=num, AH=slot, XL=first, [09h]=fillword
  80FF04  CopySramToSram ;in: AL=num, AH=direction, X/Y=first (slot A/B)
  80FF08  CopySramToWram ;in: AL=num, AH=direction, X=first, Y=slot, [09h]=addr
  80FF0C  GetChar2bpp    ;in: A=char(0000h..0FFFh), [06h]=dest_addr (64 bytes)
  80FF10  GetChar4bpp    ;in: A=char(0000h..0FFFh), [06h]=dest_addr (128 bytes)
  80FF14  GetCartType    ;out: AL/AH=Types for Slot A/B, b0=ROM, b1=SRAM, b2=?
  80FF18  GetSramSize    ;out: AL/AH=Sizes for Slot A/B, 0-4=0,2,8,32,128Kbyte
  80FF1C  FindFreeSram   ;in: AL=slot, out: AL=first_free_page, FFh=none
  80FF20  GetSramAddrTo6 ;in: AL=slot, XL=page, out: [06h]=addr
  80FF24  GetSramAddrTo9 ;in: AL=slot, XL=page, out: [09h]=addr
  80FF28  ShowHelpSwap   ;display instructions how to exchange cartridges
  80FF2C  ShowHelpNoSwap ;display instructions how to remove cartridges
  80FF30  DeleteFile     ;in: AL=first, AH=slot
  80FF34  TestSramId     ;in: AL=page, AH=slot, out: CY: 0=Used, 1=Free
  80FF38  SramToSramCopy ;in: AL=num, X=src, Y=dst; XH/YH=slot, XL/YL=first
```

Whereas,

```text
  num = number of 2Kbyte pages
  slot = slot (0 or 1 for slot A or B)
  first = first 2Kbyte page number (of a file/area) (within selected slot)
  page = single 2Kbyte page number (within selected slot)
  addr = 24bit SNES memory address
  AL/AH, XL/XH, YL/YH = LSB/MSB of A,X,Y registers
```

The BIOS functions use first 16 bytes in WRAM [0000h..000Fh] for parameters, return values, and internal workspace; when using BIOS functions, don't use that memory for other purposes. [0000h] is NMI mode, don't change that even when NOT using BIOS functions.

#### File/SRAM Functions

These functions may be (not very) helpful for managing SRAM, they are extremly incomplete, there are no functions for creating files, or for searching specific files. See the "Header" chapter for details on SRAM headers (again, the BIOS doesn't create any headers or IDs, the game must fill-in all IDs, Titles, and other values on its own).

#### Character Set

The BIOS ROM contains 4096 characters (each 16x16 pixel, aka 2x2 tiles). The characters are stored at 1bit color depth in banks 04h..07h, offset 8000h-FFFFh (20h bytes/character). The GetChar2bpp and GetChat4bpp functions can be used to copy a selected character to WRAM, with bits in plane0, and the other plane(s) zerofilled.

#### Help Functions

The two help functions are showing some endless repeated japanese instructions about how to use, insert, remove, and exchange cartridges (similar to the instructions shown when booting the BIOS without Game cartridges inserted). If you have uploaded code to the APU, be sure to return control to the APU boot-rom, otherwise the help functions will hang.
