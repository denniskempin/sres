# SNES Timing PPU Memory Accesses

Below is some info/guesses on what is happening during the 1364 master cycles of a scanline. Plus some info/guesses on if/when/why it is (or isn't) possible the change VRAM/CGRAM/OAM outside of V-Blank or Forced-Blank.

#### PPU VRAM Load

VRAM access time is 4 master cycles per word (aka 1 dot-cycle per word). In each line, the PPU fetches 34*2 words (for 34 OBJ tiles of 8pix/4bpp), plus 33*8 words (for BgMap+BgTiles) (eg. 33*4 BgMap entries plus 33*4 BgTiles at 2bpp in BG Mode 0) (or less BG layers, but more bpp per tile in other BG Modes).

As a whole, 4 master cycles per 34*2 + 33*8 words sums up to 1328 master cycles (or possibly a bit more if accesses during "long dots" should happen to take 5 master cycles per word). 1328 would leave 36 unused master cycles per scanline (or maybe MORE unused cycles if some BG layers are disabled? and/or if less than 34 OBJ tiles are located in the scanline? unknown if VRAM can be accessed during any such unused cycles? as far as known, existing games aren't trying to do so).

#### PPU Palette Load

Not much known about CGRAM access timings. There would be some theories:

```text
  1) Maybe 512 accesses/line (for frontmost main/sub-screen pixels)
  2) Maybe 1296 accesses/line (for 256*4 BG pixels + 34*8 OBJ pixels)
  3) Maybe 1360 accesses/line (for 33*4*8 BG pixels + 34*8 OBJ pixels)
```

For the 1296/1360 accesses therories, CGRAM would be inaccessible for most of the 1364 cycles.

In practice, it is possible to change CGRAM during certain timespots in the Hblank-period (not during the <whole> Hblank period, but it works <sometimes>, though doing so may require some research - and may end up with more or less fragile timings). As for when/why it is working: Maybe there some totally unused cycles, or it depends on how many OBJs are displayed in the current scanline, possibly also on the number of used and/or enabled BG layers (and thereby, maybe also allowing to change CGRAM outside of Hblank during BG-drawing period?).

#### PPU OAM load

OAM handling is done in three steps:

1) scan 128 entries (collect max 32 entries per line)  2) scan 32 entries (collect max 34 tiles per line)  3) scan 34 entries (collect max 34x8 pixels per line) OAM-Access time for Step 1 is 128*2 dots aka 128*8 master cycles (as seen in STAT77.Bit6). OAM-Access times for Steps 2 and 3 is unknown (might be some cycles per entry... or might be NULL cycles; in case OAM entries were copied to separate buffer during previous Step... and/or NULL cycles if no OBJs are in current scanline).

So, aside from the 256 known/used dot-cycles, there may (or may not) be up to 84 unused dot-cycles... possibly allowing to change OAM during Hblank(?).

Note: Mario Kart is using Forced Blank to change OAM in middle of screen.

Observe that the OAM address in Port 2102h is scattered during drawing period.
