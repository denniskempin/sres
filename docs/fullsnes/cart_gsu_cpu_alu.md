# SNES Cart GSU-n CPU ALU Opcodes

#### GSU ALU Opcodes

```text
  Opcode     Clks Flags   Native       Nocash
  5n            1 000vscz ADD Rn       add Rd,Rs,Rn ;Rd=Rs+Rn
  3E 5n         2 000vscz ADD #n       add Rd,Rs,n  ;Rd=Rs+n
  3D 5n         2 000vscz ADC Rn       adc Rd,Rs,Rn ;Rd=Rs+Rn+Cy
  3F 5n         2 000vscz ADC #n       adc Rd,Rs,n  ;Rd=Rs+n+Cy
  6n            1 000vscz SUB Rn       sub Rd,Rs,Rn ;Rd=Rs-Rn
  3E 6n         2 000vscz SUB #n       sub Rd,Rs,n  ;Rd=Rs-n
  3D 6n         2 000vscz SBC Rn       sbc Rd,Rs,Rn ;Rd=Rs-Rn-(Cy XOR 1)
  3F 6n         2 000vscz CMP Rn       cmp Rs,Rn    ;Rs-Rn
  7n            1 000-s-z AND Rn       and Rd,Rs,Rn ;Rd=Rs AND Rn     ;n=1..15!
  3E 7n         2 000-s-z AND #n       and Rd,Rs,n  ;Rd=Rs AND n      ;n=1..15!
  3D 7n         2 000-s-z BIC Rn       bic Rd,Rs,Rn ;Rd=Rs AND NOT Rn ;n=1..15!
  3F 7n         2 000-s-z BIC #n       bic Rd,Rs,n  ;Rd=Rs AND NOT n  ;n=1..15!
  Cn            1 000-s-z OR  Rn       or  Rd,Rs,Rn ;Rd=Rs OR Rn      ;n=1..15!
  3E Cn         2 000-s-z OR  #n       or  Rd,Rs,n  ;Rd=Rs OR n       ;n=1..15!
  3D Cn (?)     2 000-s-z XOR Rn       xor Rd,Rs,Rn ;Rd=Rs XOR Rn     ;n=1..15?
  3F Cn (?)     2 000-s-z XOR #n       xor Rd,Rs,n  ;Rd=Rs XOR n      ;n=1..15?
  4F            1 000-s-z NOT          not Rd,Rs    ;Rd=Rs XOR FFFFh
```

#### GSU Rotate/Shift/Inc/Dec Opcodes

```text
  03            1 000-0cz LSR          shr Rd,Rs,1  ;Rd=Rs SHR 1
  96            1 000-scz ASR          sar Rd,Rs,1  ;Rd=Rs SAR 1
  04            1 000-scz ROL          rcl Rd,Rs,1  ;Rd=Rs RCL 1 ;\through
  97            1 000-scz ROR          rcr Rd,Rs,1  ;Rd=Rs RCR 1 ;/carry
  3D 96         2 000-scz DIV2         div2 Rd,Rs   ;Rd=Rs SAR 1, Rd=0 if Rs=-1
  Dn            1 000-s-z INC Rn       inc Rn       ;Rn=Rn+1          ;n=0..14!
  En            1 000-s-z DEC Rn       dec Rn       ;Rn=Rn-1          ;n=0..14!
```

#### GSU Byte Operations

```text
  4D            1 000-s-z SWAP         ror Rd,Rs,8    ;Rd=Rs ROR 8
  95            1 000-s-z SEX          movbs Rd,Rs    ;Rd=SignExpanded(Rs&FFh)
  9E            1 000-s-z LOB          and Rd,Rs,0FFh ;Rd=Rs AND FFh  ;SF=Bit7
  C0            1 000-s-z HIB          shr Rd,Rs,8    ;Rd=Rs SHR 8    ;SF=Bit7
  70            1 000xxxx MERGE        merge Rd,r7,r8 ;Rd=R7&FF00 + R8/100h
```

Flags for MERGE are:

```text
  S = set if (result AND 8080h) is nonzero
  V = set if (result AND C0C0h) is nonzero
  C = set if (result AND E0E0h) is nonzero
  Z = set if (result AND F0F0h) is nonzero (not set when zero!)
```

#### GSU Multiply Opcodes

```text
  9F          4,8 000-scz FMULT     smulw Rd:nul,Rs,r6 ;Rd=signed(Rs*R6/10000h)
  3D 9F       5,9 000-scz LMULT     smulw Rd:R4,Rs,R6  ;Rd:R4=signed(Rs*R6)
  8n          1,2 000-s-z MULT Rn      smulb Rd,Rs,Rn ;Rd=signed(RsLsb*RnLsb)
  3E 8n       2,3 000-s-z MULT #n      smulb Rd,Rs,n  ;Rd=signed(RsLsb*0..15)
  3D 8n       2,3 000-s-z UMULT Rn     umulb Rd,Rs,Rn ;Rd=unsigned(RsLsb*RnLsb)
  3F 8n (?)   2,3 000-s-z UMULT #n     umulb Rd,Rs,n  ;Rd=unsigned(RsLsb*0..15)
```

The multiply speed can be selected via CFGR register. Do not use FMULT with Dreg=R4 (this will reportedly leave R4 unchanged). When using LMULT with Dreg=R4 then the result will be R4=MSB (and LSB is lost). Ie. if that is true then, strangely, LMULT R4 <does> work as how FMULT R4 <should> work.
