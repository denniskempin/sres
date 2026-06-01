# SNES Cart CDROM - Decoder/FIFO

CXD1800Q chip (equivalent to CXD1196AR datasheet).

IRQs can be sensed via CXD1800 Register(01h.R).

#### 21E2h.R/W - CDROM Unit CXD1800 Index (REGADR) (R/W)

```text
  7-5  -      Reserved (should be 0)
  4-0  RA4-0  Register Index
```

This register is used for selection of the internal registers.

--> When the low order 4 bits of REGADR are not 0 (hex), and a register write

```text
     or read is made by setting A0=1 and /CS=0, the low order 4 bits of
     REGADR are incremented
```

--> REGADR is cleared to 00h by rising edge of DMAEN (in DMA Control register)

#### 21E3h.R/W - CDROM Unit CXD1800 Data (R/W)

```text
  7-0  Data for register selected via REGADR
```

#### _________________________ Configuration _________________________

#### X1h.W - DRVIF - DRIVE Interface (W)

```text
  7   XSLOW    DMA/SRAM Speed (0=Slow/12 clks/320ns, 1=Fast/4 clks/120ns)
  6   C2PL1ST  DATA input C2PO-byte-order (0=Upper first, 1=Lower first)
  5   LCHLOW   Audio LRCK Polarity for Left channel (0=High, 1=Low)
  4   BCKRED   Audio BCLK Edge for strobing DATA (0=Falling, 1=Rising)
  3-2 BCKMD1-0 Audio BCLKs per WCLK cycle (0=16, 1=24, 2/3=32)
  1   LSB1ST   Audio DATA (bit?-)ordering (0=MSB First, 1=LSB first)
  0   CLKLOW   CLK Pin Output (0=8.4672MHz, 1=Fixed Low)
```

Configures how the drive is wired up. The SNES CD doesn't touch this register and leaves it at it's power-up default. The Decoder should be disabled before changing the register.

#### X2h.W - CHPCTL - Chip Control (W)

```text
  7-5 -        Reserved (should be 0)
  4   CHPRST   Chip Reset (takes 500ns)   (0=No change, 1=Reset the chip)
  3   CD-DA    CD-Digital Audio Mode      (0=Data/CDROM, 1=Audio/CD-DA)
  2   SWOPN    Sync Detection Window      (0=Only if Sync expected, 1=Anytime)
  1   RPSTART  Repeat Correction Start  (0=No change, 1=Repeat if repeat mode)
  0   ADPEN    ADPCM Decode (to be set max 11.5ms after DECINT) (0=No, 1=Yes)
```

#### X3h.W - DECCTL - Decoder Control (W)

```text
  7   AUTOCI    ADPCM Coding Information (0=Use CI Register, 1=Disc Subheader)
  6   -         Reserved (should be 0)
  5   MODESEL   Mode Select (when AUTODIST=0)               (0=MODE1, 1=MODE2)
  4   FORMSEL   Form Select (when AUTODIST=0 and MODESEL=1) (0=FORM1, 1=FORM2)
  3   AUTODIST  Auto Distinction        (0=Use MODESEL/FORMSEL, 1=Disc Header)
  2-0 DECMD2-0  Decoder Mode            (00h-07h, see below)
```

Decoder Mode values:

```text
  00h/01h = Decoder disable (to be used for CD-DA Audio mode & during config)
  02h/03h = Monitor only    (read Header/Subheader, but don't write SRAM?)
  04h     = Write only mode (write sectors to SRAM without error correction?)
  05h     = Real time correction (abort correction if it takes too long?)
  06h     = Repeat correction (allow resume via RPSTART for important sectors?)
  07h     = Inhibit (reserved)
```

X6h.W - CI - ADPCM Coding Information (to be used when AUTOCI=0) (W)

```text
  7   -        Reserved (should be 0)
  6   EMPHASIS ADPCM Emphasis           (0=Normal/Off, 1=Emphasis)
  5   -        Reserved (should be 0)
  4   BITL4H8  ADPCM Bit Length         (0=Normal/4bit, 1=8bit)
  3   -        Reserved (should be 0)
  2   FSL3H1   ADPCM Sampling Frequency (0=37800Hz, 1=18900Hz)
  1   -        Reserved (should be 0)
  0   MONOSTE  ADPCM Mono/Stereo        (0=Mono, 1=Stereo)
```

This register is used only when AUTOCI=0, allowing to use the correct ADPCM format even in case of read errors on the CI byte in sector sub header (if AUTOCI=1, such errors would trigger CIERR interrupt and omit playback of the ADPCM sector with bad CI byte).

0Dh.W - "PLBA" - Unknown  <-- shown as so in SNES CD's "CXD1800" test screen

```text
  7-0  PLBA?    ;Maybe PLBA means "PLayBAck" or even "PLayBAckwards" or so?
```

#### _________________________ Interrupt / Status _________________________

01h.R - INTSTS - Interrupt Status (0=No IRQ, 1=IRQ) (R) X4h.W - INTMSK - Interrupt Mask (0=Disable, 1=Enable) (W) X5h.W - INTCLR - Interrupt Clear/Ack (0=No change, 1=Clear/ack) (W)

```text
  7   ADPEND  ADPCM sector decode completed, and ADPCM disabled for next sector
  6   DECTOUT Decoder Time Out (no Sync within 3 sectors)
                Can occurs (only?) after the DECODER has been set to
                monitor only mode, or real time correction mode.
  5   DMACMP  DMA Complete (by DMAXFRC=0)                       (0=No, 1=Yes)
  4   DECINT  Decoder Interrupt (new "current sector" arrived)  (0=No, 1=Yes)
                If a SYNC mark is detected or internally inserted during
                execution of the write only, monitor only and real time
                correction modes by the DECODER, the DECINT status is created.
                  When the SYNC mark detected window is open, however, if the
                SYNC mark spacing is less than 2352 bytes, the DECINT status
                is not created.
                  During execution of the repeat correction mode by the
```

DECODER,

```text
                the DECINT status is created each time a correction ends.
  3   CIERR   Coding Info Error  (0=Okay, 1=Bad CI in ADPCM sector & AUTOCI=1)
  2-0 -       Reserved (should be 0)
```

DECINT Handling (new "current sector" successfully/unsuccessfully received) First check the error flags in STS and HDRFLG registers (if desired, also check MDFM and ADPCI to see how the decoder interpreted the sector).

Then check the MM:SS:FF values in HDR_xxx registers and ignore the sector if the values aren't matching up with the desired values (that may happen if the mechacon settled on sector number slightly lower than the requested seek address, it might also happen during seek-busy phase, and it might happen if a sector was skipped for some reason, which would require to issue a new seek command and to retry reading the skipped sector).

When using ADPCM playback, also check SHDR_xxx registers to see if the sector contains ADPCM data, and if it's having the desired file/channel numbers, if so, set the ADPEN bit in CHPCTL.

Otherwise, if the sector is desired to be loaded to SNES memory: Handle the CMADR either immediately, or if that isn't possible, memorize it in a queue, and handle it as soon as possible, ie. after processing older queue entries, but before the Sector Buffer location gets overwritten by newer sectors; the 32K SRAM can probably hold at least 8 sectors (8 x 924h bytes, plus some unused padding areas, possibly plus some ADPCM area; as so on PSX).

As for handling CMADR: Usually one would only read the 800h-byte data portion (without Header and Subheader), done by writing CMDADR+4 (for MODE1) or CMDADR+0Ch (for MODE2) to DMAADRC, then writing 8800h to DMAXFRC, and then reading 800h bytes from port 21E2h (usually via a SNES DMA channel).

#### 02h.R - STS - Status (R)

```text
  7   DRQ     Data Request (DRQ Pin)                            (0=?, 1=?)
  6   ADPBSY  ADPCM Playback Busy                               (0=No, 1=Busy)
  5   ERINBLK Erasure in Block; C2 flg anywhere except Syncmark (0=Okay, 1=Bad)
  4   CORINH  Correction Inhibit; MODE/FORM error & AUTODIST=1  (0=Okay, 1=Bad)
  3   EDCOK   EDC Error Detect Checksum (optional for FORM2)    (0=Bad, 1=Okay)
  2   ECCOK   ECC Error Correction Codes (not for FORM2)        (0=Bad, 1=Okay)
  1   SHRTSCT Sync Mark too early, no ECC/EDC done              (0=Okay, 1=Bad)
  0   NOSYNC  Sync Mark too late/missing, unreal SYNC inserted  (0=Okay, 1=Bad)
```

#### 03h.R - HDRFLG - Header C2-Error Flags (R)

```text
  7  MIN     Header MM   (0=Okay, 1=Error) ;\
  6  SEC     Header SS   (0=Okay, 1=Error) ; Header from MODE1/MODE2 data
  5  BLOCK   Header FF   (0=Okay, 1=Error) ; sector (ie. not for audio)
  4  MODE    Header MODE (0=Okay, 1=Error) ;/
  3  FILE    Sub-Header  (0=Okay, 1=Error) ;\Subheader exists for MODE2 only
  2  CHANNEL Sub-Header  (0=Okay, 1=Error) ; (the SNES CD BIOS wants these
  1  SUBMODE Sub-Header  (0=Okay, 1=Error) ; bits to be zero for MODE1, too)
  0  CI      Sub-Header  (0=Okay, 1=Error) ;/
```

X4h.R - HDR_MIN - Header "MM" Minute (R) X5h.R - HDR_SEC - Header "SS" Second (R) X6h.R - HDR_BLOCK - Header "FF" Frame (R) X7h.R - HDR_MODE - Header Mode (R) 08h.R - SHDR_FILE - Sub-Header File (R) 09h.R - SHDR_CH - Sub-Header Channel (R) 0Ah.R - SHDR_S-MODE - Sub-Header SubMode (R) 0Bh.R - SHDR_CI - Sub-Header Coding Info (R) Contains current sector's 4-byte Header (and 4-byte Subheader for MODE2 discs).

0Ch/0Dh.R - CMADR_L/H - Current Minute Address, Low/High (R)

```text
  15    Unused
  14-0  Pointer to 1st byte of current sector (ie. to MM:SS:FF:MODE header)
```

Note: "Minute" is meaning the "1st byte of the sector". Named so because the 1st byte the "MM" value from the "MM:SS:FF:MODE" header. The sector stored in SRAM is 924h bytes in size (ie. the whole 930h-byte sector, excluding the 12 Sync bytes).

#### XEh.R - MDFM - MODE/FORM (R)

```text
  7-5 X        Unused
  4   RMODE2   Raw MODE byte, Bit2-7 ("logic sum") (aka all six bits ORed?)
                  Indicates the logic sum of the value of the high-order 6 bits
                  of the raw MODE byte AND THE POINTER (whut pointer?).
  3   RMODE1   Raw MODE byte, Bit1
  2   RMODE0   Raw MODE byte, Bit0
  1   CMODE    Correction Mode (0=MODE1, 1=MODE2)
  0   CFORM    Correction Form (0=FORM1, 1=FORM2) (for MODE2 only)
```

These bits indicate which of the MODEs and FORMs this IC determined that the current sector was associated with when it corrected errors.

#### XFh.R - ADPCI - ADPCM Coding Information (R)

```text
  7   MUTE     DA data is muted on      (0=No, 1=Muted)      <--- from where?
  6   EMPHASIS ADPCM Emphasis           (0=Normal/Off, 1=Emphasis)
  5   EOR      End of Record                         <--- (from SubMode.Bit0)
  4   BITLNGTH ADPCM Bit Length         (0=Normal/4bit, 1=8bit)
  3   X        Unused
  2   FS       ADPCM Sampling Frequency (0=37800Hz, 1=18900Hz)
  1   X        Unused
  0   M/S      ADPCM Mono/Stereo        (0=Mono, 1=Stereo)
```

Bit5 gets 1 when the SubMode.bit0=1 and there is no error in the SubMode byte.

#### _________________________ DMA / Sector Buffer _________________________

#### 00h.R - DMADATA - SRAM-to-CPU Xfer Data (R)

```text
  7-0    Data from Sector buffer at [DMAADRC]
```

Reading increments DMAADRC and decrements DMAXFRC. However, for this special case, REGADR is NOT incremented (allowing to read DMADATA continously without needing to reset REGADR).

X7h/X8h.W - DMAADRC_L/H - SRAM-to-CPU Xfer Address, Low/High (W) 1Ah/1Bh.R - DMAADRC_L/H - SRAM-to-CPU Xfer Address, Low/High (R)

```text
  15     Unused
  14-0   Current Read address for SRAM-to-CPU transfer (incrementing)
```

X9h/XAh.W - DMAXFRC_L/H - SRAM-to-CPU Xfer Length & DMA Control, Low/High (W) 18h/19h.R - DMAXFRC_L/H - SRAM-to-CPU Xfer Length, Low/High (R) For writing X9h/XAh (with DMAEN bit inserted between other bits):

```text
  15-12 DMAXFRC11-8 Transfer Length Remain Counter DMAXFRC, bit11-8
  11    DMAEN       CPU DMA Enable (0=Inhibit, 1=Enable)
  10-8  -           Reserved (should be 0)
  7-0   DMAXFRC7-0  Transfer Length Remain Counter DMAXFRC, bit7-0
```

For reading 18h/19h (without DMAEN bit, but instead with 15bit counter range):

```text
  15    Unused      Unused
  14-0  DMAXFRC14-0 Transfer Length Remain Counter DMAXFRC, bit14-0
```

Setting DMAEN=1 does automatically set REGADR=00h (ie. select the DMADATA register). DMAEN=1 should be used whenever starting a transfer (not matter if the data is transferred via DMA, or if it's manually polled from DMADATA register).

The DMACMP IRQ will occur when DMAXFRX reaches zero (to avoid that effect, one may write DMAXFRC=0800h (DMAEN=1 and counter=000h); that will reportedly prevent the IRQ; either because the counter doesn't decrease beyond zero, or maybe it wraps to 7FFFh and thus won't expire anytime soon).

XBh/XCh.W - DRVADRC_L/H - Disc-to-SRAM Xfer Address, Low/High (W) 1Ch/1Dh.R - DRVADRC_L/H - Disc-to-SRAM Xfer Address, Low/High (R)

```text
  15     Unused
  14-0   Disc-to-SRAM Xfer Address (incrementing)
```

This register is automatically advanced when storing incoming disc data in Sector Buffer. The SNES CD BIOS doesn't touch this register at all.

Note: The datasheet has some obscure notes about needing to write the register before "write only mode and real time correction mode" (unknown how/why/when to do that).
