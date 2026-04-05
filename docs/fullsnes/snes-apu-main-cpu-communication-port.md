# SNES APU Main CPU Communication Port

```text
2140h - APUI00  - Main CPU to Sound CPU Communication Port 0 (R/W)
2141h - APUI01  - Main CPU to Sound CPU Communication Port 1 (R/W)
2142h - APUI02  - Main CPU to Sound CPU Communication Port 2 (R/W)
2143h - APUI03  - Main CPU to Sound CPU Communication Port 3 (R/W)
  7-0   APU I/O Data   (Write: Data to APU, Read: Data from APU)
```

Caution: These registers should be written only in 8bit mode (there is a hardware glitch that can cause a 16bit write to [2140h..2141h] to destroy [2143h], this might happen only in some situations, like when the cartridge contains too many ROM chips which apply too much load on the bus).

#### Uploader

```text
  Wait until Word[2140h]=BBAAh
  kick=CCh                  ;start-code for first command
  for block=1..num_blocks
    Word[2142h]=dest_addr   ;usually 200h or higher (above stack and I/O ports)
    Byte[2141h]=01h         ;command=transfer (can be any non-zero value)
    Byte[2140h]=kick        ;start command (CCh on first block)
    Wait until Byte[2140h]=kick
    for index=0 to length-1
      Byte[2141h]=[src_addr+index]      ;send data byte
      Byte[2140h]=index.lsb             ;send index LSB (mark data available)
      Wait until Byte[2140h]=index.lsb  ;wait for acknowledge (see CAUTION)
    next index
    kick=(index+2 AND FFh) OR 1 ;-kick for next command (must be bigger than
  next block  ;(if any)         ;         last index+1, and must be non-zero)
  [2142h]=entry_point           ;entrypoint, must be below FFC0h (ROM region)
  [2141h]=00h                   ;command=entry (must be zero value)
  [2140h]=kick                  ;start command
  Wait until Byte[2140h]=kick   ;wait for acknowledge
```

CAUTION: The acknowledge for the last data byte lasts only for a few clock cycles, if the uploader is too slow then it may miss it (for example if it's interrupted); as a workaround, disable IRQs and NMIs during upload, or replace the last "wait for ack" by hardcoded delay.

#### Boot ROM Disassembly

```text
  FFC0 CD EF        mov  x,EF           ;\
  FFC2 BD           mov  sp,x           ; zerofill RAM at [0001h..00EFh]
  FFC3 E8 00        mov  a,00           ; (ie. excluding I/O Ports at F0h..FFh)
                   @@zerofill_lop:      ; (though [00h..01h] destroyed below)
  FFC5 C6           mov  [x],a          ; (also sets stacktop to 01EFh, kinda
  FFC6 1D           dec  x              ; messy, nicer would be stacktop 01FFh)
  FFC7 D0 FC        jnz  @@zerofill_lop ;/
  FFC9 8F AA F4     mov  [F4],AA        ;\notify Main CPU that APU is ready
  FFCC 8F BB F5     mov  [F5],BB        ;/for communication
                   @@wait_for_cc:       ;\
  FFCF 78 CC F4     cmp  [F4],CC        ; wait for initial "kick" value
  FFD2 D0 FB        jnz  @@wait_for_cc  ;/
  FFD4 2F 19        jr   main
                   ;---
                   @@transfer_data:
                   @@wait_for_00:                               ;\
  FFD6 EB F4        mov  y,[F4]     ;index (should become 0)    ;
  FFD8 D0 FC        jnz  @@wait_for_00                          ;/
                   @@transfer_lop:
  FFDA 7E F4        cmp  y,[F4]
  FFDC D0 0B        jnz  FFE9          ------->
  FFDE E4 F5        mov  a,[F5]     ;get data
  FFE0 CB F4        mov  [F4],y     ;ack data
  FFE2 D7 00        mov  [[00]+y],a ;store data
  FFE4 FC           inc  y          ;addr lsb
  FFE5 D0 F3        jnz  @@transfer_lop
  FFE7 AB 01        inc  [01]       ;addr msb
                   @@
  FFE9 10 EF        jns  @@transfer_lop     ;strange...
  FFEB 7E F4        cmp  y,[F4]
  FFED 10 EB        jns  @@transfer_lop
                   ;- - -
                   main:
  FFEF BA F6        movw ya,[F6]                ;\copy transfer (or entrypoint)
  FFF1 DA 00        movw [00],ya    ;addr       ;/address to RAM at [0000h]
  FFF3 BA F4        movw ya,[F4]    ;cmd:kick
  FFF5 C4 F4        mov  [F4],a     ;ack kick
  FFF7 DD           mov  a,y        ;cmd
  FFF8 5D           mov  x,a        ;cmd
  FFF9 D0 DB        jnz  @@transfer_data
  FFFB 1F 00 00     jmp  [0000+x]   ;in: A=0, X=0, Y=0, SP=EFh, PSW=02h
                   ;---
  FFFE C0 FF        dw   FFC0  ;reset vector
```

#### Notes

Many games are jumping to ROM:FFC0h in order to "reset" the SPC700 back to the transmission phase. Some programs are also injumping to ROM:FFC9h (eg. mic_'s "The 700 Club" demo is using that address as dummy-entrypoint after each transfer block; until applying the actual entrypoint after the last block).

Some games may "wrap" from opcodes at RAM:FFBFh to opcodes at ROM:FFC0h. Only known example is World Cup Striker, which contains following code at FFBCh in RAM: "MOV A,80h, MOV [F1h],A", that ensures ROM enabled, and does then continue at FFC0h in ROM (in this specific case, emulators can cheat by handling wraps in their PortF1h handler, rather than checking for PC>=FFC0h after every single opcode fetch).
