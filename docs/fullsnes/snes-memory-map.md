# SNES Memory Map

The SNES uses a 24bit address bus (000000h-FFFFFFh). These 24bit addresses are often divided into 8bit bank numbers (00h-FFh) plus 16bit offset (0000h-FFFFh).

Some of these banks are broken into two 32Kbyte halves (0000h-7FFFh=System Area, 8000h-FFFFh=Cartridge ROM). Moreover, memory is divided into WS1 and WS2 areas, which can be configured to have different waitstates.

#### Overall Memory Map

```text
  Bank    Offset       Content                                      Speed
  00h-3Fh:0000h-7FFFh  System Area (8K WRAM, I/O Ports, Expansion)  see below
  00h-3Fh:8000h-FFFFh  WS1 LoROM (max 2048 Kbytes) (64x32K)         2.68MHz
     (00h:FFE0h-FFFFh) CPU Exception Vectors (Reset,Irq,Nmi,etc.)   2.68MHz
  40h-7Dh:0000h-FFFFh  WS1 HiROM (max 3968 Kbytes) (62x64K)         2.68MHz
  7Eh-7Fh:0000h-FFFFh  WRAM (Work RAM, 128 Kbytes) (2x64K)          2.68MHz
  80h-BFh:0000h-7FFFh  System Area (8K WRAM, I/O Ports, Expansion)  see below
  80h-BFh:8000h-FFFFh  WS2 LoROM (max 2048 Kbytes) (64x32K)         max 3.58MHz
  C0h-FFh:0000h-FFFFh  WS2 HiROM (max 4096 Kbytes) (64x64K)         max 3.58MHz
```

Internal memory regions are WRAM and memory mapped I/O ports.

External memory regions are LoROM, HiROM, and Expansion areas.

Additional memory (not mapped to CPU addresses) (accessible only via I/O):

```text
  OAM          (512+32 bytes) (256+16 words)
  VRAM         (64 Kbytes)    (32 Kwords)
  Palette      (512 bytes)    (256 words)
  Sound RAM    (64 Kbytes)
  Sound ROM    (64 bytes BIOS Boot ROM)
```

#### System Area (banks 00h-3Fh and 80h-BFh)

```text
  Offset       Content                                              Speed
  0000h-1FFFh  Mirror of 7E0000h-7E1FFFh (first 8Kbyte of WRAM)     2.68MHz
  2000h-20FFh  Unused                                               3.58MHz
  2100h-21FFh  I/O Ports (B-Bus)                                    3.58MHz
  2200h-3FFFh  Unused                                               3.58MHz
  4000h-41FFh  I/O Ports (manual joypad access)                     1.78MHz
  4200h-5FFFh  I/O Ports                                            3.58MHz
  6000h-7FFFh  Expansion                                            2.68MHz
```

For details on the separate I/O ports (and Expansion stuff), see:

> **See:** [SNES I/O Map](snes-i-o-map.md)

#### Cartridge ROM Capacity

The 24bit address bus allows to address 16MB, but wide parts are occupied by WRAM and I/O mirrors, which leaves only around 11.9MB for cartridge ROM in WS1/WS2 LoROM/HiROM regions (or more when also using Expansion regions and gaps in I/O area). In most cartridges, WS1 and WS2 are mirrors of each other, and most games do use only the LoROM, or only the HiROM areas, resulting in following capacities:

```text
  LoROM games --> max 2MByte ROM (banks 00h-3Fh, with mirror at 80h-BFh)
  HiROM games --> max 4MByte ROM (banks 40h-7Dh, with mirror at C0h-FFh)
```

There are several ways to overcome that limits: Some LoROM games map additional "LoROM" banks into HiROM area (BigLoROM), or into WS2 area (SpecialLoROM). Some HiROM games map additional HiROM banks into WS2 area (ExHiROM). And some cartridges do use bank switching (eg. SA-1, S-DD1, SPC7110, and X-in-1 multicarts).

32K LoROM (32K ROM banks with System Area in the same bank) ROM is broken into non-continous 32K blocks. The advantage is that one can access ROM and System Area (I/O ports and WRAM) without needing to change the CPU's current DB and PB register settings.

#### HiROM (plain 64K ROM banks)

HiROM mapping provides continous ROM addresses, but doesn't include I/O and WRAM regions "inside" of the ROM-banks.

The upper halves of the 64K-HiROM banks are usually mirrored to the corresponding 32K-LoROM banks (that is important for mapping the Interrupt and Reset Vectors from 40FFE0h-40FFFFh to 00FFE0h-00FFFFh).

#### Battery-backed SRAM

Battery-backed SRAM is used for saving game positions in many games. SRAM size is usually 2Kbyte, 8Kbyte, or 32Kbyte (or more than 32Kbyte in a few games).

There are two basic SRAM mapping schemes, one for LoROM games, and one for HiROM games:

```text
  HiROM ---> SRAM at 30h-3Fh,B0h-BFh:6000h-7FFFh    ;small 8K SRAM bank(s)
  LoROM ---> SRAM at 70h-7Dh,F0h-FFh:0000h-7FFFh    ;big 32K SRAM bank(s)
```

SRAM is usually also mirrored to both WS1 and WS2 areas. SRAM in bank 30h-3Fh is often also mirrored to 20h-2Fh (or 10h-1Fh in some cases). SRAM in bank 70h-7Dh is sometimes crippled to 70h-71h or 70h-77h, or extended to 60h-7Dh, and sometimes also mirrored to offset 8000h-FFFFh.

#### A-Bus and B-Bus

Aside from the 24bit address bus (A-Bus), the SNES is having a second 8bit address bus (B-bus), used to access certain I/O ports. Both address busses are sharing the same data bus, but each bus is having its own read and write signals.

The CPU can access the B-Bus at offset 2100h-21FFh within the System Area (ie. for CPU accesses, the B-Bus is simply a subset of the A-Bus).

The DMA controller can access both B-Bus and A-Bus at once (ie. it can output source & destination addresses simultaneously to the two busses, allowing it to "read-and-write" in a single step, instead of using separate "read-then-write" steps).

#### Bank Switching

Most SNES games are satisfied with the 24bit address space. Bank switching is used only in a few games with special chips:

```text
  S-DD1, SA-1, and SPC7110 chips (with mappable 1MByte-banks)
  Satellaview FLASH carts (can enable/disable ROM, PSRAM, FLASH)
  Nintendo Power FLASH carts (can map FLASH and SRAM to desired address)
  Pirate X-in-1 multicart mappers (mappable offset in 256Kbyte units)
  Cheat devices (and X-Band modem) can map their BIOS and can patch ROM bytes
  Copiers can map internal BIOS/DRAM/SRAM and external Cartridge memory
  Hotel Boxes (eg. SFC-Box) can map multiple games/cartridges
```

And, at the APU side, one can enable/disable the 64-byte boot ROM.
