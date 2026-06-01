# SNES Cart GSU-n Memory Map

#### MC1 Memory Map (at SNES Side)

```text
  00-3F/80-BF:3000-347F  GSU I/O Ports
  00-1F/80-9F:8000-FFFF  Game Pak ROM in LoRom mapping (1Mbyte max)
  60-7D/E0-FF:0000-FFFF  Game Pak RAM with mirrors (64Kbyte max?, usually 32K)
  Other Addresses        Open Bus
```

#### GSU1 Memory Map (at SNES Side)

```text
  00-3F/80-BF:3000-34FF? GSU I/O Ports
  00-3F/80-BF:6000-7FFF  Mirror of 70:0000-1FFF (ie. FIRST 8K of Game Pak RAM)
  00-3F/80-BF:8000-FFFF  Game Pak ROM in LoRom mapping (1Mbyte max?)
  40-5F/C0-DF:0000-FFFF  Game Pak ROM in HiRom mapping (mirror of above)
  70-71/F0-F1:0000-FFFF  Game Pak RAM with mirrors (64Kbyte max?, usually 32K)
  78-7x/F8-Fx:0000-FFFF  Unknown (maybe Additional "Backup" RAM like GSU2)
  Other Addresses        Open Bus
```

#### GSU2 Memory Map (at SNES Side)

```text
  00-3F/80-BF:3000-34FF  GSU I/O Ports
  00-3F/80-BF:6000-7FFF  Mirror of 70:0000-1FFF (ie. FIRST 8K of Game Pak RAM)
  00-3F:8000-FFFF        Game Pak ROM in LoRom mapping (2Mbyte max)
  40-5F:0000-FFFF        Game Pak ROM in HiRom mapping (mirror of above)
  70-71:0000-FFFF        Game Pak RAM       (128Kbyte max, usually 32K or 64K)
  78-79:0000-FFFF        Additional "Backup" RAM  (128Kbyte max, usually none)
  80-BF:8000-FFFF        Additional "CPU" ROM LoROM (2Mbyte max, usually none)
  C0-FF:0000-FFFF        Additional "CPU" ROM HiROM (4Mbyte max, usually none)
  Other Addresses        Open Bus
```

For HiROM mapping the address bits are shifted, so both LoROM and HiROM are linear (eg. Bank 40h contains mirrors of Bank 00h and 01h).

Although both LoROM and HiROM are supported, the header & exception vectors are located at ROM Offset 7Fxxh (in LoROM fashion), accordingly the cartridge header declares the cartridge as LoROM.

The additional ROM/RAM regions would be mapped to SNES CPU only (not to GSU), they aren't installed in existing cartridges, that implies that the "Fast" ROM banks (80h-FFh) are unused, so GSU games are restricted to "Slow" ROM.

#### GSU2 Memory Map (at GSU Side)

```text
  00-3F:0000-7FFF  Mirror of LoROM at 00-3F:8000-FFFF (for "GETB R15" vectors)
  00-3F:8000-FFFF  Game Pak ROM in LoRom mapping (2Mbyte max)
  40-5F:0000-FFFF  Game Pak ROM in HiRom mapping (mirror of above 2Mbyte)
  70-71:0000-FFFF  Game Pak RAM       (128Kbyte max, usually 32K or 64K)
  PBR:0000-01FF    Code-Cache (when having manually stored opcodes in it)
```

PBR can be set to both ROM/RAM regions (or cache region), ROMBR only to ROM region (00h-5Fh), RAMBR only to RAM region (70h-71h).

#### GSU Interrupt Vectors

The SNES Exception Vectors (at FFE4h-FFFFh) are normally located in Game Pak ROM. When the GSU is running (with GO=1 and RON=1), ROM isn't mapped to SNES memory, instead, fixed values are appearing as ROM (depending of the lower 4bit of the address):

```text
  Any Address     Exception Vectors
  [xxx0h]=0100h   -
  [xxx2h]=0100h   -
  [xxx4h]=0104h   [FFE4h]=0104h  COP Vector in 65C816 mode (COP opcode)
  [xxx6h]=0100h   [FFE6h]=0100h  BRK Vector in 65C816 mode (BRK opcode)
  [xxx8h]=0100h   [FFE8h]=0100h  ABT Vector in 65C816 mode (Not used in SNES)
  [xxxAh]=0108h   [FFEAh]=0108h  NMI Vector in 65C816 mode (Vblank)
  [xxxCh]=0100h   -
  [xxxEh]=010Ch   [FFEEh]=010Ch  IRQ Vector in 65C816 mode (H/V-IRQ & GSU-STOP)
```

It'd be best to set the Game Pak ROM vectors to the same addresses, otherwise the vectors would change when the GSU is running (or possibly, the fixed-LSBs may be mixed-up with ROM-MSBs).

GSU Cartridge Header (always at ROM Offset 7Fxxh, in LoROM fashion)

```text
  [FFD5h]=20h        Set to "Slow/LoROM" (although both LoROM/HiROM works)
  [FFD6h]=13h..1Ah   Chipset = GSUn (plus battery present/absent info)
  [FFD8h]=00h        Normal SRAM Size (None) (always use the Expansion entry)
  [FFBDh]=05h..06h   Expansion RAM Size (32Kbyte and 64Kbyte exist)
  Caution: Starfox/Star Wing, Powerslide, and Starfox 2 do not have extended
  headers (and thereby no [FFBDh] entry). RAM Size for Starfox/Starwing is
  32Kbytes, RAM Size for Powerslide and Starfox 2 is unknown.
```

There is no info in the header (nor extended header) whether the game uses a GSU1 or GSU2. Games with 2MByte ROM are typically using GSU2 (though that rule doesn't always match: Star Fox 2 is only 1MByte).

#### GSU Busses

The GSU seems to have 4 address/data busses (three external ones, and one internal cache bus):

```text
  SNES bus (for forwarding ROM/RAM access to SNES)
  ROM bus (for GSU opcode fetches, GETxx reads, and SNES reads)
  RAM bus (for GSU opcode fetches, LOAD/STORE/PLOT/RPIX, and SNES access)
  Code cache bus (for GSU opcode fetches only) (and SNES I/O via 3100h..32FFh)
```

To some level, this allows to do multiple things simultaneously: Reading a GSU opcode from cache at the same time while prefetching ROM data and forwarding the RAM or Pixel cache to RAM.
