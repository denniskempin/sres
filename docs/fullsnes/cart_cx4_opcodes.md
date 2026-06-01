# SNES Cart Capcom CX4 - Opcodes

#### CX4 Opcodes (all are 16bit wide)

```text
  Opcode         Clks NZC Syntax
  0000h            ?? ??? nop     ;nop is used as delay after "mul" opcodes
  0400h            ?? ??? -
  0800h+p0aaaaaaaa ?? ??? jmp   addr/prg_page:addr
  0C00h+p0aaaaaaaa ?? ??? jz    addr/prg_page:addr  ;Z=1 (equal)
  1000h+p0aaaaaaaa ?? ??? jc    addr/prg_page:addr  ;C=1 (above/equal)
  1400h+p0aaaaaaaa ?? ??? js    addr/prg_page:addr  ;N=1 (negative)
  1800h            ?? ??? -
  1C00h            ?? ??? finish ext_dta
  2000h            ?? ??? -
  2400h+nn0000000n ?? ??? skip<?/?/nc/c/nz/z/ns/s>  ;skip next opcode
  2800h+p0aaaaaaaa ?? ??? call  addr/prg_page:addr
  2C00h+p0aaaaaaaa ?? ??? callz addr/prg_page:addr  ;Z=1 (equal)
  3000h+p0aaaaaaaa ?? ??? callc addr/prg_page:addr  ;C=1 (above/equal)
  3400h+p0aaaaaaaa ?? ??? calls addr/prg_page:addr  ;N=1 (negative)
  3800h            ?? ??? -
  3C00h            ?? ??? ret
  4000h            ?? ??? inc   ext_ptr
  4400h            ?? ??? -
  4800h+ssoooooooo ?? ??? cmp   <op>,A/A*2/A*100h/A*10000h     ;\
  4C00h+ssoooooooo ?? ??? cmp   <imm>,A/A*2/A*100h/A*10000h    ; compare
  5000h+ssoooooooo ?? NZC cmp   A/A*2/A*100h/A*10000h,<op>     ;
  5400h+ssoooooooo ?? NZC cmp   A/A*2/A*100h/A*10000h,<imm>    ;/
  5800h+ss00000000 ?? ??? mov   A,A.?/lsb/lsw/?                ;-sign-expand
  5C00h            ?? ??? -
  6000h+nnoooooooo ?? ??? mov   A/ext_dta/?/prg_page,<op>
  6400h+nnoooooooo ?? ??? mov   A/?/?/prg_page,<imm>
  6800h+nnoooooooo ?? ??? movb  ram_dta.lsb/mid/msb/?,cx4ram[<op>]
  6C00h+nnoooooooo ?? ??? movb  ram_dta.lsb/mid/msb/?,cx4ram[ram_ptr+<imm>]
  7000h+00oooooooo ?? ??? mov   rom_dta,cx4rom[<op>*3]
  7400h            ?? ??? -
  7800h+0noooooooo ?? ??? mov   prg_page.lsb/msb,<op>
  7C00h+0noooooooo ?? ??? mov   prg_page.lsb/msb,<imm>
  8000h+ssoooooooo ?? ??C add   A,A/A*2/A*100h/A*10000h,<op>   ;\
  8400h+ssoooooooo ?? ?Z? add   A,A/A*2/A*100h/A*10000h,<imm>  ;
  8800h+ssoooooooo ?? ??? sub   A,<op>,A/A*2/A*100h/A*10000h   ; add/subtract
  8C00h+ssoooooooo ?? ??C sub   A,<imm>,A/A*2/A*100h/A*10000h  ;
  9000h+ssoooooooo ?? NZC sub   A,A/A*2/A*100h/A*10000h,<op>   ;
  9400h+ssoooooooo ?? NZC sub   A,A/A*2/A*100h/A*10000h,<imm>  ;/
  9800h+00oooooooo ?? ??? smul  MH:ML,A,<op>    ;\use NOP or other opcode,
  9C00h+00oooooooo ?? ??? smul  MH:ML,A,<imm>   ;/result is signed 48bit
  A000h            ?? ??? -
  A400h            ?? ??? -
  A800h+ssoooooooo ?? ??? xor   A,A/A*2/A*100h/A*10000h,<op>   ;\
  AC00h+ssoooooooo ?? ??? xor   A,A/A*2/A*100h/A*10000h,<imm>  ;
  B000h+ssoooooooo ?? ?Z? and   A,A/A*2/A*100h/A*10000h,<op>   ; logic
  B400h+ssoooooooo ?? ?Z? and   A,A/A*2/A*100h/A*10000h,<imm>  ;
  B800h+ssoooooooo ?? ??? or    A,A/A*2/A*100h/A*10000h,<op>   ;
  BC00h+ssoooooooo ?? ??? or    A,A/A*2/A*100h/A*10000h,<imm>  ;/
  C000h+00oooooooo ?? ??? shr   A,<op>                         ;\
  C400h+00oooooooo ?? NZ? shr   A,<imm>                        ;
  C800h+00oooooooo ?? ??? sar   A,<op>                         ;
  CC00h+00oooooooo ?? N?? sar   A,<imm>                        ; shift/rotate
  D000h+00oooooooo ?? ??? ror   A,<op>                         ;
  D400h+00oooooooo ?? ??? ror   A,<imm>                        ;
  D800h+00oooooooo ?? ??? shl   A,<op>                         ;
  DC00h+00oooooooo ?? N?? shl   A,<imm>                        ;/
  E000h+00oooooooo ?? ??? mov   <op>,A
  E400h            ?? ??? -
  E800h+nnoooooooo ?? ??? movb  cx4ram[<op>],ram_dta.lsb/mid/msb/?
  EC00h+nnoooooooo ?? ??? movb  cx4ram[ram_ptr+<imm>],ram_dta.lsb/mid/msb/?
  F000h+00oooooooo ?? ??? xchg  <op>,A
  F400h            ?? ??? -
  F800h            ?? ??? -
  FC00h            ?? ??? stop          ;stop, and clear Port [FF5E].bit6
```

#### Opcode "Middle" 2bits (Bit9-8)

Selects different parameters for some opcodes (eg. lsb/mid/msb, as shown in above descriptions).

#### Opcode Lower 8bits (Bit7-0)

Lower Bits <op>:

```text
  00h Register A
  01h Register MH       ;multiply.result.upper.24bit (MSBs are sign-expanded)
  02h Register ML       ;multiply.result.lower.24bit (same for signed/unsigned)
  03h Register ext_dta
  08h Register rom_dta
  0Ch Register ram_dta
  13h Register ext_ptr  ;24bit SNES memory address
  1Ch Register ram_ptr
  2Eh Special  snesrom[ext_ptr] (?)  ;for use by opcode 612Eh only (?)
  50h Constant 000000h
  51h Constant FFFFFFh
  52h Constant 00FF00h
  53h Constant FF0000h
  54h Constant 00FFFFh
  55h Constant FFFF00h
  56h Constant 800000h
  57h Constant 7FFFFFh
  58h Constant 008000h
  59h Constant 007FFFh
  5Ah Constant FF7FFFh
  5Bh Constant FFFF7Fh
  5Ch Constant 010000h
  5Dh Constant FEFFFFh
  5Eh Constant 000100h
  5Fh Constant 00FEFFh
  6xh Register R0..R15, aka Port [7F80h+x*3] ;(x=0h..Fh)
```

Lower Bits <imm>:

```text
  nnh Immediate 000000h..0000FFh (unsigned)
```

Lower Bits jump/call<addr>:

```text
  nnh Program Counter LSBs (within 256-word page) (absolute, non-relative)
```

Lower Bits skip<cond>:

```text
  00h Skip next opcode if selected flag is zero (conditions ?/nc/nz/ns)
  01h Skip next opcode if selected flag is set  (conditions ?/c/z/s)
```

Lower Bits for opcodes that don't use them (uuuuuuuu):

```text
  00h Unused, should be zero
```
