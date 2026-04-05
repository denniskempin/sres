# SNES Controllers Detecting Controller Support of ROM-Images

Below are some methods to detect controller support by examining ROM-images.

The methods aren't fail-proof, but may be useful to track-down controller support in many games.

#### Detection Method Summary

```text
  Type              Method
  Joypad            <none/default>
  Mouse             String "START OF MOUSE BIOS", or opcodes (see below)
  Multiplayer 5     String "START OF MULTI5 BIOS"
  Super Scope       String "START OF SCOPE BIOS", or Title=<see list>
  Lasabirdie        String "GOLF_READY!"
  X-Band Keyboard   String "ZSAW@",x,x,"CXDE$#" (keyboard translation table)
  Turbo File (STF)  String "FAT0SHVC"
  Turbo File (TFII) Opcodes "MOV Y,000Fh/MOV A,[004017h]/DEC Y/JNZ $-5"
  Exertainment      Opcodes "MOV [21C1h],A/MOV A,0Bh/MOV [21C4h],A/MOV X,20F3h"
  Barcode Battler   Opcodes "INC X/CMP X,(00)0Ah/JNC $-6(-1)/RET/36xNOP/RET"
  Voice-Kun         Opcodes "MOV [004201h],A/CLR P,20h/MOV A,D/INC A"
  Justifier         Title="LETHAL ENFORCERS     "
  M.A.C.S.          Title="MAC:Basic Rifle      "
  Twin Tap          Title="QUIZ OH SUPER        "
  Miracle Piano     Title="MIRACLE              "
  NTT Data Pad      Title="NTT JRA PAT          "
  SFC Modem         Title="NTT JRA PAT          "
  Pachinko          Title=CB,AF,BB,C2,CA,DF,C1,DD,CB,AF,BB,C2,CA,DF,C1,DD,xx(6)
  BatterUP          -  ;\
  TeeV Golf         -  ; these are probably simulating standard joypads
  StuntMaster       -  ; (and thus need no detection)
  Nordic Quest      -  ;/
```

#### START OF xxx BIOS Strings

These strings were included in Nintendo's hardware driver source code files, the strings themselves have no function (so one could simply remove them), but Nintendo prompted developers to keep them included. They are typically arranged like so:

```text
  "START OF xxx BIOS"
  [bios program code...]
  "NINTENDO SHVC xxx BIOS VER x.xx"
  "END OF xxx BIOS"
```

Whereas, the version string may be preceeded by "MODIFIED FROM ", and "VER" may be "Ver" in some cases, sometimes without space between "Ver" and "x.xx". In case of custom code one may omit the version string or replace it by "MY BIOS VERSION" or so, but one should include the "START/END OF xxx BIOS" strings to ease detection.

#### Multiplayer 5

Games that do SUPPORT the hardware should contain following string:

```text
  "START OF MULTI5 BIOS"
```

Games that do DETECT the hardware should contain following string:

```text
  "START OF MULTI5 CONNECT CHECK"
```

Games that contain only the "CHECK" part (but not the "BIOS" part) may REJECT to operate with the hardware.

Some MP5 games (eg. "Battle Cross") do lack the "START OF ..." strings.

#### Mouse

Games that do SUPPORT the hardware should contain following string:

```text
  "START OF MOUSE BIOS"
```

Some Mouse games (eg. Arkanoid Doh It Again) do lack the "START OF ..." strings. For such games, checking following opcodes may help:

```text
  MOV Y,0Ah/LOP:/MOV A,[(00)4016h+X]/DEC Y/JNZ LOP/MOV A,[(00)4016h+X]
```

The official mouse BIOS uses 24bit "004016h+X" (BF 16 40 00), Arkanoid uses 16bit "4016h+X" (BD 16 40).

Warning: Automatically activiting mouse-emulation for mouse-compatible games isn't a very good idea: Some games expect the mouse in port 1, others in port 2, so there's a 50% chance to pick the wrong port. Moreover, many games are deactivating normal joypad input when sensing a connected mouse, so automatic mouse emulation will also cause automatic problems with normal joypad input.

#### SuperScope

Games that do SUPPORT the hardware should (but usually don't) contain following string:

```text
  "START OF SCOPE BIOS"
```

In practice, the string is included only in "Yoshi's Safari" whilst all other (older and newer) games contain entirely custom differently implemented program code without ID strings, making it more or less impossible to detect SuperScope support. One workaround would be to check known Title strings:

```text
  "BATTLE CLASH         "    "METAL COMBAT         "    "T2 ARCADE            "
  "BAZOOKA BLITZKRIEG   "    "OPERATION THUNDERBOLT"    "TINSTAR              "
  "Hunt for Red October "    "SPACE BAZOOKA        "    "X ZONE               "
  "LAMBORGHINI AMERICAN "    "SUPER SCOPE 6        "    "YOSHI'S SAFARI       "
  "Lemmings 2,The Tribes"
```

#### Lasabirdie

Games that do support the hardware should contain "GOLF_READY!" string (used to verify the ID received from the hardware).

#### Turbo File Twin in STF Mode

Games that do support the hardware should contain "FAT0" and SHVC" strings, which are usually (maybe always) stored as continous "FAT0SHVC" string.

Turbo File Twin in TFII Mode or Turbo File Adapter There aren't any specific ASCII Strings. However, most (or all) games contain the "MOV Y,000F, MOV A,[004017], DEC Y, JNZ $-5" opcode sequence (exactly like so, ie. with Y=16bit, and address=24bit).

#### NSRT Header

Some ROM-images do contain information about supported controllers in NSRT Headers. In practice, most ROM-images don't have that header (but can be "upgraded" by Nach's NSRT tool).

> **See:** [SNES Cartridge ROM-Image Headers and File Extensions](snes-cartridge-rom-image-headers-and-file-extensions.md)

The NSRT format isn't officially documented. The official way to create headers seems to be to contact the author (Nach), ask him to add controller flags for a specific game, download the updated version of the NSRT tool, use it to update your ROM-image, and then you have the header (which consists of undocumented, and thereby rather useless values).
