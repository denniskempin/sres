# `sres_emulator/src/main_bus`

`MainBusImpl` is the 65816 memory map: LoRom/HiRom decode, MMIO routing, DMA/HDMA, and PPU/APU device wrappers.

## Files

| File | Owns |
|------|------|
| `mod.rs` | `MainBusImpl`. LoRom/HiRom decode, `bus_read`/`bus_write`/`bus_peek`, `advance_master_clock`. |
| `devices.rs` | `ManagedBusDeviceU24`. `SyncBusDevice`, `BatchedBusDeviceU24`, `AsyncBusDeviceU24`. |
| `dma.rs` | `DmaController`. `$420B` MDMAEN, `$420C` HDMAEN, channel `$43x0–$43xB`/`$43xF`. |
| `hdma.rs` | `hdma_setup` / `hdma_reload` / `hdma_run` on `MainBusImpl`. |
| `multiplication.rs` | `MultiplicationUnit`. `$4202–$4217`; `$4203`/`$4206` compute immediately. |

## Behaviors & Gotchas

1. `$420B` sets `dma_pending` only. `cycle_write_u8` advances the clock before the write. The next `advance_master_clock` latches `dma_active` via `update_state`; a later call runs `pending_transfers` before `Clock` ticks and PPU/APU `update_clock`.
2. HDMA is consumed at the top of `advance_master_clock` (setup then run, then GP-DMA). `consume_*` is unconditional; cycle cost is 0 unless `hdma_enabled != 0` / `hdma_any_active`. Mid-frame `$420C` uses current `A2An`/`NLTRn`/`DASn` until the next V=0 setup.
3. `cycle_read_u8` pays `clock_speed - 6` through `advance_master_clock`, then the last 6 cycles on `Clock` directly — DMA/HDMA is not sampled in those 6.
4. `BatchedBusDeviceU24` and `AsyncBusDeviceU24` `read()` flush; `peek()` does not (may be stale).
5. Batched and async `update_clock()` enqueue a clock action only when `master_clock` delta is `> 1024`.
6. `$4206` divisor 0 → quotient (`div_result`) and remainder (`mul_result`) both `0xFFFF`.
7. GP-DMA A-bus increment uses `Wrap::NoWrap` (docs say bank-wrap); HDMA uses `Wrap::WrapBank`.
8. `hdma_reload` always `bus_read`s one table byte (8 cycles) even when `NLTRn & 0x7F != 0`; that byte is discarded.

## Hardware Map

Ranges → owner. Register bitfields: `docs/index.md`. LoRom/HiRom WRAM/ROM/SRAM: `lorom_memory_map` / `hirom_memory_map` in `mod.rs`.

| Range | Owner |
|-------|-------|
| `$2100–$213F` | PPU |
| `$2140–$217F` | APU |
| `$4200–$43FF` | `Clock`, `MultiplicationUnit`, `DmaController`, joy auto-read |

## Integration

- `Cpu<MainBusImpl<PpuT, ApuT>>` calls `cycle_read_u8` / `cycle_write_u8` and `consume_nmi_interrupt` / `consume_timer_interrupt` / `interrupt_pending`. `SystemImpl` calls `consume_vblank` for frame swap.
- `SystemImpl` wraps PPU and APU in `SyncBusDevice` / `BatchedBusDeviceU24` / `AsyncBusDeviceU24` (which variant: root).
- DMA/HDMA copies through this bus's `bus_read`/`bus_write`.

## Gaps

- HDMA does not interrupt an in-progress GP-DMA; a GP-DMA spanning trigger points runs at most one deferred setup/run afterwards.
- HDMA trigger window is V=0..224 (no 239-line overscan; `Clock` does not see SETINI).
- No 5A22 errata (DMA-then-HDMA crash, INIDISP `BBADn=$00` failure, last-channel one-byte indirect read, `irqLock`).
- Named unimplemented: `WramDataPort`, `WramAddressPort`, `Wrio`, `Rdio`, `Memsel`, `SerialJoypadWrite`, `SerialJoypadRead`, `JoypadAutoReadEnable`. Unknown I/O emits `on_error`. `$4017` writes and `$43xC`–`$43xE` are explicit no-ops.
- ROM writes emit `RomWrite` and still mutate `rom`.
- GP-DMA A-bus increment keeps `Wrap::NoWrap` (gotcha 7). FastROM speed: root.

## Tests

- Unit tests in `mod.rs`: `test_lorom_memory_map_image`, `test_hirom_memory_map_image` (goldens `lorom_memory_map.png` / `hirom_memory_map.png` beside the source), `test_hirom_rom_ranges`.
- HDMA unit tests in `hdma.rs` (`RecordingDevice` mock). Clock latches: `components::clock`.
- `cargo nextest run -p sres_emulator --lib -E 'test(main_bus::)'`
