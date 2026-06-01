---
title: "OAM layout"
source_url: "https://snes.nesdev.org/wiki/OAM_layout"
pageid: 60
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

[[OAM]] is a total of 544 bytes in size, consisting of a 512 byte low table and a 32 byte high table.
It holds the properties of up to all 128 sprites.

Each of these cells represents one byte. The low table spreads information on each sprite across 4 bytes, and the high table packs information on 4 sprites into each byte.

```
--+------------------+------------------+------------------+------------------------------------------------+
L | Sprite #0        | Sprite #0        | Sprite #0        | Sprite #0                                      |
O | x pos (low bits) | y pos            | tile (low bits)  | flip v/h, priority, palette, high bit of tile  |
W |                  |                  |                  | (2 bits)  (2 bits)  (3 bits)                   |
  +------------------+------------------+------------------+------------------------------------------------+
T | Sprite #1        | Sprite #1        | Sprite #1        | Sprite #1                                      |
A | x pos (low bits) | y pos            | tile (low bits)  | flip v/h, priority, palette, high bit of tile  |
B +------------------+------------------+------------------+------------------------------------------------+
L | Sprite #2        | Sprite #2        | Sprite #2        | Sprite #2                                      |
E | x pos (low bits) | y pos            | tile (low bits)  | flip v/h, priority, palette, high bit of tile  |
  +------------------+------------------+------------------+------------------------------------------------+
  |                  |                  |                  |                                                |
  |       ....       |    ...           |      ...         |                      ...                       |
  |                  |                  |                  |                                                |
  +------------------+------------------+------------------+------------------------------------------------+
  | Sprite #127      | Sprite #127      | Sprite #127      | Sprite #127                                    |
  | x pos (low bits) | y pos            | tile (low bits)  | flip v/h, priority, palette, high bit of tile  |
--+------------------+------------------+------------------+------------------------------------------------+
H | Sprites #0-3     | Sprites #4-7     | Sprites #8-11    | Sprites #12-15                                 |
I |                  |                  |                  |                                                |
G |                  |                  |                  |                                                |
H | size select bits | size select bits | size select bits | size select bits                               |
  |  |  |  |  |      |  |  |  |  |      |  |  |  |  |      |  |  |  |  |                                    |
  |  v  v  v  v      |  v  v  v  v      |  v  v  v  v      |  v  v  v  v                                    |
T | +--+--+--+--+    | +--+--+--+--+    | +--+--+--+--+    | +--+--+--+--+                                  |
A | |s |s |s |s |    | |s |s |s |s |    | |s |s |s |s |    | |s |s |s |s |                                  |
B | |#4|#3|#2|#1|    | |#4|#3|#2|#1|    | |#4|#3|#2|#1|    | |#4|#3|#2|#1|                                  |
L | | x| x| x| x|    | | x| x| x| x|    | | x| x| x| x|    | | x| x| x| x|                                  |
E | +--+--+--+--+    | +--+--+--+--+    | +--+--+--+--+    | +--+--+--+--+                                  |
  |   ^  ^  ^  ^     |   ^  ^  ^  ^     |   ^  ^  ^  ^     |   ^  ^  ^  ^                                   |
  |   |  |  |  |     |   |  |  |  |     |   |  |  |  |     |   |  |  |  |                                   |
  |   hi x bits      |   hi x bits      |   hi x bits      |   hi x bits                                    |
  +------------------+------------------+------------------+------------------------------------------------+
  |                  |                  |                  |                                                |
  |       ....       |    ...           |      ...         |                      ...                       |
  |                  |                  |                  |                                                |
  +------------------+------------------+------------------+------------------------------------------------+
  | Sprites #112-115 | Sprites #116-119 | Sprites #120-123 | Sprites #124-127                               |
  |                  |                  |                  |                                                |
  |                  |                  |                  |                                                |
  | size select bits | size select bits | size select bits | size select bits                               |
  |  |  |  |  |      |  |  |  |  |      |  |  |  |  |      |  |  |  |  |                                    |
  |  v  v  v  v      |  v  v  v  v      |  v  v  v  v      |  v  v  v  v                                    |
  | +--+--+--+--+    | +--+--+--+--+    | +--+--+--+--+    | +--+--+--+--+                                  |
  | |s |s |s |s |    | |s |s |s |s |    | |s |s |s |s |    | |s |s |s |s |                                  |
  | |#4|#3|#2|#1|    | |#4|#3|#2|#1|    | |#4|#3|#2|#1|    | |#4|#3|#2|#1|                                  |
  | | x| x| x| x|    | | x| x| x| x|    | | x| x| x| x|    | | x| x| x| x|                                  |
  | +--+--+--+--+    | +--+--+--+--+    | +--+--+--+--+    | +--+--+--+--+                                  |
  |   ^  ^  ^  ^     |   ^  ^  ^  ^     |   ^  ^  ^  ^     |   ^  ^  ^  ^                                   |
  |   |  |  |  |     |   |  |  |  |     |   |  |  |  |     |   |  |  |  |                                   |
  |   hi x bits      |   hi x bits      |   hi x bits      |   hi x bits                                    |
  +------------------+------------------+------------------+------------------------------------------------+
```
