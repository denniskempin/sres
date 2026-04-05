# SNES Cart S-RTC (Realtime Clock) (1 game)

PCB "SHVC-LJ3R-01" with 24pin "Sharp S-RTC" chip. Used only by one japanese game:

```text
  Dai Kaiju Monogatari 2 (1996) Birthday/Hudson Soft (JP)
```

#### S-RTC I/O Ports

```text
  002800h S-RTC Read  (R)
  002801h S-RTC Write (W)
```

Both registers are 4bits wide. When writing: Upper 4bit should be zero. When reading: Upper 4bit should be masked-off (they do possibly contain garbage, eg. open-bus).

#### S-RTC Communication

The sequence for setting, and then reading the time is:

```text
  Send <0Eh,04h,0Dh,0Eh,00h,Timestamp(12 digits),0Dh> to [002801h]
  If ([002800h] AND 0F)=0Fh then read <Timestamp(13 digits)>
  If ([002800h] AND 0F)=0Fh then read <Timestamp(13 digits)>
  If ([002800h] AND 0F)=0Fh then read <Timestamp(13 digits)>
  If ([002800h] AND 0F)=0Fh then read <Timestamp(13 digits)>
  etc.
```

The exact meaning of the bytes is unknown. 0Eh/0Dh seems to invoke/terminate commands, 04h might be some configuration stuff (like setting 24-hour mode).

00h is apparently the set-time command. There might be further commands (such like setting interrupts, alarm, 12-hour mode, reading battery low & error flags, etc.). When reading, 0Fh seems to indicate sth like "time available".

The 12/13-digit "SSMMHHDDMYYY(D)" Timestamps are having the following format:

```text
  Seconds.lo  (BCD, 0..9)
  Seconds.hi  (BCD, 0..5)
  Minutes.lo  (BCD, 0..9)
  Minutes.hi  (BCD, 0..5)
  Hours.lo    (BCD, 0..9)
  Hours.hi    (BCD, 0..2)
  Day.lo      (BCD, 0..9)
  Day.hi      (BCD, 0..3)
  Month       (HEX, 01h..0Ch)
  Year.lo     (BCD, 0..9)
  Year.hi     (BCD, 0..9)
  Century     (HEX, 09h..0Ah for 19xx..20xx)
```

When READING the time, there is one final extra digit (the existing software doesn't transmit that extra digit on WRITING, though maybe it's possible to do writing, too):

```text
  Day of Week? (0..6) (unknown if RTC assigns sth like 0=Sunday or 0=Monday)
```

#### Pinouts

> **See:** [SNES Pinouts RTC Chips](snes-pinouts-rtc-chips.md)

#### Note

There's another game that uses a different RTC chip: A 4bit serial bus RTC-4513 (as made by Epson) connected to a SPC7110 chip.
