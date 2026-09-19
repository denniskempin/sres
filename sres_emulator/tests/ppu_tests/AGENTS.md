# `sres_emulator/tests/ppu_tests`

Assets for three PPU test kinds: ROM→framebuffer, snapshot→framebuffer, and debug renders. Driver: `sres_emulator/tests/ppu_tests.rs`. Pixel-perfect exact RGBA match.

## Files

| Pattern | Owns |
|---------|------|
| `{name}.sfc` | ROM for framebuffer tests and snapshot generators |
| `{name}-framebuffer.png` | Golden for ROM→framebuffer |
| `{name}.snapshot` | PPU dump for `run_snapshot_framebuffer_test` |
| `{name}.png` | Golden for snapshot tests (no `-framebuffer` suffix) |
| `{rom}.input.json` | `frame → u16` joypad map for ignored generators |
| `{rom}-{view}.png` | Debug-render goldens (`render_sprite` / `render_background` / `render_vram`) |
| `.gitignore` | `smw.sfc`, `tloz.sfc`, `dkc.sfc` |

## Behaviors & Gotchas

1. `test_colourmath` does not use `run_framebuffer_test` or `{name}-framebuffer.png`. After 30 frames it compares `colourmath-0.png` … `colourmath-4.png`; each scene advances with `update_joypads(64, 0)`, one frame, `update_joypads(0, 0)`, then 5 frames.
2. `generate_smw_ppu_snapshots`, `generate_tloz_ppu_snapshots`, and `generate_dkc_ppu_snapshots` are `#[ignore]`. They `force_headless()`, replay `{rom}.input.json` when present, and write `{rom}-{scene}.snapshot` via `save_ppu_state` (not the `.png`). Need the gitignored `{rom}.sfc` locally.
3. `{rom}.input.json` updates the pad only on listed frames; the last `u16` sticks until a later entry (recordings pair press then `0`). DKC has no recording: `dkc-jungle` is the title-attract jungle at generator frame `1800` (`PpuState` freezes one HDMA-less register set).
4. `krom_interlace_rpg` enables unimplemented interlace/high-res and still supplies the debug-render goldens.

## Integration

- Driver `ppu_tests.rs` joins `CARGO_MANIFEST_DIR` with `tests/ppu_tests`.
- ROM tests call `execute_frames` then `swap_video_frame`. Snapshot path: parent.
- Debug: `system.debug().ppu()` after 10 frames of `krom_interlace_rpg`.
- New test: copy an existing `run_framebuffer_test` / `run_snapshot_framebuffer_test` call in `ppu_tests.rs`.

## Tests

- `cargo nextest run -p sres_emulator --test ppu_tests`
- Ignored generators: `cargo nextest run -p sres_emulator --test ppu_tests --run-ignored only -E 'test(generate_)'`

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `lib/` | Symlink → `../asm_lib` |
