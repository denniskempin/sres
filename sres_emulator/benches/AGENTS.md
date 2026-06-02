# Benchmarks

Criterion benchmarks for `sres_emulator`.

## Files

- `rom_benches.rs` — Frame timing on real ROMs (`krom_adc.sfc`, `krom_blend_hicolor_3840.sfc`). Tests `SyncSystem`, `BatchedSystem`, `AsyncSystem`, and headless modes.
- `timer_benches.rs` — Micro-benchmark for `Clock::advance_master_clock()`.

## Run

```bash
cargo bench --bench rom_benches
cargo bench --bench timer_benches
```

Requires test ROMs from `sres_emulator/tests/`.

## Adding Benchmarks

1. New file in this directory.
2. Register it in `Cargo.toml` with `[[bench]] name = "..." harness = false`.
3. Standard Criterion boilerplate: `criterion_group!`, `criterion_main!`.
