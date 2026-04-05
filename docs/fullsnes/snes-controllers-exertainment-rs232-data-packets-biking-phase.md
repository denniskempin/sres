# SNES Controllers Exertainment - RS232 Data Packets Biking Phase

#### Communication Phase

```text
  SNES Packet 00h Idle (while in menu) (also used for PPU Status Response)
  SNES Packet 01h Biking Start (start biking; probably resets time/distance)
  SNES Packet 02h Biking Active (biking)
  SNES Packet 03h Biking Pause (pause biking)
  SNES Packet 04h Biking Exit (finish or abort biking)
  SNES Packet 05h User Parameters
  SNES Packet 06h Biking ?
  SNES Packet 07h Biking ?
  SNES Packet 08h Biking ?
  Bike Packet 01h ;\these might both contain same bike data,
  Bike Packet 02h ;/both required to be send (else SNES hangs)
  Bike Packet 03h ;<-- confirms/requests pause mode
```

Unknown values & commands might include things like TV control.

#### From Bike Packet xxh (Communication Phase)

```text
  ATT      Attention Code (133h, with 9th bit aka parity set = packet start)
  00h      Command (LSB=00h..07h or so, MSB=Curr Level 0..12, Pedal Resistance)
  01h      Speed in pedal rotations per minute (0..xxx) (above 200=glitches?)
  02h      Time (MSB) in 60 second units (0..255)       ;\
  03h      Time (LSB) in 1 second units (0..59)         ;/
  04h      Calories per hour (MSB) in 256 cal/hr units  ;\this used in bank B0h
  05h      Calories per hour (LSB) in 1 cal/hr units    ;/
  06h      Calories burned (MSB) in 256/4 cal units     ;\
  07h      Calories burned (LSB) in 1/4 cal units       ;/
  08h      Distance (MSB) in 65536/3600 miles           ;\
  09h      Distance (MID) in 256/3600 miles             ;
  0Ah      Distance (LSB) in 1/3600 miles               ;/
  0Bh      Pulse in heart beats per minute (1..255 bpm, or 0=No Pulse Sensor)
  0Ch      Fit Test Score (clipped to range 10..60)
  0Dh      Whatever 8bit (invalid values CRASH combo-cart games)
  0Eh      Checksum (00h-[00h..0Dh])
```

In Fit Test mode (when [0Dh]=85h), the bike seems to send Time=Zero when the test ends (after 5 minutes) - either it's counting time backwards in that mode, or it's wrapping from 5 minutes to zero? Other modes, like workout, are counting time upwards, and end when reaching the selected time goal value.

#### From SNES Packet xxh

```text
  ACK      Acknowledge Code (33h, received packet with good checksum From Bike)
  00h      Command (LSB=01h..0xh ?, MSB=Wanted Level 0..12 Pedal Resistance)
  01h      Something (MSB=often same as above MSB, LSB=Present Hill related)
  02h      Present Hill
  03h..09h Upcoming Hills (7 bytes)
  0Ah      Whatever (in MENU: 01h,02h,00h, WORKOUT:80h, FIT-TEST:85h) 80h..86h
  0Bh      Checksum (00h-[00h..0Ah])
```

#### From SNES Packet x5h (User Parameters)

```text
  ACK      Acknowledge Code (33h, received packet with good checksum From Bike)
  00h      Command (LSB=05h, MSB=Wanted Level 0..12, Pedal Resistance)
  01h      Player's Sex (00h=Female, 01h=Male)
  02h      Player's Age in years (0..99)
  03h      Player's Weight in pounds (0..255, plus below Weight Extra)
  04h      Player's Pulse in heart beats per minute
  05h      Player's Weight Extra (added to weight, eg. 399 = values FFh+90h)
  06h..09h Garbage, set to same value as [05h], but NOT counted in checksum?
  0Ah      Whatever (same as in other "From SNES" communication packets)
  0Bh      Checksum (00h-[00h..0Ah]) --- in this case, excluding [06h..09h] ?
```

Single-cart is configuring this packet for Fit Test (but is never sending the packet)? Combo-cart is often sending this packet (but with all parameters set to zero)?
