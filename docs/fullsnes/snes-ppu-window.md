# SNES PPU Window

The window feature allows to disable BG/OBJ layers in selected regions, and also to alter Color-Math effects in selected regions.

```text
2126h - WH0 - Window 1 Left Position (X1) (W)
2127h - WH1 - Window 1 Right Position (X2) (W)
2128h - WH2 - Window 2 Left Position (X1) (W)
2129h - WH3 - Window 2 Right Position (X2) (W)
```

Specifies the horizontal boundaries of the windows. Note that there are no vertical boundaries (these could be implemented by manipulating the window registers via IRQ and/or HDMA).

```text
  7-0   Window Position (00h..0FFh; 0=leftmost, 255=rightmost)
```

The "inside-window" region extends from X1 to X2 (that, including the X1 and X2 coordinates), so the window width is X2-X1+1. If the width is zero (or negative), then the "inside-window" becomes empty, and the whole screen will be treated "outside-window".

```text
2123h - W12SEL - Window BG1/BG2 Mask Settings (W)
2124h - W34SEL - Window BG3/BG4 Mask Settings (W)
2125h - WOBJSEL - Window OBJ/MATH Mask Settings (W)
  Bit  2123h 2124h 2125h
  7-6  BG2   BG4   MATH  Window-2 Area (0..1=Disable, 1=Inside, 2=Outside)
  5-4  BG2   BG4   MATH  Window-1 Area (0..1=Disable, 1=Inside, 2=Outside)
  3-2  BG1   BG3   OBJ   Window-2 Area (0..1=Disable, 1=Inside, 2=Outside)
  1-0  BG1   BG3   OBJ   Window-1 Area (0..1=Disable, 1=Inside, 2=Outside)
```

Allows to select if the window area is inside or outside the X1,X2 coordinates, or to disable the area.

```text
212Ah/212Bh - WBGLOG/WOBJLOG - Window 1/2 Mask Logic (W)
  Bit  212Ah 212Bh
  7-6  BG4   -     Window 1/2 Mask Logic (0=OR, 1=AND, 2=XOR, 3=XNOR)
  5-4  BG3   -     Window 1/2 Mask Logic (0=OR, 1=AND, 2=XOR, 3=XNOR)
  3-2  BG2   MATH  Window 1/2 Mask Logic (0=OR, 1=AND, 2=XOR, 3=XNOR)
  1-0  BG1   OBJ   Window 1/2 Mask Logic (0=OR, 1=AND, 2=XOR, 3=XNOR)
```

Allows to merge the Window 1 and 2 areas into a single "final" window area (which is then used by TMW, TSW, and CGWSEL). The OR/AND/XOR/XNOR logic is applied ONLY if BOTH window 1 and 2 are enabled (in WxxSEL registers). If only one window is enabled, then that window is used as is as "final" area. If both are disabled, then the "final" area will be empty. Note: "XNOR" means "1 XOR area1 XOR area2" (ie. the inverse of the normal XOR result).

```text
212Eh - TMW - Window Area Main Screen Disable (W)
212Fh - TSW - Window Area Sub Screen Disable (W)
  7-5  Not used
  4    OBJ (0=Enable, 1=Disable)  ;\"Disable" forcefully disables the layer
  3    BG4 (0=Enable, 1=Disable)  ; within the window area (otherwise it is
  2    BG3 (0=Enable, 1=Disable)  ; enabled or disabled as selected in the
  1    BG2 (0=Enable, 1=Disable)  ; master enable bits in port 212Ch/212Dh)
  0    BG1 (0=Enable, 1=Disable)  ;/
  -    Backdrop (Always enabled)
```

Allows to disable video layers within the window region.
