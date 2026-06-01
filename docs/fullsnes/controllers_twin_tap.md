# SNES Controllers Twin Tap

The Twin Tap from Partyroom21 (aka Yonezawa PR21) is a special controller for 8-player quiz games. The Twin Tap itself consists of 7pin SNES controller connector with two cables, and a push-button on each cable-end (one button per player). For the 8-player mode, four Twin Taps need to be connected to a multiplayer adaptor (such like Partyroom21's own "Multi Adaptor Auto").

#### The Twin Tap is supported by

```text
  Shijou Saikyou no Quiz Ou Ketteisen Super (1992) TBS/Partyroom21/S'pal (JP)
```

#### Transfer Protocol

```text
  1st         Button 2 (or 4/6/8) (1=Low=Pressed) (would be "B" on joypads)
  2nd         Button 1 (or 3/5/7) (1=Low=Pressed) (would be "Y" on joypads)
  3rd..12th   Unknown
  13th..16th  Unknown (would be ID Bit3..0 on other SNES controllers)
  17th..24th  Unknown (would be Extended ID Bit7..0 on other SNES controllers)
  25th and up Unknown
```

Judging from disassembled game code, the 4bit ID might be 00h or 0Eh, in the latter case, there <should> be also a unique Extended ID value.
