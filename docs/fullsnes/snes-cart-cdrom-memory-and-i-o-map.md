# SNES Cart CDROM - Memory and I/O Map

#### I/O Ports

```text
  21D0h.W   - BIOS Cartridge Battery RAM Lock (write 00h)
  21E0h.W   - BIOS Cartridge Battery RAM Unlock Step 2 (write 0Fh downto 01h)
  21E1h.R/W - CDROM Unit Mechacon CPU (probably the NEC chip on daughterboard)
  21E2h.R/W - CDROM Unit Decoder/FIFO Index (CXD1800Q chip)
  21E3h.R/W - CDROM Unit Decoder/FIFO Data  (CXD1800Q chip)
  21E4h.W   - CDROM Unit (?) Whatever Control/Enable or so
  21E5h.W   - BIOS Cartridge Battery RAM Unlock Step 1 (write FFh)
  ???.R/W   - NEXT connector? (maybe some kind of UART, like PSX serial port?)
  ???.R/W   - BIOS Cartridge S-WRAM chip(s) (seem be wired to /PARD and /PAWR)
  IRQ       - used for Decoder and Mechacon
```

#### APU I/O Ports

The SNES CD prototype has APU chips with uncommon part numbers, which might work slightly different than standard SNES APUs. However, adding that chips wouldn't be possible with SNES CD expansions (for existing SNES consoles).

Either old SNES consoles would need to stick with old APUs, or, theoretically, the SNES CD expansions could contain an extra APU unit (but, mapped elsewhere than 2140h-2143h).

#### Memory

```text
  00h-03h:8000h-FFFFh  BIOS Cart ROM (128Kbyte LoROM)
  80h-87h:8000h-FFFFh  BIOS Cart Work RAM (256Kbyte DRAM) (two S-WRAM chips)
  90h    :8000h-9FFFh  BIOS Cart Battery RAM (8Kbyte SRAM)
```

Special Memory regions/addresses:

```text
  00h:1Fxxh  Work RAM reserved for BIOS functions
  00h:1FF8h  Work RAM containing NMI vector (should be 4-byte "JMP far" opcode)
  00h:1FFCh  Work RAM containing IRQ vector (should be 4-byte "JMP far" opcode)
  00h:0000h  Work RAM containing IRQ/BRK/COP vectors (if used)
  00h:1000h  Load address for 800h-byte boot sector
  00h:1080h  Entrypoint for 800h-byte boot sector
  00h:E000h  CD BIOS Functions in BIOS ROM
  83h:C000h  Work RAM reserved for loading cdrom data in "VRAM mode" (16Kbyte)
```

Caution: Initial/empty SRAM may NOT be zerofilled (else the BIOS treats the checksum to be okay, with 0 files installed - but with 0000h bytes free space, which is making it impossible to create/delete any files).

Caution: RAM at 1Fxxh is reserved for BIOS functions (and NMI/IRQ vectors, even when not using any other BIOS functions), so stacktop should be 1EFFh (not

```text
1FFFh, where it'd be usually located).
```

Unknown if the memory is mirrored anywhere; particulary mirroring the S-WRAMs to C0h-C3h:0000h-FFFFh would be useful for HiROM-style games.

Unknown if the two S-WRAM chips are also mapped to B-bus (the B-bus would be useful only for DMA from ROM carts, ie. not useful for CDROM games).

#### 21E4h.W - Whatever Control/Enable or so

```text
  7-4   Unknown/Unused (always set to 0)
  3     Enable Mechacon?      (0=Off, 1=On)
  2     Enable Decoder?       (0=Off, 1=On)
  1     Maybe Reset?          (0=Normal, 1=What?)
  0     Unknown/Unused (always set to 0)
```

Set to 0Eh,00h,04h,08h,0Ch.

#### Decoder/FIFO Registers (CXD1800Q) (accessed via 21E2h/21E3h)  Decoder Write Registers

```text
  00h     -           Reserved
  01h     DRVIF       DRIVE Interface (W)
  02h     CHPCTL      Chip Control (W)
  03h     DECCTL      Decoder Control (W)
  04h     INTMSK      Interrupt Mask (0=Disable, 1=Enable) (W)      ;\interrupt
  05h     INTCLR      Interrupt Clear/Ack (0=No change, 1=Clear/ack);/
  06h     CI          ADPCM Coding Information (to be used when AUTOCI=0)
  07h     DMAADRC_L   SRAM-to-CPU Xfer Address, Low (W)               ;\
  08h     DMAADRC_H   SRAM-to-CPU Xfer Address, High (W)              ;
  09h     DMAXFRC_L   SRAM-to-CPU Xfer Length, Low (W)                ;
  0Ah     DMAXFRC_H   SRAM-to-CPU Xfer Length, High & DMA Control (W) ;/
  0Bh     DRVADRC_L   Disc-to-SRAM Xfer Address, Low (W)              ;\
  0Ch     DRVADRC_H   Disc-to-SRAM Xfer Address, High (W)             ;/
  0Dh-0Fh -           Unspecified
  0Dh     "PLBA"      <-- shown as so in SNES CD's "CXD1800" test screen
  10h-1Ch -           Mirrors of 00h-0Ch
  1Dh     -           Reserved (TEST2)
  1Eh     -           Reserved (TEST1)
  1Fh     -           Reserved (TEST0)
```

#### Decoder Read Registers

```text
  00h     DMADATA     SRAM-to-CPU Xfer Data (R)             ;-Sector Data
  01h     INTSTS      Interrupt Status (0=No IRQ, 1=IRQ) (R);-Interrupt
  02h     STS         Status (R)                            ;\
  03h     HDRFLG      Header Flags (R)                      ;
  04h     HDR_MIN     Header "MM" Minute (R)                ; important info on
  05h     HDR_SEC     Header "SS" Second (R)                ; current sector
  06h     HDR_BLOCK   Header "FF" Frame (R)                 ; (to be handled
  07h     HDR_MODE    Header Mode (R)                       ; upon "DECINT"
  08h     SHDR_FILE   Sub-Header File (R)                   ; interrupt)
  09h     SHDR_CH     Sub-Header Channel (R)                ;
  0Ah     SHDR_S-MODE Sub-Header SubMode (R)                ;
  0Bh     SHDR_CI     Sub-Header Coding Info (R)            ;
  0Ch     CMADR_L     Current Minute Address, Low (R)       ;
  0Dh     CMADR_H     Current Minute Address, High (R)      ;/
  0Eh     MDFM        MODE/FORM (R)                         ;\extra details on
  0Fh     ADPCI       ADPCM Coding Information (R)          ;/current sector
  10h-to-2            Reserved (TEST 0 to 2) (R)
  13h     -           Unspecified
  14h-17h -           Mirrors of 04h-07h (HDR_xxx)
  18h.R   DMAXFRC_L - SRAM-to-CPU Xfer Length, Low (R)      ;\allows to read
  19h.R   DMAXFRC_H - SRAM-to-CPU Xfer Length, High (R)     ; address/remain
  1Ah.R   DMAADRC_L - SRAM-to-CPU Xfer Address, Low (R)     ; values
  1Bh.R   DMAADRC_H - SRAM-to-CPU Xfer Address, High (R)    ; (needed only for
  1Ch.R   DRVADRC_L - Disc-to-SRAM Xfer Address, Low (R)    ; diagnostics)
  1Dh.R   DRVADRC_H - Disc-to-SRAM Xfer Address, High (R)   ;/
  1Eh-1Fh -           Mirrors of 0Eh-0Fh (MDFM and ADPCI)
```
