# SNES Add-On Turbo File - TFII Mode Transmission Protocol

oldest_recv_8191_bytes:

```text
  call oldest_invoke_transfer   ;start transfer
  if invoke_okay then for i=0001h to 1FFFh, oldest_recv_byte(buf[i])
  jmp oldest_reset_turbofile    ;end transfer (always, even if invoke failed)
```

oldest_send_8191_bytes:

```text
  call oldest_invoke_transfer   ;start transfer
  if invoke_okay then for i=0001h to 1FFFh, oldest_send_byte(buf[i])
  jmp oldest_reset_turbofile    ;end transfer (always, even if invoke failed)
```

oldest_invoke_transfer:

```text
  call oldest_detect_and_get_status
  if no_tf_connected then fail/exit
  if data_phase=1 then            ;oops, already invoked
    oldest_detect_and_get_status  ;abort old transfer
    jmp oldest_invoke_transfer    ;retry invoking new transfer
  [004016]=01h                         ;strobe on      ;\
  for i=1 to 15,dummy=[004017],next i  ;issue 15 clks  ; invoke transfer
  [004016]=00h                         ;strobe off     ; (16 clks total)
  dummy=[004017]                       ;issue 1 clk    ;/
  call oldest_detect_and_get_status                    ;\want flag set now
  if data_phase=0 then fail/exit                       ;/
  for i=1 to 7                                         ;\skip remaining 7 bits
    dummy=[004017]  ;<-- required?     ;issue clk      ; of unused byte 0000h
    [004016]=01h                       ;strobe on      ; (the first bit was
    [004016]=00h                       ;strobe off     ; skipped by STROBE in
  next i                                               ;/detect_and_get_status)
```

After above, the hardware byte-address is 0001h (ie. unlike as in NES version, the unused byte at address 0000h is already skipped).

oldest_detect_and_get_status:

```text
  [004016]=01h    ;strobe on
  [004016]=00h    ;strobe off
  for i=23 to 0   ;get ID/status (MSB first)
    temp=[004017]  ;issue clk & get data
    stat.bit(i)=temp.bit0
  next i
  if stat.bit(11..8)<>0Eh then no_tf_connected=1  ;major 4bit id
  if stat.bit(7..0)<>0FFh then no_tf_connected=1  ;minor 8bit id
  if stat.bit(12)=1 then data_phase=1 else data_phase=0
```

oldest_reset_turbofile:

```text
  [004016]=01h    ;strobe on
  dummy=[004017]  ;issue clk
  [004016]=00h    ;strobe off
  dummy=[004017]  ;issue clk
  [004016]=00h    ;strobe off (again?)
  jmp oldest_detect_and_get_status
```

oldest_recv_byte(data):

```text
  for i=0 to 7  ;transfer data byte (LSB first)
    temp=[004017]               ;issue clk (required?), and get bit from joy4
    data.bit(i)=temp.bit(1)     ;extract received data bit
    [004016]=01h                ;strobe on
    [004201]=80h*data.bit(i)    ;write SAME/UNCHANGED bit to hardware
    [004016]=00h                ;strobe off (WRITE CLOCK)
  next i
```

oldest_send_byte(data):

```text
  for i=0 to 7  ;transfer data byte (LSB first)
    dummy=[004017]              ;issue clk (really required for writing?)
    [004016]=01h                ;strobe on
    [004201]=80h*data.bit(i)    ;write NEW bit to hardware
    [004016]=00h                ;strobe off (WRITE CLOCK)
  next i
```
