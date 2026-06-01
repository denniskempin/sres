# NSS Interpreter Tokens

#### Tokens

```text
  00h  Reboot_Bios()
  02h  Osd_Wrstr_Direct(Len8,VramAddr16,Data16[Len], ... ,FFh,Sleep0)
  04h  Osd_Wrstr_Encrypted_Txt_Line(Yloc*12,Sleep0)
  06h  Osd_Wrstr_Prom_Title_Slot_80C0h(Len8-1,VramAddr+2000h*N,Sleep0)    ?
  08h  Osd_Wrstr_Prom_Title(Slot+80h,Len8-1,VramAddr+2000h*N,Sleep0)      ?
  0Ah  Port_00h_W_Set_Bits(OrValue)
  0Ch  Port_01h_W_Set_Bits(OrValue)
  0Eh  Port_03h_W_Set_Bits(OrValue)
  10h  Port_00h_W_Mask_Bits(AndValue)
  12h  Port_01h_W_Mask_Bits(AndValue)
  14h  Port_03h_W_Mask_Bits(AndValue)
  16h  Set_80C2h_To_Immediate(Imm8)
  18h  Set_80C3h_To_Immediate(Imm8)
  1Ah  Set_80C4h_To_Immediate(Imm8)
  1Ch  Set_80C5h_To_Immediate(Imm8)
  1Eh  Compare_And_Goto_If_Equal(Addr16,Imm8,Target)          ;\
  20h  Compare_And_Goto_If_Not_Equal(Addr16,Imm8,Target)      ; unsigned
  22h  Compare_And_Goto_If_Below_or_Equal(Addr16,Imm8,Target) ; cmp [addr],imm
  24h  Compare_And_Goto_If_Above(Addr16,Imm8,Target)          ;/
  26h  Decrement_And_Goto_If_Nonzero(Addr16,Target)
  28h  Poke_Immediate(Addr16,Imm8)
  2Ah  Sleep_Long(Sleep16)
  2Ch  Disable_Interpreter_and_Reset_Gosub_Stack()
  2Eh  Osd_Display_Num_Credit_Play(Slot*4,VramAddr16,Sleep0)
  30h  Test_And_Goto_If_Nonzero(Addr16,Imm8,Target)
  32h  Test_And_Goto_If_Zero(Addr16,Imm8,Target)
  34h  Osd_Wrstr_Indirect(Addr16,Sleep0)
  36h  Gosub_To_Subroutine(Target)   ;\max 3 nesting levels
  38h  Return_From_Subroutine()      ;/
  3Ah  Goto(Target)
  3Ch    _xxx()        ... init some values
  3Eh    _xxx()     ... init more, based on inst rom
  40h  Wait_Vblank()          ;or so (waits for Port[00h].bit6)
  42h  Osd_Wrstr_Indexed(index8,Sleep0)
  44h  Reload_Attraction_Timer()
  46h    _xxx()    ... advance to next instruction page ... or so
  48h  Handle_PageUpDown_For_Multipage_Instructions()
  4Ah  Reload_SNES_Watchdog()
  4Ch  Decrease_SNES_Watchdog_and_Goto_if_Expired(Target)
  4Eh    _xxx_osd_SPECIAL...(Slot+80h,Len8-1,VramAddr+2000h*N,Sleep0) ? bugged?
  50h    _copy_cart_flag_bit0_to_port_01_w_bit4()    ... joypad2 vs CN4
  52h  Map_Slot_80C0h()
  54h  Osd_Wrstr_Indirect_Encrypted(Addr16,Sleep0)
```

Below exist in BIOS version "03" only:

```text
  56h  Osd_Wrstr_Num_Credit_Play(VramAddr16,Sleep0)
  58h  Map_Slot_804Ch()
  5Ah    _xxx()      ;two lines: SubtractVramAddrBy1Ah_and_Strip_Underline ?
  5Ch  Osd_Wrstr_Prom_Title_Slot_804Ch_unless_Slot1_Empty(Len8,VramAddr,Sleep0)
  5Eh     Copy_8s19h_To_81E9h()      ;=VRAM Addr for Credits String
  60h  Goto_If_8s23h_Nonzero(Target)
  62h    _xxx(Target)       ;load timer from 8s24h or 8s25h goto if zero
  64h  Goto_If_GameID_is_00h_or_01h_or_02h(Target)
  66h  Create_Centered_Osd_Wrstr_Title_Function_at_84C0h(yloc*24)
  68h    _xxx()           ;... 8s25h, 8s26h, and MM:SS time-limit related ?
```

And, some general token values:

```text
  56h..7Eh  Unused_Lockup()   ;unused version "02" tokens ;\jump to an
  6Ah..7Eh  Unused_Lockup()   ;unused version "03" tokens ;/endless loop
  01h..7Fh  Crash()           ;odd token numbers jump to garbage addresses
  80h..FFh  Sleep_Short(Sleep7)  ;00h..7Fh (in LSBs of Token)
```

Sleep0 is an optional 00h-byte that can be appended after the Wrstr(Params) commands. If the 00h-byte is NOT there, then a Sleep occurs for 1 frame. If the 00h-byte is there, then token execution continues (after skipping the 00h) without Sleeping.

#### Note

INST ROM contains two interpreter functions (invoked via Gosub DF00h and Gosub DF05h).

```text
  DF00h - Custom code (quite simple in F-Zero, very bizarre in ActRaiser)
  DF05h - Display centered & underlined Title in first line
```

Available stack depth is unknown (at least one stack level is used, so there are max two free levels, or maybe less) (the DF00h function CAN use at least one stack level).

The DF05h function is used for displaying the instructions headline (when viewing instructions in Demo mode). The purpose/usage of the DF00h function is unknown; essentially, everything works fine even if it just contains a Return token; for Skill Mode games it also seems to require a Poke(8060h,00h) token.
