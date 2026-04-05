# SNES Memory Work RAM Access

The SNES includes 128Kbytes of Work RAM, which can be accessed in several ways:

```text
  The whole 128K are at 7E0000h-7FFFFFh.
  The first 8K are also mirrored to xx0000h-xx1FFFh (xx=00h..3Fh and 80h..BFh)
  Moreover (mainly for DMA purposes) it can be accessed via Port 218xh.

2180h - WMDATA - WRAM Data Read/Write (R/W)
  7-0   Work RAM Data
```

Simply reads or writes the byte at the address in [2181h-2183h], and does then increment the address by one.

Note: Despite of the fast access time on 2180h reads (faster than 7E0000h-7FFFFFh reads), there is no prefetching involved (reading 2180h always returns the currently addressed byte, even if one mixes it with writes to 2180h or to 7E0000h-7FFFFFh).

```text
2181h - WMADDL - WRAM Address (lower 8bit) (W)
2182h - WMADDM - WRAM Address (middle 8bit) (W)
2183h - WMADDH - WRAM Address (upper 1bit) (W)
```

17bit Address (in Byte-steps) for addressing the 128Kbytes of WRAM via 2180h.

#### DMA Notes

WRAM-to-WRAM DMA isn't possible (neither in A-Bus to B-Bus direction, nor vice-versa). Externally, the separate address lines are there, but the WRAM chip is unable to process both at once.

#### Timing Notes

Note that WRAM is accessed at 2.6MHz. Meaning that all variables, stack, and program code in RAM will be slow. The SNES doesn't include any fast RAM.

However, there are a few tricks to get "3.5MHz RAM":

* Sequential read from WRAM via [2180h] is 3.5MHz fast, and has auto-increment.

* DMA registers at 43x0h-43xBh provide 8x12 bytes of read/write-able "memory".

* External RAM could be mapped to 5000h-5FFFh (but usually it's at slow 6000h).

* External RAM could be mapped to C00000h-FFFFFFh (probably rarely done too).

#### Other Notes

The B-Bus feature with auto-increment is making it fairly easy to boot the SNES without any ROM/EPROM by simply writing program bytes to WRAM (and mirroring it to the Program and Reset vector to ROM area):

> **See:** [SNES Xboo Upload (WRAM Boot)](snes-xboo-upload-wram-boot.md)

Interestingly, the WRAM-to-ROM Area mirroring seems to be stable even when ROM Area is set to 3.5MHz Access Time - so it's unclear why Nintendo has restricted normal WRAM Access to 2.6MHz - maybe some WRAM chips are slower than others, or maybe they become unstable at certain room temperatures.
