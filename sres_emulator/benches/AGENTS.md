# `sres_emulator/benches`

Criterion benches for per-frame `System` cost and `Clock::advance_master_clock`.

## Files

| File | Owns |
|------|------|
| `rom_benches.rs` | Per-frame `execute_frames(1)` on two test ROMs across `SyncSystem`, `BatchedSystem`, `AsyncSystem`, and `System::force_headless` |
| `timer_benches.rs` | `Clock::advance_master_clock` micro-bench; one iteration is one NTSC frame of 8-cycle steps |

## Behaviors & Gotchas

1. ROM paths resolve from `CARGO_MANIFEST_DIR` (crate root). `Cartridge::with_sfc_file` unwrap panics if the `.sfc` is missing.
2. `krom_adc` (CPU opcode ROM) benches `SyncSystem`, `BatchedSystem`, `AsyncSystem`, and headless. `krom_blend_hicolor_3840` (PPU blend/hicolor ROM) benches only `System` and headless.
3. Cartridge load sits outside `b.iter`; each sample times only `execute_frames(1)`.
4. `timer_benches` runs `44671` steps of `8` cycles (`357368` = `262 * 1364`), one NTSC frame without the odd-frame short scanline. Headless: `force_headless()` in `sres_emulator/src`.

## Integration

- Criterion invokes the `[[bench]]` targets `rom_benches` and `timer_benches` in `sres_emulator/Cargo.toml`.
- `rom_benches.rs` loads ROMs with `Cartridge::with_sfc_file`, constructs a system via `with_cartridge`, then calls `execute_frames(1)`.
- Headless benches call `System::force_headless` → `Ppu::force_headless`.
- `timer_benches.rs` constructs `Clock::default()` and calls `advance_master_clock` only; no `MainBusImpl`, PPU, or APU.

## Tests

- `cargo bench --bench rom_benches`
- `cargo bench --bench timer_benches`
- Assets: `sres_emulator/tests/rom_tests/krom_adc.sfc`, `sres_emulator/tests/ppu_tests/krom_blend_hicolor_3840.sfc`
- Not run by `./check-all.sh` or `cargo nextest`.
