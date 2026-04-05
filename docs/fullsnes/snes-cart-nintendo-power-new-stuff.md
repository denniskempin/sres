# SNES Cart Nintendo Power - New Stuff

#### Operation during /RESET=LOW

```text
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=F0h   ;FLASH read/reset command
  [00000h]=38h, [00000h]=D0h, [00000h]=71h   ;FLASH request chip info part 1
  dummy=[00004h]                             ;Read Ready-status (bit7=1=ready)
  [00000h]=72h, [00000h]=75h                 ;FLASH request chip info part 2
  Port[2404h..2407h]=[0FF00h+(n*8)+0,2,4,6]  ;Read mapping info for File(n)
  [0AAAAh]=AAh, [05554h]=55h, [0AAAAh]=F0h   ;FLASH read/reset command
```

#### Detailed

```text
  [00000h]=38h   ;copy hidden sector to page buffer?
  [00000h]=D0h   ;  ...confirm above
  [00000h]=71h   ;read extended status register
  dummy=[00004h] ;  ...do read it (bit7=1=ready)
  [00000h]=72h   ;swap page buffer (map above buffer to cpu side?)
  [00000h]=75h   ;read page buffer to cpu
  xx=[0FFxxh]    ;  ...do read it
```

other interesting commands:

```text
  [00000h]=74h   ;write page buffer single byte from cpu
  [0xxxxh]=xx    ;  ...do write it
```

or sequential:

```text
  [00000h]=E0h   ;sequential load from cpu to page buffer
  [00000h]=num.L ;  ...byte count lsb (minus 1 ?) (0=one byte, what=100h bytes)
  [00000h]=num.H ;  ...byte cound msb (zero)
  [?]=data       ;  ...data?
  [00000h]=0Ch   ;forward page buffer to flash
  [00000h]=num.L ;  ...byte count lsb (minus 1 ?) (0=one byte, what=100h bytes)
  [addr]=num.H   ;  ...byte cound msb (zero)
  [...]?         ;  ...do something to wait until ready
```

Hidden Mapping Info Example (chip 1 at C0FFxxh, chip 2 at E0FFxxh)

```text
  C0FF00      03 11 AA 74 AA 97 00 12  ;Menu              (512K Lorom, no SRAM)
  C0FF08      00 08 29 15 4A 12 10 01  ;Super Mario World (512K Lorom, 2K SRAM)
  C0FF10      0B FF AA FF AA FF 21 FF  ;Doraemon 4        (1.5M Lorom, no SRAM)
  C0FF18      49 FF 61 FF A5 FF 51 FF  ;Dragon Slayer II  (1.5M Hirom, 8K SRAM)
  C0FF20..FF  FF-filled (byte at C0FF7Fh is 00h in some carts)  ;-unused
  E0FF00..8F  FF-filled (other values at E0FF8xh in some carts) ;\garbage, from
  E0FF90      FF FF 55 00 FF FF FF FF FF FF FF FF FF FF FF FF   ; chip-testing
  E0FFA0      FF FF FF FF FF FF 55 00 FF FF FF FF FF FF FF FF   ; or so
  E0FFB0      FF FF FF FF FF FF 55 00 FF FF FF FF FF FF FF FF   ;/
  E0FFC0..FF  FF-filled                                         ;-unused
```

There are always 8 bytes at odd addresses at C0FF01..0F, interleaved with the mapping entries 0 and 1 (though no matter if the cart uses 1, 2, or 3 mapping entries). The 'odd' bytes are some serial number, apart from the first two bytes, it seems to be just a BCD date/time stamp, ie. formatted as 11-xx-YY-MM-DD-HH-MM-SS.

New findings are that the "xx" in the "11-xx-YY-MM-DD-HH-MM-SS" can be non-BCD (spotted in the Super Puyo Puyo cart).

Some carts have extra 'garbage' at C0FF7F and E0FF80..BF.

#### Nintendo Power Commands

```text
  if [002400h]<>7Dh then skip unlocking   ;else locking would be re-enabled
  [002400h]=09h       ;\
  dummy=[002400h]     ;
  [002401h]=28h       ; wakeup sequence (needed before sending other commands,
  [002401h]=84h       ; and also enables reading from port 2400h..2407h)
  [002400h]=06h       ;
  [002400h]=39h       ;/
```

After wakeup, single-byte commands can be written to [002400h]:

```text
  [002400h]=00h   RESET and map GAME14 ? (issues /RESET pulse)
  [002400h]=01h    causes always 8x7D
  [002400h]=02h   Set STATUS.bit2=1 (/WP=HIGH, release Write protect)
  [002400h]=03h   Set STATUS.bit2=0 (/WP=LOW, force Write protect)
  [002400h]=04h   HIROM:ALL  (map whole FLASH in HiROM mode)
  [002400h]=05h   HIROM:MENU (map MENU in HiROM mode instead normal LoROM mode)
  [002400h]=06h    causes always 8x7D (aka, undoes toggle?)
  [002400h]=07h    causes always 8x7D
  [002400h]=08h    causes always 8x7D
  [002400h]=09h    no effect  ;\
  [002400h]=0ah    no effect  ;/
  [002400h]=0bh    causes always 8x7D
  [002400h]=0ch    causes always 8x7D
  [002400h]=0dh    causes always 8x7D
  [002400h]=0eh    causes always 8x7D
  [002400h]=0fh    causes always 8x7D
  [002400h]=10h    causes always 8x7D
  [002400h]=14h    causes always 8x7D
  [002400h]=20h    Set STATUS.bit3=0 (discovered by skaman) (default)
  [002400h]=21h    Set STATUS.bit3=1 (discovered by skaman) (disable ROM read?)
  [002400h]=24h    causes always 8x7D
  [002400h]=44h    no effect (once caused crash with green rectangle)
  [002400h]=80h..8Fh  ;-Issue /RESET to SNES and map GAME 0..15
  [002400h]=C5h    causes always 8x7D
  [002400h]=FFh    sometimes maps GAME14 or GAME15? (unreliable)
```
