---
title: "Reading and writing PPU memory"
source_url: "https://snes.nesdev.org/wiki/Reading_and_writing_PPU_memory"
pageid: 145
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

## CGRAM

The PPU contains an internal 256 x 15bit memory called [[CGRAM]] that holds the palette data.

The S-CPU can access the CGRAM using the [[PPU registers#CGADD|CGADD]], [[PPU registers#CGDATA|CGDATA]] and [[PPU registers#CGDATAREAD|CGDATAREAD]] registers.

- The S-CPU can only access the CGRAM during [[Timing#Vertical Blank|Vertical Blank]], [[Timing#Horizontal Blank|Horizontal Blank]] or [[PPU registers#INIDISP|Force Blank]].
  - If the CGDATA or CGDATAREAD registers are accessed during active-display the data will be read from or written to the wrong CGRAM address.
- CGDATA is a write-twice register. You must always write to CGDATA an even number of times.
  - The color data is only written to the CGRAM on the second CGDATA write.
- CGDATAREAD is a read-twice register. You should always read from CGDATAREAD an even number of times.
- You should always set the CGRAM word address with CGADD before reading or writing to CGRAM.
  - This will also reset an internal odd/even counter.
  - Mixing CGRAM reads and writes is not recommended.
- Each CGRAM color is 15 bits in size.
  - When writing to CGRAM, bit 15 is ignored
  - When reading CGRAM, bit 15 will be [[Open bus|PPU2 open bus]] and should be masked.

To write to CGRAM, first set the CGRAM word address (ie, palette color index) with an 8-bit write to CGADD. Then preform two 8-bit writes to CGDATA. After the second write to CGDATA the color data will be written to CGRAM and the internal CGRAM word address will be incremented by one. Subsequent colors can be written to CGRAM with two more 8-bit writes to CGDATA.

```
.a8
.i16
// DB access registers
// REQUIRES: h-blank, v-blank or force-blank

    // Set a single CGRAM color at `COLOR_INDEX` to `COLOR_VALUE`

    // Set CGRAM word address (color index)
    lda     #COLOR_INDEX
    sta     CGADD

    // Write low byte
    lda     #.lobyte(COLOR_VALUE)
    sta     CGDATA

    // Write high byte
    lda     #.hibyte(COLOR_VALUE)
    sta     CGDATA
```

```
Variables:
    zpFarPtr - a 3 byte pointer in zero-page.


// Write a block of colors to CGRAM.
//
// INPUT: A = starting color index
// INPUT: X = number of colors to write (MUST BE > 0)
// INPUT: zpFarPtr = palette data
// REQUIRES: Vertical-Blank or Force-Blank.
//           (There is not enough Horizontal-Blank time to run this code)
.a8
.i16
// DB access registers
.proc WriteCgramBlock
    // Set CGRAM word address (color index)
    sta     CGADD

    ldy     #0
    Loop:
        // Write low byte
        lda     [zpFarPtr],y
        sta     CGDATA
        iny

        // Write high byte
        lda     [zpFarPtr],y
        sta     CGDATA
        iny

        dex
        bne     Loop
    rts
.endproc
```

Writing to CGRAM using DMA or HDMA is preformed using the *One register, write twice* transfer pattern (DMAP pattern 2). (See [[HDMA examples#HDMA to CGRAM]] for a HDMA example.)

```
Variables:
    cgramBuffer : uint16[256] = a buffer of 256 colors in RAM


// Transfer a 256 color buffer (`cgramBuffer`) to CGRAM using DMA channel 0
//
// REQUIRES: Vertical-Blank or Force-Blank
// DB access registers
// Uses DMA channel 0
subroutine TransferBufferToCgram:
    // reset CGRAM address
    CGADD = 0


    // DMA parameters: one write-twice register, to PPU
    DMAP0 = 2

    // B-Bus address
    BBAD0 = .lobyte(CGDATA)

    // A-Bus address
    A1T0  = .loword(cgramBuffer)
    A1B0  = .bankbyte(cgramBuffer)

    // Transfer size (SHOULD BE EVEN)
    DAS0  = .sizeof(cgramBuffer)

    // Start DMA transfer on channel 0
    MDMAEN = 1 << 0
```

Reading from CGRAM is preformed with the CGDATAREAD register in a similar manner as CGRAM writes. Bit 15 of the color data is open-bus and should be masked to 0.

```
VARIABLES: zpTmpWord - a temporary uint16 variable in zero-page.

// INPUT: A = color index to read
// OUTPUT: zpTmpWord = color value
// REQUIRES: v-blank or force-blank
.a8
.i16
// DB access registers
.proc ReadCgramColor
    sta     CGADD

    // Read low-byte
    lda     CGDATAREAD
    sta     zpTmpWord

    // Read high-byte
    lda     CGDATAREAD
    // (The MSB is open-bus and should be masked)
    and     #$7f
    sta     zpTmpWord + 1

    rts
.endproc
```

## OAM: Object Attribute Memory

The PPU contains two internal RAM blocks (a 512 byte low-table and a 32 byte hi-table) that form the [[Sprites#OAM|OAM]].

The S-CPU can access the OAM using the [[PPU registers#OAMADD|OAMADD]], [[PPU registers#OAMDATA|OAMDATA]] and [[PPU registers#OAMDATAREAD|OAMDATAREAD]] PPU registers.

- The S-CPU can only access the OAM during [[Timing#Vertical Blank|Vertical Blank]] or [[PPU registers#INIDISP|Force Blank]].
- Writing to OAMADD sets an internal 9 bit OAM word address.
  - The 8th bit of the internal OAM word address (bit 0 of OAMADDH) determines which OAM table accessed.
  - The internal OAM address is reset whenever OAMADDL or OAMADDH is written to.
  - You should always set both OAMADDL and OAMADDH (eg, with a 16 bit write to OAMADD) when setting the OAM word address.
  - You should always write to OAMADD before transferring data to the OAM.
- OAMDATA is a write-twice register when writing to the OAM low-table.
  - When writing to the low-table, the data is only written to the OAM on the second OAMDATA write.
  - When writing to the hi-table, the data is written to the OAM on every OAMDATA write.
  - Despite this, you should always treat OAMDATA as a write-twice register.
- The internal OAM addresses is reset to the last value written to OAMADD when VBlank starts and the screen is enabled (not in force-blank).[[1]](#cite_note-1)
- OAMADD can also enable *OAM priority rotation*.
  - When using *OAM priority rotation*, the first-sprite is updated and may be incremented on every OAMDATA write or OAMDATAREAD read.[[2]](#cite_note-2)
  - If you are using *OAM priority rotation*, you will need to write to OAMADD any after a OAM transfer to reset the first-sprite.

Reading and writing to OAM is the same as writing to CGRAM, except the OAM address register (OAMADD) is 16 bits wide.

It is highly recommended that you create a [[VBlank routine#Buffers|544 byte OAM buffer]] in Work-RAM and only transfer data to the OAM via this buffer during the Vertical Blanking Period. (See [[VBlank routine#OAM buffer example|VBlank routine#OAM\_buffer\_example]] for an example of a DMA transfer from an OAM buffer to OAM.)

## VRAM: Video RAM

The PPU is connected to two external 32K x 8bit SRAM chips, called VRAM (Video RAM).

The PPU accesses the VRAM in one of three modes, depending on context:

- 16 bit VRAM: Both VRAM chips are combined into a single 32K x 16bit (64KB) memory. Used for [[Tiles|tile]] data (2/4/8 bpp), [[Tilemaps|nametable]] data and [[Offset-per-tile]] data.
- Two separate 16K x 8bit VRAM chips[[3]](#cite_note-3): Used by Mode 7. The low-VRAM chip holds the [[Tilemaps#Mode 7|Mode 7 Tilemap]], the high-VRAM chip holds the [[Tiles#Mode 7|Mode 7 tile data]].
- Two separate 32K x 8bit VRAM chips with a shared auto-incrementing address bus: Used by the VMAIN, VMADD, VMDATA and VMDATAREAD [[PPU registers#VRAM]] to allow the S-CPU to access VRAM.

The S-CPU can access VRAM using the [[PPU registers#VMAIN|VMAIN]], [[PPU registers#VMADD|VMADD]], [[PPU registers#VMDATA|VMDATA]], [[PPU registers#VMDATAREAD|VMDATAREAD]] registers.

- The S-CPU can only access the VRAM during [[Timing#Vertical Blank|Vertical Blank]] or [[PPU registers#INIDISP|Force Blank]].
  - If the VMAIN, VMDATA or VMDATAREAD registers are accessed during horizontal-blank or active-display the VRAM will not be read from or written to.
- VMDATA and VMDATAREAD are **not** word registers.
  - VMDATALREAD and VMDATAL will read from or write to the low-byte VRAM chip.
  - VMDATAHREAD and VMDATAH will read from or write to the high-byte VRAM chip.
  - You can perform a 16-bit read from VMDATAREAD or 16-bit write to VMDATA to read/write both VRAM chips at once.
- How the internal VRAM word address is incremented is controlled by the [[PPU registers#VMAIN|VMAIN]] register.
  - You should always write to VMAIN before performing a VRAM transfer, unless you know the exact state of the VMAIN register (ie, immediately following a previous VRAM transfer in the VBlank routine).
  - The *Address increment mode* flag (bit 7) of VMAIN determines if the internal VRAM word address is incremented on low or high byte VRAM access.
    - When *Address increment mode* is 0, the internal VRAM word address increments after writing to VMDATAL or reading from VMDATALREAD.
    - When *Address increment mode* is 1, the internal VRAM word address increments after writing to VMDATAH or reading from VMDATAHREAD.
    - To write a block of data to only the low-VRAM chip (ie, Mode 7 tilemap data): Set *Address increment mode* to 0, write the data to VMDATAL.
    - To write a block of data to only the high-VRAM chip (ie, Mode 7 tile data): Set *Address increment mode* to 1, write the data to VMDATAH.
    - To write word data to the VRAM: Set *Address increment mode* to 1, write to both VMDATAL and VMDATAH (in order).
  - The *Address increment* bits (bits 0-1) of VMAIN controls how much the internal VRAM word address will be incremented by:
    - 0b00: Increments the VRAM word address by 1.
    - 0b01: Increments the VRAM word address by 32. Useful for writing a 32-word tilemap column to VRAM.
    - 0b10 or 0b11: Increments the VRAM word address by 128. Useful for writing a 128-byte Mode 7 tilemap column to VRAM.
  - The *Address remapping* bits (bits 2-3) of VMAIN remap how the internal VRAM word address bits are connected the address bus of the two VRAM chips.
    - For most transfers the *Address remapping* bits will be 0 (no-remapping).
- Writing to VMADD will set the internal VRAM word address.
  - You should always write to both VMADDL and VMADDH (eg, with a 16 bit write to VMADD) when setting the VRAM word address.
  - Writing to VMADDL or VMADDH will cause the PPU to perform a VRAM read to the VRAM latch.
    - If the PPU is in horizontal-blank or active-display, no VRAM read will occur and the latch will contain invalid data.
- Reading from VMDATAREAD will immediately read the value of the VRAM latch, **then** perform a VRAM read and **then** increment the internal VRAM word address (depending on VMAIN).
  - This means you will need to perform a dummy read from VMDATAxREAD if you want to read multiple bytes/words from VRAM.
  - When reading a single byte or word of VRAM: Set the word address with VMADD and read the VRAM data via VMDATALREAD and/or VMDATAHREAD.
  - When reading multiple bytes/words of VRAM: Set the word address with VMADD, do a dummy read via VMDATAxREAD, repeatedly read the VRAM data from VMDATAxREAD.
  - The PPU will read from VRAM (to the VRAM latch):[[4]](#cite_note-4)
    - on every VMDATALREAD read if VMAIN.bit7 is 0 (before the internal VRAM address increments).
    - on every VMDATAHREAD read if VMAIN.bit7 is 1 (before the internal VRAM address increments).
  - If the PPU is in horizontal-blank or active-display, no VRAM read will occur and the latch will contain invalid data.

### Common VMAIN values

Common VMAIN values

|  | VMAIN value | Access | Increment | Used for |
| --- | --- | --- | --- | --- |
| Word data | $80 | 16 bit write to VMDATAL & VMDATAH | 1 | [[Tiles|2/4/8 bpp tile]] data, [[Tilemaps|tilemap]] data and [[Offset-per-tile]] data |
| Low byte | $00 | 8 bit write to VMDATAL | 1 | [[Tilemaps#Mode 7|Mode 7 tilemap]] data, low-byte of a [split tilemap](https://snes.nesdev.org/w/index.php?title=Split_tilemap&action=edit&redlink=1 "Split tilemap (page does not exist)"), [1bpp tile](https://snes.nesdev.org/w/index.php?title=1bpp_tiles&action=edit&redlink=1 "1bpp tiles (page does not exist)") data |
| High byte | $80 | 8 bit write to VMDATAH | 1 | [[Tiles#Mode 7|Mode 7 tile]] data, high-byte of a [split tilemap](https://snes.nesdev.org/w/index.php?title=Split_tilemap&action=edit&redlink=1 "Split tilemap (page does not exist)"), [1bpp tile](https://snes.nesdev.org/w/index.php?title=1bpp_tiles&action=edit&redlink=1 "1bpp tiles (page does not exist)") data |
| Tilemap column | $81 | 16 bit write to VMDATAL & VMDATAH | 32 | One [[Scrolling a large map|tilemap column]] (Only 1 column can be transferred at a time, VMADD must be set before writing the next column.) (Tilemap columns are discontiguous on 64x64 tilemaps and require 2 transfers per column) |
| Mode 7 tilemap column | $02 | 8 bit write to VMDATAL | 128 | One [[Tilemaps#Mode 7|Mode 7 tilemap]] column (Only 1 column can be transferred at a time, VMADD must be set before writing the next column) |

### Writing word data to VRAM

The most common value for VMAIN is $80, which enables sequential word access to VRAM. This is useful for writing tile data (2/4/8 bpp), tilemap data and offset-per-tile data to VRAM.

When the *Address increment mode* bit (bit 7) of VMAIN is set, word data can be written to both VRAM-chips with either:

- An 8 bit write to VMDATAL, followed by a second 8 bit write to VMDATAH
- A 16 bit write to VMDATA
- A DMA to VMDATAL and VMDATAH, using the *two registers* DMA transfer pattern (DMAP pattern 1).

```
// Write `TileData` to VRAM word address `VRAM_BG1_TILES_WADDR`.
//
// REQUIRES: Force-Blank
//           (There might not be enough Vertical-Blank time if `TileData` is too large)
.a8
.i16
// DB access registers

    // Set VMAIN to word access
    lda     #$80
    sta     VMAIN
 
    // Set VRAM word address
    ldx     #VRAM_BG1_TILES_WADDR
    stx     VMADD


    // Use a 16 bit Accumulator
    rep     #$30
.a16
 
    ldx     #0
    Loop:
        // Read one word of TileData and write it to VRAM
        lda     f:TileData,x
        sta     VMDATA
 
        inx
        inx
        cpx     #TILE_DATA_SIZE
        bcc     Loop

    // restore 8 bit Accumulator
    sep     #$20
.a8
```

Writing word data to VRAM using DMA is preformed using the *two registers* transfer pattern (DMAP pattern 1) to VMDATA.

```
// Transfer the word data at `data` to VRAM word address `vram_waddr` using DMA.
//
// REQUIRES: Vertical-Blank or Force-Blank
// DB access registers
// Uses DMA channel 0
subroutine WriteTileDataToVram(vram_waddr, data, data_size):
    // Set VMAIN to word access
    VMAIN = $80

    // Set VRAM word address
    VMADD = vram_waddr

    // DMA parameters: two registers, to PPU
    DMAP0 = 1

    // B-Bus address
    BBAD0 = .lobyte(VMDATA)

    // A-Bus address
    A1T0  = .loword(data)
    A1B0  = .bankbyte(data)

    // Transfer size
    DAS0  = data_size

    // Start DMA transfer on channel 0
    MDMAEN = 1 << 0
```

### Reading a single byte/word of VRAM

Reading a single byte/word from VRAM can be done in a similar manner as reading from CGRAM. You should always set the VMAIN register before writing to VMADD to ensure VRAM is accessed in the intended manner.

```
// Read ONE word of VRAM data from VRAM word address `X`
//
// INPUT: X - VRAM word address
// OUTPUT: Y - data at VRAM word address `X`
//
// REQUIRES: Vertical-Blank or Force-Blank.
//
// DB access registers
.a8
.i16
.proc ReadOneVramWord
    // Set VMAIN to word access
    lda     #$80
    sta     VMAIN

    // Set VRAM word address
    stx     VMADD

    // Read VRAM
    ldy     VMDATAREAD

    rts
.endproc
```

### Reading a block of VRAM

Due to the way the [[PPU registers#VMDATAREAD|PPU updates the vram\_latch]] when accessing the VMADD and VMDATAxREAD registers, a dummy read to VMDATAxREAD is required when reading a block of contiguous VRAM data.

Failure to issue a dummy read will result in an off-by-one error, with the first two bytes/words containing duplicate data from the same VRAM address.

```
// REQUIRES: Vertical-Blank or Force-Blank.
// DB access registers
.a8
.i16
    // Set VMAIN to word access
    lda     #$80
    sta     VMAIN

    // Set VRAM word address
    ldx     #$6000
    stx     VMADD           // populates vram_latch with data at VRAM word address $6000

    ldy     VMDATAREAD      // Y = data at VRAM word address $6000
    ldy     VMDATAREAD      // Y = data at VRAM word address $6000  <-- off by one error
    ldy     VMDATAREAD      // Y = data at VRAM word address $6001
    ldy     VMDATAREAD      // Y = data at VRAM word address $6002
    ldy     VMDATAREAD      // Y = data at VRAM word address $6003
```

A block of VRAM can be read into RAM using DMA (by setting the *direction* bit of [[DMA registers#DMAPn|DMAPn]] to transfer data from the B-Bus (PPU) to the A-Bus).

```
Variables:
    zpFarPtr - a 3 byte pointer in zero-page.

// Transfer VRAM word data to WRAM using DMA.
//
// INPUT: X        = VRAM word address
//        Y        = data size
//        zpFarPtr = address to write VRAM word data to (MUST be a work-RAM or cart-RAM address)
//
// REQUIRES: Vertical-Blank or Force-Blank
// Uses DMA channel 0
//
// DB access registers
.a8
.i16
.proc ReadVramWordData
    // Set VMAIN to word access
    lda     #$80
    sta     VMAIN

    // Set VRAM word address (required)
    stx     VMADD

    // Dummy read (required)
    ldx     VMDATAREAD


    // Setup DMA channel 0
    // DMA from word register VMDATAREAD to `zpFarPtr`

    // DMA parameters: two registers, PPU to CPU
    lda     #$81
    sta     DMAP0

    // B-Bus address
    lda     #.lobyte(VMDATAREAD)
    sta     BBAD0

    // A-Bus address
    ldx     zpFarPtr
    stx     A1T0
    lda     zpFarPtr + 2
    sta     A1B0

    // Transfer size (in Y register)
    sty     DAS0

    // Start DMA transfer on channel 0
    lda     #1 << 0
    sta     MDMAEN
.endproc
```

## References

1. [↑](#cite_ref-1) higan source code, sfc/ppu/object.cpp PPU::Object::scanline(), by Near
2. [↑](#cite_ref-2) higan source code, sfc/ppu/io.cpp and sfc/ppu/object.cpp, by Near (search for setFirstSprite())
3. [↑](#cite_ref-3) VRAM address bits 14 and 15 (S-PPU-1 pins 47 & 46) are shared across both VRAM chips and are always zero in Mode 7.
4. [↑](#cite_ref-4) higan source code, sfc/ppu/io.cpp, by Near
