# `sres_emulator/src/apu` — Audio Processing Unit (APU)

## Overview

This module implements the SNES Audio Processing Unit (APU), which is composed of two chips: the **SPC700** (the audio CPU) and the **S-DSP** (the digital signal processor for sound generation). In this codebase, the APU module acts as the **integration and orchestration layer** that connects these two components, manages the audio subsystem's bus, handles SNES-to-APU communication, generates audio samples based on the master system clock, and provides a buffer for audio output.

The SPC700 and S-DSP are considered standalone `components` (found in `src/components/spc700` and `src/components/s_dsp`) with their own internal logic. The `apu` directory's role is to wire them together via the `ApuBus` and drive them according to the emulator's master clock.

## Module Structure

| File | Role |
|------|------|
| **`mod.rs`** | Core `Apu` struct, clocking logic, audio buffer management, and implementation of `BusDeviceU24` for SNES main CPU interaction. |
| **`apu_bus.rs`** | The `ApuBus` struct implements the `Spc700Bus` trait and `Bus<AddressU16>`. It exposes RAM, IPL ROM, timers, DSP registers, and SNES communication ports (`channel_in`/`channel_out`) to the SPC700. |
| **`timers.rs`** | Implementation of the three APU timers (Timer 0, 1, 2), including their multi-stage frequency dividers and read/reset behavior. |
| **`test.rs`** | Integration tests that validate the SPC700 boot ROM execution and APUIO communication protocol. |

## Key Types & Responsibilities

### `Apu` (`mod.rs`)
The top-level struct that the rest of the emulator interacts with.

```rust
pub struct Apu {
    pub spc700: Spc700<ApuBus>,
    sample_buffer: AudioBuffer,
    last_sample_cycle: u64,
    current_master_clock: u64,
}
```

- **`spc700`**: The SPC700 CPU instance, parameterised over `ApuBus`.
- **Clocking**: The `Apu` tracks `current_master_clock` but does **not** advance the SPC700 on every single clock tick. Instead, it uses a **lazy catch-up** strategy to efficiently synchronize the two domains.
- **Audio Buffering**: Samples are generated and pushed into an `AudioBuffer` (capacity 1024, max 32000). The frontend pulls audio via `swap_audio_buffer`.

### `ApuBus` (`apu_bus.rs`)
The bridge between the SPC700 CPU and its memory-mapped peripherals.

```rust
pub struct ApuBus {
    pub ram: [u8; 0x10000],
    pub channel_in: [u8; 4],
    pub channel_out: [u8; 4],
    pub timers: ApuTimers,
    pub dsp: SDsp,
    pub control: ApuControlRegister,
    // ...
}
```

- **`ram`**: 64KB of SPC700 RAM.
- **`channel_in` / `channel_out`**: The four APUIO ports (`$F4-$F7` on the SPC700 side, mapped to `$2140-$2143` on the SNES main CPU side). These are the primary communication channels between the 5A22 main CPU and the SPC700.
- **`dsp`**: The `SDsp` instance, accessed via registers `$F2` (register select) and `$F3` (data read/write).
- **`timers`**: The three APU timers, mapped at `$FA-$FC` (targets) and `$FD-$FF` (outputs).
- **`control` (`$F1`)**: The control register that enables/disables timers and optionally clears APUIO ports.
- **IPL ROM**: A 64-byte boot ROM is embedded in `apu_bus.rs` and mapped at `$FFC0-$FFFF` when enabled via `control.ipl_rom_enabled()`.

### `ApuTimers` / `ApuTimer` (`timers.rs`)
There are three independent timers.

- **Timer 0 & 1**: Base divisor of **128 SPC cycles** (8kHz base).
- **Timer 2**: Base divisor of **16 SPC cycles** (64kHz base).
- **Architecture**: Each timer is a 3-stage divider:
  1. **Base counter**: Divides SPC700 clock by the base divisor (128 or 16).
  2. **Interval counter**: User-configurable 8-bit target (`$FA-$FC`). 0 means 256.
  3. **Output counter**: 4-bit counter that increments when the interval counter hits the target. Reading the output (`$FD-$FF`) returns the lower 4 bits and resets it to 0.
- **Enable behaviour**: A 0-to-1 transition in the enable flag resets the interval and output counters to 0. The base counter, however, runs continuously regardless of enable state.

### `AudioBuffer` (`mod.rs`)
A simple typed wrapper `Vec<i16>` used to pass generated audio samples to the frontend. Supports `swap` to avoid copying.

## Clocking Strategy

The SNES master clock runs at ~21.47 MHz, while the SPC700 runs at ~2.048 MHz (which is exactly `32000 * 64`).

Key timing constants from `mod.rs`:
```rust
pub const APU_SAMPLE_RATE: u32 = 32000;
pub const MASTER_CLOCK_FREQUENCY: u64 = 21477272;
pub const CYCLES_PER_SAMPLE: u64 = MASTER_CLOCK_FREQUENCY / APU_SAMPLE_RATE as u64; // ~671
pub const SPC_CLOCK_FREQUENCY: u64 = APU_SAMPLE_RATE as u64 * 64; // 2,048,000
```

### Lazy Catch-up Synchronization
The `Apu` does **not** advance the SPC700 on every master clock cycle. Instead, it catches up the SPC700 in three specific scenarios:
1. **APUIO Write**: Before writing a new value to `channel_in` (from the SNES side), the SPC700 is advanced to the current master clock. This ensures the SPC700 sees the old value for all prior cycles.
2. **APUIO Read**: Before reading from `channel_out`, the SPC700 is advanced so the read reflects the most recent state.
3. **Sample Generation**: Every `CYCLES_PER_SAMPLE`, the SPC700 is advanced to that sample boundary before `generate_sample()` is called.

This approach is critical for correctness: if the SPC700 were advanced on every bus cycle, it could process an APUIO write before the `write_apuio` method has a chance to deposit the new value.

## Bus Interaction Patterns

### SNES Main CPU -> APU (`BusDeviceU24`)
The `Apu` implements `BusDeviceU24`, exposing the four APUIO registers at `0x2140..=0x2143`.
- **`read()`**: Calls `read_apuio`, which first calls `catch_up_to_master_clock`.
- **`write()`**: Calls `write_apuio`, which first calls `catch_up_to_master_clock`, then writes to `channel_in` via `ApuBus::write_channel_in`.
- **`update_clock()`**: Updates `current_master_clock` and triggers sample generation if enough cycles have passed.

### SPC700 -> Peripherals (`Bus<AddressU16>`)
`ApuBus` implements the `Bus<AddressU16>` trait for the SPC700's 16-bit address space.
- **Memory Map**:
  - `$0000-$FFFF`: RAM (64KB).
  - `$F1`: Control register (timers enable, APUIO clear, IPL ROM enable).
  - `$F2`: DSP register select.
  - `$F3`: DSP register read/write.
  - `$F4-$F7`: APUIO `channel_out` (write) and `channel_in` (read).
  - `$FA-$FC`: Timer targets (write-only).
  - `$FD-$FF`: Timer outputs (read-only, clears on read).
  - `$FFC0-$FFFF`: IPL Boot ROM (if enabled).

### Synchronization Delay (`write_channel_in`)
There is a mechanism to model hardware timing jitter when the SNES writes to APUIO. `ApuBus::write_channel_in` checks if the write occurs within the first cycle of the current SPC instruction. If it does, the value is applied immediately; otherwise, it is placed into a pending buffer (`channel_in_pending`) that gets flushed at the start of the next SPC bus cycle.

## Audio Output Flow

1. The emulator's main loop calls `update_clock` on the `Apu` as the master clock advances.
2. When `master_clock - last_sample_cycle >= CYCLES_PER_SAMPLE`, the `Apu`:
   a. Advances the SPC700 to the sample boundary.
   b. Calls `generate_sample()`, which delegates to `self.spc700.bus.dsp.generate_sample(memory)`.
   c. Pushes the resulting `i16` sample into the `AudioBuffer`.
3. The frontend (e.g., `sres_egui`) periodically calls `swap_audio_buffer` to take ownership of the accumulated samples.

## Debugging

`ApuDebug` provides access to the internal DSP and RAM states for the debugger/frontend:
```rust
impl<'a> ApuDebug<'a> {
    pub fn dsp(&'a self) -> SDspDebug<'a> { ... }
    pub fn ram(&self) -> &[u8] { ... }
}
```

## Important Patterns & Conventions

- **No separate S-DSP file in `apu/`**: The S-DSP is a standalone component in `src/components/s_dsp`. The `ApuBus` merely exposes it to the SPC700 via the `$F2`/`$F3` register interface.
- **SPC700 generic over Bus**: The `Spc700<BusT>` struct is defined in `src/components/spc700/mod.rs`. `ApuBus` is one implementation of the `Spc700Bus` trait.
- **Trace-based testing**: `test.rs` uses a recorded trace from the Mesen emulator (`INIT_TRACE`) to assert cycle-accurate behavior of the SPC700 boot ROM.
- **Error handling on overflow**: If the `AudioBuffer` exceeds `MAX_AUDIO_BUFFER_SIZE` (32,000 samples), it is cleared and an error is logged, preventing unbounded memory growth.

## References

- SPC700 Boot ROM source: [gilligan/snesdev](https://github.com/gilligan/snesdev/blob/master/docs/spc700.txt)
- S-SMP Control Register: [snes.nesdev.org/wiki/S-SMP](https://snes.nesdev.org/wiki/S-SMP#CONTROL_-_Control_register_($F1,_write-only))
