# SNES Cart SA-1 (programmable 65C816 CPU) (aka Super Accelerator) (35 games)

> **See:** [SNES Cart SA-1 Games](snes-cart-sa-1-games.md)
> **See:** [SNES Cart SA-1 I/O Map](snes-cart-sa-1-i-o-map.md)

> **See:** [SNES Cart SA-1 Interrupt/Control on SNES Side](snes-cart-sa-1-interrupt-control-on-snes-side.md)
> **See:** [SNES Cart SA-1 Interrupt/Control on SA-1 Side](snes-cart-sa-1-interrupt-control-on-sa-1-side.md)
> **See:** [SNES Cart SA-1 Timer](snes-cart-sa-1-timer.md)
> **See:** [SNES Cart SA-1 Memory Control](snes-cart-sa-1-memory-control.md)
> **See:** [SNES Cart SA-1 DMA Transfers](snes-cart-sa-1-dma-transfers.md)
> **See:** [SNES Cart SA-1 Character Conversion](snes-cart-sa-1-character-conversion.md)
> **See:** [SNES Cart SA-1 Arithmetic Maths](snes-cart-sa-1-arithmetic-maths.md)
> **See:** [SNES Cart SA-1 Variable-Length Bit Processing](snes-cart-sa-1-variable-length-bit-processing.md)

> **See:** [SNES Pinouts SA1 Chip](snes-pinouts-sa1-chip.md)

#### Memory Map (SNES Side)

```text
  00h-3Fh/80h-BFh:2200h-23FFh  I/O Ports
  00h-3Fh/80h-BFh:3000h-37FFh  I-RAM (2Kbytes, on-chip, 10MHz fast RAM)
  00h-3Fh/80h-BFh:6000h-7FFFh  One mappable 8Kbyte BW-RAM block
  00h-3Fh/80h-BFh:8000h-FFFFh  Four mappable 1MByte LoROM blocks (max 8Mbyte)
  40h-4Fh:0000h-FFFFh          Entire 256Kbyte BW-RAM (mirrors in 44h-4Fh)
  C0h-FFh:0000h-FFFFh          Four mappable 1MByte HiROM blocks (max 8Mbyte)
```

The SA-1 supports both LoROM and HiROM mappings (eg. LoROM banks 00h-01h mirror to HiROM bank 40h). Default exception vectors (and cartridge header) are always in LoROM bank 00h (ie. at ROM offset 7Fxxh).

#### Memory Map (SA-1 Side)

Same as on SNES Side (of course without access to SNES internal WRAM and I/O ports), plus following additional areas:

```text
  00h-3Fh/80h-BFh:0000h-07FFh  I-RAM (at both 0000h-07FFh and 3000h-37FFh)
  60h-6Fh:0000h-FFFFh          BW-RAM mapped as 2bit or 4bit pixel buffer
```

Some other differences to SNES Side are: I/O Ports are different, on SA-1 side, the mappable BW-RAM area (at 6000h-7FFFh) can be also assigned as 2bit/4bit pixel buffer (on SNES Side it's always normal 8bit memory).

#### Misc 65C816 CPU at 10.74MHz

```text
  2Kbytes internal I-RAM (work ram/stack) (optionally battery backed)
  Optional external backup/work BW-RAM up to 2MByte (or rather only 2Mbit?)
  Addressable ROM up to 8MByte (64MBits)
```

The SA-1 CPU can access memory at 10.74MHz rate (or less, if the SNES does simultaneouly access cartridge memory).

The SNES CPU can access memory at 2.68MHz rate (or 3.5MHz, but that mode may not be used in combination with the SA-1).

When interrupts are disabled (in CIE/SIE), then it sounds as if the interrupt flags still do get set?

#### "BW-RAM cannot be used during character conversion DMA."

IRQ/NMI/Reset vectors can be mapped. Other vectors (BRK/COP etc) are always taken from ROM (for BOTH CPUs).

```text
    XXX pg 62..66 timings
```

#### ok XXX pg 67..78 char/bitmap  ok XXX pg 79..81 arit

```text
    XXX pg 82..86 var-len
```

#### ok XXX pg 87..90 dma

#### SA-1 Pinouts

```text
  1-126  Unknown
  127    PAL/NTSC (for CIC mode and/or HV-timer?)
  128    Unknown
```

#### SA-1 PCBs

```text
  BSC-1L3B-01    NTSC SRAM Battery FLASH-Slot (Itoi Shig. no Bass Tsuri No.1)
  SHVC-1L0N3S-20 NTSC SRAM NoBattery (Dragon Ball Z Hyper Dimension)
  SHVC-1L3B-11   NTSC SRAM Battery
  SHVC-1L5B-10   NTSC SRAM Battery
  SHVC-1L5B-11   NTSC SRAM Battery
  SHVC-1L8B-10   NTSC SRAM Battery
  SNSP-1L0N3S-01 PAL  SRAM NoBattery (Dragon Ball Z Hyper Dimension)
  SNSP-1L3B-20   PAL  SRAM Battery
```

The battery can be wired to I-RAM (on-chip SA-1 memory) or BW-RAM (aka SRAM) or both; unknown how it is wired in practice (probably to BW-RAM?).

#### Chipset/Components

```text
  U1  44pin  ROM (probably with full 16bit databus connected)
  U2  28pin  SRAM (LH52A64N-YL or LH52256ANZ or 32pin LH52A512NF)
  U3  128pin SA1 (SA1 RF5A123)
  U4  8pin   Battery controller MM1026AF  ;\only if PCB does include a battery
  BATT 2pin  CR2032                       ;/
  CN1 62pin  SNES cartridge edge-connector
  CN2 62pin  Satellaview FLASH cartridge slot  ;-only on BSC-boards
```
