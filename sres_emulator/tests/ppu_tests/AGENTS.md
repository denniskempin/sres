# `sres_emulator/tests/ppu_tests`

Assets for three PPU test kinds: ROM→framebuffer, snapshot→framebuffer, and debug renders. Driver: `sres_emulator/tests/ppu_tests.rs`. Pixel-perfect exact RGBA match.

## Files

| Pattern | Owns |
|---------|------|
| `{name}.sfc` | ROM for framebuffer tests and snapshot generators |
| `{name}-framebuffer.png` | Golden for ROM→framebuffer |
| `{name}.snapshot` | `PpuState` at `v=0` for `run_snapshot_framebuffer_test` |
| `{name}.writes` | Packed `$2100–$213F` writes (`master_clock` le64, offset le16, value u8) for that frame; missing file is empty |
| `{name}.png` | Golden for snapshot tests (no `-framebuffer` suffix) |
| `{rom}.input.json` | `frame → u16` joypad map for ignored generators |
| `{rom}-{view}.png` | Debug-render goldens (`render_sprite` / `render_background` / `render_vram`) |
| `.gitignore` | `smw.sfc`, `tloz.sfc`, `dkc.sfc` |

## Behaviors & Gotchas

1. `test_colourmath` does not use `run_framebuffer_test` or `{name}-framebuffer.png`. After 30 frames it compares `colourmath-0.png` … `colourmath-4.png`; each scene advances with `update_joypads(64, 0)`, one frame, `update_joypads(0, 0)`, then 5 frames.
2. `generate_smw_ppu_snapshots`, `generate_tloz_ppu_snapshots`, and `generate_dkc_ppu_snapshots` are `#[ignore]`. They `force_headless()`, replay `{rom}.input.json` when present, wait for `clock_info().v == 0` (`execute_one_instruction`; a `WAI` that skips `v=0` uses a later frame), then write `{rom}-{scene}.snapshot` and `{rom}-{scene}.writes` (PPU write log point, drain each scanline, panic if a drain fills `LOG_BUFFER_SIZE`). Need the gitignored `{rom}.sfc` locally.
3. `{rom}.input.json` updates the pad only on listed frames; the last `u16` sticks until a later entry (recordings pair press then `0`). DKC has no recording: `dkc-jungle` is the title-attract jungle at generator frame `1800`, not in-game Jungle Hijinxs. Current committed snapshot has no `.writes` sidecar (HDMA-less frozen sky). DKC does not use `$4218` auto-read. TLOZ USA intro is longer than the previous generator cuts; `tloz.input.json` Start/A after the title reaches Link's house for `tloz-game`.
4. `krom_interlace_rpg` enables unimplemented interlace/high-res and still supplies the debug-render goldens.
5. `krom_hdma_redspace` is the HDMA golden (CGRAM gradient via channel 0). A black image means HDMA did not run. `test_krom_hdma_redspace_snapshot_replay` captures frame 9 (`execute_frames(9)` then `v=0`) so replay matches the vblank `run_framebuffer_test(..., 10)` swaps.

## Integration

- Driver `ppu_tests.rs` joins `CARGO_MANIFEST_DIR` with `tests/ppu_tests`.
- ROM tests call `execute_frames` then `swap_video_frame`. Snapshot path: `load_state`, replay `.writes` after each `draw_scanline`.
- Debug: `system.debug().ppu()` after 10 frames of `krom_interlace_rpg`.
- New test: copy an existing `run_framebuffer_test` / `run_snapshot_framebuffer_test` call in `ppu_tests.rs`.

## Tests

- `cargo nextest run -p sres_emulator --test ppu_tests`
- Ignored generators: `cargo nextest run -p sres_emulator --test ppu_tests --run-ignored only -E 'test(generate_)'`

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `lib/` | Symlink → `../asm_lib` |
