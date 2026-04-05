# SNES Cart X-Band I/O - Whatever Stuff (External FIFO for Modem?)

Below is some additional modem stuff (additionally to the normal Rockwell Modem registers at C180h-C1BEh). The original source code refers to that extra stuff as "modem (and serial) bits". Purpose is unknown...

Maybe the Rockwell Modem chip lacks internal FIFOs, so the VBlank handler could transfer max 60 bytes/second. As a workaround, the Fred chip might contain some sort of external FIFOs, allowing to send around 4 bytes per Vblank (which would gain 240 bytes/second, ie. gaining the full bandwidth of the 2400 baud modem).

If so, then the Fred chip should be wired either to the Rockwell databus, or to the Rockwell serial bus. Despite of the possible FIFO feature, directly accessing the Rockwell RX/TX registers seems to be also supported.

```text
C118h - MStatus1 (aka mstatus1)
  0  enModem
  1  resetModem
  2  bit_8
  3  enstop
  4  onestop
  5  enparity
  6  oddparity
  7  break

C120h - TxBuff (aka txbuff)
C128h - RxBuff (aka rxbuff)
```

Some TX/RX FIFOs?

```text
C130h - ReadMStatus2 (aka readmstatus2)
  0   kRMrxready:    rxready:         equ $01  ;1 = have rx data, 0 = no data
  1   kRMframeerr:   ltchedframeerr:  equ $02
  2   kRMparityerr:  ltchedparityerr: equ $04
  3-4 kRMframecount: sfcnt:           equ $18
  6-7 unknown/unused
```

Bit3-4 is a 2bit framecounter to tell whether a byte arrived this frame or a prev frame. It's a little wacky to use because unlike VCnt, there is no separate place to read it on Fred other than right here, sharing it with the FIFO. So, you must do the following:

If there is data in the FIFO, framecount reflects the frame number of the oldest byte in the FIFO. If the FIFO is empty, however, it reflects the current frame number. Used carefullly (i.e. make sure rxready is 0 if you are using it for the current framecount), it should allow you to determine if a byte arrived in the current frame or up to 3 previous frames ago.

```text
C140h - ReadMStatus1 (aka readmstatus1)
  0   txfull:       equ $01      ; 1 = full, 0 = not full
  1   txempty:      equ $02
  2   rxbreak:      equ $04
  3   overrun:      equ $08
  4-5 smartrxretry: equ $30   smartrxnumretry:      equ     $30
  6-7 smarttxretry: equ $c0   smarttxnumretry:      equ     $c0

C148h - Guard (aka guard)
  0-7 unknown

C150h - BCnt (aka bcnt)
  0-7 unknown (whatever... B control... or B counter?)

C158h - MStatus2 (aka mstatus2)
  0   ensmartrxretry: equ     $1 ;
  1   ensmarttxretry: equ     $2 ;
  2   smart:          equ     $4 ;
  3   sync:           equ     $8 ;
  4-7 unknown/unused

C160h - VSyncWrite (aka vsyncwrite)
  0-7 unknown
```

Maybe the vsync/vblank handler must write here by software in order to reset to "V" counters?

```text
C110h - ReadMVSyncLow (aka readmvsync)     ;<--Low?
C112h - ReadMVSyncHigh (aka readmvsynclow) ;<--low?
  Range of 0 to $61. Equal to ReadSerialVCnt/2.
  Value is $5c at start of VBlank.

C138h - ReadSerialVCnt (aka readserialvcnt)
  0-7 some incrementing counter...
             ;i\feq.a: kreadserialvcnt
             ; top 8 bits of 20 bit counter tied to
             ; input clock, or it increments 1 each 4096 clks
             ; resets to zero at vblank
             ; at 24 mhz, each 170.667 usec
             ; in 1/60 sec, counts up to 97 ($61), so
             ; range is 0 to $61 (verified by observation)

                     ;i\harddef.a: kReadSerialVCnt
                     ; Top 8 bits of 19 bit counter tied to
  kFirstVCnt equ $5c ; input clock, i.e. it increments 1 each 2048 clks.
  kLastVCnt  equ $5b ; At 24 MHz, each 85.333 usec
  kMaxVCnt   equ $61 ; in 1/60 sec, counts up to 195 ($C3), so
  kMinVCnt   equ $00 ; range is 0 to $C3 (not yet verified by testing)
                     ; Value is about $B8 at start of vblank, counts up to $C3,
                     ; wraps to 0.  Note that ReadMVSyncHigh VCnt is equal
                     ; ReadSerialVCnt/2. Note also that if there is no data
                     ; in the read fifo, it appears that ReadSerialVCnt has
                     ; the value of ReadMVSyncHigh (i.e. 1/2 the resolution)

  kVCntsPerModemBit: equ     $5 ; 1 modem bit time is 1/2400 sec, or 417 usec
                                ; 417/85.333 (1 VCnt) = 4.89, rounded up gives
                                ; 5 VCnts per modem bit. Not that this refers
                                ; to ReadSerialVCnt.

  kLinesPerModemBit: equ     $7 ; 417/64 (1 horiz line time) = 6.51, rounded up
                                ; gives 7 Lines per modem bit

  ; for rx:
  ; 1. read status until rxready
  ; 2. read serialVcnt               <-- uhm, what/why?
  ; 3. read Rxbuff (reading rxbuff clears the full fifo entry)
```
