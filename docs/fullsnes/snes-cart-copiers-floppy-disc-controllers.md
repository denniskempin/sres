# SNES Cart Copiers - Floppy Disc Controllers

#### FDC Chips

Most (or all) SNES copiers are using one of the following FDCs:

```text
  40pin  GM82C765B (DIP)  (Supercom, Ufo, Pro Fighter, Smart Disc, Bung?)
  44pin  GM82C765B (SMD)  (Wild Card, GP-003)
  68pin  MCS3201FN (SMD)  (used by OLD copiers: Super Magic Drive)
  68pin  MCCS3201FN (SMD) (used by OLD copiers: Supercom & Super Magicom)
```

#### FDC Address Decoding

The 68pin MCS3201FN chips include a 10bit address bus (for decoding address 3F0h-3F7h on IBM PCs; whereas, SNES copiers are using only the lower some address bits), and an 8bit General Purpose Input. The 40pin/44pin GM82C765B chips include a 1bit address bus bundled with 3 select lines, and have no General Purpose Input register.

```text
  GM82C765B   MCS3201FN  Dir  Register
  N/A         A0-A2=0    R    General Purpose Input (pins I0..I7)
  /LDOR       A0-A2=2    W    Motor Control (bit0-7)
  /CS+A0=0    A0-A2=4    R    Main Status  (NEC uPD765 compatible)
  /CS+A0=1    A0-A2=5    RW   Command/Data (NEC uPD765 compatible)
  /LDCR       A0-A2=7    W    Transfer Rate (Density) (bit0-1)
  N/A         A0-A2=7    R    Bit7=DiskChange, Bit6-0=Zero
```

Accordingly, MCS3201FN ports are always ordered as shown above, whilst GM82C765B ports can be arranged differently (in case of the Super Wild Card, Front Fareast kept them arranged the same way as on their older Super Magicom).

#### FDC Command/Data and Main Status

> **See:** [SNES Cart Copiers - Floppy Disc NEC uPD765 Commands](snes-cart-copiers-floppy-disc-nec-upd765-commands.md)

#### FDC Motor Control

```text
  GM Bit0-7:  DSEL ,X    ,/RES,DMAEN,MOTOR1,MOTOR2,X     ,MSEL
  MCS Bit0-7: DSEL0,DSEL1,/RES,DMAEN,MOTOR1,MOTOR2,MOTOR3,MOTOR4
```

Note that, for whatever reason, most SNES Copiers are using the SECOND drive (ie. DSEL=1 instead of DSEL=0, and MOTOR2=1 instead of MOTOR1=1).

#### Transfer Rate (Density)

```text
  Val  Usage                 MCS3201FN       GM82C765B
  00h  HD (high density)     500K if /RWC=1  MFM:500K or FM:250K
  01h  DD 5.25" (double den) 300K if /RWC=0  MFM:300K if DRV=1, 250K if DRV=0
  02h  DD 3.5"(double den)   250K if /RWC=0  MFM:250K or FM:125K
  03h  N/A                   Reserved        125K
```

#### Disk Change (MCS3201FN only) (not GM82C765B)

```text
  7   Disk Change Flag
  6-0 Unused (zero)
```

Possibly useful, but purpose/usage is unclear. According to the datasheet it is for "diagnostics" purposes. Unknown when the flag gets reset, and unknown for which drive(s) it does apply.
