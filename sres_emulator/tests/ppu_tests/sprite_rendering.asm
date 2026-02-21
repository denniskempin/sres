// Sprite rendering test ROM
//
// Sets up sprites via DMA+CPU writes to VRAM/CGRAM/OAM and tests:
//   Sprite 0: basic 8x8 (palette 0 = red diagonal, x=8,   y=10)
//   Sprite 1: 8x8 alt palette (palette 1 = white,  x=28,  y=10)
//   Sprite 2: 8x8 hflip                            (x=48,  y=10)
//   Sprite 3: 8x8 vflip                            (x=68,  y=10)
//   Sprite 4: 16x16 large (4 quadrants)            (x=88,  y=10)
//   Sprite 5: 8x8 nametable-1, palette 2 (cyan)    (x=128, y=10)
//   Sprite 6: 8x8 priority 2                       (x=150, y=10)

output "sprite_rendering.sfc", create

include "lib/base.asm"
include "lib/snes_gfx.inc"

Start:
    // -------------------------------------------------------------------
    // CPU initialization
    // -------------------------------------------------------------------
    sei             // Disable interrupts
    clc
    xce             // Switch to native (65816) mode
    rep #$10        // 16-bit X/Y
    sep #$20        // 8-bit A

    // Force blanking: full brightness for later, display off now
    lda.b #$8F
    sta.w REG_INIDISP

    // -------------------------------------------------------------------
    // OBJSEL: size=0 (8x8/16x16), nametable-0 at word $0000,
    //         nametable-1 at word $1000 (NN=0, so +1<<12 = $1000)
    // -------------------------------------------------------------------
    lda.b #$00
    sta.w REG_OBSEL

    // -------------------------------------------------------------------
    // VRAM: load tile data
    // Each 4bpp tile = 32 bytes (16 words: 8 planes0+1, 8 planes2+3)
    // VRAM word addr = byte addr >> 1
    // -------------------------------------------------------------------
    // Tile 0: diagonal (color 1) at byte addr $0000 (word $0000)
    LoadVRAM(TileData_Diagonal, $0000, 32, 0)
    // Tile 1: solid color 2 at byte addr $0020 (word $0010)
    LoadVRAM(TileData_Solid2,   $0020, 32, 0)
    // Tile 16: solid color 3 at byte addr $0200 (word $0100) – 16x16 bottom-left quadrant
    LoadVRAM(TileData_Solid3,   $0200, 32, 0)
    // Tile 17: solid color 4 at byte addr $0220 (word $0110) – 16x16 bottom-right quadrant
    LoadVRAM(TileData_Solid4,   $0220, 32, 0)
    // NT1 Tile 0: diagonal at byte addr $2000 (word $1000)
    LoadVRAM(TileData_Diagonal, $2000, 32, 0)

    // -------------------------------------------------------------------
    // CGRAM: load sprite palettes (3 palettes × 16 colors × 2 bytes)
    // Sprite palettes start at CGRAM index 128
    // -------------------------------------------------------------------
    LoadPAL(PaletteData, 128, 96, 0)

    // -------------------------------------------------------------------
    // OAM: DMA the pre-built table (544 bytes) into OAM
    //   Bytes   0..511 – main OAM table (128 sprites × 4 bytes)
    //   Bytes 512..543 – high table    (128 sprites × 2 bits)
    // -------------------------------------------------------------------
    stz.w REG_OAMADDL   // OAMADD = 0 (start of main table)
    stz.w REG_OAMADDH

    lda.b #$02          // DMA mode: write byte, fixed dst, increment src
    sta.w REG_DMAP0
    lda.b #$04          // B-bus: $2104 = OAMDATA
    sta.w REG_BBAD0
    ldx.w #OAMData
    stx.w REG_A1T0L
    lda.b #(OAMData >> 16)
    sta.w REG_A1B0
    ldx.w #544          // 512 main + 32 high table
    stx.w REG_DAS0L
    lda.b #$01
    sta.w REG_MDMAEN

    // -------------------------------------------------------------------
    // PPU registers: enable OBJ on main screen
    // -------------------------------------------------------------------
    lda.b #$10          // TM: bit 4 = OBJ enable
    sta.w REG_TM

    // Disable forced blank (max brightness)
    lda.b #$0F
    sta.w REG_INIDISP

MainLoop:
    jmp MainLoop

// -----------------------------------------------------------------------
// Tile data – 4bpp SNES format
// Layout: 8 words (planes 0+1), then 8 words (planes 2+3)
// Each word is [plane_lo, plane_hi] (little-endian)
// -----------------------------------------------------------------------

// Diagonal band (color 1 = 0b0001):
//   plane0 rows: 0xC0 0xE0 0x70 0x38 0x1C 0x0E 0x07 0x03
//   plane1 (bit1 of color=0): all zeros
//   planes 2+3 (bits 2,3 of color=0): all zeros
TileData_Diagonal:
    db $C0,$00, $E0,$00, $70,$00, $38,$00  // planes 0+1: rows 0-3
    db $1C,$00, $0E,$00, $07,$00, $03,$00  // planes 0+1: rows 4-7
    db $00,$00, $00,$00, $00,$00, $00,$00  // planes 2+3: rows 0-3
    db $00,$00, $00,$00, $00,$00, $00,$00  // planes 2+3: rows 4-7

// Solid color 2 (0b0010): plane1=0xFF, plane0=0, planes2+3=0
TileData_Solid2:
    db $00,$FF, $00,$FF, $00,$FF, $00,$FF  // planes 0+1: rows 0-3
    db $00,$FF, $00,$FF, $00,$FF, $00,$FF  // planes 0+1: rows 4-7
    db $00,$00, $00,$00, $00,$00, $00,$00  // planes 2+3: rows 0-3
    db $00,$00, $00,$00, $00,$00, $00,$00  // planes 2+3: rows 4-7

// Solid color 3 (0b0011): plane0=0xFF, plane1=0xFF, planes2+3=0
TileData_Solid3:
    db $FF,$FF, $FF,$FF, $FF,$FF, $FF,$FF  // planes 0+1: rows 0-3
    db $FF,$FF, $FF,$FF, $FF,$FF, $FF,$FF  // planes 0+1: rows 4-7
    db $00,$00, $00,$00, $00,$00, $00,$00  // planes 2+3: rows 0-3
    db $00,$00, $00,$00, $00,$00, $00,$00  // planes 2+3: rows 4-7

// Solid color 4 (0b0100): plane0=0, plane1=0, plane2=0xFF, plane3=0
TileData_Solid4:
    db $00,$00, $00,$00, $00,$00, $00,$00  // planes 0+1: rows 0-3
    db $00,$00, $00,$00, $00,$00, $00,$00  // planes 0+1: rows 4-7
    db $FF,$00, $FF,$00, $FF,$00, $FF,$00  // planes 2+3: rows 0-3
    db $FF,$00, $FF,$00, $FF,$00, $FF,$00  // planes 2+3: rows 4-7

// -----------------------------------------------------------------------
// Palette data – BGR15 format (little-endian 16-bit words)
// Sprite palettes begin at CGRAM index 128.
// -----------------------------------------------------------------------
PaletteData:
    // OBJ Palette 0 (CGRAM 128-143): transparent, red, green, blue, yellow
    dw $0000, $001F, $03E0, $7C00, $03FF
    dw $0000, $0000, $0000, $0000, $0000
    dw $0000, $0000, $0000, $0000, $0000, $0000
    // OBJ Palette 1 (CGRAM 144-159): transparent, white
    dw $0000, $7FFF
    dw $0000, $0000, $0000, $0000, $0000, $0000
    dw $0000, $0000, $0000, $0000, $0000, $0000
    // OBJ Palette 2 (CGRAM 160-175): transparent, cyan
    dw $0000, $7FE0
    dw $0000, $0000, $0000, $0000, $0000, $0000
    dw $0000, $0000, $0000, $0000, $0000, $0000

// -----------------------------------------------------------------------
// OAM data – 544 bytes
//   Main table (512 bytes): 128 sprites × [X, Y, tile, attr]
//   High table  (32 bytes): 128 sprites × 2 bits each
// -----------------------------------------------------------------------
OAMData:
    // Sprites 0-6: test cases
    db   8, 10, 0, $00  // Sprite 0: x=8,   pal=0, no flip
    db  28, 10, 0, $02  // Sprite 1: x=28,  pal=1 (white)
    db  48, 10, 0, $40  // Sprite 2: x=48,  hflip
    db  68, 10, 0, $80  // Sprite 3: x=68,  vflip
    db  88, 10, 0, $00  // Sprite 4: x=88,  16x16 (size toggled in high table)
    db 128, 10, 0, $05  // Sprite 5: x=128, NT1, pal=2 (cyan)
    db 150, 10, 0, $20  // Sprite 6: x=150, priority=2
    // Sprites 7-127: hidden off-screen (y=224)
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 7-10
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 11-14
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 15-18
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 19-22
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 23-26
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 27-30
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 31-34
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 35-38
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 39-42
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 43-46
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 47-50
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 51-54
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 55-58
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 59-62
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 63-66
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 67-70
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 71-74
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 75-78
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 79-82
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 83-86
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 87-90
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 91-94
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 95-98
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 99-102
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 103-106
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 107-110
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 111-114
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 115-118
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0              // sprites 119-121
    db 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0, 0,$E0,0,0  // sprites 122-125
    db 0,$E0,0,0, 0,$E0,0,0                         // sprites 126-127
    // High table (32 bytes, 4 sprites per byte)
    db $00  // sprites 0-3:   all small, X-high=0
    db $02  // sprites 4-7:   sprite 4 = large (bit 1)
    db $00, $00, $00, $00, $00, $00, $00, $00  // sprites 8-39
    db $00, $00, $00, $00, $00, $00, $00, $00  // sprites 40-71
    db $00, $00, $00, $00, $00, $00, $00, $00  // sprites 72-103
    db $00, $00, $00, $00, $00, $00, $00, $00  // sprites 104-127 (last byte covers sprites 124-127; top 4 bits unused)
