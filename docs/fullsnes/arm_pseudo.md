# ARM Pseudo Instructions and Directives

#### ARM Pseudo Instructions

```text
  nop              mov r0,r0
  ldr Rd,=Imm      ldr Rd,[r15,disp] ;use .pool as parameter field
  add Rd,=addr     add/sub Rd,r15,disp
  adr Rd,addr      add/sub Rd,r15,disp
  adrl Rd,addr     two add/sub opcodes with disp=xx00h+00yyh
  mov Rd,Imm       mvn Rd,NOT Imm    ;or vice-versa
  and Rd,Rn,Imm    bic Rd,Rn,NOT Imm ;or vice-versa
  cmp Rd,Rn,Imm    cmn Rd,Rn,-Imm    ;or vice-versa
  add Rd,Rn,Imm    sub Rd,Rn,-Imm    ;or vice-versa
```

All above opcodes may be made conditional by specifying a {cond} field.

#### A22i Directives

```text
  org  adr     assume following code from this address on
  .gba         indicate GBA program
  .nds         indicate NDS program
  .fix         fix GBA/NDS header checksum
  .norewrite   do not delete existing output file (keep following data in file)
  .data?       following defines RAM data structure (assembled to nowhere)
  .code        following is normal ROM code/data (assembled to ROM image)
  .include     includes specified source code file (no nesting/error handling)
  .import      imports specified binary file (optional parameters: ,begin,len)
  .radix nn    changes default numeric format (nn=2,8,10,16 = bin/oct/dec/hex)
  .errif expr  generates an error message if expression is nonzero
  .if expr     assembles following code only if expression is nonzero
  .else        invert previous .if condition
  .endif       terminate .if/.ifdef/.ifndef
  .ifdef sym   assemble following only if symbol is defined
  .ifndef sym  assemble following only if symbol is not defined
  .align nn    aligns to an address divisible-by-nn, inserts 00's
  l equ n      l=n
  l:   [cmd]   l=$   (global label)
  @@l: [cmd]   @@l=$ (local label, all locals are reset at next global label)
  end          end of source code
  db ...       define 8bit data (bytes)
  dw ...       define 16bit data (halfwords)
  dd ...       define 32bit data (words)
  defs nn      define nn bytes space (zero-filled)
  ;...         defines a comment (ignored by the assembler)
  //           alias for CRLF, eg. allows <db 'Text',0 // dw addr> in one line
```

#### A22i Alias Directives (for compatibility with other assemblers)

```text
  align        .align 4          code16    .thumb
  align nn     .align nn         .code 16  .thumb
  % nn         defs nn           code32    .arm
  .space nn    defs nn           .code 32  .arm
  ..ds nn      defs nn           ltorg     .pool
  x=n          x equ n           .ltorg    .pool
  .equ x,n     x equ n           ..ltorg   .pool
  .define x n  x equ n           dcb       db (8bit data)
  incbin       .import           defb      db (8bit data)
  @@@...       ;comment          .byte     db (8bit data)
  @ ...        ;comment          .ascii    db (8bit string)
  @*...        ;comment          dcw       dw (16bit data)
  @...         ;comment          defw      dw (16bit data)
  .text        .code             .hword    dw (16bit data)
  .bss         .data?            dcd       dd (32bit data)
  .global      (ignored)         defd      dd (32bit data)
  .extern      (ignored)         .long     dd (32bit data)
  .thumb_func  (ignored)         .word     dw/dd, don't use
  #directive   .directive        .end      end
  .fill nn,1,0 defs nn
```

Alias Conditions, Opcodes, Operands

```text
  hs   cs   ;condition higher or same = carry set
  lo   cc   ;condition lower = carry cleared
  asl  lsl  ;arithmetic shift left = logical shift left
```

#### A22i Numeric Formats & Dialects

```text
  Type          Normal       Alias
  Decimal       85           #85  &d85
  Hexadecimal   55h          #55h  0x55  #0x55  $55  &h55
  Octal         125o         0o125  &o125
  Ascii         'U'          "U"
  Binary        01010101b    %01010101  0b01010101  &b01010101
  Roman         &rLXXXV      (very useful for arrays of kings and chapters)
```

Note: The default numeric format can be changed by the .radix directive (usually 10=decimal). For example, with radix 16, values like "85" and "0101b" are treated as hexadecimal numbers (in that case, decimal and binary numbers can be still defined with prefixes &d and &b).

#### A22i Numeric Operators Priority

```text
  Prio  Operator           Aliases
  8     (,) brackets
  7     +,- sign
  6     *,/,MOD,SHL,SHR    MUL,DIV,<<,>>
  5     +,- operation
  4     EQ,GE,GT,LE,LT,NE  =,>=,>,<=,<,<>,==,!=
  3     NOT
  2     AND
  1     OR,XOR             EOR
```

Operators of same priority are processed from left to right.

Boolean operators (priority 4) return 1=TRUE, 0=FALSE.

#### A22i Nocash Syntax

Even though A22i does recognize the official ARM syntax, it's also allowing to use friendly code:

```text
  mov   r0,0ffh         ;no C64-style "#", and no C-style "0x" required
  stmia [r7]!,r0,r4-r5  ;square [base] brackets, no fancy {rlist} brackets
  mov   r0,cpsr         ;no confusing MSR and MRS (whatever which is which)
  mov   r0,p0,0,c0,c0,0 ;no confusing MCR and MRC (whatever which is which)
  ldr   r0,[score]      ;allows to use clean brackets for relative addresses
  push  rlist           ;alias for stmfd [r13]!,rlist (and same for pop/ldmfd)
  label:                ;label definitions recommended to use ":" colons
```

[A22i is the no$gba debug version's built-in source code assembler.]
