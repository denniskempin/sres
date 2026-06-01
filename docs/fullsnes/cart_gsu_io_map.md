# SNES Cart GSU-n I/O Map

GSU I/O Map (in banks 00h-3Fh and 80h-BFh) During GSU operation, only SFR, SCMR, and VCR may be accessed.

```text
  3000h-3001h R0  Default source/destination register (Sreg/Dreg) (R/W)
  3002h-3003h R1  PLOT opcode: X coordinate (0000h on reset) (R/W)
  3004h-3005h R2  PLOT opcode: Y coordinate (0000h on reset) (R/W)
  3006h-3007h R3  General purpose (R/W)
  3008h-3009h R4  LMULT opcode: lower 16bits of result (R/W)
  300Ah-300Bh R5  General purpose (R/W)
  300Ch-300Dh R6  LMULT and FMULT opcodes: multiplier (R/W)
  300Eh-300Fh R7  MERGE opcode (R/W)
  3010h-3011h R8  MERGE opcode (R/W)
  3012h-3013h R9  General purpose (R/W)
  3014h-3015h R10 General purpose (conventionally stack pointer) (R/W)
  3016h-3017h R11 LINK opcode: destination (R/W)
  3018h-3019h R12 LOOP opcode: counter (R/W)
  301Ah-301Bh R13 LOOP opcode: address (R/W)
  301Ch-301Dh R14 GETxx opcodes: Game Pak ROM Address Pointer (R/W)
  301Eh-301Fh R15 Program Counter, writing MSB starts GSU operation (R/W)
  3020h-302Fh -
  3030h-3031h SFR Status/Flag Register (R) (Bit1-5: R/W)
  3032h       -
  3033h       BRAMR Back-up RAM Register (W)
  3034h       PBR   Program Bank Register (8bit, bank 00h..FFh) (R/W)
  3035h       -
  3036h       ROMBR Game Pak ROM Bank Register (8bit, bank 00h..FFh) (R)
  3037h       CFGR  Config Register (W)
  3038h       SCBR  Screen Base Register (8bit, in 1Kbyte units) (W)
  3039h       CLSR  Clock Select Register (W)
  303Ah       SCMR  Screen Mode Register (W)
  303Bh       VCR   Version Code Register (R)
  303Ch       RAMBR Game Pak RAM Bank Register (1bit, bank 70h/71h) (R)
  303Dh       -
  303Eh-303Fh CBR   Cache Base Register (in upper 12bit; lower 4bit=unused) (R)
  N/A         COLR  Color Register (COLOR,GETC,PLOT opcodes)
  N/A         POR   Plot Option Register (CMODE opcode)
  N/A         Sreg/Dreg    Memorized TO/FROM Prefix Selections
  N/A         ROM Read Buffer (1 byte) (prefetched from [ROMBR:R14])
  N/A         RAM Write Buffer (1 byte/word)
  N/A         RAM Address (1 word, or word+rambr?) (for SBK opcode)
  N/A         Pixel Write Buffer (two buffers for one 8-pixel row each)
  3100h-32FFh Cache RAM
```

#### Full I/O Map with Mirrors for Black Blob (VCR=01h)

```text
  3000h..301Fh  20h  R0-R15
  3020h..302Fh  10h  open bus
  3030h..3031h  2    status reg
  3032h..303Fh  0Eh  mirrors of status reg (except 303Bh=01h=VCR)
  3040h..305Fh  20h  mirror of R0-R15
  3060h..307Fh  20h  mirrors of status reg (except 307Bh=01h=VCR)
  3080h..30FFh  80h  open bus
  3100h..32FFh  200h cache
  3300h..332Fh  30h  open bus
  3330h..333Fh  10h  mirrors of status reg (except 333Bh=01h=VCR)
  3340h..335Fh  20h  mirror of R0-R15
  3360h..337Fh  20h  mirrors of status reg (except 337Bh=01h=VCR)
  3380h..33FFh  80h  open bus
  3400h..342Fh  30h  open bus
  3430h..343Fh  10h  mirrors of status reg (except 343Bh=01h=VCR)
  3440h..345Fh  20h  mirror of R0-R15
  3460h..347Fh  20h  mirrors of status reg (except 347Bh=01h=VCR)
  3480h..3FFFh  B80h open bus
```

#### Full I/O Map with Mirrors for GSU2 (VCR=04h)

```text
  3000h..301Fh  20h   R0-R15
  3020h..302Fh  10h   mirror of 3030h..303Fh
  3030h..303Fh  10h   status regs (unused or write-only ones return 00h)
  3040h..30FFh  C0h   mirrors of 3000h..303Fh
  3100h..32FFh  200h  cache
  3300h..34FFh  200h  mirrors of 3000h..303Fh
  3500h..3FFFh  B00h  open-bus
```
