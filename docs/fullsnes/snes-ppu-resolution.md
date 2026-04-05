# SNES PPU Resolution

#### Physical Resolution

The physical resolution (of the TV Screen) is:

```text
  256x224 for 60Hz (NTSC) consoles
  256x264 for 50Hz (PAL) consoles
```

The 50Hz/60Hz cannot be changed by software (it can be only changed by modding some pins on the mainboard).

#### Normal Picture Resolution

The vertical resolution is software-selectable, 224 or 239 lines:

```text
  256x224 near-fullscreen on 60Hz (NTSC) consoles (or Tiny Picture at 50Hz)
  256x239 not-really-fullscreen on 50Hz (PAL) consoles (or Overscan at 60Hz)
```

Most commonly the resolution is selected matching to the frame rate. With 60Hz Overscan the upper/lower lines are normally not visible (but may be useful to ensure that there is absolutely no upper/lower border visible, which may be important for avoiding flickering in interlace mode, especially with bright pictures; in contrast to the black screen border). 50Hz Tiny Picture would be a simple solution for porting 224-lines NTSC games to PAL, though it produces extra-big upper/lower borders.

#### High-Resolution Modes

There are some methods to double the resolution horizontally and/or vertically:

True High-Resolution (BG Mode 5,6) (optionally with SETINI.0) ...

Pseudo Horizontal High-Resolution (SETINI.3) ...

Pseudo Vertical High-Resolution (SETINI.0) (Interlace) ...

OBJ High-Resolution (SETINI.1) ...

#### Hires Notes

Horizontal BG Scrolling is always counted in full-pixels; so true-hires cannot be scrolled in half-pixel units (whilst pseudo hires may be scrolled in half-pixels by exchanging main/sub-screen layers and using different scroll offsets for each layer).

Vertical BG Scrolling is counted in half-pixels (only when using true hv-hires).

Mosaic is always counted in full-pixels; so a mosaic size of 1x1 (which is normally same as mosaic disabled) acts as 2x1 half-pixels in true h-hires mode (and as 2x2 half-pixels in true hv-hires mode, reportedly?) (and as 1x2 in pseudo v-hires, presumably?).

Although h-hires uses main/subscreen for the half-pixels, both main/subscreen pixels are forced to use Color 0 as backdrop color (rather than using COLDATA setting as subscreen backdrop) (this applies for both true+pseudo hires).

Window X1/X2 coordinates are always counted in full-pixels (reportedly with an odd glitch in hires mode?).

OBJ X/Y coordinates are always counted in full-pixels.

#### Hires Appearance

Horizontal hires may appear blurred on most TV sets (due to the quality of the video signal and TV-screen, and, when not using a RGB-cable, of the composite color clock). For example, non-continous white pixels (W) on black background may appear as gray pixels (g):

```text
  Source Pixels       --->  TV-Screen Pixels
  WW   W   WW   W W         WW   g   WW   ggg
  WWW   W   WW   W W        WWW   g   WW   ggg
```

To reproduce that by software: CenterPix=(LeftPix+CenterPix*2+RightPix)/4.

Using the above example on a red background looks even worse: the "g" and "ggg" portions may be barely visible (appear as slightly pastelized red shades).

Vertical hires is producing a flickering image: The scanlines jump up/down after each frame (and, due to a hardware glitch: the topmost lines also jump left/right on PAL consoles). The effect is most annoying with hard-contrast images (eg. black/white text/lines, or bright pixels close to upper/lower screen borders), it may look less annoying with blurry images (eg. photos, or anti-alized text/lines). Some modern TV sets might be buffering the even/odd frames in internal memory, and display them as flicker-free hires image(?)

#### Hires Software

```text
  Air Strike Patrol (mission overview)       (whatever mode? with Interlace)
  Bishoujo Wrestler Retsuden (some text)     (512x448, BgMode5+Interlace)
  Ball Bullet Gun (in lower screen half)     (512x224, BgMode5)
  Battle Cross (in game) (but isn't hires?)  (512x224, BgMode1+PseudoH)(Bug?)
  BS Radical Dreamers (user name input only) (512x224, BgMode5)
  Chrono Trigger (crash into Lavos sequence) (whatever mode? with Interlace)
  Donkey Kong Country 1 (Nintendo logo)      (512x224, BgMode5)
  G.O.D. (intro & lower screen half)         (512x224, BgMode5)
  Jurassic Park (score text)                 (512x224, BgMode1+PseudoH+Math)
  Kirby's Dream Land 3 (leaves in 1st door)  (512x224, BgMode1+PseudoH)
  Lufia 2 (credits screen at end of game)    (whatever mode?)
  Moryo Senki Madara 2 (text)                (512x224, BgMode5)
  Power Drive (in intro)                     (512x448, BgMode5+Interlace)
  Ranma 1/2: Chounai Gekitou Hen             (256x448, BgMode1+InterlaceBug)
  RPM Racing (in intro and in game)          (512x448, BgMode5+Interlace)
  Rudra no Hihou (RnH/Treasure of the Rudras)(512x224, BgMode5)
  Seiken Densetsu 2 (Secret of Mana) (setup) (512x224, BgMode5)
  Seiken Densetsu 3                          (512x224, BgMode5)
  Shock Issue 1 & 2 (homebrew eZine)         (512x224, BgMode5)
  SNES Test Program (by Nintendo) (Character Test includes BgMode5/BgMode6)
  Super Play Action Football (text)          (512x224, BgMode5)
  World Cup Striker (intro/menu)             (512x224, BgMode5)
```

And, reportedly (may be incorrect, unknown how to see hires in that games):

```text
  Dragonball Z Super Butoden 2-3 (when you start it up in the black screen?)
```

Notes: Ranma is actually only 256x224 (but does accidentally have interlace enabled, which causes some totally useless flickering). Jurassic/Kirby are misusing PseudoH for "blending" the main+sub screen layers (via the blurry TV picture); this allows to use Color Math for coldata additions (ie. resulting in 3-color blending: main+sub+coldata).

#### 3D Software

> **See:** [SNES 3D Glasses](snes-3d-glasses.md)
