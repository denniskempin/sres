# SNES Cart Satellaview FLASH File Header

#### Satellaview FLASH File Header

Located at offset 7Fxxh or FFxxh in file, mapped to FFxxh in bank 00h.

```text
  FFB0h 2  Maker Code (2-letter ASCII)                   ;\garbage when
  FFB2h 4  Program Type (00000100h=Tokens, Other=65C816) ; [FFDBh]=01h
  FFB6h 10 Reserved (zero)                               ;/
  FFC0h 16 Title (7bit ASCII, 8bit JIS (?), and 2x8bit SHIFT-JIS supported)
  FFD0h 4  Block Allocation Flags (for 32 blocks of 128Kbytes each) (1=used)
              Retail (demo) games usually have ffff here -- Uh ???
              (exception BS Camp Arnold Marvelous)       -- Uh ???
  FFD4h 2  Limited Starts (bit15=0=Infinite, otherwise bit14-0=Remaining Flags)
  FFD6h 1  Date (Bit7-4=Month, Bit3-0=0)             ;\copied to from
  FFD7h 1  Date (Bit7-3=Day, Bit2=0, Bit1-0=Unknown) ;/Directory Packet
  FFD8h 1  Map Mode (20h=LoROM, 21h=HiROM) (or sometimes 30h/31h)
  FFD9h 1  File/Execution Type
            Bit0-3  Unknown/unused (usually/always 0)
            Bit4    Receiver Power Down (0=No/Sound Link, 1=Power-Down 2197h)
            Bit5-6  Execution Area (0=FLASH, 1=Reloc FLASH-to-PSRAM, 2/3=Fail)
            Bit7    Skip the "J033-BS-TDM1 St.GIGA" Intro (0=Normal, 1=Skip)
  FFDAh 1  Fixed (33h)
  FFDBh 1  Unknown (usually 02h, sometimes 01h, or rarely 00h) (see FFBxh)
  FFDCh 2  Checksum complement (same as below, XORed with FFFFh)
  FFDEh 2  Checksum (all bytes added together; assume [FFB0-DF]=00h-filled)
  FFE0h 32 Exception Vectors (IRQ,NMI,Entrypoint,etc.) (for 65C816 code)
```

Entrypoint is at 800000h+[FFFCh] for 65C816 Machine Code, or at 400000h for Interpreter Tokens (ie. when [FFB2h]=00000100h, as used by few magazines like BS Goods Press 6 Gatsu Gou).

Caution: Machine Code programs are started WITHOUT even the most basic initialization (one of the more bizarre pieces: the download "screensaver" may leave HDMAs enabled when auto-starting a downloaded file).

Satellaview PSRAM File Header (download to PSRAM without saving in FLASH) Basically same as FLASH headers. FFD9h.Bit7 (skip intro) seems to be ignored (accidently using FFD9h from FLASH cartridge instead from PSRAM?). The checksum isn't verified (FFDEh must match with FFDCh, but doesn't need to match the actual file content).

Satellaview Transmit Header (located at 7Fxxh or FFxxh in File) Basically same as normal Satellaview FLASH File Header. However, after downloading a file (and AFTER storing it in FLASH memory), the BIOS does overwrite some Header entries:

```text
  FFD0h  4-byte  Block Allocation field (set to whichever used FLASH Blocks)
  FFD6h  2-byte  Date field (set to Date from Satellite Directory Entry)
  FFDAh  1-byte  Fixed Value (set to 33h)
```

Since FLASH cannot change bits from 0 to 1 (without erasing), the above values must be FFh-filled in the transmitted file (of course, for the fixed value, 33h or FFh would work) (and of course, the FFh-requirement applies only to FLASH downloads, not PSRAM downloads).

#### Notes

The title can be 16 bytes (padded with 20h when shorter), either in 7bit ASCII, 8bit JIS (or so?), or 2x8bit SHIFT-JIS, or a mixup thereof. A few files are somewhat corrupted: Title longer than 16 bytes (and thereby overlapping the Block Allocation flags).

Limited Starts consists of 15 flags (bit14-0), if the limit is enabled (bit15), then one flag is changed from 1-to-0 each time when starting the file (the 1-to-0 change can be done without erasing the FLASH sector).

Note that the checksum excludes bytes at FFB0h-FFDFh, this is different as in normal cartridges (and makes it relative easy to detect if a file contains a SNES ROM-image or a Satellaview FLASH-file/image.

Files are treated as "deleted" if their Fixed Value isn't 33h, if Limited Starts is 8000h (limit enabled, and all other bits cleared), or if Checksum Complement entry isn't equal to Checksum entry XOR FFFFh. Some FLASH dumps in the internet do have experired Limited Starts entries (so they can be used only when changing [FFD4h] to a value other than 8000h).

The FLASH card PCBs can be fitted with 1/2/4 MByte chips (8/16/32 Mbit). As far as known, all existing FLASH cards contain 1MByte chips (so Block Allocation bit8-31 should be usually always 0). Most files are occupying the whole 1MByte (so bit0-7 should be usually all set). There are also some 256Kbyte and 512Kbyte files (where only 2 or 4 bits would be set). Minimum file size would be 128Kbyte. Odd sizes like 768Kbytes would be also possible.

Unlike the Satellite Packet Headers, the FLASH/Transmit-File header contains "normal" little-endian numbers.
