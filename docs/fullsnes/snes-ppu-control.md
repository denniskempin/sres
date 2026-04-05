# SNES PPU Control

```text
2100h - INIDISP - Display Control 1 (W)
  7     Forced Blanking (0=Normal, 1=Screen Black)
  6-4   Not used
  3-0   Master Brightness (0=Screen Black, or N=1..15: Brightness*(N+1)/16)
```

In Forced Blank, VRAM, OAM and CGRAM can be freely accessed (otherwise it's accessible only during Vblank). Even when in forced blank, the TV Set keeps receiving Vsync/Hsync signals (thus producing a stable black picture). And, the CPU keeps receiving Hblank/Vblank signals (so any enabled video NMIs, IRQs, HDMAs are kept generated).

```text
  Forced blank doesn't apply immediately... so one must wait whatever
  (maybe a scanline) before VRAM can be freely accessed... or is it only
  vice-versa: disabling forced blank doesn't apply immediately/shows garbage
  pixels?

212Ch - TM - Main Screen Designation (W)
212Dh - TS - Sub Screen Designation (W)
  7-5  Not used
  4    OBJ (0=Disable, 1=Enable)
  3    BG4 (0=Disable, 1=Enable)
  2    BG3 (0=Disable, 1=Enable)
  1    BG2 (0=Disable, 1=Enable)
  0    BG1 (0=Disable, 1=Enable)
  -    Backdrop (Always enabled)
```

Allows to enable/disable video layers. The Main screen is the "normal" display.

The Sub screen is used only for Color Math and for 512-pixel Hires Mode.

```text
2133h - SETINI - Display Control 2 (W)
  7     External Synchronization (0=Normal, 1=Super Impose and etc.)
  6     EXTBG Mode (Screen expand)
          ENABLE THE DATA SUPPLIED FROM THE EXTERNAL LSI.
          FOR THE SFX, ENABLE WHEN THE SCREEN WITH PRIORITY IS USED ON MODE-7.
  5-4   Not used
  3     Horizontal Pseudo 512 Mode (0=Disable, 1=Enable)
          (SHIFT SUBSCREEN HALF DOT TO THE LEFT)
  2     BG V-Direction Display (0=224 Lines, 1=239 Lines) (for NTSC/PAL)
  1     OBJ V-Direction Display (0=Low, 1=High Resolution/Smaller OBJs)
          IN THE INTERLACE MODE, SELECT EITHER OF 1-DOT PER LINE OR 1-DOT
          REPEATED EVERY 2-LINES. IF "1" IS WRITTEN, THE OBJ SEEMS REDUCED
          HALF VERTICALLY IN APPEARANCE.
  0     V-Scanning         (0=Non Interlace, 1=Interlace) (See Port 2105h)
```
