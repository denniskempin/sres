# SNES Cart Satellaview I/O Receiver Control

#### 2194h POWER (bit0) and ACCESS (bit2-3) LED Control? (R/W)

```text
  0   Usually set  <-- is ZERO by Itoi (maybe POWER LED) (see? 2196h.Bit0)
  1   Usually zero <-- is SET by Itoi                    (see? 2196h.Bit0)
  2-3 Usually both set or both cleared (maybe ACCESS LED) (Bit2 is Access LED)
  4-7 Usually zero
```

Bit2/3 are toggled by software when writing to FLASH memory. Bit0 is usually set. Might control the POWER and ACCESS LEDs on the Satellaview's Front Panel (assuming that the LEDs are software controlled). Using other values than listed above might change the LED color (assuming they are two-color LEDs).

2195h Unknown/Unused, maybe for EXT Expansion Port (?) This register isn't used by the BIOS, nor by any games. Maybe it does allow to input/output data to the Satellaview's EXT Port.

#### 2196h Status (only bit1 is tested) (R)

```text
  0    Unknown (reportedly toggles at fast speed when 2194h.Bit0-or-1? is set)
  1    Status (0=Okay, 1=Malfunction)
  2-7  Unknown/unused
```

The BIOS is using only Bit1, that bit is tested shortly after the overall hardware detection, and also during NMI handling. Probably indicates some kind of fundamental problem (like low supply voltage, missing EXPAND-Pin connection in cartridge, or no Satellite Tuner connected).

#### 2197h Control (only bit7 is modified) (R/W)

```text
  0-6  Unknown/unused (should be left unchanged)
  7    Power Down Mode? (0=Power Down, 1=Operate/Normal) (Soundlink enable?)
```

Bit7 is set by various BIOS functions, and, notably: When [7FD9h/FFD9h].Bit4 (in Satellaview FLASH File Header) is set. Also notably: Bit7 is set/cleared depending on Town Status Entry[07h].Bit6-7.

2198h Serial I/O Port 1 (R/W) 2199h Serial I/O Port 2 (R/W) These ports are basically 3-bit parallel ports, which can be used as three-wire serial ports (with clock, data.in, data.out lines) (by doing the "serial" transfer by software). Outgoing data must be written before toggling clock, incoming data can be read thereafter.

```text
  0    Clock (must be manually toggled per data bit)
  1-5  Unknown/unused (should be 0)
  6    Chip Select - For Port 1: 1=Select / For Port 2: 0=Select
  7    Data (Write=Data.Out, Read=Data.in) (data-in is directly poll-able)
```

Bits are transferred MSB first.

Unknown which chips these ports are connected to. One port does most probably connect to the 64pin MN88821 chip (which should do have a serial port; assuming that it is a MN88831 variant). The other port <might> connect to the small 8pin SPR-BSA chip?

Possible purposes might be configuration/calibration, Audio volume control, and Audio channel selection (assuming that the hardware can decode audio data and inject it to SNES Expansion Port sound inputs).

#### Serial Port 1 (2198h)

The BIOS contains several functions for sending multi-byte data to, and receiving 16bit-units from this port. Though the functions seem to be left unused? (at least, they aren't used in the low-level portion in first 32K of the BIOS).

Port 1 specific notes: When reading (without sending), the outgoing dummy-bits should be set to all zero. Chip is selected when Bit6=1. Aside from receiving data from bit7, that bit is also polled in some cases for sensing if the chip is ready (0=Busy, 1=Ready).

#### Serial Port 2 (2199h)

Data written to this port consists of simple 2-byte pairs (index-byte, data-byte), apparently to configure some 8bit registers. Used values are:

```text
  Reg[0] = 88h (or 00h when Power-Down?) (soundlink on/off?)
  Reg[1] = 80h
  Reg[2] = 04h
  Reg[3] = 00h
  Reg[4] = 08h
  Reg[5] = 00h
  Reg[6] = 70h
  Reg[7] = Not used
  Reg[8] = 00h
  Reg[9..FF] = Not used
```

There are also BIOS functions for reading 1-byte or 3-bytes from this Port, but they seem to be left unused (but, BS Dragon Quest, and Itoi are doing 24bit reads via direct I/O, whereas Itoi wants the 1st bit to be 0=ready/okay).

Port 2 specific notes: When reading (without sending), the outgoing dummy-bits should be set to all ones. Chip(-writing) is selected when Bit6=0.
