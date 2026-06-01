---
title: "Uncommon graphics mode games"
source_url: "https://snes.nesdev.org/wiki/Uncommon_graphics_mode_games"
pageid: 25
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

A list of games that use less common graphics modes.

## BG Mode 0

[[Backgrounds#Mode 0|BG Mode 0]] is somewhat uncommon, but provides 4 independent layers of 2bpp graphics.

| Game | Notes |
| --- | --- |
| **Dragon Quest V** | Used for the title screen that shows in the new game into sequence |
| **Final Fantasy IV** | Used for the menu screen and the saved game load screen |
| **Final Fantasy V** | Used for the menu screen and the saved game load screen |
| **Kingyo Chuuihou! Tobidase! Game Gakuen** | Used during the Jaleco logo boot screen animation. |
| **The Simpsons: Bart's Nightmare** | Used during gameplay. |
| **Super Mario Kart** | For the layered "Choose your driver...." screen with separately scrolling text and ground. |
| **Star Ocean** | For menus screen (including saved game menus, shop menus, etc.) |
| **Tales of Phantasia** | For menus screen (including saved game menus, shop menus, etc.) |
| **Yoshi's Island** | The title screen uses mode 0 for the upper portion of its title screen. 6-8 "King Bowser's Castle" has a room for Kamek with 4 layers of parallaxing pillars: [video](https://youtu.be/U8btNneN8ew?t=24458) |

## BG Mode 3 8bpp

[[Backgrounds#Mode 3|BG Mode 3]] is uncommon, but provides [[Tiles#8bpp|8bpp]] graphics. Mode 3 [[Direct color]] cases are [listed below](#Direct_Color) in their own section instead.

| Game | Notes |
| --- | --- |
| **Donkey Kong Country** | Used for a palette animation drawing the green lines of the Rareware logo at boot. |
| **Kingyo Chuuihou! Tobidase! Game Gakuen** | Used extensively throughout the game for several CG scenes. |
| **Ms. Pac-Man** | Used for the title screen. |
| **Secret of Mana** | Used for the title screen. (Image appears to use a lossy JPEG-like compression technique.) |
| **Sim City 2000** | Used to display the city itself - the interface uses the 4bpp layer. |
| **Toy Story** | Used for many high-color screens throughout. |
| **Yam Yam** | Used for the title screen. |
| **Zoop** | A single 8bpp layer is used for gameplay. |

## BG Mode 4 8bpp

[[Backgrounds#Mode 4|BG Mode 4]] provides [[Tiles#8bpp|8bpp]] graphics on BG1 with [[Offset-per-tile]].

| Game | Notes |
| --- | --- |
| **Bust-a-Move** / **Puzzle Bobble** | 8bpp graphics are needed so that all combinations of adjacent bubble colors can be accommodated on the tilemap. Also uses vertical Offset-Per-Tile to shift the playfield. |
| **Rock'n'Roll Racing** | Used on the "buy equipment" screen (no Offset-per-Tile) |

## BG Mode 7 EXTBG

While [[Backgrounds#Mode 7|BG Mode 7]] was very popular, [[Backgrounds#EXTBG|BG Mode 7 EXTBG]] was not very common.

| Game | Notes |
| --- | --- |
| **Contra 3** | Used on the second level, allows the player to walk under bridges. |
| **Super Ghouls 'n Ghosts** | Used to put the level over the player's feet on a rotating level. |
| **Super Turrican 2** | Used for a boss fight. |
| **Tiny Toon's Adventures** | Used on the title screen. |
| **[extbgtest](https://forums.nesdev.org/viewtopic.php?t=24081)** | Test ROM for mode 7 EXTBG. (BG1 vs. BG2, Indexed vs. Direct Color.) |

## Direct Color

See: [[Direct color]]

| Game | Notes |
| --- | --- |
| **Actraiser 2** | The [[Mode 7]] overworld map uses Direct Color. |
| **Aerobiz** | The title screen uses mode 7 Direct Color. |
| **Romance of the Three Kingdoms II** | The title screen and introduction sequence use mode 7 Direct Color. |
| **Secret of Mana** | The spherical world map uses mode 7 Direct Color. The flat world map uses mode 3 Direct Color. While riding Flammie, press Start for the spherical map, then press R to switch to the flat map. |

## High-Resolution and Interlacing

PPU register $2133 [[PPU registers#SETINI|SETINI]] can enable hi-res mode, allowing a 512-pixel horizontal resolution.

Outside of BG Mode 5, this is usually used as a way to do a 50% blend of the main and sub screens (a.k.a. "pseudo hi-res"). Starting from 0, it causes every even column to display the sub screen and every odd column to display the main screen. The sub screen is the left column in each pixel pair. Because composite television signals had limited bandwidth, this could result in a smooth horizontal blending of the columns, but through S-Video or RGB the vertical stripes might be clearly visible.

In [[Mode 5|BG Mode 5]], hi-res is automatically forced, and the main and sub screen are automatically overridden with alternating columns of the BG layers, allowing the tilemaps to be used at double-width resolution.

PPU register $2133 [[PPU registers#SETINI|SETINI]] can also enable interlacing, causing the scanlines of every second frame (field) to be offset downward by half a line. In mode 5 this allows a double-height tilemap resolution, but at half the frame rate, and with the other visual compromises of interlacing.

Because of the visual complexity of Japanese text characters, hi-res was often used to improve their detail for legibility. On high-contrast shapes like text, this is beneficial even with the limited bandwidth of composite output.

| Game | Notes |
| --- | --- |
| **The Atlas: Renaissance Voyager** | Mode 5 hi-res used for a status area at the bottom of the map screen. |
| **Bishoujo Janshi Suchi-Pie** | Mode 5 and interlacing for some high resolution graphic screens when entering a location. |
| **Breath of Fire II** | Uses pseudo hi-res in the intro town (Gate) for shading the party under the trees in the back area on the way to Valerie |
| **Chrono Trigger** | Uses interlacing in the cockpit view when crashing the Epoch ship into Lavos (modes 7 and 1). This may have been used as an intentional jitter. After the Epoch gains its wings, but before Lavos' head is defeated, send the Epoch to 1999 AD. ([video](https://www.youtube.com/watch?v=Yv0X4kAYN5Y)) ([Save #12 slot 2](https://fantasyanime.com/squaresoft/ctsaves-srm.htm)) |
| **Crayon Shin-Chan: Arashi o Yobu Enji** | Stage 2 uses pseudo hi-res effect for a grey mist. Can be reached from the title screen by waiting for the second attract sequence. |
| **Donkey Kong Country** | After the Rareware logo shrinks and moves to the bottom right corner, it uses mode 5 to make that small logo higher resolution. |
| **Jurassic Park** | The HUD overlay and a notification box during gameplay use pseudo hi-res for a transparency effect. |
| **Kirby's Dream Land 3** | Uses pseudo hi-res for a foreground transparency effect. |
| **Maka Maka** | Has interlacing enabled in-game (but not on the title screen). |
| **Porky Pig's Haunted Holiday** | Mode 5 is used in an introductory room for many levels, giving a high resolution scene one screen wide. |
| **Power Drive** | Mode 5 and interlacing used during introduction for high resolution U.S. Gold logo and text screens. |
| **Radical Psycho Machine Racing** (a.k.a. **RPM Racing**) | Mode 5 and interlacing for high resolution graphics throughout the game. The Japanese release replaced the high-resolution gameplay with low-resolution mode instead. |
| **Street Combat** / **Ranma ½: Chounai Gekitou Hen** | Has interlacing enabled, likely by mistake. Not used for any higher vertical resolution content. |
| **[PPU Bus Activity Demo](https://forums.nesdev.org/viewtopic.php?p=174494)** | Demo by lidnariq which includes mode 6, among other things. |
| **[Two Ship Demo](https://forums.nesdev.org/viewtopic.php?p=279303)** | Demo by rainwarrior comparing mode 5 + interlacing against mode 1 graphics. |

Hi-res Japanese text:

| Game | Notes |
| --- | --- |
| **Dark Law: Meaning of Death** | Mode 5 for text boxes, menu screens and status bar during gameplay. |
| **Desert Fighter** / **Air Strike Patrol** | Mode 5 and interlacing for high resolution mission briefing text. |
| **Doukyuusei 2** | Mode 5 for text boxes and menu screens. |
| **Dragon Knight 4** | Mode 5 for text boxes and menu screens. |
| **Marvelous: Mouhitotsu no Takarajima** | Mode 5 for text boxes and menu screens. |
| **Moryou Senki Madara 2** | Mode 5 for text boxes and menu screens. |
| **Rudra no Hihou** | Mode 5 for text boxes and menu screens. |
| **Secret of Mana** | Mode 5 used for menu screen |
| **Seiken Densetsu 3 (Trials of Mana)** | Mode 5 for text boxes and menu screens. |
| **Shinseiki Odysselya II** | Mode 5 for text boxes. |
| **Stable Star: Kyuusha Monogatari** | Mode 5 and interlacing for text on the individual horse stats screen. |
| **Syvalion** | Mode 5 and interlacing for text screens at the beginning of the game. |
| **Tokimeki Memorial: Densetsu no Ki no Shita de** | Mode 5 for text boxes and menu screens. |

Links:

- [Forum thread](https://forums.nesdev.org/viewtopic.php?p=279118#p279118) - CRT photos of various hi-res effects.

## Offset-Per-Tile: Horizontal

The horizontal version of [[Offset-per-tile]] is especially rare, since scanline horizontal scroll changes can already do a smooth horizontal offset, and this hardware feature only provides a coarse one.

| Game | Notes |
| --- | --- |
| **Chrono Trigger** | During the introduction, this is used for a "shimmering" effect on the Black Omen ship. ([video](https://youtu.be/ZSn24qQ1vAQ?t=147)) |
| **Super Genjin 2** | Used for a screen transition effect on the title screen, when the intro animation isn't skipped. Uses mode 4. |
| **Super Mario All-Stars** | Used in Super Mario Bros 2, for a screen transition between title screen and character select. |

## Offset-Per-Tile: Vertical

See: [[Offset-per-tile]]

| Game | Notes |
| --- | --- |
| **Aladdin** | Used on the final boss fight. |
| **Axelay** | Used for large vertically moving obstacles on the second level. |
| **Battletoads in Battlemaniacs** | Used for a wavy effect on a flag in the game's intro. |
| **Bust-a-Move** / **Puzzle Bobble** | Used to make the playfield shift downwards without affecting the fixed frame at the sides. This game also uses mode 4. |
| **Chrono Trigger** | Used to apply a vertical waving effect on the word "Trigger" at the title screen.  Used for scrolling text in the menu screen once there is more than 3 party members |
| **Dragon Quest V** | Used to make the ship move independently from the land in the beginning. |
| **GT Racing** | Uses mode 2 for a waving flag on the title screen. |
| **Kingyo Chuuihou! Tobidase! Game Gakuen** | Uses mode 2 for the waving flag during the copyright screen, which continues to be used into the game's intro and title screen. |
| **Prehistorik Man** | Used on stage 21 (Icebergs) to simulate rotation on the ice platforms. |
| **Star Fox** | Used for the slight rotation effect on the background behind the SuperFX graphics. |
| **Super Double Dragon** | Used for the elevator in the second area of the first level. |
| **Super Turrican 2** | Used in the fourth area of the first level. |
| **Tetris Attack** | Used for independently shifting columns of gameplay. |
| **Timecop** | Used for a wobbling effect on the time machine during the introduction. |
| **Yoshi's Island** | Used for the 1-7 "Touch Fuzzy Get Dizzy" effect, and moving platforms in 6-4 "Tap-Tap The Red Nose's Fort". |

## Overscan (239 lines)

PPU register $2133 [[PPU registers#SETINI|SETINI]] can choose either 224 or 239 lines of visible picture. Most games used 224, allowing a significantly longer VBLANK period, and the extra lines weren't normally visible on contemporary NTSC televisions.

| Game | Notes |
| --- | --- |
| **Dragon Quest I & II** | NTSC (Japan). |
| **Dragon Quest V** | NTSC (Japan). |
| **Rendering Ranger R2** | NTSC (Japan), often manually blanks several lines at the bottom. Only rarely uses the manual blank time for PPU uploads (e.g. rotating ship at end of Stage 2). |
| **[SNES Test Program](https://tcrf.net/SNES_Test_Program)** | Immediately switches to overscan mode, even on NTSC consoles. |
| **Super Mario All-Stars** | PAL version only. |
| **Super Mario Kart** | PAL version only, but with some "manual" letterboxing. |
| **Super Mario World** | The launch PAL release did not use overscan, but a later PAL revision did. |
| **Super Tetris 2 + Bombliss** | NTSC (Japan). |
| **Super Tetris 3** | NTSC (Japan). |
| **Tetris & Dr. Mario** | Both NTSC and PAL versions used the overscan area. |
| **Tom & Jerry** | NTSC and PAL. |
| **Yoshi's Cookie** | NTSC and PAL. |

## See Also

- [[Tricky-to-emulate games]]
- [[Emulator tests]]
