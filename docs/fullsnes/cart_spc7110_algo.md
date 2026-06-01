# SNES Cart SPC7110 Decompression Algorithm

decompress_mode0(src,dst,len)

```text
  initialize
  while len>0
    decoded=0
    con=0, decompression_core
    con=1+decoded, decompression_core
    con=3+decoded, decompression_core
    con=7+decoded, decompression_core
    out = (out SHL 4) XOR (((out SHR 12) XOR decoded) AND Fh)
    decoded=0
    con=15, decompression_core
    con=15+1+decoded, decompression_core
    con=15+3+decoded, decompression_core
    con=15+7+decoded, decompression_core
    out = (out SHL 4) XOR (((out SHR 12) XOR decoded) AND Fh)
    [dst]=(out AND FFh), dst=dst+1, len=len-1
```

decompress_mode1(src,dst,len)

```text
  initialize
  while len>0
   if (buf_index AND 01h)=0
    for pixel=0 to 7
      a = (out SHR 2)  AND 03h
      b = (out SHR 14) AND 03h
      decoded=0
      con = get_con(a,b,c)
      decompression_core
      con = con*2+5+decoded
      decompression_core
      do_pixel_order(a,b,c,2,decoded)
    plane0.bits(7..0) = out.bits(15,13,11,9,7,5,3,1)
    plane1.bits(7..0) = out.bits(14,12,10,8,6,4,2,0)
    [dst]=plane0
   else
    [dst]=plane1
   buf_index=buf_index+1, dst=dst+1, len=len-1
```

decompress_mode2(src,dst,len)

```text
  initialize
  while len>0
   if (buf_index AND 11h)=0
    for pixel=0 to 7
      a = (out SHR 0)  AND 0Fh
      b = (out SHR 28) AND 0Fh
      decoded=0
      con=0
      decompression_core
      con=decoded+1
      decompression_core
      if con=2 then con=decoded+11 else con = get_con(a,b,c)+3+decoded*5
      decompression_core
      con=Mode2ContextTable[con]+(decoded AND 1)
      decompression_core
      do_pixel_order(a,b,c,4,decoded)
    plane0.bits(7..0) = out.bits(31,27,23,19,15,11,7,3)
    plane1.bits(7..0) = out.bits(30,26,22,18,14,10,6,2)
    plane2.bits(7..0) = out.bits(29,25,21,17,13, 9,5,1)
    plane3.bits(7..0) = out.bits(28,24,20,16,12, 8,4,0)
    bitplanebuffer[buf_index+0] = plane2
    bitplanebuffer[buf_index+1] = plane3
    [dst]=plane0
   else if (buf_index AND 10h)=0
    [dst]=plane1
   else
    [dst]=bitplanebuffer[buf_index AND 0Fh]
   buf_index=buf_index+1, dst=dst+1, len=len-1
```

#### initialize

```text
  src=directory_base+(directory_index*4)
  mode=[src+0]
  src=[src+3]+[src+2]*100h+[src+1]*10000h  ;big-endian (!)
  buf_index=0
  out=00000000h
  c=0
  top=255
  val.msb=[src], val.lsb=00h, src=src+1, in_count=0
  for i=0 to 15 do pixelorder[i]=i
  for i=0 to 31 do ContextIndex[i]=0, ContextInvert[i]=0
```

#### decompression_core

```text
  decoded=(decoded SHL 1) xor ContextInvert[con]
  evl=ContextIndex[con]
  top = top - EvolutionProb[evl]
  if val.msb > top
    val.msb = val.msb-(top-1)
    top = EvolutionProb[evl]-1
    if top>79 then ContextInvert[con] = ContextInvert[con] XOR 1
    decoded = decoded xor 1
    ContextIndex[con] = EvolutionNextLps[evl]
  else
    if top<=126 then ContextIndex[con] = EvolutionNextMps[evl]
  while(top<=126)
    if in_count=0 then val.lsb=[src], src=src+1, in_count=8
    top = (top SHL 1)+1
    val = (val SHL 1), in_count=in_count-1    ;16bit val.msb/lsb
```

do_pixel_order(a,b,c,shift,decoded)

```text
  m=0, x=a, repeat, exchange(x,pixelorder[m]), m=m+1, until x=a
  for m=0 to (1 shl shift)-1 do realorder[m]=pixelorder[m]
  m=0, x=c, repeat, exchange(x,realorder[m]), m=m+1, until x=c
  m=0, x=b, repeat, exchange(x,realorder[m]), m=m+1, until x=b
  m=0, x=a, repeat, exchange(x,realorder[m]), m=m+1, until x=a
  out = (out SHL shift) + realorder[decoded]
  c = b
```

get_con(a,b,c)

```text
  if (a=b AND b=c) then return=0
  else if (a=b) then return=1
  else if (b=c) then return=2
  else if (a=c) then return=3
  else return=4
```

EvolutionProb[0..52]

```text
  90,37,17, 8, 3, 1,90,63,44,32,23,17,12, 9, 7, 5, 4, 3, 2
  90,72,58,46,38,31,25,21,17,14,11, 9, 8, 7, 5, 4, 4, 3, 2
  2 ,88,77,67,59,52,46,41,37,86,79,71,65,60,55
```

EvolutionNextLps[0..52]

```text
  1 , 6, 8,10,12,15, 7,19,21,22,23,25,26,28,29,31,32,34,35
  20,39,40,42,44,45,46,25,26,26,27,28,29,30,31,33,33,34,35
  36,39,47,48,49,50,51,44,45,47,47,48,49,50,51
```

EvolutionNextMps[0..52]

```text
  1 , 2, 3, 4, 5, 5, 7, 8, 9,10,11,12,13,14,15,16,17,18, 5
  20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38
  5 ,40,41,42,43,44,45,46,24,48,49,50,51,52,43
```

Mode2ContextTable[0..14]  ;only entries 3..14 used (entries 0..2 = dummies)

```text
  0 ,0 ,0 ,15,17,19,21,23,25,25,25,25,25,27,29
```
