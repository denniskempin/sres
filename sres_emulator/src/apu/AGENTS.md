# `sres_emulator/src/apu`

SPC700 + S-DSP orchestration: `Apu` drives `Spc700<ApuBus>` from the master clock and fills `AudioBuffer`.

## Files

| File | Owns |
|------|------|
| `mod.rs` | `Apu`; `catch_up_and_promote_channel_out` / `update_clock`; `AudioBuffer`; `ApuDebug` |
| `apu_bus.rs` | `ApuBus` (`Spc700Bus`); `$F4–$F7` CPUIO; `promote_channel_out`; `ApuControlRegister`; IPL ROM |
| `timers.rs` | `ApuTimers` / `ApuTimer`; three timers; 3-stage divider |
| `test.rs` | Mesen boot-ROM trace (`INIT_TRACE`) |

## Behaviors & Gotchas

| Purpose | Master | SPC | Where |
|---------|--------|-----|-------|
| Sample boundaries | 21477272 Hz | 32000 × 64 | `mod.rs` (`CYCLES_PER_SAMPLE`) |
| Catch-up / CPUIO | 21477270 Hz | 32040 × 64 | `spc700/mod.rs` (`catch_up_to_master_clock`) |

Catch-up matches Mesen2 `SpcClockSpeedAdjustment` (+40 Hz).

1. `$F4–$F7` writes enqueue on `channel_out_pending` at the write-cycle SPC time. `promote_channel_out` runs after `catch_up_to_master_clock` inside `catch_up_and_promote_channel_out`. Atomic SPC `step` would otherwise expose the write up to one instruction early.
2. `peek_apuio` does not call `promote_channel_out`, so it may lag a CPU `read()` while writes are pending. Batched `read()` flush before APUIO: `main_bus/devices.rs`.
3. Timer enable 0→1 resets interval and output; the base counter still advances while disabled (`ApuTimer::enable` / `update`).
4. `$F1` `clear_apuio12` zeros `channel_in[0]`/`[1]` and `clear_channel_out` on channels 0 and 2 (drops pending). `clear_apuio34` zeros `channel_in[2]`/`[3]` only.
5. `AudioBuffer` at `MAX_AUDIO_BUFFER_SIZE` (32000) is `clear()`ed and an error is logged (`update_clock`).
6. Direct `Spc700::step` tests must call `promote_channel_out` before inspecting `channel_out`.

## Hardware Map

| Range | Owner |
|-------|-------|
| `$2140–$2143` | S-CPU APUIO (`Apu` `BusDeviceU24`) |
| `$F1–$FF` | SPC MMIO (`ApuBus`) |
| `$FFC0–$FFFF` | IPL ROM when `ipl_rom_enabled` |

Register semantics: `docs/index.md`.

## Integration

- `MainBusImpl` maps `$2140–$217F` to `Apu`. `update_clock` from `advance_master_clock` and after each CPU memory access.
- `Apu::update_clock` runs `catch_up_and_promote_channel_out`, then `generate_sample` at each `CYCLES_PER_SAMPLE` boundary.
- CPUIO out-port deferral lives in `ApuBus` + `Apu`, not in `components/spc700`.
- `SDsp` lives in `components/s_dsp`; `ApuBus` owns the instance. `generate_sample` passes `&ram`.
- `SystemImpl::swap_audio_buffer` (root) calls `Apu::swap_audio_buffer`.

## Gaps

- Unimplemented-register / unmapped / open-bus policy: root. This directory has no additional unimplemented MMIO.

## Tests

- Lib tests: Mesen IPL trace (`test.rs` `INIT_TRACE`) and timer unit tests (`timers.rs`).
- `cargo nextest run -p sres_emulator --lib -E 'test(apu::)'`
