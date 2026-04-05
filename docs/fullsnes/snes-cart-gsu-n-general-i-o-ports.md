# SNES Cart GSU-n General I/O Ports

3000h-301Fh - R0-R15 - CPU Registers (R/W) 16bit CPU registers (see GSU I/O map for additional details on each register).

Writes to 3000h-301Eh (even addresses) do set LATCH=data.

Writes to 3001h-301Fh (odd addresses) do apply LSB=LATCH and MSB=data.

Writes to 301Fh (R15.MSB) do also set GO=1 (and start GSU code execution).

```text
3030h/3031h - SFR - Status/Flag Register (R) (Bit1-5: R/W)
  0  -    Always 0                                                        (R)
  1  Z    Zero Flag     (0=NotZero/NotEqual, 1=Zero/Equal)                (R/W)
  2  CY   Carry Flag    (0=Borrow/NoCarry, 1=Carry/NoBorrow)              (R/W)
  3  S    Sign Flag     (0=Positive, 1=Negative)                          (R/W)
  4  OV   Overflow Flag (0=NoOverflow, 1=Overflow)                        (R/W)
  5  GO   GSU is running (cleared on STOP) (can be forcefully=0 via 3030h)(R/W)
  6  R    ROM[R14] Read (0=No, 1=Reading ROM via R14 address)             (R)
  7  -    Always 0                                                        (R)
  8  ALT1 Prefix Flag           ;\for ALT1,ALT2,ALT3 prefixes             (R)
  9  ALT2 Prefix Flag           ;/                                        (R)
  10 IL   Immediate lower 8bit flag ;\Unknown, probably set/reset internally
  11 IH   Immediate upper 8bit flag ;/when processing opcodes with imm operands
  12 B    Prefix Flag           ;-for WITH prefix (used by MOVE/MOVES opcodes)
  13 -    Always 0                                                        (R)
  14 -    Always 0                                                        (R)
  15 IRQ  Interrupt Flag (reset on read, set on STOP) (also set if IRQ masked?)
```

This register is read/write-able even when the GSU is running; reading mainly makes sense for checking GO and IRQ bits, writing allows to clear the GO flag (thereby aborting the GSU program; the write does most likely also destroy the other SFR bits, so one cannot pause/resume).

```text
3034h - PBR - Program Bank Register (8bit, bank 00h..5Fh,70h..71h) (R/W)
3036h - ROMBR - Game Pak ROM Bank Register (8bit, bank 00h..5Fh) (R)
303Ch - RAMBR - Game Pak RAM Bank Register (1bit, bank 70h..71h) (R)
```

Memory banks for GSU opcode/data accesses. PBR can be set to both ROM and RAM regions, ROMBR/RAMBR only to ROM or RAM regions respectively. Existing cartridges have only 32Kbyte or 64Kbyte RAM, so RAMBR should be always zero.

According to book2 (page 258), the screen base is also affected by RAMBR (unknown if that is true, theoretically, SCBR is large enough to address more than 64Kbytes without RAMBR).

```text
303Eh/303Fh - CBR - Cache Base Register (upper 12bit; lower 4bit=unused) (R)
```

Code-Cache Base for Game Pak ROM/RAM. The register is read-only, so the SNES cannot directly write to it, however, the SNES can set CBR=0000h by writing GO=0 (in SFR register).

```text
3033h - BRAMR - Back-up RAM Register (W)
  0   BRAM Flag (0=Disable/Protect, 1=Enable)
  1-7 Not used (should be zero)
```

This register would be used only if the PCB does have a separate "Backup" RAM chip mapped to 780000h-79FFFFh (additionally to the Game Pak RAM chip). None of the existing PCBs is having that extra RAM chip, so the register is having no function. (Note: However, some PCBs do include a battery wired to Game Pak RAM chip, anyways, that type of "backup" isn't affected by this register).

```text
303Bh - VCR - Version Code Register (R)
  0-7 GSU Chip Version (01h..0xh ?)
```

Known versions: 1=MC1/Blob, ?=MC1/SMD, ?=GSU1, ?=GSU1A, 4=GSU2, ?=GSU2-SP1.

```text
3037h - CFGR - Config Register (W)
  0-4 -   Not used (should be zero)
  5   MS0 Multiplier Speed Select (0=Standard, 1=High Speed Mode)
  6   -   Not used (should be zero)
  7   IRQ Interrupt Mask (0=Trigger IRQ on STOP opcode, 1=Disable IRQ)
```

MS0 <must> be zero in 21MHz mode (ie. only CFGR.Bit5 or CLSR.Bit0 may be set).

MS0 is implemented in GSU2 (maybe also other chips), it is not implemented on Black Blob MC1 (which is always using slow multiply mode).

```text
3039h - CLSR - Clock Select Register (W)
  0   CLS Clock Select (0=10.7MHz, 1=21.4MHz)
  1-7 -   Not used (should be zero)
```

CLS exists on all GSU variants (including Black Blob MC1) (however, there are rumours that the fast mode was "bugged" on older MC1, unknown if that's true).

N/A - ROM Buffer - Prefetched Byte(s?) at [ROMBR:R14] N/A - Sreg/Dreg - Memorized TO/FROM Prefix Selections

```text
3100h..32FFh - Cache RAM
```
