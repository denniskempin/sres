# SNES Cart SPC7110 Direct Data ROM Access

4810h Data ROM Read from [Base] or [Base+Offs], and increase Base or Offs 481Ah Data ROM Read from [Base+Offset], and optionally set Base=Base+Offs Reportedly, Testing leads to believe that the direct ROM read section starts out as inactive.

One of the ways to activate direct reads is to write a non-zero value to $4813.

No other action need be taken. You can write a non-zero value and immediately write a zero to it and that's OK.  The order of writes to $4811/2/3 don't seem to matter so long as $4813 has been written to once with a non-zero value.  There may be a way to deactivate the direct reads again (maybe a decompression cycle?).

There appears to be another way to activate direct reads that is more complex.

4811h Data ROM Base, bit0-7   (R/W) 4812h Data ROM Base, bit8-15  (R/W) 4813h Data ROM Base, bit16-23 (R/W)

4814h Data ROM Offset, bit0-7   ;\optionally Base=Base+Offs 4815h Data ROM Offset, bit8-15  ;/on writes to both of these registers 4816h Data ROM Step, bit0-7 4817h Data ROM Step, bit8-15

#### 4818h Data ROM Mode

```text
  0   Select Step   (for 4810h) (0=Increase by 1, 1=Increase by "Step" Value)
  1   Enable Offset (for 4810h) (0=Disable/Read Ptr, 1=Enable/Read Ptr+Offset)
  2   Expand Step from 16bit to 24bit           (0=Zero-expand, 1=Sign-expand)
  3   Expand Offset from 8bit?/16bit to 24bit   (0=Zero-expand, 1=Sign-expand)
  4   Apply Step (after 4810h read)    (0=On 24bit Pointer, 1=On 16bit Offset)
  5-6 Special Actions (see below)
  7   Unused (should be zero)
```

Special Actions:

```text
  0=No special actions
  1=After Writing $4814/5 --> 8 bit offset addition using $4814
  2=After Writing $4814/5 --> 16 bit offset addition using $4814/5
  3=After Reading $481A   --> 16 bit offset addition using $4814/5
```

Reportedly,

```text
  4818 write: set command mode,
  4818 read: performs action instead of returning value, unknown purpose
  command mode is loaded to $4818 but only set after writing to both $4814
  and $4815 in any order
  $4811/2/3 may increment on a $4810 read depending on mode byte)
  $4814/$4815 is sometimes incremented on $4810 reads (depending on mode byte)
```

Note: the data rom command mode is activated only after registers $4814 and $4815 have been written to, regardless of the order they were written to

4831h Data ROM Bank for D00000h-DFFFFFh (1MByte, using HiROM mapping) 4832h Data ROM Bank for E00000h-EFFFFFh (1MByte, using HiROM mapping) 4833h Data ROM Bank for F00000h-FFFFFFh (1MByte, using HiROM mapping)

4830h SRAM Chip Enable/Disable (bit7: 0=Disable, 1=Enable) 4834h SRAM Bank Mapping?, workings unknown
