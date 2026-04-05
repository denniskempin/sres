# SNES Controllers Miracle Piano Controller Port

#### Miracle Controller Port Transfer

#### Read Direction (invoked by SHORT Strobe signal)

```text
  1st         Data Present Flag (0=High=None, 1=Low=Yes)
  2nd..9th    Data Bit7..0      (MSB First, inverted 1=LOW=Zero)
  10th..12th  Unknown
  13th..16th  Unknown (would be ID Bit3..0 on other SNES controllers)
  17th and up Unknown
```

Write Direction (invoked by LONG Strobe signal, data output on STROBE line)

```text
  1st..8th    Data Bit7..0      (MSB First, 0=LOW=Zero)
```

Observe that read/write direction depends on length of initial Strobe signal (so games that are reading joypad with other strobe-lengths might mess up things).

10th bit and up (including the 4bit Controller ID) might be garbage (depending on how the 8051 CPU in the keyboard handles the data transfer). However, with appropriate timings, detecting a Miracle could be done via the "Firmware version request" MIDI command.

Note: The NES and SNES Miracle software expects the piano keyboard connected to Port 1, and a normal joypad connected to Port 2.

#### miracle_recv_byte

```text
  [004016h]=01h                             ;strobe on
  delay (strobe=1 for 102 master clks)      ;short delay = READ mode
  [004016h]=00h                             ;strobe off
  data_present_flag = [004016h].bit0        ;data present flag (1=LOW=Yes)
  for i=7 to 0
    data.bit(i)=NOT [004016h].bit0          ;data bits (MSB first, 1=LOW=Zero)
  next i
```

#### miracle_send_byte

```text
  [004016h]=01h                             ;strobe on (start bit)
  delay (strobe=1 for 528 master clks)      ;long delay = WRITE mode
  for i=7 to 0
    [004016h].bit0=data.bit(i)              ;data bits (MSB first, 1=HIGH=One)
    dummy=[004016h]                         ;issue short CLK pulse
  next i
  [004016h]=00h                             ;strobe off (stop/idle)
  delay (strobe=0 for min 160 master clks)  ;medium delay
```
