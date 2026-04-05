# SNES Cart Satellaview Expansion Data (at end of Directory Packets)

#### Directory (Software Channel 1.1.0.6)

Loaded to 7FC000h and then copied to 7EC000h. Size (max) 16Kbytes.

```text
  00h   1   Directory ID (compared to Directory ID in Town Status packet)
  01h   1   Number of Folders (Buildings, People, or hidden folders)
  02h   3   Unknown/unused
  05h   ..  Folders (and File) entries (see previous chapter) ;\together
  ..    1   Number of Expansion Data Entries (00h=none)       ; max 3FFBh
  ..    ..  Expansion Data Entries                            ;/bytes
```

#### Expansion Data Entry Format (after folder/file area)

```text
  00h   1   Flags (bit0=1=Invalid) (bit1-7=unknown/unused)
  01h   1   Unknown/unused
  02h   2   Length (N) (16bit) (this one is BIG-ENDIAN)
  04h   N   Expansion Chunk(s) (all values in chunks are LITTLE-ENDIAN)
```

For for some reason, there may be more than one of these entries, but the BIOS does use only the first entry with [00h].Bit0=0=Valid (any further entries are simply ignored).

#### Chunk 00h (End)

```text
  00h        1   Chunk ID (00h)
```

Ends (any further following chunks are ignored). Chunk 00h should be probably always attached at the end of the chunk list (although it can be omitted in some cases, eg. after Chunk 02h).

#### Chunk 01h (Custom Building)

All values in this chunk are LITTLE-ENDIAN.

```text
  00h        1   Chunk ID (01h)
  01h        2   Chunk Length (73h+L1..L5) (LITTLE-ENDIAN)
  03h        11h Message Box Headline (max 16 chars, terminated by 00h)      ;\
  14h        20h BG Palette 5 (copied to WRAM:7E20A0h/CGRAM:50h)  (16 words) ;
  34h        38h BG Data (copied to WRAM:7E4C8Ch/BG1 Map[06h,0Dh])(4x7 words);/
  6Ch        2   Length of BG Animation Data (L1)                            ;\
  6Eh        L1  BG Animation Data (for Custom Building) (see below)         ;/
  6Eh+L1     2   Tile Length (L2) (MUST be nonzero) (zero would be 64Kbytes) ;\
                 BUG: Below Data may not start at WRAM addr 7Exx00h/7Exx01h  ;
                 (if so, data is accidently read from address-100h)          ;
  70h+L1     L2  Tile Data (DMAed to VRAM Word Addr 4900h) (byte addr 9200h) ;/
  70h+L1+L2  2   Length (L3) (should be even, data is copied in 16bit units) ;\
  72h+L1+L2  L3  Cell to Tile Xlat (copied to 7E4080h, BUG:copies L3+2 bytes);/
  72h+L1..L3 2   Length (L4) (MUST be even, MUST be nonzero, max 30h)        ;\
  74h+L1..L3 L4  Cell Solid/Priority List (copied to 7E45D0h,copies L4 bytes);/
  74h+L1..L4 2   Unknown/unused, probably Length (L5)                        ;\
  76h+L1..L4 L5  Door Location(s) (byte-pairs: xloc,yloc, terminated FFh,FFh);/
                 Door Locations work only if BG1 cells also have bit15 set!
                 Bit15 must be set IN FRONT of the door, this is effectively
                 reducing the building size from 4x7 to 4x6 cells.
                 Note: Animated BG cells are FORCEFULLY having bit15 cleared!
```

This chunk may be followed by Chunk 02h. If so, it processes Chunk 02h (and ends thereafter). Otherwise it ends immediately (ie. when the following Chunk has ID=00h) (or also on any "garbage" ID other than 02h).

#### Chunk 02h (Custom Persons)

```text
  00h        1   Chunk ID (02h)
  01h        2   Chunk Length (N) (LITTLE-ENDIAN) (N may be even,odd,zero)
  03h        N   Data (copied to 7F0000h) (N bytes) (max 0A00h bytes or so)
                  00h 4  7F0000h  Person 2Ch - Token Interpreter Entrypoint
                  04h 4  7F0004h  Person 2Dh - Token Interpreter Entrypoint
                  08h .. 7F0008h  General Purpose (Further Tokens and Data)
```

Copies the Data, and ends (any further following chunks are ignored). If Person 2Ch/2Dh are enabled in the Town Status packet, then the person thread(s) are created with above entrypoint(s). The threads may then install whatever OBJs (for examples, see the table at 99DAECh, which contains initial X/Y-coordinates and Entrypoints for Person 00h..3Fh).

#### Chunk 03h..FFh (Ignored/Reserved for future)

```text
  00h        1   Chunk ID (03h..FFh)
  01h        2   Chunk Length (N) (LITTLE-ENDIAN)
  03h        N   Data (ignored/skipped)
```

Skips the data, and then goes on processing the following chunk.

#### BG Animation Data (within Chunk 01h) (for Custom Building)

```text
  00h     2   Base.xloc (0..47) (FFFFh=No Animation Data)
  02h     2   Base.yloc (0..47)
  04h     X*4 Group(s) of 2 words (offset_to_frame_data,duration_in_60hz_units)
  04h+X*4 2   End (FFFEh=Loop/Repeat animation, FFFFh=Bugged/One-shot animat.)
  06h+X*4 ..  BG Animation Frame Data Block(s) (see below)
  ..      0-2 Padding (to avoid 100h-byte boundary BUG in L2)
```

Xloc/Yloc should be usually X=0006h,Y=000Dh (the location of the Custom Building, at 7E4C8Ch) (although one can mis-use other xloc/yloc values to animate completely different map locations; this works only if the Custom Building's folder contains files/items).

BUG: The one-shot animation is applied ONLY to map cells that are INITIALLY visible (ie. according to BG scroll offsets at time when entering the town).

#### BG Animation Frame Data Block(s)

```text
  00h     Y*8 Group(s) of 4 words (xloc, yloc, bg1_cell, bg2_cell)
  00h+Y*8 2   End of Frame List (8000h) (ie. xloc=8000h=end)
```

NOTE: offset_to_frame_data is based at [94] (whereas, [94] points to the location after Chunk 01h ID/Length, ie. to the base address of Chunk 01h plus 3).

If the animation goes forwards/backwards, one may use the same offsets for both passes.

The Custom Bulding has 4x7 cells in non-animated form (so, usually one should use xloc=0..3, yloc=0..6). Cells can be FFFFh=Don't change (usually one would change only BG1 foreground cells, and set BG2 background cells to FFFFh). For animated BG1 cells, Bit10-15 are stripped for some stupid reason (in result, animated BG1 cells cannot be flagged as Doors via Bit15).

#### BG Cell to Tile Translation

Each 16x16 pixel Cell consists of four 8x8 Tiles. The Translation table contains tiles arranged as Upper-Left, Lower-Left, Upper-Right, Lower-Right.

The 16bit table entries are 10bit BG tile numbers, plus BG attributes (eg. xflip).

#### Building/Map Notes

The PPU runs in BG Mode 1 (BG1/BG2=16-color, BG3=4-color). VRAM used as so:

```text
  BG1 64x32 map at VRAM:0000h-07FFh, 8x8 tiles at VRAM:1000h-4FFFh (foreground)
  BG2 64x32 map at VRAM:0800h-0FFFh, 8x8 tiles at VRAM:1000h-4FFFh (background)
  BG3 32x32 map at VRAM:5000h-53FFh, 8x8 tiles at VRAM:5000h-6FFFh (menu text)
  OBJ 8x8 and 16x16 tiles at VRAM:6000h-7FFFh (without gap)        (people)
  Custom BG1/BG2 Tiles are at VRAM:4900h-xxxxh (BG.Tile No 390h-xxxh)
  Custom OBJ Tiles at VRAM:7C00h-xxxxh (OBJ.Tile No 1C0h..xxxh)
```

The PPU BG Palettes are:

```text
  BG.PAL0 Four 4-Color Palettes (for Y-Button BG3 Menu)
  BG.PAL1 Four 4-Color Palettes (for Y-Button BG3 Menu)
  BG.PAL2 Buildings
  BG.PAL3 Buildings
  BG.PAL4 Buildings
  BG.PAL5 Custom Palette
  BG.PAL6 Landscape (Trees, Phone Booth)         (colors changing per season)
  BG.PAL7 Landscape (Lawn, Streets, Water, Sky)  (colors changing per season)
```

The town map is 48x48 cells of 16x16 pix each (whole map=768x768pix).

```text
  Custom BG1/BG2 Cells are at WRAM:7E4080h-7E41FFh (Cell No 3D0h-3FFh)
  Custom Cell Solid/Priority...
```
