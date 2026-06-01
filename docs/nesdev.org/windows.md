---
title: "Windows"
source_url: "https://snes.nesdev.org/wiki/Windows"
pageid: 132
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

**Windows** are used to hide layers from regions of the screen. On a per-scanline basis, they can be used to cut out areas of:

- [[Backgrounds|Background]] layers (BG1, BG2, BG3, BG4)
- The [[Sprites|Sprite]] layer (OBJ)
- The main screen
- The sub screen ([[Color math]])

## Registers

See: [[PPU registers#Windows|PPU registers: Windows]]

| Name | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- |
| [[PPU registers#W12SEL|W12SEL]] | $2123 | DdCc BbAa | W8 | Enable (ABCD) and Invert (abcd) windows for BG1 (AB) and BG2 (CD). |
| [[PPU registers#W34SEL|W34SEL]] | $2124 | DdCc BbAa | W8 | Enable (EFGH) and Invert (efgh) windows for BG3 (EF) and BG2 (GH). |
| [[PPU registers#WOBJSEL|WOBJSEL]] | $2125 | LlKk JjIi | W8 | Enable (IJKL) and Invert (ijkl) windows for OBJ (IJ) and color (KL). |
| [[PPU registers#WH0|WH0]] | $2126 | LLLL LLLL | W8 | Window 1 left position. |
| [[PPU registers#WH1|WH1]] | $2127 | RRRR RRRR | W8 | Window 1 right position. |
| [[PPU registers#WH2|WH2]] | $2128 | LLLL LLLL | W8 | Window 2 left position. |
| [[PPU registers#WH3|WH3]] | $2129 | RRRR RRRR | W8 | Window 2 right position. |
| [[PPU registers#WBGLOG|WBGLOG]] | $212A | 4433 2211 | W8 | Window mask logic for BG layers (00=OR, 01=AND, 10=XOR, 11=XNOR). |
| [[PPU registers#WOBJLOG|WOBJLOG]] | $212B | .... CCOO | W8 | Window mask logic for OBJ (O) and color (C). |
| [[PPU registers#TMW|TMW]] | $212E | ...O 4321 | W8 | Main screen layer window enable. |
| [[PPU registers#TSW|TSW]] | $212F | ...O 4321 | W8 | Sub screen layer window enable. |
| [[PPU registers#CGWSEL|CGWSEL]] | $2130 | MMSS ..AD | W8 | main/sub screen color window black/transparent regions (MS), fixed/subscreen (A), direct color (D). |

## Window

The SNES has two window devices which can be used to selectively cut holes out of parts of the picture.

Each window has a left and right pixel position ([[PPU registers#WH0|WH0]]/[[PPU registers#WH1|WH1]]/[[PPU registers#WH2|WH2]]/[[PPU registers#WH3|WH3]]) where the window should hide the affected layers. The window effect (1 output) is at left <= X <= right. If left > right then the window effect is nowhere (all 0 output). [[HDMA]] can be used to [[Drawing window shapes|create shapes]] by adjusting the left/right position for each scanline.

Each window can be separately enabled for the various layers, and also inverted ([[PPU registers#W12SEL|W12SEL]]/[[PPU registers#W34SEL|W34SEL]]). If the window is inverted, the left/right position instead defines where the window should *not* hide the affected layers (if left > right, the window effect is everywhere, all 1 output). If the window is disabled then invert has no effect (all 0 output).

Additionally if two windows are enabled for the same layer, boolean logic can be applied between them ([[PPU registers#WBGLOG|WBGLOG]]/[[PPU registers#WOBJSEL|WOBJSEL]]). An individual window is 0 when showing the layer, and 1 when hiding it. This logic will combine the effect of both windows in the chosen way.

The windows can be selectively applied to the main screen and sub screen layers ([[PPU registers#TMW|TMW]]/[[PPU registers#TSW|TSW]]).

Finally, the color window can be used to black areas of the main screen, or create a transparent region on the sub screen to mask [[Color math]] ([[PPU registers#CGWSEL|CGWSEL]]).

## Window Mask Logic

If both windows are enabled on the same layer, they will be combined using either OR, AND, XOR or XNOR boolean logic as set by the [[PPU registers#Window mask logic|Window mask logic registers]] (WBGLOG/WOBJLOG).

When the window mask logic bits are combined with the *invert window* flags they form 16 different configurations, 10 of which are unique.

[![](https://snes.nesdev.org/w/images/snes/9/9c/Window_mask_logic_table.png)](https://snes.nesdev.org/wiki/File:Window_mask_logic_table.png)

The 4 window mask logic settings combined with the 2 invert window flags

## See Also

- [[Drawing window shapes]]
