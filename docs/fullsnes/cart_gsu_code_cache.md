# SNES Cart GSU-n Code-Cache

#### ROM/RAM-Code-Cache (512-byte cache)

This cache is used only for Opcode fetches from ROM or RAM (not for reading/writing Data to/from ROM nor RAM) (however, it does slightly increase data access speed in so far that data can be read/written via Gamepak bus, simultaneously while fetching opcodes from the cache).

32 lines of 16-bytes.

#### CACHE

#### LJMP

#### STOP

ABORT after STOP, one "must" clear GO by software to clear the cache

Cache Area "SNES_Addr = (CBR AND 1FFh)+3100h". For example, a CACHE opcode at C3A5h will set CBR to C3A0h, and the (initially empty) cached region will be C3A0h..C59Fh, when code gets loaded into the cache, GSU:C3A0h..C3FFh shows up at SNES:32A0h..32FFh, and GSU:C400h..C59Fh at SNES:3100h..329Fh.

#### Writing to Code-Cache (by SNES CPU)

First of, set GO=0 (ie. write SFR=0000h), this forces CBR=0000h, and marks all cache lines as empty. Then write opcodes 16-byte lines at 3100h..32FFh, writing the last byte of a line at [3xxFh] will mark the line as not-empty.

Thereafter, the cached code can be excuted (by setting R15 to 0000h..01Fxh), in this case, the GSU can be operated without RON/RAN flags being set - unless R15 leaves the cached area (this occurs also when a STOP is located in last byte of last cache line; the hardware tries to prefetch one byte after STOP), or unless ROM-DATA is accessed (via GETxx opcodes) or unless RAM-DATA is accessed (via LOAD/STORE or PLOT/RPIX opcodes). Ie. usually one would have RAN set (unless all incoming/outgoing parameters can be passed though R0..R14 registers).

#### Code-Cache Loading Notes

The 16-byte cache-lines are loaded alongside while executing opcodes (rather than first loading the whole 16-byte-line, and then executing the opcodes within it; which would be slightly slower). There are two special cases related to jumps: If current cache-line isn't fully loaded then hardware keeps loading the remaining bytes (from jump-origin to end-of-line). If the jump-target isn't aligned by 16 (and isn't yet cached), then the hardware loads the leading bytes (from start-of-line to jump-target). After that two steps, normal execution continues at the jump-target address.

The leading-stuff also occurs on CACHE instruction.

```text
  CACHE sets CBR to "R15 AND FFF0h" (whereas R15=address after CACHE opcode)
  LJMP sets CBR to "R15 AND FFF0h" (whereas R15=jump target address)
  SNES write to SFR register with GO=0 sets CBR=0000h
  (all of the above three cases do also mark all cache lines as empty)
```

All Code-Cache lines are marked as empty when executing CACHE or LJMP opcodes, or when the SNES clears the GO flag (by writing to SFR). The STOP opcode however (which also clears GO), doesn't empty the cache, so one may eventually re-use the cached values when restarting the GSU (however, if PBR or code in GamePak RAM has changed, then one must clear the cache by writing GO=0).

According to cache description (page 132), Cache-Code is 6 times faster than ROM/RAM. However, according to opcode descriptions (page 160 and up), cache is only 3 times faster than ROM/RAM. Whereas, maybe 6 times refers to 21MHz mode, and 3 times to 10MHz mode?

The CACHE opcode is typically executed prior to loops and/or at the begin of often-used sub-functions (or ideally, the loop and any used subfunctions should both fit into the 512-byte cache region).
