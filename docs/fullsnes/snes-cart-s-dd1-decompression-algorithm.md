# SNES Cart S-DD1 Decompression Algorithm

#### decompress_init(src)

```text
  input=[src], src=src+1
  if (input AND C0h)=00h then num_planes = 2
  if (input AND C0h)=40h then num_planes = 8
  if (input AND C0h)=80h then num_planes = 4
  if (input AND C0h)=C0h then num_planes = 0
  if (input AND 30h)=00h then high_context_bits=01c0h, low_context_bits=0001h
  if (input AND 30h)=10h then high_context_bits=0180h, low_context_bits=0001h
  if (input AND 30h)=20h then high_context_bits=00c0h, low_context_bits=0001h
  if (input AND 30h)=30h then high_context_bits=0180h, low_context_bits=0003h
  input=(input SHL 11) OR ([src+1] SHL 3), src=src+1, valid_bits=5
  for i=0 to 7 do bit_ctr[i]=00h, prev_bits[i]=0000h
  for i=0 to 31 do context_states[i]=00h, context_MPS[i]=00h
  plane=0, yloc=0, raw=0
```

decompress_byte(src,dst)

```text
  if num_planes=0
    for plane=0 to 7 do GetBit(plane)
    [dst]=raw, dst=dst+1
  else if (plane AND 1)=0
    for i=0 to 7 do GetBit(plane+0), GetBit(plane+1)
    [dst]=prev_bits[plane] AND FFh, dst=dst+1, plane=plane+1
  else
    [dst]=prev_bits[plane] AND FFh, dst=dst+1, plane=plane-1
    yloc=yloc+1, if yloc=8 then yloc=0, plane = (plane+2) AND (num_planes-1)
```

#### GetBit(plane)

```text
  context = (plane AND 1) SHL 4
  context = context OR ((prev_bits[plane] AND high_context_bits) SHR 5)
  context = context OR (prev_bits[plane] AND low_context_bits)
  pbit=ProbGetBit(context)
  prev_bits[plane] = (prev_bits[plane] SHL 1) + pbit
  if num_planes=0 then raw = (raw SHR 1)+(pbit SHL 7)
```

#### ProbGetBit(context)

```text
  state=context_states[context]
  code_size=EvolutionCodeSize[state]
  if (bit_ctr[code_size] AND 7Fh)=0 then
    bit_ctr[code_size]=GetCodeword(code_size)
  pbit=context_MPS[context]
  bit_ctr[code_size] = bit_ctr[code_size]-1
  if bit_ctr[code_size]=00h    ;"GolombGetBit"
    context_states[context]=EvolutionLpsNext[state]
    pbit=pbit XOR 1
    if state<2 then context_MPS[context]=pbit
  else if bit_ctr[code_size]=80h
    context_states[context]=EvolutionMpsNext[state]
  return pbit
```

#### GetCodeword(code_size)

```text
  if valid_bits=0 then input=input OR [src], src=src+1, valid_bits=8
  input=input SHL 1, valid_bits=valid_bits-1
  if (input AND 8000h)=0 return 80h+(1 SHL code_size)
  tmp=((input SHR 8) AND 7Fh) OR (7Fh SHR code_size)
  input=input SHL code_size, valid_bits=valid_bits-code_size
  if valid_bits<0 then
    input=input OR (([src] SHL (-valid_bits))
    src=src+1, valid_bits=valid_bits+8
  return RunTable[tmp]
```

EvolutionCodeSize[0..32]

```text
  0 , 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3
  4 , 4, 5, 5, 6, 6, 7, 7, 0, 1, 2, 3, 4, 5, 6, 7
```

EvolutionMpsNext[0..32]

```text
  25, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16,17
  18,19,20,21,22,23,24,24,26,27,28,29,30,31,32,24
```

EvolutionLpsNext[0..32]

```text
  25, 1, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15
  16,17,18,19,20,21,22,23, 1, 2, 4, 8,12,16,18,22
```

RunTable[0..127]

```text
  128, 64, 96, 32, 112, 48, 80, 16, 120, 56, 88, 24, 104, 40, 72, 8
  124, 60, 92, 28, 108, 44, 76, 12, 116, 52, 84, 20, 100, 36, 68, 4
  126, 62, 94, 30, 110, 46, 78, 14, 118, 54, 86, 22, 102, 38, 70, 6
  122, 58, 90, 26, 106, 42, 74, 10, 114, 50, 82, 18,  98, 34, 66, 2
  127, 63, 95, 31, 111, 47, 79, 15, 119, 55, 87, 23, 103, 39, 71, 7
  123, 59, 91, 27, 107, 43, 75, 11, 115, 51, 83, 19,  99, 35, 67, 3
  125, 61, 93, 29, 109, 45, 77, 13, 117, 53, 85, 21, 101, 37, 69, 5
  121, 57, 89, 25, 105, 41, 73,  9, 113, 49, 81, 17,  97, 33, 65, 1
```
