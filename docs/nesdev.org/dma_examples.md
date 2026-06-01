---
title: "DMA examples"
source_url: "https://snes.nesdev.org/wiki/DMA_examples"
pageid: 59
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

This page provides examples on how to use [[DMA registers]] to do fast copies on the SNES.

These examples use the following defines to make the code clearer:

```
; Registers            Also known as...
DMAMODE      = $4300 ; DMAPn
DMAPPUREG    = $4301 ; BBADn
DMAADDR      = $4302 ; A1TnL
DMAADDRHI    = $4303 ; A1TnH
DMAADDRBANK  = $4304 ; A1Bn
DMALEN       = $4305 ; DASnL
DMALENHI     = $4306 ; DASnH

; Configuration for $43n0
; OR these together to get the desired effect
DMA_LINEAR   = $00
DMA_01       = $01
DMA_00       = $02
DMA_0011     = $03
DMA_0123     = $04
DMA_0101     = $05
DMA_FORWARD  = $00
DMA_CONST    = $08
DMA_BACKWARD = $10
DMA_INDIRECT = $40
DMA_READPPU  = $80

; These defines are meant for a 16-bit write to $43n0 and $43n1
; and they set up the channel for several common cases.
DMAMODE_PPULOFILL = (<VMDATAL << 8) | DMA_LINEAR | DMA_CONST
DMAMODE_PPUHIFILL = (<VMDATAH << 8) | DMA_LINEAR | DMA_CONST
DMAMODE_PPUFILL   = (<VMDATAL << 8) | DMA_01     | DMA_CONST
DMAMODE_RAMFILL   = (<WMDATA  << 8) | DMA_LINEAR | DMA_CONST
DMAMODE_PPULODATA = (<VMDATAL << 8) | DMA_LINEAR | DMA_FORWARD
DMAMODE_PPUHIDATA = (<VMDATAH << 8) | DMA_LINEAR | DMA_FORWARD
DMAMODE_PPUDATA   = (<VMDATAL << 8) | DMA_01     | DMA_FORWARD
DMAMODE_CGDATA    = (<CGDATA  << 8) | DMA_00     | DMA_FORWARD
DMAMODE_OAMDATA   = (<OAMDATA << 8) | DMA_00     | DMA_FORWARD
```

## Copying OAM

This is a simple DMA example that sends an [[OAM]] buffer to the PPU.

```
; Copy `oamBuffer` to OAM
; Timing: Force-Blank or Vertical-Blank
.proc CopyOAM
  php

  rep #$20          ; Set A to 16-bit

  stz OAMADD        ; Reset the OAM address

  lda #DMAMODE_OAMDATA
  sta DMAMODE
  lda #oamBuffer    ; Copy from OAM buffer in RAM
  sta DMAADDR
  lda #544          ; 512 bytes + 32 bytes = 544
  sta DMALEN

  sep #$20          ; Set A to 8-bit

  lda #^oamBuffer   ; Set the bank byte of the source address
  sta DMAADDRBANK

  ; Start the DMA
  lda #1
  sta MDMAEN

  plp
  rtl
.endproc
```

## DMA as part of a scrolling update

This example demonstrates using DMA to write to video RAM while [[Scrolling a large map]], and it might make sense to put something like it in a game's vblank handler. Notice the writes to [[PPU registers]] - the DMA unit only handles writing the actual data, so anything else (such as setting the destination address) has to be done normally. This also demonstrates using [[PPU registers#VMAIN|VMAIN]] to write downwards through a tilemap.

```
  .a16
  ; Does a column need to be updated?
  lda ColumnUpdateAddress
  beq :+
    stz ColumnUpdateAddress
    sta VMADDL

    ; Write to VMDATAL and VMDATAH
    lda #DMAMODE_PPUDATA
    sta DMAMODE

    ; Copy from the buffer
    lda #.loword(ColumnUpdateBuffer)
    sta DMAADDR

    ; A tilemap column is 32 tiles long, and each tile is 2 bytes
    lda #32*2
    sta DMALEN

    sep #$20 ; 8-bit accumulator

    lda #^ColumnUpdateBuffer
    sta DMAADDRBANK
    lda #$81   ; Increment on VMDATAH write, increment by 32
    sta VMAIN

    lda #1     ; Start the DMA
    sta MDMAEN

    lda #$80   ; Increment on VMDATAH write, increment by 1
    sta VMAIN

    rep #$20   ; 16-bit accumulator
  :
```

## Updating Tilemap Rows

### Updating a single tilemap row in a 64 tile-wide background

Tilemap columns 31 and 32 of a 64 tile-wide background is non-contiguous. Transferring an 64 tile tilemap row requires two separate DMA transfers to the following locations:

- rowBuffer[  0 -  63] to VRAM word address rowBufferVramWaddr + 0
- rowBuffer[ 64 - 128] to VRAM word address rowBufferVramWaddr + 0x400

The DMA A-Bus address will be incremented during the transfer (unless it is in *fixed address* mode). This means DMAADDR will be prefilled with the address of the second half of the row buffer after the first DMA transfer.

```
; 8 bit A
; 16 bit Index
; DB access registers
; DP = 0
;
; rowBuffer             u16[64] - buffer containing a 64 tile tilemap row (128 bytes in size)
; rowBufferVramWaddr    u16     - the VRAM word address to transfer rowBuffer to


    ; VRAM word addressing
    lda #$80
    sta VMAIN


    ;
    ; First DMA transfer.
    ; Transfer `rowBuffer` bytes 0 - 63 to VRAM word address `rowBufferVramWaddr`
    ;

    ; Set VRAM word address
    ldx rowBufferVramWaddr
    stx VMADD

    ; Word transfer to VMDATA
    ldx #DMA_01 | ((VMDATAL & 0xff) << 8)
    stx DMAMODE                             ; also sets B Bus Address

    ; Set DMA source address
    ldx #rowBuffer & 0xffff
    stx DMAADDR
    lda #rowBuffer >> 16
    sta DMAADDRBANK

    ; Length of the DMA transfer (32 words, 64 bytes)
    ldx #64
    stx DMALEN

    ; Start DMA transfer
    lda #1
    sta MDMAEN


    ;
    ; Second DMA transfer.
    ; Transfer `rowBuffer` bytes 64 - 127 to VRAM word address `rowBufferVramWaddr + 0x400`
    ;

    ; Set VRAM word address to `rowBufferVramWaddr + 0x400`
    lda rowBufferVramWaddr
    sta VMADDL

    lda rowBufferVramWaddr + 1
    clc
    adc #$04
    sta VMADDH


    ; No need to set DMAMODE or DMAPPUREG, it remains unchanged after a DMA transfer.

    ; After the first transfer, DMAADDR will contain `rowBuffer + 64`.
    ; There is no need to set DMAADDR or DMAADDRBANK.

    ; We must write to DMALEN, it will be 0 after a successful DMA transfer.
    ldx #64
    stx DMALEN

    ; Start DMA transfer
    lda #1
    sta MDMAEN
```

### Updating two tilemap rows in a 64 tile-wide background

Due to the discontiguous nature of the tilemap in a 64 tile-wide background, a transfer of a contiguous 64x2 word grid to VRAM requires 4 DMA transfers:

- rowBuffer[  0 -  63] to VRAM word address rowBufferVramWaddr + 0
- rowBuffer[128 - 191] to VRAM word address rowBufferVramWaddr + 0x020
- rowBuffer[ 64 - 127] to VRAM word address rowBufferVramWaddr + 0x400
- rowBuffer[192 - 255] to VRAM word address rowBufferVramWaddr + 0x420

This transfer can be simplified by using two DMA channels to transfer the first and third quarters of the rowBuffer, one after another, in a single DMA transfer. Afterwords, the second and fourth quarters can be transferred to VRAM.

- rowBuffer[  0 -  63] and rowBuffer[128 - 191] to VRAM word address rowBufferVramWaddr + 0
- rowBuffer[ 64 - 127] and rowBuffer[192 - 255] to VRAM word address rowBufferVramWaddr + 0x400

```
; 8 bit A
; 16 bit Index
; DB access registers
; DP = 0
;
; rowBuffer             u16[128] - buffer containing a 64x2 tilemap grid (256 bytes in size)
; rowBufferVramWaddr    u16      - the VRAM word address to transfer rowBuffer to


    ; VRAM word addressing
    lda #$80
    sta VMAIN


    ;
    ; First DMA transfer (using DMA channels 0 & 1).
    ; Transfer `rowBuffer` bytes 0 - 63 and bytes 128 - 191 to VRAM word address `rowBufferVramWaddr`
    ;

    ; Set VRAM word address
    ldx rowBufferVramWaddr
    stx VMADD

    ; Word transfer to VMDATA
    ldx #DMA_01 | ((VMDATAL & 0xff) << 8)
    stx DMAMODE + $00                       ; also sets B Bus Address
    stx DMAMODE + $10

    ; Set DMA source addresses
    ldx #rowBuffer & 0xffff
    stx DMAADDR + $00

    ldx #(rowBuffer + 128) & 0xffff
    stx DMAADDR + $10

    lda #rowBuffer >> 16
    sta DMAADDRBANK + $00
    sta DMAADDRBANK + $10

    ; Transfer size for each DMA channel (32 words, 64 bytes)
    ldx #64
    stx DMALEN + $00
    stx DMALEN + $10

    ; Start DMA transfer for channels 0 & 1
    lda #$03
    sta MDMAEN



    ;
    ; Second DMA transfer (using DMA channels 0 & 1).
    ; Transfer `rowBuffer` bytes 64 - 127 and bytes 192 - 255 to VRAM word address `rowBufferVramWaddr + 0x400`
    ;

    ; Set VRAM word address to `rowBufferVramWaddr + 0x400`
    lda rowBufferVramWaddr
    sta VMADDL

    lda rowBufferVramWaddr + 1
    clc
    adc #$04
    sta VMADDH


    ; No need to set DMAMODE or DMAPPUREG, it remains unchanged after a DMA transfer.

    ; After the first transfer, DMAADDR channel 0 will contain `rowBuffer + 64`
    ; and DMAADDR channel 1 will contain `rowBuffer + 192`
    ;
    ; There is no need to set DMAADDR or DMAADDRBANK.

    ; We must write to DMALEN, it will be 0 after a successful DMA transfer.
    ldx #64
    stx DMALEN + $00
    stx DMALEN + $10

    ; Start DMA transfer for channels 0 & 1
    lda #$03
    sta MDMAEN
```

## Fixed address DMA transfers

The *fixed address* DMA mode is useful for clearing and filling blocks of memory.

### WRAM clear

These are reusable subroutines for clearing out sections of RAM, which a program might want to do at the start of a level, or in [[Init code]]. Both routines take a start address in X, and a size in Y.

The source address of a DMA transfer to WMDATA cannot be in Work-RAM, because the SNES cannot handle using DMA to copy from one section of Work-RAM to another. If you intend to fill Work-RAM with a non-zero value, use a source address in Cartridge-RAM or ROM.

Note: This routine cannot be used to clear the entire bank $7E of Work-RAM, as it will override the stack and crash the program.

Note: Due to [[Errata#DMA|a hardware bug]] on early SNES consoles it's not recommended to do this while [[HDMA]] is enabled.

```
.i16 ; 16-bit index registers assumed
.proc MemClear
  php
  sep #$20   ; 8-bit accumulator
  stz WMADDH ; Set high bit of WRAM address to zero - meaning the first 64KB of RAM
UseHighHalf:
  stx WMADDL ; WRAM address, bottom 16 bits
  sty DMALEN

  ; Configure DMA to write to WMDATA, and keep the source address constant
  ldx #DMAMODE_RAMFILL
ZeroSource:
  stx DMAMODE

  ; The zero byte used as the source needs to come from somewhere in ROM
  ; here it's taken from the second byte of a "STX $4300"
  ldx #.loword(ZeroSource+1)
  stx DMAADDR
  ; Set the bank byte of the source address too
  lda #^MemClear
  sta DMAADDRBANK

  ; Start the DMA
  lda #1
  sta MDMAEN
  plp
  rtl
.endproc

; Clear a section of bank 7F instead
.proc MemClear7F
  php
  sep #$20
  lda #1     ; Use the second 64KB of RAM
  sta WMADDH
  bra MemClear::UseHighHalf
.endproc
```

### Filling VRAM

A fixed byte DMA transfer can be used to clear a block of VRAM. Unlike the clear Work-RAM routine above, the source of the DMA can be a Work-RAM memory address. This allows us to fill VRAM with a byte value of our choosing.

```
; 8 bit A
; 16 bit Index
; DB access registers
; DP = 0
;
; Uses a single zeropage byte variable (zpTmpByte)


; Clears all of VRAM (using DMA)
.proc ResetVram
    ldx #0
    ldy #0
; fallthrough
.endproc



; Clear a block of VRAM (using DMA)
;
; IN: X - VRAM word address
; IN: Y - size (in bytes)
.proc ClearVram
    lda #0
; fallthrough
.endproc



; Fill a block of VRAM with a byte value (using DMA)
;
; IN: X - VRAM word address
; IN: Y - size in bytes (if 0 then 64KiB of VRAM is filled)
; IN: A - byte value
.proc FillVRAM
    ; Store value to fill in zeropage
    sta zpTmpByte


    ; VRAM word addressing
    lda #$80
    sta VMAIN

    ; Set VRAM word address
    stx VMADD


    ; Length of the DMA transfer
    sty DMALEN

    ; Fixed byte transfer to word register VMDATA
    ldx #DMA_01 | DMA_CONST | ((VMDATA & 0xff) << 8)
    stx DMAMODE                                         ; also sets B Bus Address

    ; Set DMA source address
    ldx #zpTmpByte
    stx DMAADDR
    stz DMAADDRBANK             ; zeropage bank is always 0

    ; Disable HDMA (prevents the model-1 HDMA/DMA crash)
    stz HDMAEN

    ; Start DMA transfer
    lda #1
    sta MDMAEN

    rts
.endproc
```

### Filling VRAM with a word value

Filling VRAM with a word value is a bit more complicated. The *fixed address* DMA mode only allows for byte fills. Fortunately the VMAIN register provides a method of writing to the low and high bytes of VRAM separately, allowing for a VRAM word fill to be preformed in two DMA transfers.

```
; 8 bit A
; 16 bit Index
; DB access registers
; DP = 0
;
; Uses a single zeropage word variable (zpTmpWord)


; Fills a 32x32 tilemap in VRAM with a given word value (using DMA)
; IN: X = VRAM word address
; IN: Y = word value
.proc FillVramTilemap
    ; Store value to fill in zeropage
    sty zpTmpWord

    ; Must not modify X, it is still required after the first DMA transfer


    ; Disable HDMA (prevents the model-1 HDMA/DMA crash)
    stz HDMAEN


    ;
    ; Transfer the low byte of zpTmpWord to VMDATAL `32*32` times
    ;

    ; VRAM byte addressing to VMDATAL
    stz VMAIN

    ; Set VRAM word address
    stx VMADD

    ; Fixed byte transfer to byte register VMDATAL
    ldy #DMA_LINEAR | DMA_CONST | ((VMDATAL & 0xff) << 8)
    sty DMAMODE                                             ; also sets B Bus Address

    ; Set DMA source address to the low byte of zpTmpWord
    ldy #zpTmpWord
    sty DMAADDR
    stz DMAADDRBANK             ; zeropage bank is always 0

    ; Length of the DMA transfer
    ldy #32 * 32
    sty DMALEN

    ; Start DMA transfer
    lda #1
    sta MDMAEN


    ;
    ; Transfer the high byte of zpTmpWord to VMDATAH `32*32` times
    ;

    ; VRAM byte addressing to VMDATAH
    lda #$80
    sta VMAIN

    ; Set VRAM word address
    stx VMADD

    ; Change DMA B-Bus address to VMDATAH
    lda #VMDATAH & 0xff
    sta DMAPPUREG
    ; DMAMODE is already set

    ; Set DMA source address to the high byte of zpTmpWord
    lda #zpTmpWord + 1
    sta DMAADDR
    ; DMAADDRHI and DMAADDRBANK is already set

    ; Length of the DMA transfer
    ldy #32 * 32
    sty DMALEN

    ; Start DMA transfer
    lda #1
    sta MDMAEN

    rts
.endproc
```

## See Also

- [[DMA registers]]
- [[HDMA examples]]

## Links

- [DMA Palette](https://nesdoug.com/2020/05/16/dma-palette/) - by nesdoug
- [Grog's Guide to DMA and HDMA on the SNES](https://wiki.superfamicom.org/grog%27s-guide-to-dma-and-hdma-on-the-snes) - superfamicom.org wiki
- [DMA tutorial](https://en.wikibooks.org/wiki/Super_NES_Programming/DMA_tutorial) - Super NES Programming Wikibooks
