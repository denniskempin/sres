# SNES Cart Capcom CX4 (programmable RISC CPU) (Mega Man X 2-3) (2 games)

> **See:** [SNES Cart Capcom CX4 - I/O Ports](snes-cart-capcom-cx4-i-o-ports.md)
> **See:** [SNES Cart Capcom CX4 - Opcodes](snes-cart-capcom-cx4-opcodes.md)
> **See:** [SNES Cart Capcom CX4 - Functions](snes-cart-capcom-cx4-functions.md)
> **See:** [SNES Pinouts CX4 Chip](snes-pinouts-cx4-chip.md)

#### Capcom CX4 - 80pin chip

Used only by two games:

```text
  Mega Man X2 (1994) Capcom (NA) (JP) (EU)   ;aka Rockman X2
  Mega Man X3 (1995) Capcom (NA) (JP)
```

The CX4 chip is actually a Hitachi HG51B169 as confirmed by decapping.

Note: The CX4 is occassionally referred to as C4 (the real chip name is CX4, the C4 variant is some kind of scene slang).

#### CX4 Memory Map

```text
  I/O  00-3F,80-BF:6000-7FFF
  ROM  00-3F,80-BF:8000-FFFF
  SRAM 70-77:0000-7FFF (not installed; reads return 00h)
```

#### MISC MISC MISC

Commands are executed on the CX4 by writing the command to 0x7F4F while bit 6 of 0x7F5E is clear. Bit 6 of 0x7F5E will stay set until the command has completed, at which time output data will be available.

[Registers]

```text
  $7f49-b = ROM Offset
  $7f4d-e = Page Select
  $7f4f = Instruction Pointer
  Start Address = ((Page_Select * 256) + Instruction Pointer) * 2) + ROM_Offset
```

[Memory layout]  Program ROM is obviously 256x16-bit pages at a time. (taken from the SNES ROM)  Program RAM is 2x256x16-bit. (two banks)    ;<-- uh, that means cache?

Data ROM is 1024x24-bit. (only ROM internal to the Cx4)  Data RAM is 4x384x16-bit.                   ;<-- uh, but it HAS 8bit data bus?

Call stack is 8-levels deep, at least 16-bits wide.

#### CX4ROM (3Kbytes) (1024 values of 24bit each)

```text
  Index      Name  ;Entry     = Table Contents   = Formula
  -------------------------------------------------------------------------
  000..0FFh  Div   ;N[0..FFh] = FFFFFFh..008080h = 800000h/(00h..FFh)
  100..1FFh  Sqrt  ;N[0..FFh] = 000000h..FF7FDFh = 100000h*Sqrt(00h..FFh)
  200..27Fh  Sin   ;N[0..7Fh] = 000000h..FFFB10h = 1000000h*Sin(0..89')
  280..2FFh  Asin  ;N[0..7Fh] = 000000h..75CEB4h = 800000h/90'*Asin(0..0.99)
  300..37Fh  Tan   ;N[0..7Fh] = 000000h..517BB5h = 10000h*Tan(0..89')
  380..3FFh  Cos   ;N[0..7Fh] = FFFFFFh..03243Ah = 1000000h*Cos(0..89')
```

Sin/Asin/Tan/Cos are spanning only 90' out of 360' degress (aka 80h out of 200h degrees). Overflows on Div(0) and Cos(0) are truncated to FFFFFFh. All values are unsigned, and all (except Asin/Tan) are using full 24bits (use SHR opcode to convert these to signed values with 1bit sign + 23bit integer; for Div one can omit the SHR if divider>01h).

#### CX4 Component List (Megaman X2)

```text
  PCB "SHVC-2DC0N-01, (C)1994 Nintendo"
  U1 32pin P0 8M MASK ROM  (LH538LN4 = 8Mbit)
  U2 32pin P1 4/8 MASK ROM (LH534BN2 or LH5348N2 or so = 4Mbit)
  U3 80pin CX4 (CAPCOM CX4 DL-2427, BS169FB)
  U4 18pin CIC (F411A)
  X1  2pin 20MHz
  J  62pin Cart Edge connector (unknown if any special pins are actually used)
```

#### CX4 Component List (Megaman X3)

```text
  PCB "SHVC-1DC0N-01, (C)1994 Nintendo"
  U1 40pin MASK ROM  (TC5316003CF = 16Mbit)
  U2 80pin CX4 (CAPCOM CX4 DL-2427, BS169FB)
  U3 18pin CIC (F411A)
  X1  2pin 20MHz
  J  62pin Cart Edge connector (unknown if any special pins are actually used)
```

#### CX4 Cartridge Header (as found in Mega Man X2/X3 games)

```text
  [FFBD]=00h ;expansion RAM size (none) (there is 3KB cx4ram though)
  [FFBF]=10h ;CustomChip=CX4
  [FFD5]=20h ;Slow LoROM (but CX4 opcodes are probably using a faster cache)
  [FFD6]=F3h ;ROM+CustomChip (no battery, no sram)
  [FFD7]=0Bh ;rom size (X2: 1.5MB, rounded-up to 2MB) (X3: real 2MB)
  [FFD8]=00h ;sram size (none) (there is 3KB cx4ram though)
  [FFDA]=33h ;Extended Header (with FFB0h-FFBFh)
```

#### ROM Enable

On SHVC-2DC0N-01 PCBs (ie. PCBs with two ROM chips), the 2nd ROM chip is reportedly initially disabled, and can be reportedly enabled by setting [7F48h]=01h (that info doesn't match up with how 7F48h is used by the existing games; unknown if that info is correct/complete).

#### CX4 CPU Misc

All values are little-endian (opcodes, I/O Ports, cx4rom-ROM-Image, etc).

Call Stack is reportedly 16 levels deep, at least 16bits per level.

Carry Flag is CLEARED on borrow (ie. opposite as on 80x86 CPUs).

#### CX4 Timings (Unknown)

All opcode & DMA timings are 100% unknown. The CX4 is said to be clocked at 20.000MHz, but this might be internally divided, possibly with different waitstates for different memory regions or different opcodes.

The ROM speed is 2.68Mhz (according to the cartridge header), and 16bit opcodes are passed through 8bit databus (though one may assume that the CX4 contains an opcode cache) (cache might be divided into 200h-byte pages, so, far-jumps to other pages might be slow, maybe/guessed).

The "skip" opcodes are "jumping" to the location after the next opcode (this probably faster than the actual "jmp" opcodes).

After Multiply opcodes one should insert one "nop" (or another instruction that doesn't access the MH or ML result registers).

Reading data bytes from SNES ROM requires some complex timing/handling:

```text
  612Eh   movb   ext_dta,[ext_ptr]          ;\these 3 opcodes are used to
  4000h   inc    ext_ptr                    ; read one byte from [ext_ptr],
  1C00h   finish ext_dta                    ;/and to increment ext_ptr by 1
```

The exact meaning of the above opcodes is unknown (which one does what part?).

It is also allowed to use the middle opcode WITHOUT the "prepare/wait" part:

```text
  4000h   inc    ext_ptr                    ;-increment ext_ptr by 1
```

In that case, "ext_ptr" is incremented, but "ext_dta" should not be used (might be unchanged, or contain garbage, or receive data after some cycles?).
