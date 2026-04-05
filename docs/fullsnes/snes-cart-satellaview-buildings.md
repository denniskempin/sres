# SNES Cart Satellaview Buildings

#### Home Building (Starting Point)

This building can be entered at any time, the four japanese options are:

```text
  1) Load File from FLASH Card
  2) Delete File from FLASH Card
  3) Download File (only files that are "Available at Home") (max 32 files)
  4) Delete Settings in SRAM
```

#### Buildings

The buildings are numbered roughly anti-clockwise from 00h..1Fh, starting at lower-left of the town map:

```text
  00h Robot Skyscraper (lower-left)
  01h News Center
  02h Parabol Antenna
  03h Junkfood
  04h Police
  05h Maths +-x/
  06h Beach Shop (Shop for Predefined ROM Items)
  07h Turtle-Arena
  08h C-Skyscaper (Shop for Predefined ROM Items)
  09h Red-Heart Church
  0Ah Red \\\ Factory (upper-right corner)
  0Bh Dracula Gift-Shop
  0Ch Cow-Skull Church
  0Dh Spintop/Abacus (near Maths +-X/)
  0Eh Blank Skyscraper (near Parabol Antenna)
  0Fh Sign (near Red Factory) (works only ONCE)     (or custom building)
  10h Greek Buddah Temple (upper-end)
  11h Bigger Neighbor's Building
  12h Smaller Neighbor's Building (unknown how to get in there)
  13h Phone Booth (can be entered only with Telephone Card item)
  14h Sewerage (near Spintop) (Shop for Predefined ROM Items)
  15h Unused
  16h Unused ;\these Building-Folders MUST not exist (else BIOS randomly
  17h Unused ;/crashes, accidently trying to animate Building number "44h/3")
  18h Special Location without folder: Player's Home
  19h Special Location without folder: Hydrant (near police)
  1Ah Special Location without folder: Talking Tree (near C-Skyscraper)
  1Bh Special Location without folder: Fountain (or Fountain Replacement)
  1Ch Special Location without folder: Beach Toilets (Railway Station)
  1Dh Special Location without folder: Ocean's Shore
  1Eh Special Location without folder: Unused
  1Fh Special Location without folder: Unused
  20h-3Fh Building-Folders with these IDs do destroy memory (don't use!)
```

Buildings can be entered only if there is a corresponding folder (in the directory), and only if the folder contains at least one file or item (for the three pre-defined Shops it works also if the folder is empty).
