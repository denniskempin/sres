# SNES Cart SPC7110 with RTC-4513 Real Time Clock (1 game)

RTC from Epson/Seiko. Used by one game from Hudson Soft:

```text
  Far East of Eden Zero (with RTC-4513) (1995) Red Company/Hudson Soft (JP)
```

#### SPC7110 I/O Ports for RTC-4513 Access

```text
  4840h RTC Chip Select (bit0: 0=Deselect: CE=LOW, 1=Select: CE=HIGH)
  4841h RTC Data Port   (bit0-3: Command/Index/Data)
  4842h RTC Status      (bit7: 1=Ready, 0=Busy) (for 4bit transfers)
```

#### Usage

Switch CE from LOW to HIGH, send Command (03h=Write, 0Ch=Read), send starting Index (00h..0Fh), then read or write one or more 4bit Data units (index will automatically increment after each access, and wraps from 0Fh to 00h at end of data stream). Finally, switch CE back LOW.

#### Epson RTC-4513 Commands

```text
  03h    Write-Mode
  0Ch    Read-Mode
```

#### Epson RTC-4513 Register Table

```text
  Index  Bit3   Bit2   Bit1    Bit0   Expl.
  0      Sec3   Sec2   Sec1    Sec0   Seconds, Low
  1      LOST   Sec6   Sec5    Sec4   Seconds, High
  2      Min3   Min2   Min1    Min0   Minutes, Low
  3      WRAP   Min6   Min5    Min4   Minutes, High
  4      Hour3  Hour2  Hour1   Hour0  Hours, Low
  5      WRAP   PM/AM  Hour5   Hour4  Hours, High
  6      Day3   Day2   Day0    Day0   Day, Low     ;\
  7      WRAP   RAM    Day5    Day4   Day, High    ;
  8      Mon3   Mon2   Mon1    Mon0   Month, Low   ; or optionally,
  9      WRAP   RAM    RAM     Mon4   Month, High  ; 6x4bit User RAM
  A      Year3  Year2  Year1   Year0  Year, Low    ;
  B      Year7  Year6  Year5   Year4  Year, High   ;/
  C      WRAP   Week2  Week1   Week0  Day of Week
  D      30ADJ  IRQ-F  CAL/HW  HOLD   Control Register D
  E      RATE1  RATE0  DUTY    MASK   Control Register E
  F      TEST   24/12  STOP    RESET  Control Register F
```

Whereas, the meaning of the various bits is:

```text
  Sec    Seconds (BCD, 00h..59h)
  Min    Minutes (BCD, 00h..59h)
  Hour   Hours   (BCD, 00h..23h or 01h..12h)
  Day    Day     (BCD, 01h..31h)
  Month  Month   (BCD, 01h..12h)
  Year   Year    (BCD, 00h..99h)
  Week   Day of Week (0..6) (Epson suggests 0=Monday as an example)
  PM/AM  Set for PM, cleared for AM (is that also in 24-hour mode?)
  WRAP   Time changed during access (reset on CE=LOW, set on seconds increase)
  HOLD   Pause clock when set (upon clearing increase seconds by 1 if needed)
  LOST   Time lost (eg. battery failure) (can be reset by writing 0)
  IRQ-F  Interrupt Flag (Read-only, set when: See Rate, cleared when: See Duty)
  RATE   Interrupt Rate (0=Per 1/64s, 1=Per Second, 2=Per Minute, 3=Per Hour)
  DUTY   Interrupt Duty (0=7.8ms, 1=Until acknowledge, ie. until IRQ-F read)
  MASK   Interrupt Disable (when set: IRQ-F always 0, STD.P always High-Z)
  TEST   Reserved for Epson's use (should be 0) (auto-cleared on CE=LOW)
  RAM    General purpose RAM (usually 3bits) (24bits when Calendar=off)
  CAL/HW Calendar Enable (1=Yes/Normal, 0=Use Day/Mon/Year as 24bit user RAM)
  24/12  24-Hour Mode (0=12, 1=24) (Time/Date may get corrupted when changed!)
  30ADJ  Set seconds to zero, and, if seconds was>=30, increase minutes
  STOP   Stop clock while set (0=Stop, 1=Normal)
  RESET  Stop clock and reset seconds to 00h (auto-cleared when CE=LOW)
```

If WRAP=1 then one must deselect the chip, and read time/date again.

Serial data is transferred LSB first.

On-chip 32.768kHz quartz crystal.

#### Pin-Outs

> **See:** [SNES Pinouts RTC Chips](snes-pinouts-rtc-chips.md)
