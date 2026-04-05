# SNES Cart CDROM - BIOS Cartridge

Contains extra DRAM, some small battery-backed SRAM, and the BIOS ROM. The DRAM and SRAM are rather small, and there's no coprocessor. However, this is only prototype, and Nintendo could have easly expanded the BIOS cartridge (without needing to modify the actual CDROM hardware).

For example, there have been rumours about a 32bit CPU being planned, and SRAM might have been intended to be replaced by a bigger memory chip (or possibly by an external FLASH cart as used in Satellaview BIOS carts).

#### BIOS User Interface

```text
  START  --> Load CDROM (if any)
  SELECT --> SRAM Manager (in there: Up/Down=Select, B=Delete, Y=Exit)
  A+X    --> Test Screen (in there: Up/Down/B --> Menu Selection)
```

Self Check tests:

```text
  Page1: VRAM, CGRAM, OAM, WRAM, DMA, TIMER, SOUND (sound test works only once)
  Page2: BIOS_DRAM, BIOS_SRAM, CDROM DECODER, CD-PLAYER I/F
  The DECODER test seems to try to count sectors/second on STOPPED drive,
  that might fail on real HW, or it might work with the NOSYNC bit triggered?
```

ADPCM Test:

```text
  Use Up/Down and L/R Buttons to select File/Channel and MM:SS:FF
  Press B to play ADPCM audio (eg. from PSX disc with ADPCM at selected values)
  Press Y to toggle Normal/Double speed, press Select to go back to menu
  Observe that APU is muting sound output (unless previously running Selfcheck)
```

Communication (Mechacon) Test:

```text
  Use L/R Buttons to select a command, use B to issue the command, Select=Exit
  Use Up/Down and L/R Buttons to change variable parameters
```

CXD-1800 (Decoder) Test:

```text
  Use Up/Down and L/R Buttons to change Write values
  Use Y to toggle Read/Write, X to toggle IRQ, Select=Exit
```

00h-03h:8000h-FFFFh - BIOS Cart ROM (128Kbyte LoROM) (Sticker 0.95 SX) The BIOS has CRC32=3B64A370h and the ROM/EPROM is badged "0.95 SX", there are some ASCII strings in the file:

```text
  "Super Disc boot ROM ver.0.95 Jul. 14, 1992 by Tomomi Abe at SONY "
  "Super Disc BIOS program ver.0.93 by Tomomi Abe. May. 26 1992 at SONY. "
  01h,"CD001",01h,00h,"SUPERDISC",23x00h  ;28h-byte ISO volume descriptor
```

The cart header at 7FC0h-7FDFh is just FFh-filled and IRQ/NMI vectors point to RAM:

```text
  7FC0  FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF
  7FD0  FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF,FF
  7FE0  00,00,00,00,00,00,00,00,00,00,F8,1F,00,00,FC,1F
  7FF0  00,00,00,00,00,00,00,00,00,00,00,00,00,80,00,00
```

That uncommon combination of FFh's and IRQ/NMI vectors can be used to detect if a ROM image is having Super Disc support.

80h-87h:8000h-FFFFh - BIOS Cart Work RAM (256Kbyte DRAM) (two S-WRAM chips) This expands the SNES's internal 128KBytes to a total of 384Kbytes Work RAM.

Allowing to load code and data from CDROM to CPU memory space.

90h:8000h-9FFFh - BIOS Cart Battery RAM (8Kbyte SRAM) 21D0h.W - BIOS Cartridge Battery RAM Lock (write 00h) 21E0h.W - BIOS Cartridge Battery RAM Unlock Step 2 (write 0Fh downto 01h) 21E5h.W - BIOS Cartridge Battery RAM Unlock Step 1 (write FFh) These ports seem to be used to write-protect the battery backed SRAM (the BIOS functions are automatically locking/unlocking the SRAM when saving/deleting game position files).
