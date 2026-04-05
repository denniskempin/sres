# SNES Controllers Miracle Piano MIDI Commands

The Miracle is always using MIDI messages (no matter if the messages are transferred through MIDI or RS232 or NES/SNES/Genesis controller cables). Below lists the supported MIDI messages (including "Undocumented" messages, which are used by the Miracle's SNES software, although they aren't mentioned in the Miracle's Owner's Manual).

#### MIDI Information Sent FROM/TO The Miracle keyboard

```text
  Expl.                     Dir  Hex
  Note off (Undocumented)     W  8#h,<key>,00h   ;same as Note ON with velo=00h
  Note on/off command       R/W  9#h,<key>,<velo>
  Main volume level           W  B0h,07h,<vol>
  Sustain on/off command    R/W  B#h,40h,<flag>
  Local control on/off        W  B0h,7Ah,<flag>
  All notes off               W  B#h,7Bh,00h
  Patch change command (*)  R ?? C#h,<instr>     ;TO keyboard = Undocumented
  Miracle button action     R    F0h,00h,00h,42h,01h,01h,<bb>,F7h
  Unknown (Undocumented)      W  F0h,00h,00h,42h,01h,02h,<??>,F7h   ;???
  Keyboard buffer overflow  R    F0h,00h,00h,42h,01h,03h,01h,F7h
  Midi buffer overflow      R    F0h,00h,00h,42h,01h,03h,02h,F7h
  Firmware version request    W  F0h,00h,00h,42h,01h,04h,F7h
  Miracle firmware version  R    F0h,00h,00h,42h,01h,05h,<maj>,<min>,F7h
  Patch split command         W  F0h,00h,00h,42h,01h,06h,0#h,<lp>,<up>,F7h
  Unknown (Undocumented)      W  F0h,00h,00h,42h,01h,07h,F7h        ;???
  All LEDs on command         W  F0h,00h,00h,42h,01h,08h,F7h
  LEDs to normal command      W  F0h,00h,00h,42h,01h,09h,F7h
  Reset (Undocumented)        W  FFh
```

Direction: R=From keyboard, W=To keyboard

Notes: (*) Patch change FROM Keyboard is sent only in Library mode.

```text
  N#h         Hex-code with #=channel (#=0 from keyb, #=0..7 to keyb)
  <key>       Key (FROM Miracle: 24h..54h) (TO Miracle: 18h..54h/55h?)
  <velo>      Velocity (01h..7Fh, or 00h=Off)
  <vol>       Volume (00h=Lowest, 7Fh=Full)
  <flag>      Flag (00h=Off, 7Fh=On)
  <instr>     Instrument (00h..7Fh) for all notes
  <lp>        Instrument (00h..7Fh) for notes 24?/36-59, lower patch number
  <up>        Instrument (00h..7Fh) for notes 60-83/84?, upper patch number
  <maj>.<min> Version (from version 1.0 to 99.99)
  <bb>        button on/off (bit0-2:button number, bit3:1=on, bit4-7:zero)
```

Data from piano is always sent on first channel (#=0). Sending data to piano can be done on first 8 channels (#=0..7), different instruments can be assigned to each channel. Although undocumented, the SNES software does initialize 16 channels (#=0..0Fh), unknown if the hardware does support/ignore those extra channels (from the instrument table: it sounds as if one could use 16 single-voice channels or 8 dual-voice channels).
