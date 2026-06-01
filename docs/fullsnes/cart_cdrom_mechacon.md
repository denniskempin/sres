# SNES Cart CDROM - Mechacon

The Mechacon handles all the drive mechanics (motor start/stop, seeking, tracking, gain, balance). Essentinally it's covering only the "Audio" part (streaming bits and watching the SubQ-channel's position info) without being aware of "Digital" data in CDROM Headers & Data Blocks.

However, the same mechanics are also used for "Playing" CDROM data discs (ie. seek the desired sector in MM:SS:FF notation, then issue Play command to start reading).

Observe that seeking may inaccuratly settle "nearby" of the desired target address (ie. one must check the Data header's MM:SS:FF bytes from the Decoder chip, and ignore any sectors with smaller sector numbers, or eventually retry seeking if the sector number is higher as planned).

21E1h.R/W - CDROM Unit Mechacon CPU (probably the NEC chip on daughterboard)

```text
  7     Transfer Ready IRQ      (R)
  6-4   -
  3-0   Data                    (R/W)
```

#### Mechacon Commands

```text
  Access MM/SS/FF     BmmssffF                --> FFFFFFFx
  Access Track/Index  CttiiF                  --> FFFFFx
  Stop                D01F                    --> FFFx
  Play                D02F                    --> FFFx
  Pause               D03F                    --> FFFx
  Open/Close          D04F                    --> FFFx
  Fast Forward        D10F                    --> FFFx
  Fast Reverse        D11F                    --> FFFx
  Forward             D12F                    --> FFFx
  Reverse             D13F                    --> FFFx
  Key Direct          D40F                    --> FFFx
  Key Ignore          D41F                    --> FFFx
  Continous Play      D42F                    --> FFFx
  Auto Track Pause    D43F                    --> FFFx
  Auto Index Pause    D44F                    --> FFFx
  Normal Speed        D45F                    --> FFFx
  Double Speed        D46F                    --> FFFx
  Q-Data Request      D50F 0000000000000000F  --> FFFx ................x
  Status Request      D51F 01234F             --> FFFx .....x
  Nop/Flush ?         F                       --> x
```

#### Q-Data Request Digits

These 16 digits are probably 8 bytes straight from 12-byte SubQ Position data in BCD format (probably Track, Index, MM:SS:FF, AMM:ASS:AFF) (ie. probably excluding the ADR/Control byte, Reserved byte, and the two CRC bytes).

#### Status Request Digits  Digit(0) - Disc Type

```text
  Bit0: Disc Type (or maybe Track Type) (0=Audio, 1=Data)
  Bit1-3: Unknown/unused
```

#### Digit(1)

```text
  Unknown/unused
```

#### Digit(2) - Drive state

```text
  00h  No Disc
  01h  Stop
  02h  Play
  03h  Pause
  04h  Fast Reverse
  05h  Fast Forward
  06h  Slow Reverse
  07h  Slow Forward
  08h  ?
  09h  ?
  0Ah  Access, Seek
  0Bh  Access, Read TOC
  0Ch  Tray Open
  0Dh  ?
  0Eh  ?
  0Fh  ?
```

#### Digit(3)

```text
  Unknown/unused
```

#### Digit(4)

```text
  Unknown/unused
```

Unknown bits & digits might include double-speed flag, LCD pad buttons, or such stuff.
