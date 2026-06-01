---
title: "Color math"
source_url: "https://snes.nesdev.org/wiki/Color_math"
pageid: 97
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

**Color math** allows the SNES to do some limited blending between layers or colours. This can be useful for effects like transparent shadows, ghosts that fade away, or a translucent colored text box.

## Registers

The following registers directly affect color math:

| Name | Address | Bits | Type | Notes |
| --- | --- | --- | --- | --- |
| [[PPU registers#TM|TM]] | $212C | ...O 4321 | W8 | Main screen layer enable (PPU registers#). |
| [[PPU registers#TS|TS]] | $212D | ...O 4321 | W8 | Sub screen layer enable (#). |
| [[PPU registers#CGWSEL|CGWSEL]] | $2130 | MMSS ..AD | W8 | main/sub screen color math window regions (MS), fixed/subscreen (A), direct color (D). |
| [[PPU registers#CGADSUB|CGADSUB]] | $2131 | MHBO 4321 | W8 | Color math add/subtract (M), half (H), backdrop (B), layer enable (O4321). |
| [[PPU registers#COLDATA|COLDATA]] | $2132 | BGRC CCCC | W8 | Fixed color channel select (BGR) and value (C). |

In addition to the above, color math can be independently affected by [[Windows|windowing]].

As with most PPU registers, color math settings can be changed on a per-scanline basis if needed, typically using [[HDMA]].

## Color math sources

There are two options for a color source to blend, selected by [[PPU registers#CGWSEL|CGWSEL]] bit 1:

- Fixed color
- Sub screen

### Fixed color

This option applies the blend against a single fixed color, specified by [[PPU registers#COLDATA|COLDATA]].

This is especially useful for blending against a vertical gradient, using [[HDMA]] to change COLDATA on each line.

Fixed color is also compatible with [[Backgrounds#High resolution|high resolution]] mode.

### Sub screen

The **sub screen** reuses the pixels calculated during rendering to composite a second version of the visible screen.

This allows us to select one set of layers for the main screen ([[PPU registers#TM|TM]]),
and a different set of layers for the sub screen ([[PPU registers#TS|TS]]),
and use color math to blend between the two.

Because [[Backgrounds#High resolution|high resolution]] mode repurposes the sub screen for every second column,
it may not be practical to use the sub screen for color math at the same time. (Fixed color does not have this complication.)

## Masking

When rendering, the main screen remembers which layer its visible pixel came from, and this can be used to selectively apply color math to only some layers.

[[PPU registers#CGADSUB|CGADSUB]] lets you designate whether color math can be used wherever the main screen shows: sprites (OBJ), a background (BG1-4), or the backdrop (CGRAM entry 0).
If any of these layers are not enabled, it will instead show only the main screen result without color math.

Additionally, sprites using palettes 0-3 will reject color math. This allows us to have both opaque and transparent sprites at the same time.
Because this selection is made based on the main screen only, if we wished to blend with sprites using these palettes we could place them on the sub screen instead.

[[PPU registers#CGWSEL|CGWSEL]] allows [[Windows]] to disable color math using the color window.

## Blend operations

The top two bits of [[PPU registers#CGADSUB|CGADSUB]] control whether the blend operation is an addition or subtraction, and also whether to half the result. This creates 4 available blend modes.

For each pixel, if the color math operation is not rejected by masking, it blends each RGB color channel:
1. Take the main screen color.
2. Add (or subtract) the fixed (or sub screen) color.
3. Divide by 2, optionally.
4. If the result for a color channel is < 0 or > 31, it will be clamped to 0 or 31.

### Add

Additive blending simply adds the two colors together.

This can lighten colors, but not darken them.

Because the strength of the added color is determined by its palette, we can fade an additive blend toward invisible just by fading the relevant palette colors toward black. (Anything + 0 = the same thing.)

### Subtract

This subtracts the sub screen (or fixed color) from the main screen.

This can darken colors, but not lighten them.

Typically the palette used for the sub screen layer being subtracted will be inverted (i.e. 31 - the color we want). Subtracing white from the main screen will produce black, subtracting blue from the main screen produces a yellow filter, etc..

As with additive blending, we can fade the subtractive blend toward invisible by fading its palette colors toward black. (Anything - 0 = the same thing.)

### Add + Half

This is a 50% blend between the main screen and sub screen (or fixed color).

Unlike additive or subtractive blending alone, we can't adjust the percentage of fade here by adjusting the palettes. It is always only 50%.

Note that because sprites count as a single layer (OBJ), we cannot blend sprites against other sprites. Only different layers can be blended.

### Subtract + Half

This mode is rarely used, because it is essentially the same as subtractive but then the result is reduced to half brightness.

Because subtractive can only darken the picture to begin with, there is not much reason to use this feature to darken it further.

## Examples

### Add + Half Water

[![](https://snes.nesdev.org/w/images/snes/1/16/Colourmath_addhalf.png)](https://snes.nesdev.org/wiki/File:Colourmath_addhalf.png)

On the main screen, we enable an extra BG layer the covers the sprites with opaque water. On the sub screen, we leave the water layer off. When combined, we get a 50% transparency on the water.

If the water was on the sub screen instead, sprites with palettes 0-3 would end up masking the color math, and appear on top of the water instead of blended.

Examples: *Secret of Mana*.

### Additive Horizon

[![](https://snes.nesdev.org/w/images/snes/3/37/Dizworld_additive.png)](https://snes.nesdev.org/wiki/File:Dizworld_additive.png)

Here a faded horizon effect is applied by using additive blending with a fixed color.
COLDATA is updated using [[HDMA]] to create a vertical gradient.

An effect like this is seen in many games with a [[Mode 7 perspective effects|mode 7 perspective effect]].
Since mode 7 only has 1 background layer, any color math would have to come from a fixed color, or sprites.
Subtractive mode could be used to fade toward black, instead of white.

Examples: *F-Zero*, *Secret of Mana* (flying map), *Final Fantasy VI* (flying map), *Secret of Evermore* (flying map), *Demon's Crest* (flying map), *Super Star Wars* (stage 2 landspeeder).

### Additive Fadeout

[![](https://snes.nesdev.org/w/images/snes/f/f8/Colourmath_add_fade1.png)](https://snes.nesdev.org/wiki/File:Colourmath_add_fade1.png)

[![](https://snes.nesdev.org/w/images/snes/8/85/Colourmath_add_fade2.png)](https://snes.nesdev.org/wiki/File:Colourmath_add_fade2.png)

By adjusting the palette colours of things used for an additive blend, you can fade their intensity up and down. Darken their palette all the way to black, and they disappear entirely,

Note that only the ghosts are using palettes 4-7, and the other sprites on the main screen do not participate in the additive blend. Also note that ghosts can blend with the background, but not other sprites.

Examples: *Super Mario World* (Sunken Ghost Ship), *Prehistorik Man* (Stage 23: Dino Graveyard).

### Subtractive Darkness

[![](https://snes.nesdev.org/w/images/snes/c/cc/Colourmath_sub_dark.png)](https://snes.nesdev.org/wiki/File:Colourmath_sub_dark.png)

Subtractive blending can be used to darken areas of the picture.

In this first example, because it's subtracted from the main screen, the sub screen contains the inverse of the effect you want to see. Bright areas of the sub screen darken the image. Red areas of the sub screen remove red and leave behind cyan, etc..

However, notice that two of the sprites are not darkened or colored by the blend. This is because they are using palettes 0-3 and mask the blend. This feature can be useful for keeping specific sprites unaffected where desired, such as a HUD that needs to be always visible.

Examples: *Yoshi's Island* (1-6: Shy-Guys on Stilts, underground section).

[![](https://snes.nesdev.org/w/images/snes/8/83/Colourmath_sub_dark_inverse.png)](https://snes.nesdev.org/wiki/File:Colourmath_sub_dark_inverse.png)

If you need all sprites to participate in the blend, this can be accomplished with a workaround: swap the main and sub screens, and invert the entire palette. (A color can be inverted simply with XOR by $3FFF.)

Because **A - B = (-B) - (-A)**, the subtractive result is the same, but because sprites are now on the sub screen they do not mask the blend, and can receive the effect of darkness.

Note that here, the main screen is now a more direct brightness/color-filter map, rather than inverted. White is bright. Black is dark. Red filters red, etc..

Examples: *Donkey Kong Country* (Gorilla Glacier: Torchlight Trouble!).

## Links

- [Transparency](https://wiki.superfamicom.org/transparency) - article at superfamicom.org
- [colourmath](https://github.com/bbbradsmith/SNES_stuff/tree/main/colourmath) - demo ROM used for some of the examples above
