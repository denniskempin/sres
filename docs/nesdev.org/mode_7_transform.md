---
title: "Mode 7 transform"
source_url: "https://snes.nesdev.org/wiki/Mode_7_transform"
pageid: 80
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

[[Backgrounds#Mode 7|Background mode 7]] has the ability to perform an [affine transformation](https://en.wikipedia.org/wiki/Affine_transformation) on its [[Tilemap#Mode 7]].

Instead of rendering normally, this tilemap (effectively a 1024x1024 pixel square) can be given an arbitrary 2D affine transformation, which means this is a square that can:

- Translate or slide its position up, down, left or right.
- Rotate at any angle.
- Zoom in and out or be squashed, but the scaling must be uniform along any axis (i.e. it can stretch along a straight line, but it cannot "bend").
- Shear or skew.

It is conceptually similar to [texture mapping](https://en.wikipedia.org/wiki/Texture_mapping) a single quad on a modern GPU. Conversely, it is also like selecting a [parallelogram](https://en.wikipedia.org/wiki/Parallelogram) region from the tilemap, and stretching its four corners to the rectangle of the screen.

The affine transformation can be changed every scanline via [[HDMA]], allowing versatile perspective and distortion effects.

:   See: [[Mode 7 perspective effects]]

The usual BG1 scrolling registers are replaced with [[PPU registers#M7HOFS|M7HOFS]], [[PPU registers#M7VOFS|M7VOFS]], which scroll the tilemap after transformation.

The affine transformation is applied by [[PPU registers#M7A|M7A]], [[PPU registers#M7B|M7B]], [[PPU registers#M7C|M7C]], [[PPU registers#M7D|M7D]], with an additional pivot-point center offset via [[PPU registers#M7X|M7X]], [[PPU registers#M7Y|M7Y]].

ABCD defines a [transformation matrix](https://en.wikipedia.org/wiki/Transformation_matrix), which combined with the offset and pivot maps screen pixel coordinats (Sx,Sy) to texel coordinates (Tx,Ty):

```
+-       -+   +-                 -+   +-   -+   +-  -+
| M7A M7B |   | Sx + M7HOFS - M7X |   | M7X |   | Tx |
|         | * |                   | + |     | = |    |
| M7C M7D |   | Sy + M7VOFS - M7Y |   | M7Y |   | Ty |
+-       -+   +-                 -+   +-   -+   +-  -+
```

## Affine Matrix

**M7A**, **M7B**, **M7C**, **M7D** together define how to map the tilemap "texture" to the screen, as pixels are rasterized left to right, top to bottom. In this explanation a *pixel* is an output pixel on the screen, and a *texel* is the color fetched from the 1024x1024 background tilemap. Each of these is an 8.8 fixed point value.

When you move one *pixel* to the right on the screen:

- **M7A** is how many *texels* to move to the right on the background.
- **M7C** is how many *texels* to move down.

When you move one *pixel* down on the screen:

- **M7B** is how many *texels* to move right.
- **M7D** is how many *texels* to move down.

In modern computer graphics terms: (M7A,M7C) and (M7B,M7D) are [2D vectors](https://en.wikipedia.org/wiki/Vector_(mathematics_and_physics)) defining Δ*u* and Δ*v* for [texture mapping](https://en.wikipedia.org/wiki/Texture_mapping).

This is why the [[Init code|recommended default values]] of (1,0) (0,1) makes mode 7 behave like a normal background. 1 pixel to the right = 1 texel to the right. 1 pixel down = 1 texel down.

**Scaling** can be accomplished by changing the length of these vectors.

- (2,0) (0,1) will move 2 texels right for every 1 pixel, shrinking by 1/2 in the horizontal.
- (1,0) (0,-0.1) will move 0.1 texels up for every 1 pixel, stretching by 10 in the vertical and flipping upside down.

**Rotation** can be accomplished by rotating these vectors.

- (*cos* ϴ, *sin* ϴ) (*-sin* ϴ, *cos* ϴ) form a standard [rotation matrix](https://en.wikipedia.org/wiki/Rotation_matrix) that will rotate the map by ϴ degrees.

:   :   On screen: it will rotate the map counter-clockwise relative to the pivot-point.
    :   On map: it will rotate the view parallelogram clockwise.

- (0.866, 0.500) (-0.500, 0.866) will rotate by 30 degrees (CCW on screen, CW on map).

**[Shearing](https://en.wikipedia.org/wiki/Shear_mapping)** creates a slanted mapping by adding to one coordinate unevenly.

- (1,0) (0.2,1) causes the background to gradually slide to the left as it proceeds down the screen.

Scaling, rotation, and shearing together can be combined into a single [transformation matrix](https://en.wikipedia.org/wiki/Transformation_matrix) in A/B/C/D. This can be computed by [matrix multiplication](https://en.wikipedia.org/wiki/Matrix_multiplication).

## Center Adjustment

**M7X** and **M7Y** define a *texel* coordinate that becomes the center (a.k.a. pivot-point) of the scaling/rotation applied by the affine transformation matrix ABCD. This allows you to rotate around some other point besides the top left of the map.

**M7HOFS** and **M7VOFS** define a starting point for rasterization. After the transformation ABCD/XY is applied, this is a *pixel* coordinate that shifts the top-left of the screen. E.g. increasing M7HOFS by 1 will have the effect of moving whatever is in view 1 pixel to the left. With the default matrix this is exactly the same as just scrolling the map in other modes.

## Summary

On screen:

- Start with a view of the top left 256x224 pixels of the tilemap.
- From that view, pick a pivot-point on the map (M7X,M7Y) and rotate and scale around that point (ABCD).
- Now (M7HOFS,M7VOFS) will move (right,down) in screen-space, scrolling over the transformed map.

On the map:

- (A,C) and (B,D) are vectors that define the angle and size of a parallelogram that will be the screen's view.
- The top-left corner of the parallelogram is a more complicated computation involving all 8 register values.
- Increasing M7HOFS or M7VOFS by 1 will move the top-left corner along the direction of the parallelogram sides equivalent to 1 screen pixel.

## External Links

- [Novasquirrel: Mode 7 Preview](https://novasquirrel.github.io/Mode7Preview/) - Javascript preview of mode 7 register effect
- [Telinc1: Mode 7 Simulator](https://telinc1.github.io/mode7/) - Javascript preview of mode 7 register effect
