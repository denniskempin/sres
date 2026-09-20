# `sres_egui/src`

Native/WASM egui frontend: home screen, `EmulatorApp` loop, cpal audio, and debugger shell over `System`.

## Files

| File | Owns |
|------|------|
| `main.rs` | Dual `main()`: native `ResArgs` + `eframe::run_native`; WASM `eframe::WebRunner` on `#emulator_canvas` |
| `app.rs` | `EmulatorApp` (`eframe::App::ui`): cartridge load, joypads, texture present, debugger vs audio-paced run |
| `embedded_roms.rs` | `EMBEDDED_ROMS` (`CategoryInfo`, `RomFileInfo`) included from `OUT_DIR/embedded_roms_generated.rs` |
| `debug.rs` | `DebugUi`, `DebugCommand`, `InternalLink`; module file for `debug/` |
| `test_utils.rs` | `widget_snapshot` / `snapshot_options` for `egui_kittest` PNG goldens |
| `util.rs` | `RingBuffer`, `EguiImageImpl` (`Image` → egui `ColorImage`), cfg-split `Instant` |
| `audio.rs` | `AudioOutput`: cpal stream + `Arc<Mutex<AudioBufferQueue>>`; `update` calls `swap_audio_buffer` |
| `home.rs` | `home_screen`: clickable `EMBEDDED_ROMS` cards; loads with `Cartridge::with_sfc_data` |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `debug/` | Debugger UI panels (CPU, PPU, APU, memory, log, unimplemented, syntax) |

## Behaviors & Gotchas

1. `load_cartridge` enables the debugger (`app.rs:84`); `DebugUi` starts on `DebugCommand::Pause` (`debug.rs:51`). Debugger-off playback uses `execute_for_audio_samples` (`app.rs:247-248`). Debug `Run` uses `execute_for_duration(stable_dt)` (`debug.rs:75`).
2. `main_display` updates the egui texture only when `swap_video_frame` returns true (`app.rs:212-216`). `emulator_ui` always `request_repaint` (`app.rs:280`).
3. Native `main` may pass a CLI `Cartridge`; WASM always starts with `None` and shows `home_screen` (`main.rs:53-61`, `main.rs:90`).
4. `Instant` is `std::time::Instant` natively and `Date.now()` milliseconds on WASM (`util.rs:80-114`).
5. The cpal callback drains `AudioBufferQueue` on the audio thread; UI-thread `AudioOutput::update` pushes via `swap_audio_buffer` (`audio.rs:97-127`). Each APU `i16` is written to both stereo channels (`audio.rs:107-108`).
6. `EMBEDDED_ROMS` is generated at build time from `sres_egui/roms/<category>/*.sfc` into `OUT_DIR/embedded_roms_generated.rs` (`embedded_roms.rs:21-22`).
7. `App::ui` consumes drops before home/emulator (`app.rs:286`). Non-`.sfc` is ignored with `log::warn` (`app.rs:101-103`). Native prefers `with_sfc_file` when `path.is_file()`, else `bytes()` (`app.rs:107-122`). WASM `bytes_async` writes `pending_dropped_rom` (`app.rs:125-135`); `load_pending_dropped_rom` takes the bytes on a later frame so the MutexGuard is not held across `load_cartridge` (`app.rs:138-148`).

## Integration

- Native `main` and WASM `WebRunner` construct `EmulatorApp`. Root owns `cargo run` / `trunk serve`.
- `EmulatorApp` owns `System` (root: UI uses `BatchedSystem`). Public execution: `with_cartridge`, `execute_for_audio_samples` / `execute_for_duration` / `execute_frames` / `execute_scanlines` / `execute_one_instruction`, `swap_video_frame`, `swap_audio_buffer`, `update_joypads`, `debugger()`.
- `Cartridge` and `Framebuffer` come from `sres_emulator::components::{cartridge,ppu}` (not re-exported by `lib.rs`).
- Playback does not call `cpu.step()`. `DebugUi::modals` peeks with `emulator.cpu.bus.peek_u8` (`debug.rs:148`).
- `home_screen` and file drops call `load_cartridge`. `AudioOutput::update` is the sole `swap_audio_buffer` site.

## Gaps

- Dropped ROMs are not persisted (WASM `web-sys` Storage is unused).
- `InternalLink::Spc700ProgramCounter` is a no-op (`debug.rs:143`).
- puffin profiler window is commented out (`debug.rs:129-131`).

## Tests

- No tests in this directory. `test_utils.rs` feeds `debug/` `egui_kittest` snapshots (`UPDATE_SNAPSHOTS=1` writes `sres_egui/tests/snapshots/`).
- Crate: `cargo test -p sres_egui` (`egui_kittest` in `sres_egui/Cargo.toml` `[dev-dependencies]`; four `debug/` tests listed).
