# NSS I/O Ports - EEPROM and PROM

#### Memory A000h.R - EEPROM Input

```text
  7   EEPROM Data In (0=Low=Zero, 1=High=One)
  6   EEPROM Ready   (0=Low=Busy, 1=High=Ready)
  5-0 Unknown/unused
```

#### Memory E000h.W - EEPROM Output

```text
  7   Unknown/set     (should be always 1)
  6-5 Unknown/unused  (should be always 0)
  4   EEPROM Clock    (0=Low=Clock, 1=High=Idle) ;(Data In/Out must be stable
  3   EEPROM Data Out (0=Low=Zero, 1=High=One)   ;on raising CLK edge)
  2-1 Unknown/unused  (should be always 0)       ;(and updated on falling edge)
  0   EEPROM Select   (0=High=No, 1=Low=Select)
```

#### Note

E000h (W) and Exxxh (W) are probably mirrors of each other. If so, some care should be taken not to conflict PROM and EEPROM accesses.

Memory Exxxh.R.W.EXEC - Ricoh RP5H01 serial 72bit PROM (Decryption Key) Data Write:

```text
  7-5  Unknown/unused
  4    PROM Test Mode (0=Low=6bit Address, 1=High=7bit Address)
  3    PROM Clock     (0=Low, 1=High) ;increment address on 1-to-0 transition
  2-1  Unknown/unused
  0    PROM Address Reset (0=High=Reset Address to zero, 1=Low=No Change)
```

Data Read and Opcode Fetch:

```text
  7-5  Always set (MSBs of RST Opcode)
  4    PROM Counter Out (0=High=One, 1=Low=Zero) ;PROM Address Bit5
  3    PROM Data Out    (0=High=One, 1=Low=Zero)
  2-0  Always set (LSBs of RST Opcode)
```

The BIOS accesses the PROM in two places:

```text
  1st PROM check: Accessed via E37Fh, this part decrypts the 32h-byte area.
    the first data bit is read at a time when PROM reset is still high,
    and reset is then released after reading that data bit. At this point,
    there's a critical glitch: If the data bit was 1=Low, then the decryption
    code chooses to issue a 1-to-0 CLK transition at SAME time as when
    releasing reset - the PROM must ignore this CLK edge (otherwise half
    of the games won't work).
  2nd PROM check: Accessed via EB27h, this part decrypts the double-encrypted
    title (from within the 32h-byte area) and displays on the OSD layer,
    alongsides it does verify a checksum at DC3Fh.
    Note: The program code hides in the OSD write string function, and gets
    executed when passing invalid VRAM addresses to it; this is usually done
    via Token 06h.
    This is initially done shortly after the 1st PROM check (at that point
    just for testing DC3Fh, with "invisible" black-on-black color attributes).
```

And, there are two more (unused/bugged) places:

```text
  3rd PROM check: Accessed via FB37h, this part is similar to 2nd PROM check,
    but sends garbage to OSD screen, and is just meant to verify checksum at
    DD3Fh. However, this part seems to be bugged (passing FB37h to the RST
    handler will hang the BIOS). The stuff would be invoked via Token 4Eh,
    but (fortunately) the BIOS is never doing that.
  4th PROM check: Accessed via ExExh, this part is comparing the 1st eight
    bytes of the PROM with a slightly encrypted copy in INST ROM. However,
    in F-Zero, the required pointer at [2Eh-2Fh] in the 32h-byte area is
    misaligned, thus causing the check to fail. The stuff would be invoked
    from inside of NMI handler (when [80ECh] nonzero), but (fortunately) the
    BIOS is never doing that.
```

Note: All (used) PROM reading functions use RST vectors which are executing Z80 code in INST ROM. Accordingly, the code in INST ROM can be programmed so that it works with PROM-less cartridges.

#### PROM Dumps

Theoretically, dumping serial PROMs is ways easier than dumping parallel ROMs/EPROMs - but, as by now, nobody does ever seem to have done this. Anyways, with a brute-force program, it's possible to find matching PROM values for decrypting known title strings.

```text
  Title                 PROM content
  ActRaiser             B9,4B,F5,72,E4,9E,25,FF,F2,F2,00,00,F2,F2,00,00
  AMAZING TENNIS        2D,EB,21,3B,9A,81,86,93,57,57,00,00,57,57,00,00
  F-ZERO                49,63,FA,03,B5,DF,F6,17,B7,B7,00,00,B7,B7,00,00
  LETHAL WEAPON         7F,9B,42,99,D4,C2,A9,0A,CB,CB,00,00,CB,CB,00,00
  NCAA Basketball       DB,35,54,07,A0,EF,A2,72,F8,F8,00,00,F8,F8,00,00
  New Game 1 [Contra 3] 3A,BC,E6,47,10,DD,45,AF,FC,FC,00,00,FC,FC,00,00
  ROBOCOP 3             6A,06,DC,99,5F,3A,5C,D1,5D,5D,00,00,5D,5D,00,00
  Super Mario World     AE,D4,A8,1C,EC,DA,8D,EA,7D,7D,00,00,7D,7D,00,00
  SUPER SOCCER          6C,57,7E,3C,8F,1F,AB,F2,3D,3D,00,00,3D,3D,00,00
  Super Tennis          86,B7,8E,BD,74,A3,6E,56,9F,9F,00,00,9F,9F,00,00
  The Addams Family     C1,70,F2,7F,3A,EC,D3,02,67,67,00,00,67,67,00,00
  The Irem Skins Game   D7,3F,FE,6A,B7,3A,18,AA,D6,D6,00,00,D6,D6,00,00
```

#### Mitsubishi M6M80011 64x16 Serial EEPROM Protocol

All values transferred LSB first.

```text
  Write Enable:  Send C5h,xxh
  Write Disable: Send 05h,xxh
  Write Word:    Send 25h,addr, Send lsb,msb
  Read Word:     Send 15h,addr, Read lsb,msb
  Read Status:   Send 95h,mode, Read stat...
    (mode: 0=Busy, 1=WriteEnable, 2=ECC Flag)
    (stat: endless repeated bits, 0=Busy/WriteEnable/ECC_Correct)
    (                             1=Ready/WriteDisable/ECC_Incorrect)
```

#### M6M80011 Pin-Out (2x4pin version)

```text
  1=/CS, 2=/CLK, 3=DTA.IN, 4=DTA.OUT, 5=GND, 6=RESET, 7=RDY/BUSY, 8=VCC
```

#### NSS EEPROM Format (Coinage Settings)

```text
  00h-3Bh Fifteen 4-byte chunks (unused entry when 1st byte = 00h)
           Byte0: Upper Nibble: Checksum (all other 7 nibbles added together)
           Byte0: Lower Nibble: Price (Number of credits for this game, 1..9)
           Byte1: GameID
           Byte2: Time Minutes (BCD) (time limit per game)
           Byte3: Time Seconds (BCD) (time limit per game)
  3Ch     Right Coinage and Unused (bit7-4=Unused, but must be 1..9)
  3Dh     Left Coinage and Flags (bit7=Music, bit6=Freeplay, bit5-4=Unused)
  3Eh-3Fh Checksum (all bytes at [00h..3Dh] added together)
  40h-7Fh Backup Copy of 00h..3Fh
```
