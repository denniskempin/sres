# `main_bus`

SNES main system bus (65816 CPU memory map, DMA, device wrappers).

---

## Files

| File | Purpose |
|------|---------|
| `mod.rs` | `MainBusImpl`. Memory maps (LoRom/HiRom), bus read/write/peek, register decode, IRQ/NMI delegation to `Clock`. |
| `devices.rs` | `ManagedBusDeviceU24` trait. Wrappers: `SyncBusDevice`, `BatchedBusDeviceU24`, `AsyncBusDeviceU24`. |
| `dma.rs` | `DmaController`. 8 channels, MDMAEN trigger, transfer patterns, timing. HDMA is NOT implemented. |
| `multiplication.rs` | `MultiplicationUnit`. Hardware multiply/divide via `$4202-$4217`. |

---

## Core Types

- **`MainBusImpl<PpuT, ApuT>`** — Central bus. Implements `Bus<AddressU24>` and `MainBus`.
- **`DmaController`** — 8-channel DMA. Triggered by write to `$420B` (MDMAEN). Runs before clock advance.
- **`MultiplicationUnit`** — Math coprocessor. Writes to `$4203`/`$4206` trigger immediate compute.
- **`MemoryBlock`** — Enum for RAM/RAM/SRAM/Register/Unmapped regions.

---

## Memory Mapping

Determined by `MappingMode` (LoRom or HiRom from cartridge header).

- **LoRom**: Banks `$00-$3F` and `$80-$BF`. ROM at `$8000+`. WRAM at `$0000-$1FFF` / `$7E-$7F`. SRAM at `$70-$7D:$0000-$7FFF`.
- **HiRom**: Banks `$00-$3F` and `$80-$BF`. ROM at `$8000+` (64KB banks). SRAM at `$30-$3F:$6000-$7FFF`.

Access speeds: FAST (6 cycles), SLOW (8 cycles), XSLOW (12 cycles for `$4000-$41FF`).

---

## Register Decoding

| Range | Handler |
|-------|---------|
| `$2100-$213F` | PPU |
| `$2140-$217F` | APU |
| `$420B`, `$420C`, `$4300-$43FF` | `DmaController` |
| `$4200`, `$4207-$420A`, `$4210-$4212` | `Clock` (interrupts) |
| `$4202-$4206` | `MultiplicationUnit` (write) |
| `$4214-$4217` | `MultiplicationUnit` (read) |
| `$4218-$421B` | Joypads |

Unmapped access returns `0` and logs an error.

---

## Device Wrappers

- **`SyncBusDevice`** — Pass-through, no buffering.
- **`BatchedBusDeviceU24`** — Buffers writes. Flushes on `read()` or `sync()`. `update_clock()` throttled to >1024 cycles.
- **`AsyncBusDeviceU24`** — Inner device runs on background thread. Communications via `sync_channel(1024)`.

---

## Non-Obvious Behaviors & Gotchas

1. **DMA runs before clock advance.** When a CPU write triggers `$420B`, `dma_pending` is set. On the next `advance_master_clock`, DMA transfers execute **before** `Clock` ticks and PPU/APU update.
2. **Read triggers flush.** `BatchedBusDeviceU24::read()` and `AsyncBusDeviceU24::read()` force synchronization. Do not rely on peek being cheap if the device is batched/async.
3. **HDMA is not implemented.** Writing `$420C` logs a warning.
4. **FastROM is not implemented.** Banks `$80+` memory access speed logic has a TODO.
5. **Division by zero** in `MultiplicationUnit` returns `0xFFFF` for both quotient and remainder.
6. **Clock delegation.** `MainBusImpl` does not handle NMI/IRQ timers itself. It delegates to the embedded `Clock` component (`components/clock.rs`).

---

## Testing

- `test_lorom_memory_map_image` / `test_hirom_memory_map_image` generate golden `.png` files representing memory layout.
- `test_hirom_rom_ranges` checks ROM mirror addresses.
