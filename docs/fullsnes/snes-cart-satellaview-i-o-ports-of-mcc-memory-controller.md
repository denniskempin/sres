# SNES Cart Satellaview I/O Ports of MCC Memory Controller

#### MCC I/O Ports

The MCC chip is a simple 16bit register, with the bits scattered across various memory banks (probably because the MCC chip doesn't have enough pins to decode lower address bits).

To change a bit: [bit_number*10000h+5000h]=bit_value*80h

```text
  005000h Unknown/Unused
  015000h Bank 00h-3Fh and 80h-FFh (0=FLASH, 1=PSRAM) (?)
  025000h Mapping for PSRAM/FLASH (0=32K/LoROM, 1=64K/HiROM)
  035000h Bank 60h-6Fh (0=FLASH, 1=PSRAM) (?)
  045000h Unknown (set when mapping PSRAM as Executable or Streaming Buffer)
  055000h Bank 40h-4Fh (0=PSRAM, 1=FLASH) ;\probably also affects Banks 00h-3Fh
  065000h Bank 50h-5Fh (0=PSRAM, 1=FLASH) ;/and maybe 80h-BFh when BIOS is off?
  075000h Bank 00h-1Fh (0=PSRAM/FLASH, 1=BIOS)
  085000h Bank 80h-9Fh (0=PSRAM/FLASH, 1=BIOS)
  095000h Unknown/Unused
  0A5000h Unknown/Unused
  0B5000h Unknown/Unused
  0C5000h Bank C0h-FFh FLASH Reads? (0=Disable, 1=Enable)
  0D5000h Bank C0h-FFh FLASH Writes (0=Disable, 1=Enable)
  0E5000h Apply Changes to Other MCC Registers (0=Unused/Reserved, 1=Apply)
  0F5000h Unknown/Unused
```

Bits C and D are R/W (the other ones maybe, too, probably except bit E) Bit 5,6 might also enable FLASH reads,writes in bank 40h-7Dh ?

#### Satellaview BIOS Cartridge Memory Map

```text
  00-0F:5000       MCC I/O Ports (Memory Control, BIOS/PSRAM/FLASH Enable)
  10-1F:5000-5FFF  SRAM             (32Kbyte SRAM in 4K-banks)
  xx-3F:6000-7FFF  PSRAM        (Mirror of 8K at PSRAM offset 06000h..07FFFh)
  00-3F:8000-FFFF  PSRAM/FLASH/BIOS in 32K-banks (Slow LoROM mapping)
  40-4F:0000-FFFF  PSRAM/FLASH  (for Executables with Slow HiROM mapping)
  50-5F:0000-FFFF  PSRAM/FLASH  (for Executables with Slow HiROM mapping)
  60-6F:0000-FFFF  FLASH/PSRAM  (for use as Work RAM or Data Files)
  70-77:0000-FFFF  PSRAM
  80-BF:8000-FFFF  PSRAM/FLASH/BIOS  in 32K-banks (Fast LoROM mapping)
  C0-FF:0000-FFFF  PSRAM/FLASH       (FLASH with R/W Access)
```

#### Memory

```text
  BIOS ROM  1MByte (LoROM mapping, 20h banks of 32Kbytes each)
  FLASH     1Mbyte   (can be mapped as LoROM, HiROM, or Work Storage)
  PSRAM     512Kbyte (can be mapped as LoROM, HiROM, or Work RAM)
  SRAM      32Kbyte (mapped in eight 4K banks)
```

Note: FLASH is on an external cartridge, size is usually 1MByte (as shown above).
