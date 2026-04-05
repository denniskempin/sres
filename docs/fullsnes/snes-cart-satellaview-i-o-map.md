# SNES Cart Satellaview I/O Map

#### Receiver I/O Map (DCD-BSA chip)

```text
  2188h Stream 1 Hardware Channel Number, Lsb (R/W)
  2189h Stream 1 Hardware Channel Number, Msb (R/W)
  218Ah Stream 1 Queue Size (number of received 1+22 byte Units) (R)
  218Bh Stream 1 Queue 1-byte Status Units (Read=Data, Write=Reset)
  218Ch Stream 1 Queue 22-byte Data Units  (Read=Data, Write=Reset/Ack)
  218Dh Stream 1 Status Summary (R)
  218Eh Stream 2 Hardware Channel Number, Lsb (R/W)
  218Fh Stream 2 Hardware Channel Number, Msb (R/W)
  2190h Stream 2 Queue Size (number of received 1+22 byte Units) (R)
  2191h Stream 2 Queue 1-byte Status Unit(s?) (Read=Data, Write=Reset)
  2192h Stream 2 Queue 22-byte? Data Unit(s?) (Read=Data, Write=Reset/Ack)
  2193h Stream 2 Status Summary (R)
  2194h POWER (bit0) and ACCESS (bit2-3) LED Control? (R/W)
  2195h Unknown/Unused, maybe for EXT Expansion Port (?)
  2196h Status (only bit1 is tested) (R)
  2197h Control (only bit7 is modified) (R/W)
  2198h Serial I/O Port 1 (R/W)
  2199h Serial I/O Port 2 (R/W)
```

Flash Card I/O Map (when mapped to bank C0h and up)

```text
  C00000h  Type 1-4   Detection Command             (W)
  C00002h  Type 1-4   Detection Status              (R)
  C0FFxxh  Type 1-4   Detection Response            (R)
  C00000h  Type 1,3,4 Command for Type 1,3,4        (W)
  C00000h  Type 1,3,4 Status (normal commands)      (R)
  C00004h  Type 1,3   Status (erase-entire command) (R)
  C02AAAh  Type 2     Command/Key for Type2         (W)
  C05555h  Type 2     Command/Status for Type2      (R/W)
  xx0000h  Type 1-4   Erase 64K Sector Address      (W)
  xxxxxxh  Type 1-4   Write Data Address            (W)
```

#### BIOS Cartridge MCC-BSC Chip Ports

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
  095000h Unknown/Unused (except: used by BS Dragon Quest, set to 00h)
  0A5000h Unknown/Unused (except: used by BS Dragon Quest, set to 80h)
  0B5000h Unknown/Unused (except: used by BS Dragon Quest, set to 80h)
  0C5000h Bank C0h-FFh FLASH Reads? (0=Disable, 1=Enable)
  0D5000h Bank C0h-FFh FLASH Writes (0=Disable, 1=Enable)
  0E5000h Apply Changes to Other MCC Registers (0=Unused/Reserved, 1=Apply)
  0F5000h Unknown/Unused
```

Bits C and D are R/W (the other ones maybe, too).
