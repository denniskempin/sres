# SNES Cart Capcom CX4 - Functions

CX4 Functions (as contained in Mega Man X2/X3 ROMs) The CX4 functions are located at SNES address 02:8000-02:9FFF (aka CX4 addresses at PAGE:PC=0000:00..000F:FF with BASE=028000):

```text
  PAGE:PC__Function_____________________________
  0000:00  build_oam
  0001:00  scale_tiles  ;<-- (seems to be unused by Mega Man games)
  0002:00  hires_sqrt   ;<-- (seems to be unused by Mega Man games)
  0002:03  sqrt         ;<-- (seems to be unused by Mega Man games)
  0002:05  propulsion
  0002:07  get_sin      ;<-- (seems to be unused by Mega Man games)
  0002:0A  get_cos      ;<-- (seems to be unused by Mega Man games)
  0002:0D  set_vector_length
  0002:10  triangle1
  0002:13  triangle2
  0002:15  pythagorean
  0002:1F  arc_tan
  0002:22  trapeziod
  0002:25  multiply
  0002:2D  transform_coordinates
  0003:00  scale_rotate1
  0005:00  transform_lines
  0007:00  scale_rotate2
  0008:00  draw_wireframe_without_clearing_buffer
  0008:01  draw_wireframe_with_clearing_buffer
  000B:00  disintergrate
  000C:00  wave
  000E:00  test_set_r0_to_00h ;\sixteen 4-word functions,
  ...      ...                ; located at 000E:00+4*(0..15)
  000E:3C  test_set_r0_to_0Fh ;/setting R0 to 00h..0Fh
  000E:40  test_2K_ram_chksum
  000E:54  test_square           ;R1:R2 = R0*R0
  000E:5C  test_immediate_register  ;copy 16 cpu constants to 30h-bytes RAM
  000E:89  test_3K_rom_chksum  ;"immediate_rom"
```

Both Mega Man X2 and X3 are containing 1:1 the same CX4 code (the only two differences are different ROM bank numbers for "Wireframe" vertices):

```text
  Mega Man X2:  [0008:3B]="or a,a,28h"  [000A:C4]="mov a,28h"  ;ROM bank 28h
  Mega Man X3:  [0008:3B]="or a,a,08h"  [000A:C4]="mov a,08h"  ;ROM bank 08h
```

That differences apply to the US/Canada versions. There <might> be further differences in Japanese and/or European versions(?)
