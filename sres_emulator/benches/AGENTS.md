# Benchmarks Summary for `sres_emulator`

This directory contains Criterion-based benchmarks for the `sres_emulator` crate. The benchmarks are registered in `Cargo.toml` as separate binary targets with `harness = false` (the standard Criterion pattern).

---

## Running the Benchmarks

From the workspace root or `sres_emulator/`:

```bash
# Run specific benchmark
cargo bench --bench rom_bench
cargo bench --bench timer_benches

# Run all benchmarks
cargo bench
```

The benchmarks depend on test ROMs located under `sres_emulator/tests/`. Make sure those ROMs (e.g. `krom_adc.sfc`, `krom_blend_hicolor_3840.sfc`) are available, otherwise the benchmarks will fail to load.

---

## Files

### `rom_benches.rs`

Measures the per-frame execution time of the emulator while running real ROMs. This is the primary "real-world" throughput benchmark.

**What it measures:**
- Time for `system.execute_frames(1)` on two different test ROMs.
- Compares different `System` variants and rendering modes.

**Benchmark functions:**

| Benchmark | Description |
|-----------|-------------|
| `krom_adc_frame_time_sync` | `SyncSystem` executing one frame of `krom_adc.sfc` |
| `krom_adc_frame_time_batched` | `BatchedSystem` executing one frame of `krom_adc.sfc` |
| `krom_adc_frame_time_async` | `AsyncSystem` executing one frame of `krom_adc.sfc` |
| `krom_adc_frame_time_headless` | `System` (default) in headless mode executing one frame of `krom_adc.sfc` |
| `krom_blend_frame_time` | `System` (default) executing one frame of `krom_blend_hicolor_3840.sfc` (PPU blend test) |
| `krom_blend_frame_time_headless` | `System` in headless mode executing one frame of `krom_blend_hicolor_3840.sfc` |

**Test ROMs used:**
- `tests/rom_tests/krom_adc.sfc` — CPU-focused test (ADC instruction).
- `tests/ppu_tests/krom_blend_hicolor_3840.sfc` — PPU-focused test (blend/hicolor mode).

**Patterns / Conventions:**
- Each benchmark creates a fresh `System` (or variant) inside the closure to avoid warm-up bias.
- `force_headless()` is used to disable actual frame output when measuring CPU-only cost.
- Two ROMs are chosen to represent CPU-heavy and PPU-heavy workloads.

### `timer_benches.rs`

Micro-benchmark for the low-level clock/timer advance routine.

**What it measures:**
- Time for `Clock::advance_master_clock(delta)`.
- This function is extremely hot—called many times per frame—so even small regressions matter.

**Benchmark function:**

| Benchmark | Description |
|-----------|-------------|
| `advance_master_clock` | Calls `timer.advance_master_clock(black_box(8))` 44,671 times in a tight loop per iteration, simulating one frame of timer advances. |

**Patterns / Conventions:**
- Uses `std::hint::black_box` on the input to prevent the compiler from constant-folding.
- The loop count (44,671) matches the expected number of `advance_master_clock` invocations in a single frame.

---

## Criterion Configuration

Both files follow the standard Criterion boilerplate:

```rust
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

In `Cargo.toml`:

```toml
[lib]
bench = false

[[bench]]
name = "rom_benches"
harness = false

[[bench]]
name = "timer_benches"
harness = false
```

`criterion = "0.8"` is a dev-dependency.

---

## Key Takeaways for Future Agents

1. **Add new micro-benchmarks** in a new `*.rs` file under `sres_emulator/benches/` and register it with a `[[bench]]` entry in `Cargo.toml`.
2. **Add new ROM-based benchmarks** by adding a new `c.bench_function(...)` block in `rom_benches.rs` and pointing it at a `.sfc` file under `tests/`.
3. **Keep benchmarks focused:** `rom_benches.rs` for end-to-end frame timing, `timer_benches.rs` for hot-path micro-ops.
4. **Headless vs. non-headless:** Use `force_headless()` when you want to isolate emulation logic from rendering/output overhead.
