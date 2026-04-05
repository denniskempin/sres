---
title: "DSP-1"
source_url: "https://snes.nesdev.org/wiki/DSP-1"
pageid: 197
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

```
Command ($308000 - $3FBFFF / $B08000 - $BFBFFF in Mode 20, $006000 - $0F6FFF / $806000 - $8F6FFF if Mode 21, RW)
7  bit  0 (Write)
---- ----
CCCC CCCC
|||| ||||
++++-++++- Command / Parameters

7  bit  0 (Read)
---- ----
RRRR RRRR
|||| ||||
++++-++++- Command Result
```

```
Status ($30C000 - $3FFFFF / $B0C000 - $BFFFFF in Mode 20, $007000 - $0F7FFF / $807000 - $8F7FFF if Mode 21, R)
7  bit  0
---- ----
R... ....
|
+--------- Data Request (0 = DSP Busy; 1 = Ready for R/W)
```

Data Types:

Caption text

| Name | Description | Bits | Data Range | Unit |
| --- | --- | --- | --- | --- |
| A | Angle | 8 | -180° -> 180° | 2pi/2^16 |
| T | Fixed Point Decimal | 16 | -1.0 -> 0.999969... | 2^-15 |
| I8 | Int with Decimal (Fixed) | 16 | -128.0 -> 127.996039... | 2^-8 |
| I | Integer | 16 | -32768 -> 32767 | 1 |
| 2I | Double Int | 17 | -65536 -> 65534 | 2 |
| CI | Cyclic Int | 16 | -32768 -> 32767, Loops | 1 |
| U | Unsigned Int | 16 | 0 -> 65535 | 1 |
| D | Double Prec Int | 32 | -2147483648 -> 2147483647 | 1 |
| L | Low Word of Double | 16 | --- | --- |
| H | High Word of Double | 16 | --- | --- |
| D2 | Double Prec Half Int | 32 | -1073741824 -> 1073741823 | 2^-1 |
| L2 | Low Word of Double Half | 16 | --- | --- |
| H2 | High Word of Double Half | 16 | --- | --- |
| M | Float Coefficient | 16 | -1.0 -> 0.999969... | 1 |
| C | Float Exponent | 16 | -32768 -> 32767 | 1 |

Commands:

Caption text

| Name | Opcode (Hex) | Inputs (In Order) | Outputs (In Order) | Equation | Command Cycles | Input Cycles | Output Cycles | Description |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 16 bit Multiply | $00 | T/I, T/I | T/H2 | I1 \* I2 = O1 | 6 | 12, 4 | 4 | Multiplies two Values. Product is Rounded to <= 15 bits |
| Float Inverse | $10 | M, C | M, C | 1 / I1 \* 2^I2 = O1 \* 2^O2 | 6 | 12, 73 | 2, 4 | Calculate the inverse of a Floating Point number |
| Triangle | $04 | A, T/I | T/I, T/I | O2 = I2 \* cos(I1), O1 = I2 \* sin(I1) | 6 | 12, 24 | 3, 4 | Calculate the Sine of an Angle (I1) and Radius (I2) and the Product of the Cosine and Radius |
| Radius | $08 | I, I, I | L2, H2 | I1² + I2² + I3² = Ox | 6 | 14, 4, 4 | 2, 4 | Caculate the Vector Size |
| Range | $18 | T/I, T/I, T/I, T/I | T/H2 | I1² + I2² + I3² - I4² = O | 6 | 12, 4, 4, 8 | 4 | Subtract the Square of the Specified Range from the Vector Size |
| Distance | $28 | I/T, I/T, I/T | I/T | sqrt(I1² + I2² + I3²) = O | 6 | 15, 4, 127 | 4 | Calculate the Vector SIze (Abs) |
| Rotate | $0C | A , I , I | I , I | (I2, I3) [cos(I1) - sin(I1), sin(I1) cos(I1) = O1, O2 | 6 | 12, 3, 37 | 2, 4 | Calculate XY Coordinate (O1 & O2) after rotating XY (I2 & I3) with I1 (Z Axis, Counterclockwise) |
| Polar | $1C | A, A, A, I, I, I | I, I, I | (I4, I5, I6) \* [cos(I3) 0 sin(I3), 0 1 0, -sin(I3) 0 cos(I3)] \* [1 0 0, 0 cos(I2) -sin(I2), 0 sin(I2) cos(I2] \* [cos(I1) -sin(I1) 0, sin(I1) cos(I1) 0, 0 0 1] = (O1, O2, O3) | 6 | 13, 3, 2, 2, 2, 107 | 6, 2, 4 | 3D Rotation. Coord In: XYZ (I4, I5, I6) Angle In: XYZ (I1, I2, I3) Coord Out: XYZ (O1, O2, O3) |
