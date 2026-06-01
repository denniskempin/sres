# SNES Cart GSU-n CPU MOV Opcodes

#### GSU MOV Opcodes (Register/Immediate)

```text
  Opcode     Clks Flags   Native       Nocash
  2s 1d         2 000---- MOVE Rd,Rs   mov Rd,Rs   ;Rd=Rs
  2d Bs         2 000vs-z MOVES Rd,Rs  movs Rd,Rs  ;Rd=Rs (with flags, OV=bit7)
  An pp         2 000---- IBT Rn,#pp   mov Rn,pp   ;Rn=SignExpanded(pp)
  Fn xx yy      3 000---- IWT Rn,#yyxx mov Rn,yyxx ;Rn=yyxx
```

#### GSU MOV Opcodes (Load BYTE from ROM)

```text
  EF          1-6 000---- GETB         movb Rd,[romb:r14]    ;hi=zero-expanded
  3D EF       2-6 000---- GETBH        movb Rd.hi,[romb:r14] ;lo=unchanged
  3E EF       2-6 000---- GETBL        movb Rd.lo,[romb:r14] ;hi=unchanged
  3F EF       2-6 000---- GETBS        movbs Rd,[romb:r14]   ;hi=sign-expanded
```

#### GSU MOV Opcodes (Load/Store Byte/Word to/from RAM)

```text
  3D 4n         6 000---- LDB (Rn)     movb Rd,[ramb:Rn]  ;Rd=Byte[..] ;n=0..11
  4n            7 000---- LDW (Rn)     mov Rd,[ramb:Rn]   ;Rd=Word[..] ;n=0..11
  3D Fn lo hi  11 000---- LM Rn,(hilo) mov Rn,[ramb:hilo] ;Rn=Word[..]
  3D An kk     10 000---- LMS Rn,(yy)  mov Rn,[ramb:kk*2] ;Rn=Word[..]
  3D 3n       2-5 000---- STB (Rn)     movb [ramb:Rn],Rs  ;Byte[..]=Rs ;n=0..11
  3n          1-6 000---- STW (Rn)     mov [ramb:Rn],Rs   ;Word[..]=Rs ;n=0..11
  3E Fn lo hi 4-9 000---- SM (hilo),Rn mov [ramb:hilo],Rn ;Word[..]=Rn
  3E An kk    3-8 000---- SMS (yy),Rn  mov [ramb:kk*2],Rn ;Word[..]=Rn
  90          1-6 000---- SBK          mov [ram:bk],Rs    ;Word[LastRamAddr]=Rs
```

Words at odd addresses are accessing [addr AND NOT 1], with data LSB/MSB swapped. LDB does zero-expand result (Rd.hi=00h). STB does store Rs.lo (ignores Rs.hi). SBK does "writeback" to most recently used RAM address (eg. can be used after LM) (unknown if whole 17bit, including ramb, are saved).

#### GSU ROM/RAM Banks

```text
  3E DF         2 000---- RAMB         movb ramb,Rs ;RAMBR=Rs & 01h ;RAM Bank
  3F DF         2 000---- ROMB         movb romb,Rs ;ROMBR=Rs & FFh ;ROM Bank
```

#### GSU Bitmap Opcodes

```text
  3D 4E         2 000---- CMODE        movb por,Rs           ;=Rs&1Fh
  4E            1 000---- COLOR        movb color,Rs         ;=Rs&FFh
  DF          1-6 000---- GETC         movb color,[romb:r14] ;=[membyte]
  4C         1-48 000---- PLOT         plot [r1,r2],color ;Pixel=COLR, R1=R1+1
  3D 4C     20-74 000-s-z RPIX         rpix Rd,[r1,r2] ;Rd=Pixel? FlushPixCache
```

Unknown if RPIX always sets SF=0, theoretically 2bit/4bit/8bit pixel-colors cannot be negative, unless it uses bit7 as sign, or so?
