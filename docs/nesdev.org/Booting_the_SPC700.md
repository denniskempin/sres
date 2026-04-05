---
title: "Booting the SPC700"
source_url: "https://snes.nesdev.org/wiki/Booting_the_SPC700"
pageid: 40
retrieved_at: "2026-04-05T23:38:15.892282+00:00"
---

When the SNES powers on or resets, the [[SPC700]] starts running a small program which does some initialization and then waits to communicate with the 65c816. At this point the 65c816 can instruct the SPC700 to load a new program into its RAM and start running it. This page explains how to do this.

Communication with the SPC700 involves four registers - $2140, $2141, $2142 and $2143. These four bytes correspond to $f4, $f5, $f6 and $f7 on the SPC700-side, respectively. When one side writes to a register, the other can read its corresponding register to see what value was most recently written. This guide will refer to them as `APUIO0`, `APUIO1`, `APUIO2` and `APUIO3`.

## Waiting for the SPC700 to be ready

Before the 65c816 can communicate with the SPC700, it has to wait for the SPC700 to signal that it's finished its initialization and is ready. It shows this by writing $AA to `APUIO0` and $BB to `APUIO1`. The 65c816 can poll both at once with a 16-bit read (though checking one after the other works fine too):

```
  ; 16-bit accumulator
  lda #$BBAA
waitBBAA:
  cmp APUIO0
  bne waitBBAA
```

## Setting the starting address

After the SPC700 is ready, the 65c816 specifies what address it wants to start writing to. This is done through these steps:

- 65c816 writes starting address to `APUIO2` and `APUIO3`
- 65c816 writes something other than zero to `APUIO1`
- 65c816 writes $CC to `APUIO0`
- SPC700 writes $CC to `APUIO0` as acknowledgement
- 65c816 waits for this acknowledgement

```
  ; 8-bit accumulator
  Destination = $0200 ; Starting address in SPC700 RAM
  lda #<Destination
  sta APUIO2
  lda #>Destination
  sta APUIO3
  lda #$CC
  sta APUIO1          ; Must be any nonzero value (because zero means "start the program")
  sta APUIO0          ; Must be $CC

wait:                 ; Wait for the SPC700 to acknowledge this
  cmp APUIO0
  bne wait
```

SPC700 programs usually start at $0200, because $0000-$00FF contains zeropage and registers, and $0100-$01FF contains the stack, so $0200 is the next available space after those.

## Sending data

With the address set, the 65c816 can start sending over data. Bytes are sent one-at-a-time and the SPC700 has to acknowledge each byte the 65c816 sends.

- 65c816 writes a byte to `APUIO1`
- 65c816 writes the lower byte of the destination index to `APUIO0`
- SPC700 writes the lower byte of the destination index to `APUIO0` as acknowledgment
- 65c816 waits for this acknowledgement

Each time the 65c816 sends a byte, the destination address (and the index that must be written to `APUIO0`) increases by one.

```
  ; 8-bit accumulator, 16-bit index registers
  ldx #0
Loop:
  ; Send the next byte
  lda Data,x
  sta APUIO1

  ; Send the lower byte of the current index, to tell the SPC700 that a new byte has been sent.
  txa
  sta APUIO0

  ; Wait for acknowledgement (SPC700 will echo back the lower byte)
Wait:
  cmp APUIO0
  bne Wait

  ; Move onto the next byte
  inx
  ; Has the 65c816 sent all the data yet?
  cpx #DataLength
  bne Loop
```

## Writing to a different address

After setting the address, the 65c816 can change it and start writing to a new address.

- 65c816 writes a new starting address to `APUIO2` and `APUIO3`
- 65c816 writes a value to `APUIO0` that's at least 2 higher than the previously written value
- SPC700 writes this value back to `APUIO0` as acknowledgement
- 65c816 waits for this acknowledgement

```
  ; 8-bit accumulator
  Destination = $0200 ; New address in SPC700 RAM
  lda #<Destination
  sta APUIO2
  lda #>Destination
  sta APUIO3

  lda APUIO0          ; Must be at least 2 higher than the previous APUIO0 value.
  ina
  ina
  sta APUIO0          ; Tell the SPC700 to change to a new address.

wait:                 ; Wait for the SPC700 to acknowledge this.
  cmp APUIO0
  bne wait
```

Alternatively it's possible to make a routine that will work for setting the address the first time as well as for any further changes:

```
  ; 8-bit accumulator
  Destination = $0200 ; Starting address in SPC700 RAM
  lda #<Destination
  sta APUIO2
  lda #>Destination
  sta APUIO3

  lda APUIO0          ; Must be at least 2 higher than the previous APUIO0 value
  clc                 ; but must also be $CC the first time. This works because 
  adc #$22            ; the SPC700 sets APUIO0 to $AA when it starts, and $AA+$22=$CC

  bne Skip            ; APUIO1 must be set to something other than zero, so make sure
  ina                 ; that zero is never sent.
Skip:
  sta APUIO1          ; Something other than zero, to indicate sending more data
  sta APUIO0          ; Tell the SPC700 to change to a new address.

wait:                 ; Wait for the SPC700 to acknowledge this.
  cmp APUIO0
  bne wait
```

## Running the SPC700 program

Once the 65c816 has finished sending over the program, it can tell the SPC700 to start running it.

- 65c816 writes starting address to `APUIO2` and `APUIO3`
- 65c816 writes zero to `APUIO1`
- 65c816 writes a value to `APUIO0` that's at least 2 higher than the previously written value
- SPC700 writes this value back to `APUIO0` as acknowledgement
- 65c816 waits for this acknowledgement

```
  ; 8-bit accumulator
  Destination = $0200 ; Program's address in SPC700 RAM
  lda #<Destination
  sta APUIO2
  lda #>Destination
  sta APUIO3

  stz APUIO1          ; Zero = start the program that was sent over

  lda APUIO0          ; Must be at least 2 higher than the previous APUIO0 value.
  ina
  ina
  sta APUIO0          ; Tell the SPC700 to start running the new program.

wait:                 ; Wait for the SPC700 to acknowledge this.
  cmp APUIO0
  bne wait
```

## Writing to DSP registers directly

The SPC700's bootloader can be used to write to DSP registers that only the SPC700 can access, without writing any SPC700 code. To do this, set the starting address to $00F2 and write a DSP register number, and then DSP register value. This can be repeated for however many DSP registers the 65c816 wants to change.

This is a simple way to play some sound before adding an actual SPC700 music engine to the game.

## References

- [How to Write to DSP Registers Without any SPC-700 Code](https://wiki.superfamicom.org/how-to-write-to-dsp-registers-without-any-spc-700-code)
- [lorom-template - blarggapu.s](https://github.com/pinobatch/lorom-template/blob/master/src/blarggapu.s)
- [Fullsnes - SNES APU Main CPU Communication Port](https://problemkaputt.de/fullsnes.htm#snesapumaincpucommunicationport) - includes a disassembly of the SPC700 boot ROM
