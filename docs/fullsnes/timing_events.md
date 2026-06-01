# SNES Timing H/V Events

#### Summary of Vertical Timings

```text
  V=0              End of Vblank, toggle Field, prefetch OBJs of 1st scanline
  V=0..224/239     Perform HDMA transfers (before each line & after last line)
  V=1              Begin of Drawing Period
  V=225/240        Begin of Vblank Period (NMI, joypad read, reload OAMADD)
  V=240            Short scanline in Non-interlaced 60Hz field=1
  V=311            Long scanline in Interlaced 50Hz field=1
  V=261/311        Last Line (in normal frames)
  V=262/312        Extra scanline (occurs only in Interlace Field=0)
  V=VTIME          Trigger V-IRQ or HV-IRQ
```

Detailed List (H=Horizontal, V=Vertical, F=Field)

```text
  H=0, V=0, F=0         SNES starts at this time after /RESET
  H=0, V=0              clear Vblank flag, and reset NMI flag (auto ack)
  H=0, V=225            set Vblank flag
  H=0.5, V=225          set NMI flag
  H=1                   clear hblank flag
  H=1, V=0              toggle interlace FIELD flag
  H=HTIME+3.5           H-IRQ
  H=2.5, V=VTIME        V-IRQ   (or HV-IRQ with HTIME=0)
  H=HTIME+3.5, V=VTIME  HV-IRQ  (when HTIME=1..339)
  H=6, V=0              reload HDMA registers
  H=10, V=225           reload OAMADD
  H=22-277(?), V=1-224  draw picture
  H=32.5..95.5, V=225   around here, joypad read begins (duration 4224 clks)
  H=133.5               around here, REFRESH begins (duration 40 clks/10 dots)
  H=274                 set hblank flag
  H=278, V=0..224       perform HDMA transfers
  H=323,327             seen as long-PPU-dots (but not as long-CPU-dots)
  H=323,327, V=240, F=1 seen as normal-PPU-dots (in short scanline 240) (60Hz)
  H=339                 this is last PPU-dot (in normal and short scanlines)
  H=340, V=311, F=1     this is last PPU-dot in long scanlines (50Hz+Interlace)
  CPU.H=339             this is last CPU-dot (in normal scanlines)
  CPU.H=338, V=240, F=1 this is last CPU-dot (in short scanlines)
  CPU.H=340, V=311, F=1 this is last CPU-dot (in long scanlines)
  H=0?, V=0             reset OBJ overflow flags in 213Eh (only if not f-blank)
  H=0?+INDEX*2, V=YLOC  set OBJ overflow bit6 (too many OBJs in next line)
  H=0?, V=YLOC+1        set OBJ overflow bit7 (too many OBJ pix in this line)
  ...?
```

xxx joypad read xxx reload mosaic h/v counter (at some point during vblank) xxx count mosaic v counter (at some point in each scanline)

Note that a PAL TV-set can display around 264 lines (about 25 more than supported by the 239-line mode).

Note that a superscope pointing at pixel (X, Y) on the screen will latch approximately dot X+40 on scanline Y+1.

#### PPU H-Counter-Latch Quantities

When latching PPU H/V-latches via reading [2137h] by software, 341*4 times at evenly spread locations, one will statistically get following H values:

```text
  0..132    4 times (normal)
  133       3 times (occurs sometimes at H=133.5, and always at H=133.0)
  134       1 time  (occurs sometimes at H=134.x)
  135..142  never   (refresh is busy, cpu is stopped)
  143       1 time  (occurs sometimes at H=143.x)
  144       3 times (occurs sometimes at H=144.0, and always at H=144.5)
  145..322  4 times (normal)
  323       6 times (seen as long dot) (or 5.99 times if NTSC+InterlaceOff)
  324..326  4 times (normal)
  327       6 times (seen as long dot) (or 5.99 times if NTSC+InterlaceOff)
  328..339  4 times (normal)
  340       never   (doesn't exist) (or 0.01 times if PAL+InterlaceOn)
  341-511   never   (doesn't exist)
```

For the 1 and 3 times effect, one would expect the 40 clk refresh as so:

```text
  --- H=133.5--><--H=134.0 ------------------ H=143.5--><--H=144.0 ---
  ccccccccccccccRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRcccccccccccccc
```

but, sometimes (randomly at 50:50 chance) it occurs somewhat like so:

```text
  ccccccccccccRRccRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRccRRccccccccccc
```

whereas "c"=CPU, "R"=Refresh (ie. sometimes, the CPU wakes up within the Refresh period). Unknown why & when exactly that stuttering refresh occurs.

Opcodes may begin before Refresh, and end after Refresh (rather than forcefully finishing the Opcode before starting Refresh), so above statistics are same for latching via "MOV A,[2137h]" (4 cycles) and "MOV A,[002137h]" (5 cycles) opcodes.

Writing [4201h] (instead of reading [3137h]) shifts the visible refresh-related H values (H=136..143=never, H=135/144 once, H=134/H=145 thrice), but obviously doesn't alter the amounts of visible PPU-related H values.
