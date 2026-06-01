# CPU Assembler Directives/Syntax

Below are some common 65XX assembler directives, and the corresponding expressions in 80XX-style language.

```text
  65XX-style    80XX-style         Expl.
  .native       .nocash            select native or nocash syntax
  *=$c100       org 0c100h         sets the assumed origin in memory
  *=*+8         org $+8            increments origin, does NOT produce data
  label         label:             sets a label equal to the current address
  label=$dc00   label equ 0dc00h   assigns a value or address to label
  .by $00       db 00h             defines a (list of) byte(s) in memory
  .byt $00      defb 00h           same as .by and db
  .wd $0000     dw 0000h           defines a (list of) word(s) in memory
  .end          end                indicates end of source code file
  |nn           [|nn]              force 16bit "00NN" instead 8bit "NN"
  #<nnnn        nnnn AND 0FFh      isolate lower 8bits of 16bit value
  #>nnnn        nnnn DIV 100h      isolate upper 8bits of 16bit value
```

#### Special Directives

```text
  .65xx       Select 6502 Instruction Set
  .nes        Create NES ROM-Image with .NES extension
  .c64_prg    Create C64 file with .PRG extension/stub/fixed entry
  .c64_p00    Create C64 file with .P00 extension/stub/fixed entry/header
  .vic20_prg  Create VIC20/C64 file with .PRG extension/stub/relocated entry
  end entry   End of Source, the parameter specifies the entrypoint
```

The C64 files contain Basic Stub "10 SYS<entry>" with default ORG 80Eh.

#### VIC20 Stub

The VIC20 Stub is "10 SYSPEEK(44)*256+<entry>" with default ORG 1218h, this relocates the entryoint relative to the LOAD address (for C64: 818h, for VIC20:

1018h (Unexpanded), 0418h (3K Expanded), 1218h (8K and more Expansion). It does NOT relocate absolute addresses in the program, if the program wishes to run at a specific memory location, then it must de-relocate itself from the LOAD address to the desired address.
