# HD64180 Memory Mapping and Control

#### Memory Managment Unit (MMU)

The Memory Managment Unit translates "virtual" 16bit CPU memory addresses to "physical" 19bit address bus.

```text
  0000h -------> +--------------------+
                 | Common Area 0      | Phys19bit = Virt16bit + 00000h
  BA*1000h ----> +--------------------+
                 | Bank Area          | Phys19bit = Virt16bit + BBR*1000h
  CA*1000h ----> +--------------------+
                 | Common Area 1      | Phys19bit = Virt16bit + CBR*1000h
  FFFFh -------> +--------------------+
```

The 16bit CPU address space is divided into three areas (of which, the first two areas can be 0 bytes in size: BA=0 disables Common Area 0, CA=BA disables Bank Area).

38h - CBR - MMU Common Base Register (Common Area 1) (00h on Reset) 39h - BBR - MMU Bank Base Register (Bank Area) (00h on Reset)

```text
  7   Unused (should be zero) (but, used on chips with 20bit address bus)
  0-6 Base in 4K-units within "physical" 19bit 512K address space
```

3Ah - CBAR - MMU Common/Bank Area Register (F0h on Reset)

```text
  4-7 CA Start of Common Area 1 (End of Bank Area) (0Fh upon Reset)
  0-3 BA Start of Bank Area (End of Common Area 0) (00h upon Reset)
```

This is in 4K-units within the "virtual" 16bit 64K address space. Results on CA<BA are undefined.

#### 36h - RCR - Refresh Control Register (FCh on Reset)

```text
  7   REFE  DRAM Refresh Enable (0=Disable, 1=Enable)
  6   REFW  DRAM Refresh Wait (0=Two Clocks, 1=Three Clks)
  5-2 -     Unused (should be all-ones)
  1-0 CYC   DRAM Refresh Interval (0..3=10,20,40,80 states)
```

Note: The hardware outputs an 8bit Refresh address on A0..A7. A classic Z80 did output only 7bits, via using the CPU's "R" register (accessible with MOV A,R and MOV R,A opcodes). The HD64180 does still increment lower 7bit of "R" in same/similar fashion as on Z80, but, as far as I understand, without affecting any bits of the actual refresh address (and vice-versa, without the RCR-register settings affecting the way how "R" gets incremented).

#### 3Fh - ICR - I/O Control Register (1Fh on Reset)

```text
  7-6 IOA   Base Address of Internal I/O ports (0..3=0000h,0040h,0080h,00C0h)
  5   IOSTP Stop Internal ASCI, CSI/O, PRT-Timers (0=No, 1=Pause)
  4-0 -     Unused (all ones on Reset)
```

Note: There is a "z180.h" file in the internet that claims that ICR "does not move" (ie. that the "IOA" bits affect only Port 00h-3Eh, but not Port 3Fh itself). Unknow where that info comes from, and unknown if it's correct (the HD64180 and Z180 datasheets do not mention that effect).

#### Memory Address Bus Width

According to official specs, the address bus is 19bits wide (although the same specs claim that it can address up to 1Mbyte, which would require 20bits, unknown how that could work). [The 68pin and 80pin chip versions do actually have a new A19 pin, which doesn't exist on 64pin chips] Observe that A18 can be misused as Square-Wave output or as General-purpose output (see Timer chapter); when using that feature, one should normally connect only A0..A17 to memory address bus - otherwise, if A18 is wired to memory, the feature would cause the physical address to be ANDed with 3FFFFh or ORed with 40000h (this "allows" to some futher, but rather useless, bankswitching).

#### Waitstate Control

For Memory and I/O Waitstate control, see DCNTL register (in DMA chapter).

#### DMA

DMA transfers can directly access 19bit addresses, without using the MMU.

#### Unused Bits

Several internal registers contain unused bits. According to the datasheet, upon reset, these bits are set to all-ones, or all-zero (the setting varies from register to register). Unknown if it's possible and/or allowed to change these bits.

#### Reserved Registers

Registers 11h-13h, 19h-1Fh, 2Dh, 35h, 37h, 3Bh-3Eh are Reserved. Unknown if it's possible and/or allowed to read/write these registers.
