# SNES Cart Seta ST018 (pre-programmed ARM CPU) (1 game)

Seta ST018 - 160pin SETA D6984 ST018 chip (PCB SHVC-1DE3B-01) The chip is used by a single game only:

```text
  Hayazashi Nidan Morita Shogi 2 (ST018) (1995) Random House/SETA Corp. (JP)
```

#### ARM CPU Reference

> **See:** [ARM CPU Reference](arm-cpu-reference.md)

#### ST018 Memory Map (ARM Side)

```text
  00000000h ROM 128K  -- with 32bit databus
  20000000h
  40000000h I/O ports
  60000000h probably (absent) external ROM/EPROM ;can redirect exceptions here?
  80000000h
  A0000000h ROM 32K ? -- with 8bit databus
  C0000000h
  E0000000h RAM 16K
```

#### ST018 I/O Map (ARM Side)

```text
  40000010h.R  Data from SNES (reset STAT.3 and get latched data-to-arm)
  40000020h.R  Status         (get STAT)
  40000000h.W  Data to SNES   (set STAT.0 and latch data-to-snes)
  40000010h.W  Flag to SNES   (set STAT.2 on writing any value) (IRQ?)
  40000020h.W  Config 1
  40000024h.W  Config 2
  40000028h.W  Config 3
  4000002Ch.W  Config 4
```

#### ST018 I/O Map (SNES Side)

```text
  3800h.R      Data to SNES   (reset STAT.0 and get latched data-from-arm)
  3802h.R      Ack Flag       (reset STAT.2 and get dummy data?)
  3804h.R      Status         (get STAT)
  3802h.W      Data from SNES (set STAT.3 and latch data-from-snes)
  3804h.W      Reset ARM      (00h=Normal, 01h=HardReset, FFh=SoftReset?)
```

#### ST018 Status Register

There are two status registers, ARM:40000020h.R and SNES:3804h.R. Bit0 of that two registers appears to be same for ARM and SNES, the other are used only by either CPU (as shown below), although they might be actually existing on both CPUs, too.

```text
  0 SNES ARM  ARM-to-SNES Data Present     (0=No, 1=Yes)
  1 -    -    Unknown/Unused               (unknown)
  2 SNES -    ARM-to-SNES IRQ Flag?        (0=No, 1=Yes)
  3 -    ARM  SNES-to-ARM Data Present     (0=No, 1=Yes)
  4 SNES -    Fatal Problem                (0=Okay, 1=SNES skips all transfers)
  5 -    ARM  Redirect ARM to 600000xxh    (0=No, 1=Yes)
  6 SNES -    Unused (unless [FF41h]<>00h) (0=Busy, 1=Ready)
  7 SNES -    ARM Reset Ready              (0=Busy, 1=Ready)
```

STAT.2 might be IRQ signal (ST018.pin12 connects to SNES./IRQ pin), but the Shogi game contains only bugged IRQ handler (without ACK); instead it's just polling STAT.2 by software.

#### ST018 Component List

```text
  PCB "SHVC-1DE3B-01, (C) 1995 Nintendo"
  U1  32pin LH534BN6 LoROM 512Kx8 (alternately 40pin) (PCB: "4/8/16/32M")
  U2  28pin LH52A64N SRAM 8Kx8                        (PCB: "64K")
  U3 160pin Seta ST018, (C)1994-5 SETA                (PCB: "ST0018/ST0019")
  U4  16pin 74LS139A (demultiplexer)                  (PCB: "LS139")
  U5   8pin /\\ 532 26A (battery controller)          (PCB: "MM1026")
  U6  18pin FA11B CIC                                 (PCB: "CIC")
  BATT 2pin Maxell CR2032T (3V battery for U2)
  X1   3pin [M]21440C 21.44MHz (plastic oscillator)   (PCB: "21.44MHz")
  P1  62pin SNES Cart Edge connector (plus shield)
```

Note: U5 is located on PCB back side for some weird reason. The chip name is "ST018", although the PCB text layer calls it "ST0018" (with double zero).

#### ST018 ARM Timings (mostly unknown)

The ARM CPU is clocked by a 21.44MHz oscillator, but unknown if there is some internal clock multiplier/divider for the actual CPU clock (if so, then it might even be controlled via I/O ports for low power mode purposes).

Unknown if there is any code/data cache, and unknown if there are any memory waitstates (if so, timings might differ for 8bit/32bit access, for sequential/nonsequential access, and for different memory regions).

#### ST018 ARM Memory (mostly unknown)

Unknown if there any memory mirrors, or unused regions (possibly filled with 00h, or FFh, or with garbage), or regions that do trap memory exceptions.

Unknown if there any unused extra I/O ports or memory regions.

The 128K ROM, I/O area, and 16K RAM seem to support both 8bit and 32bit access.

The 32K ROM is used only with 8bit access; unknown what happens on 32bit access to that region.

Effects on misaligned 32bit RAM writes are probably ignoring the lower address bits, and writing to "ADDR AND (NOT 3)" (at least it like so on ARMv4/ARMv5) (the case is important because there's a ST018 BUG that does "str r14,[r2,2]", which should be 8bit STRB, not mis-aligned 32bit STR).

#### ST018 ARM Other Stuff (mostly unknown)

Unknown if there's any coprocessor or SMULL/UMULL extension (the BIOS doesn't use such stuff, but CP14/CP15 are more or less common to be present).

The CPU seems to use ARMv3 instruction set (since the BIOS is using ARMv3 features: 32bit program counter and CPSR register; but isn't using any ARMv4 features such like BX, LDRH, Sys mode, or THUMB code) (also possible that ARMv4 processors haven't even been available at time when the ST018 was developed in 1994/1995).

#### ST018 Commands

```text
  00h..9Fh Unused
  A0       Debug: Reboot
  A1       Debug: Get Version 4 ;\maybe major/minor version (or vice-versa)
  A2       Debug: Get Version 5 ;/
  A3       Debug: Dump 80h bytes from address NNNNNNNNh
  A4       Debug: Dump NNh bytes from address NNNNNNNNh
  A5       Debug: Write NNh bytes to address NNNNNNNNh
  A8        do_high_level_func_0_1_with_reply_flag
  A9        do_high_level_func_1_1_with_reply_flag
  AA       UploadBoardAndSomethingElse (send 9x9 plus 16 bytes to 0E0000400h)
  AB       Write_1_byte_to_0E0000468h (usually value=02h)
  AC       Read ARM "R12" register value
  AD       Read 1 byte from 0E0000464h (LEN)
  AE        do_high_level_func_2_with_reply_flag
  AF       Read 1 byte from 0E0000464h (LEN+1)*2
  B0       Read (LEN+1)*2 bytes from 0E000046Ch  ;LEN as from cmd ADh/AFh
  B1        do_high_level_func_0_X_Y_with_reply_flag (send 2 bytes: X,Y)
  B2        do_high_level_func_1_X_Y_with_reply_flag (send 2 bytes: X,Y)
  B3        do_high_level_func_4_with_1_reply_byte   (recv 1 byte)
  B4        do_high_level_func_5_with_1_reply_byte   (recv 1 byte)
  B5        do_high_level_func_6_with_1_reply_byte   (recv 1 byte)
  B6        do_high_level_func_7_with_1_reply_byte   (recv 1 byte)
  B7        do_high_level_func_3_with_reply_flag
  B8h..F0h Unused
  F1       Selftest 1  ;if response.bit2=1, receive 2 error bytes
  F2       Selftest 2  ;if response<>00h, receive 2 error bytes
  F3       Debug: Dump 128Kbyte ROM from 00000000h ;\for HEX-DUMP display
  F4       Debug: Dump 32Kbyte ROM from A0000000h  ;/
  F5       Debug: Get Chksum for 128K ROM at 00000000h
  F6       Debug: Get Chksum for 32K ROM at A0000000h
  F7h..FFh Unused
```

Note: Command A5h allows to write code to RAM, and also to manipulate return addresses on stack, thus allowing to execute custom ARM code.
