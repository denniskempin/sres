# SNES Cart Satellaview BIOS Function Summary

BIOS functions must be called with BIOS ROM enabled in Bank 80h-9Fh (in some cases it may be required also in Bank 00h-1Fh), and with DB=80h.

Incoming/outgoing Parameters are passed in whatever CPU registers and/or whatever WRAM locations. WRAM is somewhat reserved for the BIOS (if a FLASH file changes WRAM, then it should preserve a backup-copy in PSRAM, and restore WRAM before calling BIOS functions) (WRAM locations that MAY be destroyed are 7E00C0h..7E00FFh and 7E1500h..7E15FFh, and, to some level: 7F0000h..7FFFFFh, which is used only as temporary storage).

#### Hooks (usually containing RETF opcodes)

```text
  105974 boot_hook (changed by nocash fast-boot patch)
  105978 nmi_hook
  10597C irq_vector
  105980 download_start_hook --> see 9B8000
  105984 file_start_hook --> see 958000
  105988 whatever_hook --> see 99xxxx
```

#### SRAM Vectors

```text
  10598C detect_receiver
  105990 port_2194_clr_bit0
  105994 port_2196_test_bit1
```

#### Copy Data Queue to RAM Buffer

```text
  105998 set_port_218B_and_218C_to_01h
  10599C set_port_218C_to_00h
  1059A0 read_data_queue
```

#### Port 2199h (serial port 2) (maybe satellite audio related)

```text
  1059A4 init_port_2199_registers
  1059A8 send_array_to_port_2199  ;BUGGED?
  1059AC recv_3x8bit_from_port_2199
  1059B0 send_16bit_to_port_2199
  1059B4 recv_8bit_from_port_2199
```

#### Port 2198h (serial port 1) (unused/expansion or so)

```text
  1059B8 port_2198_send_cmd_recv_multiple_words
  1059BC port_2198_send_cmd_recv_single_word
  1059C0 port_2198_send_cmd_send_verify_multiple_words
  1059C4 port_2198_send_cmd_send_verify_single_word
  1059C8 port_2198_send_cmd_send_single_word
  1059CC port_2198_send_10h_send_verify_single_word
  1059D0 port_2198_send_cmd_verify_FFFFh
  1059D4 port_2198_send_20h_verify_FFFFh
  1059D8 recv_2198_skip_x BUGGED!
  1059DC recv_2198_want_x
  1059E0 send_30h_to_port_2198
  1059E4 send_00h_to_port_2198
  1059E8 send_8bit_to_port_2198
  1059EC wait_port_2198_bit7
```

#### Forward Data Queue from RAM to Target

```text
  1059F0 forward_data_queue_to_target
  1059F4 forward_queue_to_wram
  1059F8 forward_queue_to_psram
  1059FC forward_queue_to_entire_flash
  105A00 forward_queue_to_entire_flash_type1
  105A04 forward_queue_to_entire_flash_type2
  105A08 forward_queue_to_entire_flash_type3
  105A0C forward_queue_to_entire_flash_type4
  105A10 forward_queue_to_flash_sectors
  105A14 forward_queue_to_flash_sectors_type1
  105A18 forward_queue_to_flash_sectors_type2
  105A1C forward_queue_to_flash_sectors_type3
  105A20 forward_queue_to_flash_sectors_type4
  105A24 forward_queue_to_channel_map  ;with 5-byte frame-header
  105A28 forward_queue_to_town_status
```

#### FLASH Files

```text
  105A2C scan_flash_directory
  105A30 allocate_flash_blocks
  105A34 .. prepare exec / map file or so
  105A38 verify_file_checksum
  105A3C get_flash_file_header_a
  105A40 delete_flash_file_a
  105A44 get_flash_file_header_5A
  105A48 copy_file_header
  105A4C search_test_file_header, out:[57]
  105A50 test_gamecode_field
  105A54 copy_file_to_psram
  105A58 get_file_size
  105A5C decrease_limited_starts
```

#### Memory Mapping

```text
  105A60 map_flash_as_data_file  (for non-executable data-files?)
  105A64 map_psram_as_data_file  (for non-executable data-files?)
  105A68 .. mapping and copy 512Kbytes ?
  105A6C map_flash_for_rw_access
  105A70 map_flash_for_no_rw_access
  105A74 map_flash_for_reloc_to_psram
  105A78 .. mapping (unused?)
  105A7C map_flash_as_lorom_or_hirom
  105A80 execute_game_code
  105A84 .. map_psram_for_streaming ???
  105A88 map_psram_as_lorom_or_hirom
  105A8C .. copy 256Kbytes...
```

#### FLASH Memory

```text
  105A90 flash_abort
  105A94 flash_abort_type1
  105A98 flash_abort_type2
  105A9C flash_abort_type3
  105AA0 flash_abort_type4
  105AA4 flash_erase_entire
  105AA8 flash_erase_entire_type1
  105AAC flash_erase_entire_type2
  105AB0 flash_erase_entire_type4 ;4!
  105AB4 flash_erase_entire_type3
  105AB8 flash_test_status   ERASE-PROGRESS
  105ABC flash_test_status_type1
  105AC0 flash_test_status_type2
  105AC4 flash_test_status_type4 ;4!
  105AC8 flash_test_status_type3
  105ACC flash_erase_first_sector
  105AD0 flash_erase_first_sector_type1
  105AD4 flash_erase_first_sector_type2
  105AD8 flash_erase_first_sector_type3
  105ADC flash_erase_first_sector_type4
  105AE0 flash_erase_next_sector
  105AE4 flash_erase_next_sector_type1
  105AE8 flash_erase_next_sector_type2
  105AEC flash_erase_next_sector_type3
  105AF0 flash_erase_next_sector_type4
  105AF4 flash_write_byte
  105AF8 flash_write_byte_type1
  105AFC flash_write_byte_type2
  105B00 flash_write_byte_type3
  105B04 flash_write_byte_type4
  105B08 flash_get_free_memory_size
  105B0C flash_get_and_interprete_id
  105B10 flash_get_id
  105B14 flash_init_chip
  105B18 flash_init_chip_type1
  105B1C flash_init_chip_type2
  105B20 flash_init_chip_type3
  105B24 flash_init_chip_type4
```

#### Satellite Directory

```text
  105B28 apply_satellite_directory
  105B2C directory_find_8bit_folder_id
  105B30 directory_find_32bit_file_channel
  105B34 test_if_file_available
  105B38 download_file_and_include_files
  105B3C directory_find_32bit_bugged
```

Misc...

```text
  105B40 .. initialize stuff on reset
  105B44 download_nmi_handling (with download_callback etc.)
  105B48 download_nmi_do_timeout_counting
  105B4C nmi_do_led_blinking
  105B50 mark_flash_busy
  105B54 mark_flash_ready
  105B58 set_port_2197_bit7
  105B5C clr_port_2197_bit7
  105B60 detect_receiver_and_port_2196_test_bit1
  105B64 init_flash_chip_with_err_29h
  105B68 init_flash_chip_with_err_2Ah
  105B6C detect_receiver_and_do_downloads
  105B70 do_download_function
  105B74 retry_previous_download
  105B78 set_target_id_and_search_channel_map
  105B7C apply_target_for_download
  105B80 clear_queue_and_set_13D1_13D2
  105B84 flush_old_download   ;[218C]=0, clear some bytes
```

#### Invoke Download Main Functions

```text
  105B88 download_to_whatever (BUGGED)
  105B8C download_channel_map
  105B90 download_welcome_message
  105B94 download_snes_patch
  105B98 download_town_status
  105B9C download_town_directory
  105BA0 download_to_memory
```

#### Download sub functions

```text
  105BA4 add_download_array
  105BA8 wait_if_too_many_downloads
  105BAC do_download_callback
  105BB0 dload_channel_map_callback_1
  105BB4 dload_channel_map_callback_2
  105BB8 dload_welcome_message_callback
  105BBC dload_snes_patch_callback
  105BC0 dload_town_status_callback_1
  105BC4 dload_town_status_callback_2
  105BC8 dload_town_directory_callback_1
  105BCC dload_town_directory_callback_2
  105BD0 .. flash status
  105BD4 dload_to_mem_wram_callback1          ;\
  105BD8 dload_to_mem_wram_callback2          ;
  105BDC dload_to_mem_psram_callback1         ;
  105BE0 dload_to_mem_psram_callback2         ; dload_to_memory_callbacks
  105BE4 dload_to_mem_entire_flash_callback1  ;
  105BE8 dload_to_mem_entire_flash_callback2  ;
  105BEC dload_to_mem_free_flash_callback1    ;
  105BF0 dload_to_mem_free_flash_callback2    ;/
  105BF4 dload_to_mem_entire_flash_callback_final
  105BF8 dload_to_mem_free_flash_callback_final
  105BFC reset_interpreter_and_run_thread_958000h
  105C00 verify_channel_map_header
  105C04 raise_error_count_check_retry_limit
  105C08 search_channel_map
  105C0C post_download_error_handling
  105C10 .. erase satellite info ?
```

#### APU Functions

```text
  105C14 apu_flush_and_clear_queues
  105C18 apu_flush_raw
  105C1C apu_message
  105C20 apu_nmi_handling
  105C24 apu_upload_extra_thread
  105C28 apu_upload_curr_thread
  105C2C apu_enable_effects_music_b
  105C30 apu_enable_effects_music_a
  105C34 apu_mute_effects_and_music
  105C38 apu_enable_effects_only
```

#### Reset

```text
  105C3C reboot_bios (this one works even when BIOS=disabled or WRAM=destroyed)
```

#### Further Stuff

```text
  105C96 Unused 7 bytes (used for nocash fast-boot patch)
  105C9D Unused 3 bytes (zero)
  105CA0 Token Vectors (16bit offsets in bank 81h)
```

#### BIOS Tables

```text
  105xxx Tables in SRAM (see above)
  808000 Unsorted ptrs to BIOS Functions, Token-Extensions, and OBJ-Tile-Data
  9FFFF0 Pointers to source data for APU uploads
```

#### Additional BIOS Functions (without SRAM-Table vectors)

These are some hardcoded BIOS addresses (used by some FLASH programs).

```text
  808C2A Invoke_dma_via_ax_ptr
  8091B6 Create_machine_code_thread
  809238 Pause_machine_code_thread
  80938F Do nothing (retf) (used as dummy callback address)
  80ABC8 ...whatever
  80AC01 ...whatever
  80B381 Upload_gui_border_shape_to_vram
  80B51B Clear_text_window_content
  80B91E Fill_400h_words_at_7E76000_by_0080h   ;clear whole BG3 map in WRAM
  80EB99 Injump_to_APU_Town_Status_handling (requires incoming pushed stuff)
  81C210 Reset_interpreter
  81C29A Set_interpreter_enable_flag
  81C2B0 Create_interpreter_thread
  81C80E Deallocate_all_obj_tiles_and_obj_palettes
```

Note: Some of the above functions are also listed in the table at 808000h.

#### Returning to BIOS

If an executable file wants to return control to BIOS, it must first reset the APU (if it has uploaded code to it), and then it can do one the following:

Perform a warmboot (the BIOS intro is skipped, but the Welcome message is re-displayed, and the player is moved back to the Home building):

```text
  jmp 105C3Ch   ;srv_reboot_bios (simple, but quite annoying)
```

Or return to the BIOS NMI handler (from within which the executable was started) this is done by many games (player returns to the most recently entered building, this is more elegant from the user's view, though requires a messy hack from the programmer's view):

```text
  call restore_wram             ;-restore WRAM (as how it was owned by BIOS)
  jmp  far (($+4) AND 00FFFFh)  ;-PB=00h (so below can map BIOS to bank 80h)
  mov  a,80h   ;\                               ;\
  push a ;=80h ;                                ; set DB=80h, and
  pop  db      ;/                               ; enable BIOS in bank 80h-9Fh
  mov  [085000h],a ;map BIOS to bank 80h-9Fh    ; (though not yet in 00h-1Fh)
  mov  [0E5000h],a ;apply                       ;/
  call far 99D732h  ;super-slow ;out: M=0       ;-upload [9FFFF0h] to APU
```

.assume p=10h  ;(above set M=0, and keeps X=unchanged)

```text
  call far 81C210h                              ;-Reset Token Interpreter
  call far 81C29Ah                              ;-Enable/Unpause Interpreter
  call far 80937Fh ;set NMI callback to RETF prevent FILE to be executed AGAIN)
  mov  x,[13B2h]       ;BIOS online flag (8bit)  ;\skip below if offline
  jz   @@skip                                    ;/
  push pb       ;\retadr for below               ;\
  push @@back-1 ;/                               ;
  push db       ;-incoming pushed DB for below   ;
  push 7E00h    ;\                               ; init apu effects/music
  pop  db       ; incoming current DB for below  ; (according to APU bits
  pop  db ;=7Eh ;/                               ; in town status packet)
  jmp  far 80EB99h ;--> injump to 105BC0h        ;
```

@@back:                                         ;

```text
  .assume p=20h  ;(above set's it so)            ;
  ;(if executed, ie. not when @@skip'ed)         ;/
```

@@skip:

```text
  clr  p,30h // .assume p=00h  ;below call 81C2B0h requires M=0, X=0
  mov  [0CDEh],0000h                            ;-mark fade-in/out non-busy
  mov  a,0099h     ;\                           ;\
  mov  [0BEh],a    ; 99D69A ;BIOS - enter town  ; create_interpreter_thread
  mov  a,0D69Ah    ;/                           ; (99D69Ah = enter town)
  call far 81C2B0h                              ;/
  set  p,20h // .assume p=20h
  mov  a,81h       ;\enable NMI and joypad (unstable: BIOS isn't yet mapped!)
  mov  [4200h],a   ;/caution: ensure that no NMI occurs in next few clk cycles
  mov  a,80h                                    ;\enable BIOS also in bank 0,
  mov  [075000h],a ;map BIOS to bank 00h-1Fh    ; and return to BIOS NMI
```

#### handler

```text
  jmp  far 80BC27h ;apply [0E5000h]=a, and retf ;/
```
