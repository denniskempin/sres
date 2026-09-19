# `sres_emulator/src/main_bus`

`MainBusImpl` is the 65816 memory map: LoRom/HiRom decode, MMIO routing, DMA, and PPU/APU device wrappers.

## Files

| File | Owns |
|------|------|
| `mod.rs` | `MainBusImpl`. LoRom/HiRom decode, `bus_read`/`bus_write`/`bus_peek`, `advance_master_clock`. |
| `devices.rs` | `ManagedBusDeviceU24`. `SyncBusDevice`, `BatchedBusDeviceU24`, `AsyncBusDeviceU24`. |
| `dma.rs` | `DmaController`. `$420B` MDMAEN trigger. |
| `multiplication.rs` | `MultiplicationUnit`. `$4202–$4217`; `$4203`/`$4206` compute immediately. |

## Behaviors & Gotchas

1. `$420B` sets `dma_pending` only. `cycle_write_u8` advances the clock before the write. The next `advance_master_clock` latches `dma_active` via `update_state`; a later call runs `pending_transfers` before `Clock` ticks and PPU/APU `update_clock`.
2. `cycle_read_u8` pays `clock_speed - 6` through `advance_master_clock`, then the last 6 cycles on `Clock` directly — DMA is not sampled in those 6.
3. `BatchedBusDeviceU24` and `AsyncBusDeviceU24` `read()` flush; `peek()` does not (may be stale).
4. Batched and async `update_clock()` enqueue a clock action only when `master_clock` delta is `> 1024`.
5. `$4206` divisor 0 → quotient (`div_result`) and remainder (`mul_result`) both `0xFFFF`.
6. `Bus::reset` rebuilds `Clock` and PPU only. `DmaController` state (`dma_pending`, channel regs) survives.
7. GP-DMA A-bus increment uses `Wrap::NoWrap`. Docs say bank wrap. Do not change without the `dma_*` ROM-outcome tests.

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
- DMA copies through this bus's `bus_read`/`bus_write`.
- Unit tests can construct `MainBusImpl::new` with any `BusDeviceU24`. `Cartridge::with_program` mapping: `components`.

## Gaps

- HDMA: `$420C` (root) and `$43x7` log a warning; no transfers run.
- FastROM and serial `$4016`/`$4017`: see root / src Gaps.

## Tests

- Unit tests in `mod.rs`: `test_lorom_memory_map_image`, `test_hirom_memory_map_image` (goldens `lorom_memory_map.png` / `hirom_memory_map.png` beside the source), `test_hirom_rom_ranges`.
- `cargo nextest run -p sres_emulator --lib -E 'test(main_bus::)'`
