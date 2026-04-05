# RTC S-3520 (Real-Time Clock)

Seiko/Epson S-3520CF Serial 4bit Real-Time Clock (RTC) Contains the usual Time/Date registers, plus 120bit battery-backed RAM (aka 15 bytes) (organized in 2 pages of 15 x 4bits).

This chip is used in both Nintendo Super System (NSS), and in Super Famicom Box.

#### Seiko/Epson S-3520CF Register Table

```text
  Index  Bit3   Bit2   Bit1    Bit0  ;Expl.
  ___Registers in Mode 0_____________ ______________
  0      Sec3   Sec2   Sec1    Sec0  ;Seconds, Low     ;\
  1      0      Sec6   Sec5    Sec4  ;Seconds, High    ;
  2      Min3   Min2   Min1    Min0  ;Minutes, Low     ; Read/Increment-able
  3      0      Min6   Min5    Min4  ;Minutes, High    ;
  4      Hour3  Hour2  Hour1   Hour0 ;Hours, Low       ; (reading returns the
  5      PM/AM  0      Hour5   Hour4 ;Hours, High      ; counter value)
  6      0      Week2  Week1   Week0 ;Day of Week      ;
  7      Day3   Day2   Day0    Day0  ;Day, Low         ; (writing any dummy
  8      0      0      Day5    Day4  ;Day, High        ; value does increment
  9      Mon3   Mon2   Mon1    Mon0  ;Month, Low       ; counter value by 1)
  A      0      0      0       Mon4  ;Month, High      ;
  B      Year3  Year2  Year1   Year0 ;Year, Low        ;
  C      Year7  Year6  Year5   Year4 ;Year, High       ;/
  D      TPS    30ADJ  CNTR    24/12 ;Control Register ;-Read/Write-able
  E      STA    LOST   0       0     ;Status Register  ;-Read only
  ___Registers in Mode 1_____________ ________________
  0-E    x      x      x       x     ;Reserved         ;-Don't use
  ___Registers in Mode 2_____________ ________________
  0-E    SRAM   SRAM   SRAM    SRAM  ;SRAM Page 0      ;-Read/Write-able
  ___Registers in Mode 3_____________ ________________
  0-E    SRAM   SRAM   SRAM    SRAM  ;SRAM Page 1      ;-Read/Write-able
  ___Mode Register (in Mode 0..3)____ ________________
  F      SYSR   TEST   Mode1   Mode0 ;Mode Register    ;-Read/Write-able
```

Whereas, the meaning of the various bits is:

```text
  Sec    Seconds (BCD, 00h..59h)
  Min    Minutes (BCD, 00h..59h)
  Hour   Hours   (BCD, 00h..23h or 01h..12h)
  Day    Day     (BCD, 01h..31h)
  Month  Month   (BCD, 01h..12h)
  Year   Year    (BCD, 00h..99h)
  Week   Day of Week (0..6) (SFC-Box: Unknown assignment) (NSS: 0=Sunday)
  PM/AM  Set for PM, cleared for AM (this is done even when in 24-hour mode)
  24/12  24-Hour Mode (0=12, 1=24) (Time/Date may get corrupted when changed?)
  TPS    Select Reference Waveform for output on Pin8 (0=1024Hz, 1=1Hz)
  30ADJ  Set seconds to zero, and, if seconds was>=30, increase minutes
  CNTR   Reset Counters (0=Normal, 1=Reset)
  SYSR   Reset Counters and Control/Status/Mode Registers (0=Normal, 1=Reset)
  LOST   Time Lost (0=Okay, 1=Lost/Battery failure) (can be reset... how?)
  STA    Time Stable (0=Stable/Sec won't change in next 3.9ms, 1=Unstable)
  Mode   Mode for Register 0-E (0=RTC, 1=Reserved, 2=SramPage0, 3=SramPage1)
```

If STA=0 then it's safe to read the time (counters won't change within next 3.9ms aka 1/256 seconds). If STA=1 then one should wait until STA=0 before reading the time (else one may miss counter-carry-outs).

#### Serial Access

Set /CLK and /CS to HIGH as default level. Set /WR to desired direction (before dragging /CS low). Then set /CS to LOW to invoke transfer. Then transfer index/data/garbage (usually 8 clks for WRITES, and 16 clks for READS). Then set /CS back HIGH.

Index/Data/Garbage Nibbles are 4bit each (transferred LSB first). Bits should be output (to DataIn) on falling CLK edge (note: the NSS is doing that properly, the SFC-Box actually outputs data shortly after falling CLK), and can be read (from DataOut) at-or-after raising CLK edge. The separate nibbles are:

```text
  Nibble   To RTC                       From RTC
  1st      Index I                      Garbage (old index or so)
  2nd      Data I    (or dummy)         Garbage (data from old index or so)
  3rd      Index II  (or dummy)         Garbage (index I or so)
  4th      Data II   (or dummy)         Data I
  5th      Index III (or dummy)         Garbage (index II or so)
  6th      Data III  (or dummy)         Data II
```

For Writes, one needs to send only 2 nibbles (of which, 2nd nibble is used only for Control & SRAM writes, for Counter-Increment writes it's only a dummy value).

For Reads, one needs to send/receive at least 4 nibbles (though most of them are dummies/garbage; actually used are 1st-To-RTC, and 4th-From-RTC). If desired, one can read two or more registers by reading/writing 6 or more nibbles (the NSS BIOS does so).

#### Pin-Outs

> **See:** [SNES Pinouts RTC Chips](snes-pinouts-rtc-chips.md)
