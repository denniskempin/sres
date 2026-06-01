# SNES Cart GSU-n CPU Misc

#### Uncached ROM/RAM-Read-Timings

```text
  ROM Read:   5 cycles per byte at 21MHz, or 3 cycles per byte at 10MHz
  RAM Write: 10 cycles per word at 21MHz, or unknown at 10MHz?
  RAM Write: unknown number of cycles per byte?
  ROM/RAM Opcode-byte-read: 3 cycles at both 21MHz and 10MHz?
```

The uncached timings aren't well documented. Possibly ROM/RAM-byte read/write are all having the same timing (3/5 clks at 10/21MHz) (and RAM-word 6/10)?

#### Jump Notes

Jumps can be implemented by JMP/Bxx opcodes, or by using R15 as destination register. In all cases, the next BYTE after the jump opcode is fetched as opcode byte, and is executed before continuing at the jump-target address.

Possible situations are:

```text
  1) jump + NOP                 ;very simple
  2) jump + ONE-BYTE-OPCODE     ;still quite simple
  3) jump + MULTI-BYTE-OPCODE   ;rather strange
  4) Prefix + jump + ONE-BYTE-SUFFIX
  5) Prefix + jump + MULTI-BYTE-SUFFIX
```

In case 3, the first opcode-byte is picked from the address after jump, the following byte(s) from the jump-destination.

In case 4/5, the prefix is located before the jump, the next byte after the jump (this works only with Bxx jumps) (whilst JMP/LJMP or MOV/ALU R15,dest do reset the prefix), and any further bytes at the jump-destination.

#### Mistakes in book2.pdf

BGE/BLT are exchanged with each other. MOVES src/dst operands are exchanged.

LJMP bank/offs operands are exchanged.

#### GSU Undoc opcodes

UMULT #n, WITH, XOR Rn, XOR #n are sorts of undocumented; they should be described (on page 280), but the alphabetical list ends abruptly after UMULT Rn. However, they are listed in the summary (page 101) and in the index (page 409). The WITH opcode is also mentioned in various other places.

page 121: R15 after STOP (strange, is that true?) (yes, it is) page 122: cache/cbr after ABORT

MOV R13,R15  sets R13 to addr of next opcode after MOV (eg. for LOOP start) LINK n       sets R11 to addr+n of next opcode (eg. for "CALLs" via jmp)

#### GSU Power Consumption

The GSU does (when it is running) increase the power consumption, this can overload the SNES power supply if additional peripherals are connected. GSU software should detect which controllers are connected, and refuse to start the GSU if a controller with high power consumption (or with unknown power consumption) is connected. The standard joypads are okay. A Multiplayer 5 adaptor isn't okay (at least, when multiple controllers are connected to it).

#### After STOP

Restarting (somewhere(?) after STOP) is possible by setting GO-flag (done by Dirt Trax FX).
