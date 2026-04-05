---
title: "VBlank routine"
source_url: "https://snes.nesdev.org/wiki/VBlank_routine"
pageid: 83
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

The VBlank routine is that code that runs during the Vertical Blanking Period. It can be located in:

- The [[VBlank interrupts|VBlank Interrupt ISR]]
- The main-loop. Simply pause execution of main-loop until the start of the Vertical Blanking Period before executing the VBlank routine.
- An IRQ Interrupt Service Routine (used by games that [extend VBlank with letterboxing](https://snes.nesdev.org/w/index.php?title=Extended_VBlank&action=edit&redlink=1 "Extended VBlank (page does not exist)"))

SNES games can have multiple VBlank routines. For example, an RPG might have different VBlank routines for:

- Menus
- Exploration (towns, dungeons, etc)
- Mode 7 world map
- Battles
- Mini-games

## Shadow variables

A register shadow variable is a RAM variable that will hold (shadow) the intended value of a PPU register. This variable can be read or written to at any time. When it is safe to write to the register, usually during the Vertical Blanking period, the shadow variable will be transferred to its associated register.

Shadow variables offer many advantages over directly writing to the register:

- Shadow variables can be read or written at any time.
- Shadow variables allow the code to read the shadowed-state of write-only registers.
- Write-twice registers can store their state in a 16 bit (word) variable. This allows the main-loop to access the shadow with a 16 bit Accumulator or Index.
- Reading a shadow variable is significantly faster then calculating the intended value of a register. The slow calculation can be preformed in the main-loop and the write to the PPU register can be preformed during VBlank.

Shadow variables should be allocated in Low-RAM. This allows the VBlank routine to access both the shadow-variable and the register with the *addr* addressing mode.

### Write-only byte register example

The following is an example of a shadow variable for the [[PPU registers#MOSAIC|MOSAIC]] register. Notice how shadow variable allows the main-loop to read the shadowed register state and how tiny the VBlank code is.

Main-loop code:

```
;   mosaicShadow   u8 - shadow of the MOSAIC register
;
;   zpTmpByte      u8 - zeropage temporary byte variable


; Increase mosaic size by 1 (with clamping), leave mosaic enable bits unchanged
;
; DB access Low RAM ($00-$3f, $7e, $80-$bf)
.a8
.i16
    ; extract mosaic enable bits
    lda mosaicShadow
    and #0xf0
    sta zpTmpByte

    ; increase mosaic size bits (with clamping)
    lda mosaicShadow
    inc
    and #0x0f
    bne :+
        ; mosaic size overflowed
        lda #0xf
    :

    ; combine enable and size bits
    ora zpTmpByte
    sta mosaicShadow
```

VBlank code:

```
;   mosaicShadow   u8 - shadow of the MOSAIC register

; In VBlank
; DB access registers ($00-$3f, $80-$bf)
.a8
.i16
    lda mosaicShadow
    sta MOSAIC
```

### Write-twice register example

The following is an example of a shadow variable for the [[PPU registers#BGnHOFS|BG1HOFS]] and [[PPU registers#BGnVOFS|BG1VOFS]] scroll registers. Notice how the main-loop code writes to the shadow variables in 16 bit mode, while the VBlank code is required to read the shadow variables one byte at a time.

Main-loop code:

```
;   bg1HOffset    u16 - shadow of the BG1HOFS register
;   bg1VOffset    u16 - shadow of the BG1VOFS register

; DB access Low RAM ($00-$3f, $7e, $80-$bf)
.a8
.i16
    ; Set BG1 scroll offset with a 16 bit Index register
    ldx #0
    stx bg1HOffset

    ldx #.loword(-1)
    stx bg1VOffset
```

VBlank code:

```
;   bg1HOffset    u16 - shadow of the BG1HOFS register
;   bg1VOffset    u16 - shadow of the BG1VOFS register


; In VBlank
; DB access registers ($00-$3f, $80-$bf)
.a8
.i16
    ; Transfer a 16 bit variable to a write twice register
    lda bg1HOffset
    sta BG1HOFS
    lda bg1HOffset + 1
    sta BG1HOFS

    lda bg1VOffset
    sta BG1VOFS
    lda bg1VOffset + 1
    sta BG1VOFS
```

## Buffers

A common method of updating VRAM, OAM or CGRAM while the screen is active is to use a fixed-size buffer. The main-loop would read and write to the buffer, filling it with data. When the buffer is ready, the main-loop would write to one or more control variables. Later, the VBlank routine will read the control variable to determine if the buffer should be transferred and if so, transfers the buffer to the PPU using [[DMA examples|DMA]].

The buffer can be allocated anywhere in RAM and the control variables should be allocated in Low-RAM.

At minimum, there should be a single control variable that determines if the buffer is to be transferred to the PPU on the next VBlank. This variable is usually a byte flag and can be either a transfer-on-zero byte flag or transfer-on-non-zero byte flag (depending on preference). The transfer flag is reset after the VBlank routine has transferred the buffer to the PPU.

Optionally, a buffer can also have control variables for the buffer's destination address and/or transfer size. If there are multiple control variables, the transfer flag should be written last.

To prevent glitches and screen tearing, the buffer should not be modified after the transfer flag is enabled. The simplest method of achieving this is to only write to the buffer once per frame.

Common use-cases for buffers include:

- OAM buffer
- CGRAM buffer (useful for color transitions)
- A Monospaced [Text buffer](https://snes.nesdev.org/w/index.php?title=Text_buffer&action=edit&redlink=1 "Text buffer (page does not exist)")
- A tile buffer for a [[Variable width font]]
- A tilemap row buffer (see [[Scrolling a large map]] and [[DMA examples#Updating Tilemap Rows]])
- A tilemap column buffer (see [[Scrolling a large map]])

### OAM buffer example

VBlank code:

```
;   oamBuffer                   u8[544] - OAM buffer data.
;   oamBufferTransferFlag       u8      - byte flag.  If non-zero, `oamBuffer` is transferred to OAM on the next VBlank.  Must be in Low-RAM.


; In VBlank
; DB access registers ($00-$3f, $80-$bf)
.a8
.i16
    ; Read oamBuffer transfer flag
    lda oamBufferTransferFlag
    beq SkipOamTransfer
        ; transfer flag is set

        ; Reset OAM address
        ldx #0
        stx OAMADD


        ; Transfer oamBuffer to OAM using DMA channel 0
        ldx #DMA_LINEAR | ((OAMDATA & 0xff) << 8)
        stx DMAMODE                                 ; also sets B Bus Address

        ldx #oamBuffer
        stx DMAADDR
        lda #.bankbyte(oamBuffer)
        sta DMAADDRBANK

        ldx #544
        stx DMALEN

        lda #1
        sta MDMAEN


        ; Reset oamBuffer transfer flag
        stz oamBufferTransferFlag

SkipOamTransfer:
```

Main-loop code:

```
    GameLoop:
        ; ...

        ; process players, enemies, particles, etc drawing their sprites into the `oamBuffer`

        ; ...


        ; Notify the VBlank routine that the oamBuffer has changed
        lda #1
        sta oamBufferTransferFlag


        ; Wait until the VBlank routine has been processed
        jsr WaitForVBlank

        jmp GameLoop
```

### Tilemap column buffer example

This following is an example of a buffer with multiple control variables.

VBlank code

```
;   columnBuffer                u16[32] - buffer containing 32 tilemap columns (64 bytes in size)
;   columnBufferVramWaddr       u16     - VRAM word address to transfer `columnBuffer` to.  Must be in Low-RAM.
;   columnBufferTransferFlag    u8      - byte flag.  If non-zero, `columnBuffer` is transferred to VRAM on the next VBlank.  Must be in Low-RAM.


; In VBlank
; DB access registers ($00-$3f, $80-$bf)
.a8
.i16
    ; Read columnBufferTransferFlag transfer flag
    lda columnBufferTransferFlag
    beq SkipColumnBufferTransfer
        ; transfer flag is set

        ; VRAM word access, increment by 32
        lda #$81
        sta VMAIN

        ; Set VRAM word address
        ldx columnBufferVramWaddr
        stx VMADD


        ; Transfer column buffer to VRAM using DMA channel 0

        ldx #DMA_01 | ((VMDATA & 0xff) << 8)
        stx DMAMODE                             ; also sets B Bus Address

        ldx #columnBuffer
        stx DMAADDR
        lda #.bankbyte(columnBuffer)
        sta DMAADDRBANK

        ldx #64
        stx DMALEN

        lda #1
        sta MDMAEN


        ; Change VRAM access back to "word access, increment by 1"
        ; This is not required if your VBlank code sets `VMAIN` before every VRAM transfer.
        lda #$80
        sta VMAIN


        ; Reset transfer flag
        stz columnBufferTransferFlag

SkipColumnBufferTransfer:
```

## Queues

Queues are useful when you want to transfer multiple items to the PPU. They usually consist of a list of updates that are to be processed during the VBlank routine. The main-loop would add entries to the queue. Later, the VBlank routine will process the entries and then reset the queue.

Queues should be implemented with care. There is a limited amount of VBlank-time and a queue that is too large can easily cause a VBlank overrun. Code that interacts with a queue must gracefully handle a queue-is-full condition.

Common use-cases for queues include:

- Queue of [MetaTile](https://snes.nesdev.org/w/index.php?title=Metatiles&action=edit&redlink=1 "Metatiles (page does not exist)") updates to transfer to VRAM.
- Queue of sprite tiles to DMA to VRAM.
- Queue of DMA transfers to VRAM.

### Metatile queue example

The following is an example of a queue for drawing 2x2 MetaTiles in the middle of the screen.

Variables:

```
; Structure of Word Arrays queue (last-in, first-out)
;
; QUEUE_SIZE = 4                        - Constant.  Number of entries in the queue
;
; Variables (Must be in Low-RAM):
;   mtQueueIndex        u16             - Current position in the queue.
;                                         The queue is full if `mtQueueIndex == 2 * QUEUE_SIZE`.
;                                         MUST be even, MUST be <= `2 * QUEUE_SIZE`.
;
;   mtQueueVramWaddr    u16[QUEUE_SIZE] - VRAM word address to transfer the MetaTile to
;   mtQueueTopLeft      u16[QUEUE_SIZE] - Tilemap word for the top-left quadrant of the MetaTile
;   mtQueueTopRight     u16[QUEUE_SIZE] - Tilemap word for the top-right quadrant of the MetaTile
;   mtQueueBottomLeft   u16[QUEUE_SIZE] - Tilemap word for the bottom-left quadrant of the MetaTile
;   mtQueueBottomRight  u16[QUEUE_SIZE] - Tilemap word for the bottom-right quadrant of the MetaTile
```

Init:

```
; DB access low-RAM
.a8
.i16
    ; Reset queue position (queue is now empty)
    ldy #0
    sty mtQueueIndex
```

Main-Loop code:

```
; Add a MetaTile to the MetaTile update queue.
; On the next VBlank it will be transferred to VRAM.
;
; NOTE: This function will fail if the queue is full.
;
; INPUT: Y = VRAM word address of the MetaTile (must point to an even tilemap row)
;        X = MetaTile index (within a hypothetical MetaTile data structure)
;
; OUTPUT: carry set if MetaTile is added to the queue
;         carry clear if queue is full (MetaTile is not added to the queue)
;
; DB access low-RAM
.a8
.i16
.proc QueueMetaTileUpdate

    rep #$20
.a16
    tya

    ; Get queue position and check if the queue is full
    ldy mtQueueIndex
    cpy #QUEUE_SIZE * 2
    bcs QueueIsFull

    ; Queue is not empty, add entry to queue
    sta mtQueueVramWaddr,y

    ; Copy tilemap data to the queue
    lda MetaTileData_topLeft,x
    sta mtQueueTopLeft,y

    lda MetaTileData_topRight,x
    sta mtQueueTopRight,y

    lda MetaTileData_bottomLeft,x
    sta mtQueueBottomLeft,y

    lda MetaTileData_bottomRight,x
    sta mtQueueBottomRight,y

    ; Increment queue index
    iny
    iny
    sty mtQueueIndex


    sep #$20
.a8
    ; Return true
    sec
    rts



.a16
QueueIsFull:
    sep #$20
.a8
    ; Return false
    clc
    rts
.endproc
```

VBlank code:

```
; Transfer MetaTile Update Queue to VRAM
;
; In VBlank
; DB access registers ($00-$3f, $80-$bf)
.a8
.i16
    ldy mtQueueIndex
    beq MetaTileQueueEmpty
        ; Queue is not empty

        ; VRAM word access, increment by 1
        lda #$80
        sta VMAIN

        ; Use a 16 bit accumulator to set VMADD
        rep #$20
    .a16

        MetaTileQueueLoop:
            ; Queue is not empty, transfer MetaTile to VRAM

            ; Set VRAM word address for the top-half of the MetaTile
            lda mtQueueVramWaddr,y
            sta VMADD

            ; Transfer top-half of the MetaTile to VRAM
            ; (using X register as we want A unchanged)
            ldx mtQueueTopLeft,y
            stx VMDATA

            ldx mtQueueTopRight,y
            stx VMDATA


            ; Set VRAM word address to the bottom-half of the MetaTile (32 tiles below the top-half).
            ; Using `ora` is faster as it does not require a `clc` instruction, but it only works
            ; if bit 5 of `mtQueueVramWaddr,y` is clear (ie, points to an even tilemap row)
            ;
            ; A = mtQueueVramWaddr,y
            ora #32
            sta VMADD

            ; Transfer bottom-half of the MetaTile to VRAM
            ldx mtQueueBottomLeft,y
            stx VMDATA

            ldx mtQueueBottomRight,y
            stx VMDATA


            ; Decrement queue index and loop until all queue entries are processed
            dey
            dey
            bpl MetaTileQueueLoop


        ; Reset queue position (queue is now empty)
        stz mtQueueIndex

        sep #$20
    .a8

MetaTileQueueEmpty:
```

## HDMA

The Vertical Blanking Period is the best time to update the HDMA registers while the display is active.

- HDMA must be setup and enabled before the start of the next frame. A few cycles after the start of scanline 0, the DMA controller will setup the [[DMA registers#Other HDMA registers|HDMA state registers]] for all active HDMA channels.
  - Enabling a HDMA channel too late will not reset the HDMA table position and cause invalid values to be written to the target register for an unknown number of scanlines, resulting in a glitched screen.
  - Technically, HDMA can be enabled mid-frame (although it is not recommended). This requires the programmer to manually setup the HDMA state registers and enable the HDMA channels at the correct time.
- Avoid using the same DMA channel for DMA and HDMA transfers.
  - This prevents the HDMA registers from being clobbered by the DMA transfer code.
  - HDMA will stop any active DMA transfers use the same channel for DMA and HDMA.
- Avoid VBlank overruns when using HDMA.
  - An overrun could cause HDMA to be enabled too late, causing invalid values to be written to the target registers, creating a glitched screen.
  - An overrun could cause a [[Errata#DMA|model-1 crash]] if a DMA transfer finishes just before a HDMA transfer starts (on version 1 of the 5A22 S-CPU).

## Reading Joypads

The end of the VBlank routine, after the VBlank-time critical code been executed, is a good place to [[Controller reading|read the controller registers]]. However, there are a few caveats:

- The [[MMIO registers#Auto-read results|`JOY1-JOY4` registers]] must not be read while Joypad auto-read is active. The *Joypad auto-read in-progress flag* of `HVBJOY` can be used to pause execution until Joypad auto-read is completed.
- Some emulators do not have accurate auto-read timings. Joypad reading code that does not poll `HVBJOY` might not work on these emulators and code that works on emulator may not work on a real console.
- Joypad auto-read starts shortly after VBlank. Code that polls the *Joypad auto-read in-progress* flag of `HVBJOY` too early (after VBlank starts and before auto-read starts) can erroneously read `JOY1-JOY4` while the auto-read is active.
- The joypad-state variables should not be written to during a lag frame. Failure to account for lag-frames could cause a heisenbug if the joypad state variables suddenly change in the middle of the Main-Loop.
