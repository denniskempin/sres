# SNES Cartridge CIC Notes

#### Program Counter (PC)

The 10bit PC register consists of a 3bit bank (which gets changed only by call/jmp/ret opcodes), and a 7bit polynomial counter (ie. not a linear counter). After fetching opcode bytes, PC is "incremented" as so:

```text
  PC = (PC AND 380h) + (PC.Bit0 XOR PC.Bit1)*40h + (PC AND 7Eh)/2
```

Ie. the lower 7bit will "increment" through 127 different values (and wrap to 00h thereafter). Address 7Fh is unused (unless one issues a JMP 7Fh opcode, which would cause the CPU to hang on that address).

```text
  Format     <------------- Valid Address Area ---------->      <--Stuck-->
  Linear     00 01 02 03 04 05 06 07 08 09 0A ... 7C 7D 7E  or  7F 7F 7F 7F
  Polynomial 00 40 60 70 78 7C 7E 3F 5F 6F 77 ... 05 02 01  or  7F 7F 7F 7F
```

To simplify things, programming tools like assemblers/disassemblers may use "normal" linear addresses (and translate linear/polynomial addressses when needed - the polynomial addresses are relevant only for encoding bits in jmp/call opcodes, and for how the data is physically arranged in the chip ROMs and in ROM-images).

#### ROM-Images

The existing ROM-images are .txt files, containing "0" and "1" BITS in ASCII format, arranged as a 64x64 (or 96x64) matrix (as seen in decapped chips).

```text
  Line 1..32   --->   Address X+9Fh..80h            ;\Lines (Y)
  Line 33..64  --->   Address X+1Fh..00h            ;/
  Column  1+(n*W) --> Data Bit(n) of Address 000h+Y ;\  ;\
  Column  2+(n*W) --> Data Bit(n) of Address 020h+Y ;   ; Columns (X)
  Column  3+(n*W) --> Data Bit(n) of Address 040h+Y ;   ;
  Column  4+(n*W) --> Data Bit(n) of Address 060h+Y ;   ; chips with 200h-byte
  Column  5+(n*W) --> Data Bit(n) of Address 100h+Y ;   ; (W=8) (64x64 bits)
  Column  6+(n*W) --> Data Bit(n) of Address 120h+Y ;   ;
  Column  7+(n*W) --> Data Bit(n) of Address 140h+Y ;   ;
  Column  8+(n*W) --> Data Bit(n) of Address 160h+Y ;   ;/
  Column  9+(n*W) --> Data Bit(n) of Address 200h+Y ;
  Column 10+(n*W) --> Data Bit(n) of Address 220h+Y ;  chips with 300h-byte
  Column 11+(n*W) --> Data Bit(n) of Address 240h+Y ;  (W=12) (96x64 bits)
  Column 12+(n*W) --> Data Bit(n) of Address 260h+Y ;/
```

Cautions: The bits are inverted (0=1, 1=0) in some (not all) dumps. Mind that the bytes are arranged in non-linear polynomial fashion (see PC register).

Recommended format for binary ROM-images would be to undo the inversion (if present), and to maintain the polynomial byte-order.

Note: Known decapped/dumped CICs are D411 and 3195A, and... somebody decapped/dumped a CIC without writing down its part number (probably=6113).

#### CIC Timings

The NES CICs are driven by a 4.000MHz CIC oscillator (located in the console, and divided by 4 in the NES CIC). The SNES CICs are driven by the 24.576MHz APU oscillator (located and divided by 8 in the console's audio circuit, and further divided by 3 in the SNES CIC) (exception are older SNES mainboards, which are having a separate 4.00MHz resonator, like the NES).

Ie. internally, the CICs are clocked at 1.000MHz (NES) or 1.024MHz (SNES). All opcodes are executed within 1 clock cycles, except for the 2-byte long jumps (opcodes 78h-7Fh) which take 2 clock cycles. The "skip" opcodes are forcing the following opcode to be executed as a "nop" (ie. the skipped opcode still takes 1 clock cycle; or possibly 2 cycles when skipping long jump opcodes, in case the CPU supports skipping 2-byte opcodes at all).

After Reset gets released, the CICs execute the first opcode after a short delay (3195A: randomly 1.0 or 1.25 cycles, D413A: constantly 1.33 cycles) (whereas, portions of that delay may rely on a poorly falling edge of the incoming Reset signal).

#### CIC Ports

```text
  Name  Pin  Dir  Expl
  P0.0  1    Out  Data Out    ;\SNES version occassionally swaps these
  P0.1  2    In   Data In     ;/pins by software (ie. Pin1=In, Pin2=Out)
  P0.2  3    In   Random Seed (0=Charged/Ready, 1=Charging/Busy)
  P0.3  4    In   Lock/Key    (0=Cartridge/Key, 1=Console/Lock)
  P1.0  9    Out  Reset SNES  (0=Reset Console, 1=No)
  P1.1  10   Out  Reset Key   (0=No, 1=Reset Key)
  P1.2  11   In   Unused, or Reset Speed A (in 3195A) ;\blink speed of reset
  P1.3  12   In   Unused, or Reset Speed B (in 3195A) ;/signal (and Power LED)
  P2.0  13   -    Unused
  P2.1  14   -    Unused
  P2.2  15   -    Unused
  P2.3  -    -    Unused
  P3.0  -    RAM  Unused, or used as "noswap" flag (in SNES CIC)
  P3.1  -    -    Unused
  P3.2  -    -    Unused
  P3.3  -    -    Unused
```

P0.0-P2.2 are 11 external I/O lines (probably all bidirectional, above directions just indicates how they are normally used). P2.3-P3.3 are 5 internal bits (which seem to be useable as "RAM"). Pin numbers are for 16pin NES/SNES DIP chips (Pin numbers on 18pin SNES SMD chips are slightly rearranged).

P4,P5,P6,P7,P8,P9,PA,PB,PC,PD,PF are unknown/unused (maybe 12x4 further bits, or mirrors of P0..P3).

#### CIC Stream Seeds

There are different seeds used for different regions. And, confusingly, there is a NES-CIC clone made Tengen, which uses different seeds than the real CIC (some of the differences automatically compensated when summing up values, eg.

8+8 gives same 4bit result as 0+0, other differences are manually adjusted by Tengen's program code).

Many of the reverse-engineered NES seeds found in the internet are based on the Tengen design (the USA-seeds extracted from the decapped Tengen chip, the EUR/ASIA/UK-seeds based on sampled Nintendo-CIC data-streams, and then converted to a Tengen-compatible seed format). To convert them to real CIC seeds:

```text
  Nintendo[1..F] = Tengen[1..F] - (2,0,0,0,0,0,8,8,8,8,8,8,8,8,2)
```

There are other (working) variations possible, for example:

```text
  Nintendo[1..F] = Tengen[1..F] - (2,0,0,0,0,A,E,8,8,8,8,8,8,8,2)
  (That, for Tengen-USA seeds. The Tengen-style-EUR/ASIA/UK seeds may differ)
```

Whereas, the random seed in TengenKEY[1] is meant to be "r+2" (so subtracting 2 restores "r").

#### CIC Stream Logs

There are some stream logs with filename "XXXX-N.b" where XXXX is the chip name, and N is the random seed, and bytes in the file are as so:

```text
  Byte 000h, bit0-7 = 1st-8th bit on Pin 1 (DTA.OUT on NES)(DTA.OUT/IN on SNES)
  Byte 001h, bit0-7 = 1st-8th bit on Pin 2 (DTA.IN on NES) (DTA.IN/OUT on SNES)
  Byte 002h, bit0-7 = 9th-16th bit on Pin 1
  Byte 003h, bit0-7 = 9th-16th bit on Pin 2
  etc.
```

Caution: The "N" in the filename is taken as if the seed were transferred in order Bit 3,2,1,0 (actually it is Bit 3,0,1,2). Ie. file "3195-1.b" would refer to a NES-EUR-CIC with seed r=4. The signals in the files are sampled at 1MHz (ie. only each fourth 4MHz cycle).

#### The 6113 Chip

The 6113 chip was invented in 1987, and it replaced the 3193 chip in US/Canadian cartridges (while US/Canadian consoles kept using 3193 chips). When used in cartridges, the 6113 does usually "emulate" a 3193 chip. But, for whatever reason, it can do more:

```text
  Console  Cartridge  Notes
  3193     3193       Works (the "old" way)        ;\used combinations
  3193     6113       Works (the "new" way)        ;/
  6113     6113       Works (special seed/timing)  ;\
  6113     3193       Doesn't work                 ; not used as far as known
  6113     ??         Might work (??=unknown chip) ;/
```

When used in consoles, the 6113 uses slightly different timings and seed values (and does request cartridges with 6113 chips to use the 6113-mode, too, rather than emulating the 3193).

One guess: Maybe Nintendo originally used different CICs for NTSC regions (like 3193/3194 for USA/Canada/SouthKorea), and later combined them to one region (if so, all NES consoles in Canada or SouthKorea should contain 3194/6113 chips, unlike US consoles which have 3193 chips).

3195A Signals (NES, Europe)

The I/O ports are HIGH (for Output "1"), or LOW-Z (for Output "0" or Input).

Raising edges take circa 0.5us, falling edges take circa 3us.

```text
  4MHz Clock Units     ...............................
  1MHz Clock Units       .   .   .   .   .   .   .   .
                          ___________                  ;\Console+Cartridge
  Data Should-be       __|           |________________ ;/should be 3us High
                           __________                  ;\actually 2.5us High
  Data From Console    __.'          ''----.......____ ;/and 3us falling
                           __________                  ;\
  Data From Cartridge  __.'          ''----.......____ ; either same as console
    or, delayed:            __________                 ; or 0.25us later
  Data From Cartridge  ___.'          ''----.......___ ;/
```

After Power-up, the Cartridge CIC does randomly start with good timing, or with all signals delayed by 0.25us. In other words, the Cartridge CIC executes the first opcode 1.0us or 1.25us (four or five 4MHz cycles) after Reset gets released. However, for some reason, pushing the Reset Button doesn't alter the timing, the random-decision occurs only on Power-up.

D413A Signals (SNES, Europe)

The D413A signals are looking strange. First, the software switches signals High for 3us, but the actual signals are 3.33us High. Second, the signals on one pin are constantly jumping back'n'forth by 1.33us (in relation to the other pin).

```text
  3.072MHz Clock Units ...............................
  1.024MHz Clock Units   .  .  .  .  .  .  .  .  .  .
                                ________               ;\Console+Cartridge
  Data Should-be       ________|        |_____________ ;/should be 3us High
                                _________              ;\actually 3.33us high
  Data From/To Console ________|         '--..._______ ;/and 2us falling
                            _________                  ;\
  Data From/To Cart    ____|         '--...___________ ; 1.33us earlier
   or, delayed                      _________          ; or 1.33us later
  Data From/To Cart    ____________|         '--...___ ;/
```

The earlier/later effect occurs because the SNES CICs are occassionally reversing the data-direction of the pins. Ie. in practice, Data from Cartridge is constantly 1.33us LATER than from Console.

Software-wise, the D411 (and probably D413A) is programmed as if the Cartridge CIC would start "immediately", but in practice, it starts 1.33us (four 3.072MHz cycles) after releasing Reset (that offset seems to be constant, unlike as on the 3195A where it randomly changes between 1.0us and 1.25us).
