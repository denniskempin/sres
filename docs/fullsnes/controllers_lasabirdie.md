# SNES Controllers Lasabirdie (golf club)

The Lasabirdie is a golf club made in 1995 by Ricoh. Supported by only one game (which came shipped with the device):

```text
  Lasabirdie - Get in the Hole (1995) Ricoh/Good House (JP)
```

#### Lasabirdie "Golf Mat" and "Golf Club"

```text
     _______________________________                       __
    |...............................|      insert two     / /
    |:                             :|      AA batteries  / / <-- handle
    |: #      Golf Ball          # :|            |      / /      (rather
    |: #   _ /                   # :|            |     / /       short)
    |: #  (_)       #            # :|            V    / /
    |: #                         # :|          ______/ /
    |: #                         # :|         |       /
   _|:.............................:|          \     / <-- yellow laser symbol
  / |  LT RT   UP DN   A  B   RICOH |           \___/
  | |_______________________________|             |
  / <---- cable (to SNES controller port 2)       |  <-- laser beam
```

The so-called Golf Mat is actually a (not very flat) plastic box, with a mimmicked (half) golf ball mounted on it, the three black fields (shown as ### in the ASCII drawing) might contain laser sensors, the front panel has six buttons: Left/Pause, Right, Up, Down, A/Start and B/Cancel.

For additional/better menu controls, one can connect a normal joypad to SNES port 1.

Below describes the overall transfer protocol. Unknown if/how/what kind of motion, speed, and/or direction information is transmitted via that protocol.

#### Lasabirdie Controller Data (connected to Port 2)

```text
  1st         Button B (CANCEL)    (1=Low=Pressed)
  2nd         Button DOWN          (1=Low=Pressed)
  3rd..6th    Nibble Data bit3-0   (1=Low=One?) (MSB first)
  7th         Nibble Available     (toggles CLK like)
  8th         Packet Available     (1=Low=Yes)
  9th         Button A (START)     (1=Low=Pressed)
  10th        Button UP            (1=Low=Pressed)
  11th        Button LEFT (PAUSE)  (1=Low=Pressed)
  12th        Button RIGHT         (1=Low=Pressed)
  13th..16th  ID Bit3-0       (unknown)  ;read, but not checked by software
  17th..24th  Extra ID Bit7-0 (unknown)  ;read, but not checked by software
  25th and up Unknown/Unused (probably all one, or all zero ?)
```

#### Command Bytes

Command bytes are used to select a specific packet, and (during the packet transfer) to select nibbles within the previously selected packet:

```text
  20h        select "GOLF_READY!" ID string packet
  22h        select version string packet
  30h        select whatever data packet?
  3Fh        select whatever data packet?
  40h..55h   sent while receiving nibbles number 0..21
  5Fh        terminate transfer (or re-select default packet type?)
```

Bytes are output via Port 4201h at a fixed baudrate of circa 10000 bits/second:

```text
  output 1 start bit ("0")
  output 8 data bits (MSB first)
  output 2 stop bits ("0","0")
  release line (output "1", until the next byte transferred in next frame)
```

Exact time per bit is 2140 master cycles (10036 bps at 21.47727MHz NTSC clock).

#### Packets

Packets consist of 11 bytes, transferred in 22 nibbles (of 4bit each). For whatever reason, the software receives only one nibble per frame, so a complete packet-transfer takes about 0.36 seconds. The bits are transferred MSB first (bit3,bit2,bit1,bit0), whilst nibbles are transferred LSB first (bit3-0, bit7-4). The 11-byte packets can contain following data:

```text
  "GOLF_READY!"                 ;-ID-string packet  ;\without checksum
  FFh,FFh,0,0,0,0,0,0,0,0,0     ;-Empty packet      ;/
  9 chars, 1 unknown, 1 chksum  ;-Version-string    ;\with checksum
  10 data bytes, 1 chksum       ;-Normal packet     ;/
```

The checksum (if it is present) is calculated by summing up all 10 data bytes, and adding MSB+LSB of the resulting 16bit sum (ie. sum=sum+sum/100h). The version string packet contains 9 characters (unknown content), one unused byte (unknown value), and the checksum byte. Other packet(s) contain whatever controller/motion data (unknown content).

Below is the procedure for receiving a packet (before doing that, one should first select a packet, eg. send_byte(20h) for receiving the ID string).

```text
  if [421Ah].bit8 = 0 then exit       ;-exit if no packet available
  for i=0 to 21
    old_state = [421Ah].bit9
   @@wait_lop:
    send_byte(40h+i)
    wait_vblank
    if [421Ah].bit9 <> old_state then jmp @@wait_done
    wait_vblank
    if [421Ah].bit9 <> old_state then jmp @@wait_done
    jmp @@wait_lop
   @@wait_done:
    nibble=([421Ah] SHR 10) AND 0Fh
    if (i AND 1)=0 then buf[i/2]=nibble, else buf[i/2]=buf[i/2]+nibble*10h
  next i
  send_byte(5Fh)                ;-terminate transfer or so
```
