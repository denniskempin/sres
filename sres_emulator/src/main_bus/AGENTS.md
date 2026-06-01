# `main_bus` Module Summary

This directory contains the SNES main bus implementation used by the 65816 CPU, plus wrappers for bus-attached devices and the DMA/multiplication hardware.

---

## What is the `main_bus`?

The main bus is the central memory-mapped interconnect between the 65816 CPU and the rest of the system (PPU, APU, WRAM, SRAM, ROM, IO registers).  
**Primary struct:** `MainBusImpl<PpuT, ApuT>` in `mod.rs`.

It implements two key traits:
- `Bus<AddressU24>` (from `common/bus.rs`) — generic peek/read/write/IO operations.
- `MainBus` (from `components/cpu/mod.rs`) — NMI/timer interrupt consumption and `clock_info()`.

---

## Files

| File | Purpose |
|------|---------|
| `mod.rs` | `MainBusImpl`, memory maps (LoRom / HiRom), memory-access speed logic, bus read/write/peek, clock advancement, IRQ/NMI delegation. |
| `devices.rs` | `ManagedBusDeviceU24` trait and three wrappers: `SyncBusDevice`, `BatchedBusDeviceU24`, `AsyncBusDeviceU24`. |
| `dma.rs` | `DmaController` with 8 channels, MDMAEN/HDMAEN register handling, transfer pattern logic, timing calculation. |
| `multiplication.rs` | `MultiplicationUnit` — hardware multiply/divide registers (`$4202-$4206` write, `$4214-$4217` read). |
| `lorom_memory_map.png` | Golden test image for LoROM memory map visualization. |
| `hirom_memory_map.png` | Golden test image for HiROM memory map visualization. |

---

## Key Types and Traits

### Addressing
- **`AddressU24`** (`common/address.rs`): `{ bank: u8, offset: u16 }`.  
  SNES addresses are 24-bit. Used everywhere in the main bus.

### Bus Traits
- **`Bus<AddressT>`** (`common/bus.rs`):
  - `peek_u8(&self, addr) -> Option<u8>`
  - `cycle_read_u8(&mut self, addr) -> u8`
  - `cycle_write_u8(&mut self, addr, value)`
  - `cycle_io(&mut self)`
  - `reset(&mut self)`
  - Also provides `cycle_read_u16`, `cycle_write_u16`, `peek_u16`, `cycle_read_generic`, etc.
- **`BusDeviceU24`** (`common/bus.rs`): Trait for PPU/APU-style peripherals:
  - `peek`, `read`, `write`, `update_clock`, `reset`, plus `const NAME`.
- **`MainBus`** (`components/cpu/mod.rs`): Extends `Bus<AddressU24>` with:
  - `consume_nmi_interrupt(&mut self) -> bool`
  - `consume_timer_interrupt(&mut self) -> bool`
  - `clock_info(&self) -> ClockInfo`

### MainBusImpl Fields
```rust
pub struct MainBusImpl<PpuT: BusDeviceU24, ApuT: BusDeviceU24> {
    pub(crate) ppu: PpuT,
    pub(crate) apu: ApuT,
    clock: Clock,
    wram: Vec<u8>,      // 64 MB backing, mirrored as needed
    sram: Vec<u8>,      // from cartridge
    rom: Vec<u8>,       // 64 MB backing
    clock_speed: u64,
    dma_controller: DmaController,
    multiplication: MultiplicationUnit,
    joy1: u16,
    joy2: u16,
    mapping_mode: MappingMode,
    debug_event_collector: DebugEventCollectorRef<MainBusEvent>,
}
```

---

## Memory Mapping

Mapping is determined by `MappingMode` (`LoRom` or `HiRom`) from the cartridge header.

### `MemoryBlock` enum
```rust
enum MemoryBlock {
    Ram(usize),
    Rom(usize),
    Sram(usize),
    Register,
    Unmapped,
}
```

### LoRom (`lorom_memory_map`)
- **Banks `$00-$3F`:**
  - `$0000-$1FFF` → WRAM (low mirror)
  - `$2000-$7FFF` → Registers
  - `$8000-$FFFF` → ROM (32 KB per bank)
- **Banks `$40-$6F`:**
  - `$0000-$7FFF` → Unmapped
  - `$8000-$FFFF` → ROM
- **Banks `$70-$7D`:**
  - `$0000-$7FFF` → SRAM
  - `$8000-$FFFF` → ROM
- **Banks `$7E-$7F`:** Full 64 KB WRAM
- **Banks `$80-$BF`:** Mirror of `$00-$3F` (with ROM mapped at bank minus `$80`)
- **Banks `$C0-$FF`:** ROM only (bank minus `$80`)

### HiRom (`hirom_memory_map`)
- **Banks `$00-$2F`:**
  - `$0000-$1FFF` → WRAM
  - `$2000-$5FFF` → Registers
  - `$6000-$7FFF` → Unmapped
  - `$8000-$FFFF` → ROM (64 KB per bank)
- **Banks `$30-$3F`:**
  - `$6000-$7FFF` → SRAM
  - Rest same as `$00-$2F`
- **Banks `$40-$7D`:** Unmapped
- **Banks `$7E-$7F`:** Full 64 KB WRAM
- **Banks `$80-$BF`:** Mirror of `$00-$3F` with ROM at bank minus `$80`
- **Banks `$C0-$FF`:** ROM (bank minus `$C0`)

### Memory Access Speed (`memory_access_speed`)
Determines how many master cycles a bus access costs:
- **FAST** = 6 cycles
- **SLOW** = 8 cycles
- **XSLOW** = 12 cycles (accesses in `$4000-$41FF` region)

The speed depends on the bank and offset. Banks `$80+` are currently treated as non-FastROM (TODO comment).

---

## Register Decoding in `MainBusImpl`

| Offset Range | Device / Handler |
|--------------|------------------|
| `$2100-$213F` | PPU |
| `$2140-$217F` | APU |
| `$420B`, `$420C`, `$4300-$43FF` | `DmaController` |
| `$4200`, `$4207-$420A`, `$4210-$4212` | `Clock` (NMI/IRQ/timing) |
| `$4202-$4206` | `MultiplicationUnit` (write) |
| `$4214-$4217` | `MultiplicationUnit` (read) |
| `$4218-$421B` | Joypad 1 / Joypad 2 (16-bit each) |
| `$4016-$4017` | Serial joypad (warn: unimplemented) |

Unmapped reads return `0` and log an error via the debug collector. Unmapped writes log an error.

---

## Clock & Interrupt Handling

The `Clock` component (`components/clock.rs`) is embedded in `MainBusImpl` and handles:
- **Master clock advancement**
- **Scanline / dot counters** (`v`, `h_counter`, `f` frame counter)
- **NMI generation** on vblank start (if `nmi_enable` is set)
- **H/V timer IRQs** (modes: Off, TriggerH, TriggerV, TriggerHV)
- **DRAM refresh stalls** (~536 cycles into each scanline, CPU pauses 40 cycles)
- **Short scanline** on line 240 of odd frames (1360 vs 1364 cycles)

`MainBusImpl` implements `MainBus` by delegating:
- `consume_nmi_interrupt()` → `clock.consume_nmi_interrupt()`
- `consume_timer_interrupt()` → `clock.consume_timer_interrupt()`
- `clock_info()` → `clock.clock_info()`

---

## DMA (`dma.rs`)

### `DmaController`
- 8 channels (`DmaChannel`), each with:
  - `parameters: DmaParameters` (direction, fixed/decrement, transfer pattern)
  - `bus_a_address: AddressU24` (A-bus source/dest)
  - `bus_b_address: AddressU24` (B-bus register address, default `$21FF`)
  - `byte_count: u16` (0 = 65536)

### Trigger
Writing to **`$420B` (MDMAEN)** sets `dma_pending` as a bitmask of channels to run.  
On the next `advance_master_clock` call, if DMA is active, the controller builds a list of `(source, destination)` byte transfers and returns the total duration in master cycles. The bus then performs each transfer with `bus_read` + `bus_write` and advances the clock accordingly.

### Transfer Patterns (`DmaTransferPattern`)
Determines B-bus address offsets per byte: `0`, `0,1`, `0,0`, `0,0,1,1`, `0,1,2,3`, plus undocumented variants.

### HDMA
Writing to **`$420C` (HDMAEN)** logs a warning: "HDMA not implemented."

### Timing
DMA timing includes:
- Start sync overhead (align to 8-cycle boundary)
- DMA overhead (8 cycles)
- 8 cycles per byte transferred
- 8 cycles per active channel overhead
- End sync overhead (align to CPU cycle speed)

---

## Multiplication / Division (`multiplication.rs`)

`MultiplicationUnit` handles hardware math registers:

| Register | Address | Action |
|----------|---------|--------|
| `WRMPYA` | `$4202` | Write multiplicand A |
| `WRMPYB` | `$4203` | Write multiplicand B → **immediately** computes `A*B` into `mul_result` |
| `WRDIVL` | `$4204` | Write dividend low |
| `WRDIVH` | `$4205` | Write dividend high |
| `WRDIVB` | `$4206` | Write divisor → **immediately** computes quotient + remainder |
| `RDDIVL` | `$4214` | Read division result low |
| `RDDIVH` | `$4215` | Read division result high |
| `RDMPYL` | `$4216` | Read multiplication/remainder low |
| `RDMPYH` | `$4217` | Read multiplication/remainder high |

Division by zero returns `0xFFFF` for both quotient and remainder.

---

## Device Wrappers (`devices.rs`)

### `ManagedBusDeviceU24<InnerT: BusDeviceU24>`
A trait for wrappers that control when the inner device is synchronized:
- `inner(&self) -> Self::InnerRef<'_>`
- `inner_mut(&mut self) -> Self::InnerRefMut<'_>`
- `sync(&mut self)`

### Implementations
1. **`SyncBusDevice<DeviceT>`** — Pass-through wrapper. No buffering. `sync()` is a no-op.
2. **`BatchedBusDeviceU24<DeviceT>`** — Buffers writes in a `Vec<BusAction>` (up to 32 KB).  
   - `read()` triggers `flush()`.
   - `update_clock()` only pushes a `Clock` action if >1024 cycles passed since last flush.
   - `sync()` triggers `flush()`.
   - Useful for reducing overhead when the inner device is expensive to update (e.g., PPU).
3. **`AsyncBusDeviceU24<DeviceT>`** — Runs the inner device on a dedicated background thread.  
   - Writes and clock updates are sent via a `sync_channel(1024)`.
   - `read()` triggers `flush()` (waits for lock).
   - `peek()` and `reset()` lock the `Arc<Mutex<DeviceT>>` directly.
   - `sync()` triggers `flush()`.
   - Useful for running the APU on a separate thread.

---

## Patterns and Conventions

### Bus Read/Write Lifecycle
1. `cycle_read_u8` / `cycle_write_u8` (from `Bus` trait) are the CPU-facing entry points.
2. They first set `clock_speed = memory_access_speed(addr)`.
3. They call `advance_master_clock(cycles)`.
4. Inside `advance_master_clock`, pending DMA transfers are executed **before** advancing the clock.
5. Then the clock is advanced, and `ppu.update_clock()` + `apu.update_clock()` are called.
6. Finally, the actual `bus_read` / `bus_write` is performed.

### Debug Events
`MainBusEvent::Read(addr, value)` and `MainBusEvent::Write(addr, value)` are emitted on every bus access via `DebugEventCollectorRef`. Errors (unmapped access, unimplemented registers) are also reported through this collector.

### Address Wrapping
`AddressU24` supports explicit wrapping modes: `WrapPage`, `WrapBank`, `NoWrap`. The DMA controller uses `Wrap::NoWrap` for address increments.

---

## Testing

The module includes image-based regression tests for memory maps:
- `test_lorom_memory_map_image` generates `lorom_memory_map.png`
- `test_hirom_memory_map_image` generates `hirom_memory_map.png`
- `test_hirom_rom_ranges` checks specific ROM mirror addresses.

These tests create gradient-colored images where each pixel color represents a `MemoryBlock` type and index.

---

## Quick Reference for Future Agents

- **Adding a new register:** Add a match arm in `bus_read`, `bus_write`, and `bus_peek` inside `mod.rs`. If it belongs to a subcomponent (DMA, multiplication, clock), delegate there.
- **Changing memory map:** Edit `lorom_memory_map` or `hirom_memory_map`. Update the corresponding `.png` golden file by running the test.
- **Adding DMA patterns:** Extend `DmaTransferPattern` and the `bus_b_pattern` vector generation in `DmaController::pending_transfers`.
- **FastROM support:** Currently marked with `TODO` in `memory_access_speed` and `hirom_memory_map` / `lorom_memory_map`.
- **HDMA:** Not implemented; only `MDMAEN` DMA is functional.
