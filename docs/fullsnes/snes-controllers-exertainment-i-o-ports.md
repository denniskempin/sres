# SNES Controllers Exertainment - I/O Ports

#### Exertainment I/O Port Summary (Expansion Port Unit)

```text
  21C0h.0 TL16C550AN - RX Data FIFO (R)                          ;\when     (?)
  21C0h.0 TL16C550AN - TX Data FIFO (W)                          ; DLAB=0   (?)
  21C1h.0 TL16C550AN - Interrupt Control (R/W)                   ;/         00h
  21C0h.1 TL16C550AN - Baudrate Divisor Latch LSB, Bit0-7 (R/W)  ;\when     (-)
  21C1h.1 TL16C550AN - Baudrate Divisor Latch MSB, Bit8-15 (R/W) ;/DLAB=1   (-)
  21C2h   TL16C550AN - Interrupt Status (R)                                 01h
  21C2h   TL16C550AN - FIFO Control (W)                                     00h
  21C3h   TL16C550AN - Character Format Control (R/W)     ;<--- Bit7=DLAB   00h
  21C4h   TL16C550AN - Handshaking Control (R/W)                            00h
  21C5h   TL16C550AN - RX/TX Status (R) (Write=reserved for testing)        60h
  21C6h   TL16C550AN - Handshaking Status (R) (Write=unknown/reserved)      0xh
  21C7h   TL16C550AN - Scratch (R/W)                                        (-)
  21C8h   74HC374N (U3) - RAM address MSBs and SPI-style Serial Port (W)    (-)
  21C9h   Not used
  21CAh   74HC374N (U4) - RAM address LSBs (W)                              (-)
  21CBh   Not used
  21CCh   RAM (U5) data byte to/from selected RAM addr (R/W)   (battery backed)
  21CDh   Not used
  21CEh   Not used
  21CFh   ? initially set to 00h (not changed thereafter) (W) ;\maybe one of
  21Dxh   Not used                                            ; these resets
  21DFh   ? initially set to 80h (not changed thereafter) (W) ;/the TL16C550AN?

21C0h..21C7h - TL16C550AN (U1) (RS232 Controller)
```

> **See:** [SNES Controllers Exertainment - RS232 Controller](snes-controllers-exertainment-rs232-controller.md)
> **See:** [SNES Controllers Exertainment - RS232 Data Packets & Configuration](snes-controllers-exertainment-rs232-data-packets-configuration.md)
> **See:** [SNES Controllers Exertainment - RS232 Data Packets Login Phase](snes-controllers-exertainment-rs232-data-packets-login-phase.md)
> **See:** [SNES Controllers Exertainment - RS232 Data Packets Biking Phase](snes-controllers-exertainment-rs232-data-packets-biking-phase.md)

```text
21C8h - 74HC374N (U3) - RAM address MSBs and SPI-style Serial Port (W)
  0   Serial Port Select (0=Select, 1=Idle)
  1   Serial Port Data   (transferred LSB first)
  2   Serial Port Clock  (0=Idle) (data must be stable on 0-to-1 transition)
  3-7 Upper 5bit of 13bit RAM address (see Ports 21CAh/21CCh)
```

Used to send two 16bit values (20F3h and 0470h) during initialization (and to send more data later on). This controls some OSD video controller (possibly also the picture-in-picture function). Used values are:

```text
  20xxh = set address (00h..EFh = yloc*18h+xloc) (24x10 chars)
  20Fxh = set address (F0h..F3h = control registers 0..3)
  1C20h = ascii space with attr=1Ch ?
  1E20h = ascii space with attr=1Eh ?
  1Exxh = ascii chars ":" and "0..9" and "A..Z" (standard ascii codes)
  1Exxh = lowercase chars "a..z" (at "A..Z"+80h instead of "A..Z"+20h)
  0000h = value used for control regs 0 and 1
  0111h = value used for control reg 2
  0070h,0077h,0470h = values used for control reg 3
```

Unknown if/which bicycle versions are actually using the OSD feature, maybe it has been an optional or unreleased add-on. OSD output is supported in the Program Manager's Workout & Fit Test, apparently NOT for drawing the OSD layer on top of the SNES layer, probably rather for displaying OSD while watching TV programs.

Note: The general purpose "/OUT1" bit (RS232 port 21C4h.Bit2) is also output via the serial port connector (purpose is unknown, might be OSD related, or TV enable, or LED control, or whatever).

```text
21C8h - 74HC374N (U3) - RAM address MSBs and SPI-style Serial Port (W)
21CAh - 74HC374N (U4) - RAM address LSBs (W)
21CCh - SRAM (U5) - RAM data byte to/from selected RAM address (R/W)
  Port 21CAh.Bit0-7 = RAM Address Bit0-7 (W)   ;\13bit address, 0000h..1FFFh
  Port 21C8h.Bit3-7 = RAM Address Bit8-12 (W)  ;/
  Port 21C8h.Bit0-2 = See Serial Port description (W)
  Port 21CCh.Bit0-7 = RAM Data Bit0-7 (R/W)
```

Used to access battery-backed 8Kbyte SRAM in the expansion port unit. Note:

There are additional 2Kbytes of SRAM in the Mountain Bike game cartridge (mapped to 700000h).

21CFh (W) initially set to 00h (not changed thereafter) 21DFh (W) initially set to 80h (not changed thereafter) Used to configure/reset whatever stuff during initialization (not used thereafter). Maybe one of these ports resets the TL16C550AN?
