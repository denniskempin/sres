# SNES Cart X-Band I/O - LED and Debug

```text
C168h - LEDData (aka leddata)
  0-7  probably controls the LEDs (can be also used for other stuff)
```

Note: Sega version has 7 LEDs, SNES version has only 3 LEDs. Unknown which of the 8 bits are controlling which LEDs.

```text
C16Ah - LEDEnable (aka ledenable)
  0-7  seems to select data-direction for LED pins (0=input, 1=output)
```

#### Debug Connection via LED ports

People at Catapult have reportedly used modified X-Band PCBs during debugging:

The seven genesis LEDs replaced by a DB25 connector with 8 wires (7 debug signals, plus GND, probably connected to a PC parallel/printer port). That hardware mod also used special software (some custom X-Band BIOS on FLASH/ROM, plus whatever software on PC side).

#### Unknown 64bit Number via LED ports

The SNES X-Band BIOS is reading a 64bit number via serial bus (which might connect to exteral debug hardware, or to 'unused' smart card pins, or to whatever), done via two I/O Ports:

```text
  FBC168h Data        ;bit2 (data, in/out)       ;\there is maybe also a reset
  FBC16Ah Direction   ;bit2 (0=input, 1=output)  ;/flag, eventually in bit5 ?
```

The sequence for reading the 64bits is somewhat like so:

```text
  Data=Output(0), Delay (LOOPx01F4h)
  Data=Output(1), Delay (LOOPx01F4h)
  Data=Input
  wait until Data=1 or fail if timeout
  wait until Data=0 or fail if timeout
  wait until Data=1 or fail if timeout
  Delay (LOOPx02BCh)
  for i=1 to 4
    Data=Output(0), Delay (NOPx8)
    Data=Output(1), Delay (NOPx8)
    Data=Input, Delay (LOOPx0050h)
  for i=1 to 4
    Data=Output(0), Delay (NOPx8)
    Data=Output(1), Delay (LOOPx003Ch)
    Data=Input, Delay (LOOPx001Eh)
  Data=Input, Delay (LOOPx0064h)
  for i=0 to 63
    Data=Output(1), Delay (NOPx8)
    Data=Input, Delay (LOOPx000Ah)
    key.bit(i)=Data, Delay (LOOPx004Bh)
```

For the exact timings (Delays and other software overload), see the BIOS function (at D7BE78h). Before doing the above stuff, the BIOS initializes [FBC168h]=40h, [FBC16Ah]=FFh (this may be also required).

The 64bit number is received LSB first, and stored in SRAM at 3FD8h-3FDFh.

Whereas the last byte is a checksum across the first 7 bytes, calculated as so:

```text
  sum=00h
  for i=0 to 55
    if (sum.bit(0) xor key.bit(i))=1 then sum=sum/2 xor 8Ch else sum=sum/2
```

For example, if the 7 bytes are "testkey", then the 8th byte must be 2Fh. Or, another simplier example would be setting all 8 bytes to 00h.
