# SNES Cart GSU-n Pixel-Cache

#### RAM-Pixel-Write-Cache (two 8-pixel rows)

pixel cache is flushed when:

```text
  1) cache full
  2) doing rpix <--- this does also WAIT until it is flushed
  3) changing r1 or r2 (really?)
```

#### Pixel Cache

Primary Pixel Cache (written to by PLOT) Secondary Pixel Cache (data copied from Primary Cache, this WAITs if Secondary cache wasn't yet forwarded to RAM) (if less than 8 flags are set, data is merged with old RAM data).

Each cache contains 8 pixels (with 2bit/4bit/8bit depth), plus 8 flags (indicating if (nontransparent) pixels were plotted).

Pixel X/Y coordinates are 8bit wide (using LSBs of R1/R2 registers).

(X and F8h) and (Y and FFh) are memorized, when plotting to different values, Primary cache is forwarded to Secondary Cache, this happens also when all 8 cache flags are set.

Do not change SCREEN MODE (how to do that at all while GSU is running? SCMR is writeable for changing RAN/RON, but changing the other SCMR bits during GSU execution would be rather unpredictable) when data is in pixel caches (use RPIX to force the caches to be flushed). Before STOP opcode, do also use RPIX to force the caches to be flushed.

RPIX isn't cached, it does always read data from RAM, not from cache. Moreover, before reading RAM, RPIX does force both pixel caches (unless they are empty) to be forwarded to RAM. This is making RPIX very slow (trying to read/modify pixels via RPIX+PLOT would work very slow). So far, RPIX is mainly useful for forcing the pixel caches to be forwarded to RAM (and to WAIT until that forwarding has completed).
