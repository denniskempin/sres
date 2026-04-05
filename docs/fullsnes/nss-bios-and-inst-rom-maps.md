# NSS BIOS and INST ROM Maps

#### NSS BIOS ROM (32K mapped to 0000h-7FFFh)

```text
  0000h   Reset Vector
  0008h   RST Handlers (internally used by PROM checks)
  0066h   NMI Handler (unknown source, probably Vblank or Vsync or so)
  3FFDh   Hardcoded Token Address (used by F-Zero INST ROM)
  5F30h   Hardcoded Return-Address from 2nd PROM check in INST ROM
```

#### NSS INST ROM (8K mapped to C000h-DFFFh)

```text
  [C034h]+00h..31h   Encrypted Data (to be decrypted via PROM data)
  [C034h]+32h..33h   Chksum on above 32h bytes (all BYTEs added together)
  [C67Fh]+C600h      RST 38h for 1st PROM check ;\
  [C67Fh]*100h+7Fh   RST 28h for 1st PROM check ; for decrypting the
  [C77Fh]+C700h      RST 20h for 1st PROM check ; 32h-byte area
  [C77Fh]*100h+7Fh   RST 30h for 1st PROM check ;/
  [D627h]+D600h      RST 38h for 2nd PROM check ;\for decrypting the
  [D627h]*100h+27h   RST 28h for 2nd PROM check ; 21-byte title (and
  [D727h]+D700h      RST 20h for 2nd PROM check ; verifying [DC3Fh])
  [D727h]*100h+27h   RST 30h for 2nd PROM check ;/
  [(where are?)]     RST's   for 3rd PROM check ;-this part looks bugged
  [DC15h+00h..29h]   Spaces,FFh,"-credit play" (with underline attr) (for Menu)
  [DC3Fh]            8bit chksum for 2nd PROM security check
  [DD3Fh]            8bit chksum for 3rd PROM security check
  [DEF1h..DEFFh]     Title (for Bookkeeping) (in 8bit OSD characters)
  [DF00h..DF02h]     Token Entrypoint 1 (Goto token)
  [DF05h..DF07h]     Token Entrypoint 2 (Goto token) (overlaps below DF06h!)
  [[DF06h]+6]        Title Xloc+Odd MSBs (for title-centering via token 66h)
  [NNNNh]            Further locations accessed via pointers in 32h-byte area
  [C032h]            16bit Ptr to inst.chksum.lsb ;\all WORDs at C000..DFFF
  [DFFEh]            16bit Ptr to inst.chksum.msb ;/added together
```

32h-Byte Area at [C034h]+00h..31h (encrypted via PROM data)

```text
  00h      Flags
             Bit0 Player 2 Controls (0=CN4 Connector, 1=Normal/Joypad 2)
             Bit1 Unused (should be 0)
             Bit2 Unused (should be 0)
             Bit3 Continue Type (0=Normal/Resume Game, 1=Reset Game)
             Bit4 Continue (1=Prompt "Insert Coin to Continue" in Skill Mode)
             Bit5 Used entry (must be 1) (otherwise treated as empty slot)
             Bit6 Checksum Type ([2Ah,2Bh] and num "0" bits in chk[2Eh-2Fh])
             Bit7 Skill Mode (0=Time-Limit Mode, 1=Skill Mode)
  01h      GameID (must be a unique value; BIOS rejects carts with same IDs)
  02h-16h  Title (21 OSD chars) (needs second PROM decryption pass)
  17h-18h  Attraction/Demo Time (in "NMI" units) ("You Are Now Viewing...")
  19h-1Ah  VRAM Addr for Inserted Credits string (during game play)
  1Bh-1Ch  Ptr to List of Encrypted Instruction Text Lines
           (len byte, followed by len+1 pointers to 24-word text strings)
  1Dh      Default Price (number of credits per game) (LSB must be 01h..09h)
  1Eh      Time Minutes (BCD) ;\(TIME mode: MUST be 01:00 .. 30:00 and LSB
  1Fh      Time Seconds (BCD) ;/MUST be 0 or 5)
           (In SKILL mode: [1Eh]=0Dh, some Continue delay used when Flags.4=1)
  20h-21h  VRAM Addr for Remaining Time value (unused in Skill Mode)
  22h      SNES Watchdog (SNES must read joypads every N frames; 00h=Disable)
  23h         ??? Byte... (jump enable for token 60h) (allow money-back?)
  24h         ???Byte, alternate for [25h]?
  25h         ???Byte, time-limit related; combined with [1Eh..1Fh,26h..27h]?
  26h-27h     ???Word (unused for GameID 00-02; these use 00C0h/0140h)
  28h-29h  Unused (0000h)
  2Ah-2Bh  Checksum adjust (optional XOR value for [30h-31h], when Flags.6=1)
  2Ch-2Dh  Encrypted.ptr to 4th check xfer.order.XOR.byte (eg.byte 07h=reverse)
  2Eh-2Fh  Encrypted.ptr to 4th check 8-byte key (sometimes depends [01h])
  30h-31h  Checksum accross [00h..2Fh], eventually XORed with [2Ah]:[2Bh]
```

Note: After decryption, the above 32h-bytes are stored at 8s00h..8s31h (with s=0..2 for slot 1-3).

Note: Instructions can be viewed by pressing Instructions Button, either during game, or in demo mode.

#### Skill Mode Notes

There are some variants (unknown how exactly to select which variant):

```text
  Game RESTARTS after Game Over (if one still has credits)
  Game CONTINUES after Game Over (if one still has credits)
```

And, if one does NOT have credits remaining:

```text
  Game PROMPTS insert coin to CONTINUE (eg. ActRaiser)
  Game ABORTS and goes to Game Menu
```

And, for supporting Skill Mode, the DF00h function must contain a Poke(8060h,00h) token.

#### GameID Notes

Known values used by original games are 00h..09h, FDh, and FFh. The homebrew Magic Floor game is using ID 3Fh. The no$sns/a22i tool assigns IDs 40h..BFh based on the game Title checksum (that assignment does more or less reduce risk that different homebrew games could conflict with each other).

#### Tools

The a22i assembler (in no$sns debugger, v1.3 and up) allows to create INST ROM files with title, instructions, checksums, time/skill settings, and special PROM-less RST handlers. For details see the "magicnss.a22" sample source code in the "magicsns.zip" package.
