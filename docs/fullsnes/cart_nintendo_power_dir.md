# SNES Cart Nintendo Power - Directory

#### Directory Area

```text
  ROM Offset  SNES Address Size   Content
  060000h     8C8000h      2000h  File 0 (Menu)
  062000h     8CA000h      2000h  File 1
  064000h     8CC000h      2000h  File 2
  066000h     8CE000h      2000h  File 3
  068000h     8D8000h      2000h  File 4
  06A000h     8DA000h      2000h  File 5
  06C000h     8DC000h      2000h  File 6
  06E000h     8DE000h      2000h  File 7
  070000h     8E8000h      10000h Unused (FFh-filled)
```

The last 64Kbyte are probably usable as further file entries in cartridges bigger than 4Mbyte (the Menu software in the existing cartridges is hardcoded to process only files 1..7) (whilst Port 2400h seems to accept 4bit file numbers).

#### Directory Entry Format

```text
  0000h 1    Directory index (00h..07h for Entry 0..7) (or FFh=Unused Entry)
  0001h 1    First 512K-FLASH block (00h..07h for block 0..7)
  0002h 1    First 2K-SRAM block    (00h..0Fh for block 0..15)
  0003h 2    Number of 512K-FLASH blocks (mul 4) (=0004h..001Ch for 1..7 blks)
  0005h 2    Number of 2K-SRAM blocks (mul 16)   (=0000h..0100h for 0..16 blks)
  0007h 12   Gamecode (eg. "SHVC-MENU-  ", "SHVC-AGPJ-  ", or "SHVC-CS  -  ")
  0013h 44   Title in Shift-JIS format (padded with 00h's) (not used by Menu)
  003Fh 384  Title Bitmap (192x12 pixels, in 30h*8 bytes, ie. 180h bytes)
  01BFh 10   Date "MM/DD/YYYY" (or "YYYY/MM/DD" on "NINnnnnn" carts)
  01C9h 8    Time "HH:MM:SS"
  01D1h 8    Law  "LAWnnnnn" or "NINnnnnn" (eg. "LAW01712", or "NIN11001")
  01D9h 7703 Unused (1E17h bytes, FFh-filled)
  1FF0h 16   For File0: "MULTICASSETTE 32" / For Files 1-7: Unused (FFh-filled)
```

#### Directory Index

Directory Index indicates if the Entry is used (FFh=unused). If it is used, then it must be equal to the current directory entry number (ie. a rather redundant thing, where the index is indexing itself). The lower 4bit of the index value is used for game selection via Port 2400h.

#### First FLASH/SRAM Block

The First FLASH block number is stored in lower some bits of [0001h].

The First SRAM block number is stored in lower some bits of [0002h].

The directory doesn't contain any flag that indicates HiROM or LoROM mapping.

There is no support for fragmented FLASH/SRAM files (ie. the programming station must erase & rewrite the entire cartridge, with the old used/unused blocks re-ordered so that they do form continous memory blocks).

Number of FLASH/SRAM Blocks (displayed in Menu) These entries are used to display the amount of used/unused blocks. Free FLASH blocks are shown as blue "F" symbols, free SRAM blocks as red "B" symbols, used blocks as gray "F" and "B" symbols. Pressing the X-button in the menu indicates which blocks are being used by the currently selected game.

#### Title Bitmap (displayed in Menu)

The 192x12 pixel title bitmap is divided into eight 24x12 pixel sections, using a most bizarre encoding: Each section is 30h bytes in size (enough for 32 pixel width, but of these 32 pixels, the "middle" 4 pixels are overlapping each other, and the "right-most" 4 pixels are unused. The byte/pixel order for 12 rows (y=0..11) is:

```text
  Left 8 pixels   = (Byte[00h+y*2])
  Middle 8 pixels = (Byte[01h+y*2]) OR (Byte[18h+y*2] SHR 4)
  Right 8 pixels  = (Byte[18h+y*2] SHL 4) OR (Byte[19h+y*2] SHR 4)
```

The result is displayed as a normal 192-pixel bitmap (without any spacing between the 24-pixel sections). The bits in the separate bytes are bit7=left, bit0=right. Color depth is 1bit (0=dark/background, 1=bright/text). The bitmap does usually contain japanese text without "special" features, though it could also be used for small icons, symbols, bold text, greek text, etc.

#### Text Fields (not used by Menu)

The Shift-JIS Title, and ASCII Game Code, Date, Time, Law & Multicassette strings aren't used by the Menu. The 5-digit Law number is usually (but not always) same for all files on the cartridge, supposedly indicating the station that has programmed the file.

```text
  LAW = games installed on kiosks in Lawson Convenience Store chain
  NIN = games pre-installed by nintendo (eg. derby 98)
```

The Multicassette number does probably indicate the FLASH size in MBits (it's always 32 for the existing 32Mbit/4MByte cartridges).
