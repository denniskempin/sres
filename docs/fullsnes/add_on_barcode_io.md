# SNES Add-On Barcode Transmission I/O

The Barcode Battler outputs barcodes as 20-byte ASCII string, at 1200 Baud, 8N1. The NES software receives that bitstream via Port 4017h.Bit2. The SNES software requires a BBII Interface, which converts the 8bit ASCII digits into 4bit nibbles, and inserts SNES controller ID and status codes, the interface should be usually connected to Controller Port 2 (although the existing SNES games seem to accept it also in Port 1).

#### Barcode Battler (with BBII Interface) SNES Controller Bits

```text
  1st..12th   Unknown/unused (probably always 0=High?)
  13th..16th  ID Bits3..0          (MSB first, 1=Low=One) (must be 0Eh)
  17th..24th  Extended ID Bits7..0 (MSB first, 1=Low=One) (must be 00h..03h)
              (the SNES programs accept extended IDs 00h..03h, unknown
              if/when/why the BBII hardware does that send FOUR values)
  25th        Status: Barcode present (1=Low=Yes)
  26th        Status: Error Flag 1 ?
  27th        Status: Error Flag 2 ?
  28th        Status: Unknown      ?
```

Following bits need/should be read ONLY if the "Barcode Present" bit is set.

```text
  29th-32th   1st Barcode Digit, Bits3..0  (MSB first, 1=Low=One)
  33th-36th   2nd Barcode Digit, Bits3..0  (MSB first, 1=Low=One)
  37th-40th   3rd Barcode Digit, Bits3..0  (MSB first, 1=Low=One)
  41th-44th   4th Barcode Digit, Bits3..0  (MSB first, 1=Low=One)
  45th-48th   5th Barcode Digit, Bits3..0  (MSB first, 1=Low=One)
  49th-52th   6th Barcode Digit, Bits3..0  (MSB first, 1=Low=One)
  53th-56th   7th Barcode Digit, Bits3..0  (MSB first, 1=Low=One)
  57th-60th   8th Barcode Digit, Bits3..0  (MSB first, 1=Low=One)
  61th-64th   9th Barcode Digit, Bits3..0  (MSB first, 1=Low=One)
  65th-68th   10th Barcode Digit, Bits3..0 (MSB first, 1=Low=One)
  69th-72th   11th Barcode Digit, Bits3..0 (MSB first, 1=Low=One)
  73th-76th   12th Barcode Digit, Bits3..0 (MSB first, 1=Low=One)
  77th-80th   13th Barcode Digit, Bits3..0 (MSB first, 1=Low=One)
  81th and up Unknown/unused
       Above would be 13-digit EAN-13 codes
       Unknown how 12-digit UPC-A codes are transferred   ;\whatever leading
       Unknown if/how 8-digit EAN-8 codes are transferred ; or ending padding?
       Unknown if/how 8-digit UPC-E codes are transferred ;/
```

For some reason, delays should be inserted after each 8 bits (starting with 24th bit, ie. after 24th, 32th, 40th, 48th, 56th, 64th, 72th bit, and maybe also after 80th bit). Unknown if delays are also needed after 8th and 16th bit (automatic joypad reading does probably imply suitable delays, but errors might occur when reading the ID bits via faster manual reading).

#### Barcode Battler RAW Data Output

Data is send as 20-byte ASCII string. Bytes are transferred at 1200 Bauds:

```text
  1 Start bit (must be LOW)
  8 Data bits (LSB first, LOW=Zero, HIGH=One)
  1 Stop bit  (must be HIGH)
```

The first 13 bytes can contain following strings:

```text
  "nnnnnnnnnnnnn"    ;13-digit EAN-13 code (ASCII chars 30h..39h)
  <Unknown>          ;12-digit UPC-A code (with ending/leading padding?)
  "     nnnnnnnn"    ;8-digit EAN-8 code (with leading SPC-padding, ASCII 20h)
  <Unknown>          ;8-digit UPC-E code (with ending/leading padding?)
  "ERROR        "    ;indicates scanning error
```

The last 7 bytes must contain either one of following ID strings:

```text
  "EPOCH",0Dh,0Ah    ;<-- this is sent/accepted by existing hardware/software
  "SUNSOFT"          ;<-- this would be alternately accepted by the NES game
```

There are rumours that one "must" use a mono 3.5mm plug in order to receive data - that's obviously bullshit, but it might indicate that the middle pin of stereo plugs must be GNDed in order to switch the Barcode Battler into transmit mode(?)
