# SNES Cart CDROM - BIOS Functions

#### SNES CD BIOS Function Summary (jump opcodes at 00:E0xxh)

```text
  00h:E000h  cdrom_InitDetect          ;00h       ;\
  00h:E003h  cdrom_LoadFromDisc        ;01h       ;
  00h:E006h  cdrom_SendMechaconCommand ;02h       ; Main Functions
  00h:E009h  cdrom_WramToVramDMA       ;03h       ;
  00h:E00Ch  cdrom_PollMechacon        ;04h       ;/
  00h:E00Fh  no_function               ;05h..0Fh
  00h:E030h  cdrom_SramTest            ;10h       ;\
  00h:E033h  cdrom_SramGetDirectory    ;11h       ;
  00h:E036h  cdrom_SramSaveFile        ;12h       ; SRAM Functions
  00h:E039h  cdrom_SramLoadFile        ;13h       ;
  00h:E03Ch  cdrom_SramDeleteFile      ;14h       ;/
  00h:E03Fh  no_function               ;15h..1Fh
  00h:E060h  cdrom_DecoderDataMode     ;20h       ;\
  00h:E063h  cdrom_DecoderAudioMode    ;21h       ; Misc Functions
  00h:E066h  cdrom_DecoderTestDecint   ;22h       ;/
  00h:E069h  no_function               ;23h
  00h:Exxxh  crash                     ;24h and up
```

#### ______________________________ Main Functions ______________________________

#### 00h:E000h - cdrom_InitDetect

Initializes variables and NMI/IRQ handlers at [1Fxxh], and tries to flush any old mechacon IRQs, and to issue a mechacon get status command.

```text
  out: cy=error (0=okay, 1=no cdrom hardware)
```

#### 00h:E003h - cdrom_LoadFromDisc

```text
  in: [1F00h]=source address (24bit LBA, or 3-byte MM,SS,FF address)
  in: [1F03h]=read mode (flag byte) (usually 40h for LBA with normal data)
  in: [1F04h]=destination address (24bit wram address) (or 16bit vram address)
  in: [1F07h]=transfer length (max 7FFFh bytes, or maybe max 87FFh works, too)
  in: [1F09h]=max number of sub-q mismatches or so (usually 0Fh)
  in: [1F33h]=file and channel bytes (for ADPCM mode only)
  out: cy=error (0=okay, 1=bad)
```

Flag byte format:

```text
  7    VRAM Mode (0=Load to WRAM, 1=Forward sectors from WRAM to VRAM)
  6    Source Address format (0=MM:SS:FF in non-BCD, 1=24bit LBA)
  5    ADPCM Mode (0=No, 1=Play ADPCM file/channel until EOR/EOF)
  4    Prevent loading (0=No, 1=Skip everything except ADPCM, if enabled)
  3-0  Unused (should be 0)
```

The cdrom_LoadFromDisc function uses DMA7 to transfer data from Disc to WRAM, the "VRAM Mode" additionally uses DMA6 for forwarding incoming data from a WRAM buffer (at 83h:C000h-FFFFh) to VRAM.

#### 00h:E006h - cdrom_SendMechaconCommand

Allows to send mechacon commands (normally not required, the LoadFromDisc functions does automatically issue seek+play+pause commands).

```text
  in: a=command (8bit)
  in: [1Fxxh]=optional parameters (for command 00h and 01h)
  out: cy=error (0=okay, 1=bad)
  out: [1F2E]=last response digit (unknown purpose, checked after seek_mmssff)
```

Command numbers are:

```text
  00h  seek_tr_indx   CxxxxF --> FFFFFx       ;in: [1F0Fh..1F12h]=four nibbles
  01h  seek_mmssff    BxxxxxxF --> FFFFFFFx   ;in: [1F13h..1F18h]=six nibbles
  02h  stop           D01F --> FFFx
  03h  play           D02F --> FFFx
  04h  pause          D03F --> FFFx
  05h  open_close     D04F --> FFFx
  06h  fast_forward   D10F --> FFFx
  07h  fast_reverse   D11F --> FFFx
  08h  forward        D12F --> FFFx
  09h  reverse        D13F --> FFFx
  0Ah  key_direct     D40F --> FFFx
  0Bh  key_ignore     D41F --> FFFx
  0Ch  continous      D42F --> FFFx
  0Dh  track_pause    D43F --> FFFx
  0Eh  index_pause    D44F --> FFFx
  0Fh  req_sub_q      D50F_0000000000000000F  ;out:[1F1Eh..1F2Dh]=16 nibbles
  10h  req_status     D51F_01234F             ;out:[1F19h..1F1Dh]=5 nibbles
  11h  normal_speed   D45F --> FFFx
  12h  double_speed   D46F --> FFFx
  13h  flush          F --> a
  N/A  ?              D14F --> FFFx
  N/A  ?              D15F --> FFFx
```

00h:E009h - cdrom_WramToVramDMA (custom NMI handler callback) Usually done automatically by the default BIOS NMI handler: If CDROM loading is done in "VRAM mode", then this functions forwards the incoming CDROM data from WRAM to VRAM.

00h:E00Ch - cdrom_PollMechacon (custom IRQ handler callback) Usually done automatically by the default BIOS IRQ handler: If a mechacon command is being transmitted, then this function handles incoming mechacon response nibbles, and sends further mechacon parameter nibbles (until completion of the command sequence).

#### ______________________________ SRAM Functions ______________________________

#### 00h:E030h - cdrom_SramTest

Tests the SRAM checksum, does range checks on free memory size and number of files, automatically reformats/erases the SRAM in case of errors.

```text
  out: cy=error (0=okay, 1=bad, reformatted sram)
```

#### 00h:E033h - cdrom_SramGetDirectory

Returns the whole SRAM directory with max 32 files, each 16-byte entry consists of 14-byte filename, folled by 16bit filesize value.

```text
  in: DB:Y = destination address (200h byte buffer)
  out: cy=error (0=okay, 1=bad)
  out: a=number of files actually used                      ;\returned only
  out: [DB:Y+0..1FF]=directory (unused entries 00h-filled)  ;/when cy=0=okay
```

#### 00h:E036h - cdrom_SramSaveFile

```text
  in: DB:Y = source address (14-byte name, 16bit length, filebody[length])
  out: cy=error (0=okay, 1=bad, directory or memory full)
```

#### 00h:E039h - cdrom_SramLoadFile

```text
  in: DB:Y = source address (14-byte name, 16bit length, filebody[length])
  out: filebody[length] is overwritten by loaded file
       (zeropadded if the specified length exceeded the specified filesize)
  out: cy=error (0=okay, 1=bad, file not found)
```

#### 00h:E03Ch - cdrom_SramDeleteFile

```text
  in: DB:Y = source address (14-byte name)
  out: cy=error (0=okay, 1=bad, file not found)
```

Character Set for SRAM Filenames (shown when pressing SELECT in BIOS)

```text
  00h..09h  "0..9"
  0Ah..23h  "A..Z"
  24h..27h  Space, Slash, Dash, Dot
  28h..7Fh  Japanese symbols
  80h..FFh  Cause directory sort-order corruption when creating/deleting files
```

#### ______________________________ Misc Functions ______________________________

00h:E060h - cdrom_DecoderDataMode 00h:E063h - cdrom_DecoderAudioMode (CD-DA) These functions are just setting the decoder to data/audio mode, there are no parameters or return values.

#### 00h:E066h - cdrom_DecoderTestDecint

Runs a test on measuring the number of DECINT's per second (aka sectors per second), passes okay when measuring 75+/-5 or 150+/-10 DECINTs (ie. both single & double speed mode should pass). Execution time of the test is 1 second.

```text
  out: cy=error (0=okay, 1=bad)
```

#### ______________________________ Bugs & Glitches _____________________________

Instead of using unsigned maths, the BIOS used a lot of signed comparisions without overflow checking.

This is restricting the CDROM filesize to max 7FFFh (or possibly 87FFh might work when subtracting the first sector unit).

SRAM filename characters are also using that signed maths for the filename sort order (using characters 80h..FFh can have unpredictable results when adding/removing SRAM files; which may cause new comparision overflows to occur/disappear).

SRAM is intended to hold max 32 files, however, that limit is checked when overwriting old files (not when creating new files): Results are that one cannot overwrite any files if the cart contains 32 files or more, whilst, on the other hand, one could create even more then 32 files.

Booting the BIOS seems to be instantly STOPPING the drive motor (after the BIOS intro/delay), apparently preventing the drive to spin-up, and to read the TOC, or even to load data from the disc - until going through the "PRESS START" nag screen.
