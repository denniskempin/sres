---
title: "Scrolling a large map"
source_url: "https://snes.nesdev.org/wiki/Scrolling_a_large_map"
pageid: 34
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

A lot of games have their action take place in an area that's too big to fit on the screen at once. To support this, SNES tilemaps can be bigger than the screen, and a programmer can use [scrolling](https://snes.nesdev.org/w/index.php?title=Scrolling&action=edit&redlink=1 "Scrolling (page does not exist)") to choose which section of it should be displayed. However, the biggest available sizes are still too small for most games' needs.

To get around this limitation, a game can simulate an infinitely large tilemap. This involves determining the scrolling direction and continually writing new data ahead of what the player can currently see. The reason this works is because tilemaps wrap around - attempting to display data past the end of one will just loop back to the left or top of the map.

[![](https://snes.nesdev.org/w/images/snes/b/bf/NTS_scrolling_seam.gif)](https://snes.nesdev.org/wiki/File:NTS_scrolling_seam.gif)

A game might want to do something like this (repeated for each axis if the game scrolls both horizontally and vertically):

- Make a copy of the current scroll position
- Calculate the new scroll position
- Compare the previous scroll position to the new one to determine if the tilemap needs to be updated
  - Something like `(Old ^ New) & TileSize` is an easy way to check if scrolling has passed a tile boundary
- If an update is required, check if the new or old position is greater, to determine the direction
- Pick a column/row on the tilemap that should be updated, using the direction and the scroll position
- Calculate the address of that row/column within video RAM, as well as the address of that row/column within the level data
- Fill a buffer with tile data in a game-specific way
- Copy the buffer into video RAM during the next vblank period

The [[PPU registers#VMAIN|VMAIN]] register's "increment by 32 words" mode helps with writing columns of tile data
