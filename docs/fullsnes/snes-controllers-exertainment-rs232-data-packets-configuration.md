# SNES Controllers Exertainment - RS232 Data Packets & Configuration

From Bike to SNES (16 bytes: ATT code, command, 13-byte-data, checksum)

```text
  Bike Packet 01h ;\these might both contain same bike data,
  Bike Packet 02h ;/both required to be send (else SNES hangs)
  Bike Packet 03h ;<-- confirms/requests pause mode
  Bike Packet 08h Login Part 1 (ID string)
  Bike Packet 09h PPU Status Request (with ignored content)
  Bike Packet 0Ah Login Part 3 (reply to random values)
  Bike Packet 0Ch Login Part 5 (fixed values 00,FF,00,0C,..)
```

From SNES to Bike (13 bytes: ACK code, command, 10-byte-data, checksum)

```text
  SNES Packet 00h Idle (zerofilled) or PPU Status Response (with "RAD" string)
  SNES Packet 01h Biking Start (start biking; probably resets time/distance)
  SNES Packet 02h Biking Active (biking)
  SNES Packet 03h Biking Pause (pause biking)
  SNES Packet 04h Biking Exit (finish or abort biking)
  SNES Packet 05h User Parameters
  SNES Packet 06h Biking ?
  SNES Packet 07h Biking ?
  SNES Packet 08h Biking ?
  SNES Packet 09h Login Part 2 (random values)
  SNES Packet 0Bh Login Part 4 (based on received data)
  SNES Packet 0Dh Login Part 6 (login okay)
  SNES Packet 0Fh Logout (login failed, or want new login)
  SNES Packet 0Ah,0Ch,0Eh (?) (unused?)
```

#### Packet Details

> **See:** [SNES Controllers Exertainment - RS232 Data Packets Login Phase](snes-controllers-exertainment-rs232-data-packets-login-phase.md)
> **See:** [SNES Controllers Exertainment - RS232 Data Packets Biking Phase](snes-controllers-exertainment-rs232-data-packets-biking-phase.md)

#### RS232 Character Format

The character format is initialized as [21C3h]=3Bh, which means,

```text
  1 start bit, 8 data bits, sticky parity, 1 stop bit, or, in other words:
  1 start bit, 9 data bits, no parity, 1 stop bit
```

The sticky parity bit (aka 9th data bit) should be set ONLY in the Bike's ATT characters (133h), all other data (and ACK codes) should have that bit cleared.

#### RS232 Baudrate

The baudrate is aimed at 9600 bits/sec. The ACE Baudrate Divisor is set to 0023h aka 35 decimal (in both NTSC and PAL versions), with the ACE being driven by the 5.3MHz Dot Clock. The resulting exact timings are:

```text
  NTSC: 5.36931750MHz/35/16 = 9588.067 Hz
  PAL:  5.32034250MHz/35/16 = 9500.612 Hz
```

Notes: The Dot Clock has some slight stuttering on long dots during hblank (but doesn't disturb the baudrate too much). The PAL baudrate doesn't match too well, however, it is the divisor setting closest to 9600 baud.

#### RS232 Handshaking

The RS232 connector has only 3 pins (RX, TX, GND). The RTS/CTS handshaking signals are thus not used (nor are any Xon/Xoff handshaking characters used).

However, there is some sort of handshaking: The "From Bike" packets are preceeded by a ATT (Attention) character (value 133h, with 9th data bit set, aka sticky parity bit set), this allows to resynchronize to packet-start boundaries in case of lost data bytes.

In the other direction, the "From SNES" packets should be sent only in response to successfully received "From Bike" packets.

The packets are small enough to fit into the 16-byte FIFOs of the ACE chip. The baudrate is a bit too low to send 16-byte packets in every frame, so the Bike is apparently pinging out packets at some lower rate.

Note: The SNES software accepts the ATT (Attention) characters only if [21C5h] returns exactly E5h (data present, TX fifo empty, and "error" flags indicating the received parity bit being opposite as normal).

#### RS232 Interrupts

The ACE Interrupts are left unused: the IRQ pin is probably not connected to SNES, ACE interrupts are disabled via [21C1h]=00h, and ACE interrupt ID in [21C2h] isn't polled by software.
