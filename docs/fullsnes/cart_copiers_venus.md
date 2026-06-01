# SNES Cart Copiers - Venus (Multi Game Hunter)

MGH (Multi Game Hunter) from Venus.

The 32Kbyte BIOS contains both SNES/65C816 code (entrypoint at [FFFCh]) and Genesis/Z80 code (entrypoint at 0000h).

```text
  006000..007FFF -- RAM or so
  035800..035807 -- I/O Ports
  ---
  006400.R          id "SFCJ"
  007D00..007EFF.R  checksummed
  035800.W    set to C0h
  035801.W    set to A0h
  035802.W    set to 0000h or 06h
  035803.W    set to 04h
  035804.R    disk status?  (bit7,bit6)
  035805.W    disk command? (set to 0Bh) (not a uPD765 command?)
  035806.W    set to 00h or ([AAh] ROR 1)
  035807.W    set to 00h or [ABh]
```

#### FDC Accress via 80C51 CPU

Like many other copiers, the MGH does use a "normal" MCS3201FN controller, but, it does indirectly access it through a 80C51 CPU. For example,

```text
    05       cmd (write sec)                                 ;\
    [0B6B]   track          ;\less parameters as than        ; write sector
    [0BA4]   head           ; directly accessing a uPD765    ; command
    [0BA2]   sector         ;/                               ; (at PC=CBAFh)
    [[18]+y] data... (200h bytes)                            ;/
```
