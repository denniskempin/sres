# PPU Integration Tests – Summary for AI Agents

**Directory**: `sres_emulator/tests/ppu_tests`  
**Test Source File**: `sres_emulator/tests/ppu_tests.rs`

## What lives here?

This directory contains **integration tests for the SNES Picture Processing Unit (PPU)**.  The main Rust test file (`../ppu_tests.rs`) drives every test; the files here are the **input ROMs**, **golden reference images**, **PPU state snapshots**, **input recordings**, and **hand-written assembly** that those tests consume.

---

## Table of Contents

1. [High-Level Philosophy](#high-level-philosophy)
2. [Golden Image Comparisons](#golden-image-comparisons)
3. [Test Patterns & Categories](#test-patterns--categories)
   - [A. Krom Test ROMs – Framebuffer Rendering](#a-krom-test-roms--framebuffer-rendering)
   - [B. Manual Assembly Test ROMs](#b-manual-assembly-test-roms)
   - [C. Colour-Math Iterative Test](#c-colour-math-iterative-test)
   - [D. Debug Rendering Tests](#d-debug-rendering-tests)
   - [E. Commercial-ROM Snapshot Tests (SMW, TLoZ)](#e-commercial-rom-snapshot-tests-smw-tloz)
4. [File Naming Conventions](#file-naming-conventions)
5. [Special Fixtures / Helpers](#special-fixtures--helpers)
6. [Updating Golden Images & Snapshots](#updating-golden-images--snapshots)
7. [How to add a new PPU test](#how-to-add-a-new-ppu-test)

---

## 1. High-Level Philosophy

All tests validate the PPU by **rendering pixels and comparing them to known-good images** ("golden images" stored as `.png`).  There are two broad strategies:

| Strategy | When to use | How it works |
|---|---|---|
| **ROM → Framebuffer** | Krom test ROMs, manual assembly ROMs | Load an `.sfc` file, execute *N* frames, grab the `Framebuffer`, turn it into RGBA, compare with `{test}-framebuffer.png`. |
| **Snapshot → Framebuffer** | Commercial ROMs (SMW, TLoZ) that cannot be redistributed | Play the ROM for a certain number of frames (with recorded joypad input), **save the PPU internal state** to a `.snapshot` file.  Later tests load that snapshot into a fresh `Ppu`, draw scanlines 0–255, and compare the result to `{snapshot}.png`. |

No assertions on CPU registers or RAM are made in this file—**purely pixel output**.

---

## 2. Golden Image Comparisons

The shared helper `compare_to_golden` (in `ppu_tests.rs`) enforces the following rules:

- **Path convention**: look for `{path_prefix}.png`.
- **If the golden exists**: load it with the `image` crate, do a **exact byte-wise equality** (`golden == image.inner`).
- **On mismatch**: dump the actual rendered image to `{path_prefix}.actual.png` and `panic!`.
- **If the golden does NOT exist**: save the rendered image as the new golden (seed workflow).

> ⚠️ **Important**: Golden images must be committed.  They serve as the spec.  An `.actual.png` is a failure artifact and should never be committed.

---

## 3. Test Patterns & Categories

### A. Krom Test ROMs – Framebuffer Rendering

These are existing public-domain or home-brew SNES test ROMs (by **Krom**, a well-known SNES test-suite author).  They exercise real hardware behaviours via a small ROM image.

| Test name | Frames | What it covers |
|---|---|---|
| `krom_hdma_redspace` | 10 | HDMA colour gradient (background colour via HDMA) |
| `krom_rings` | 10 | Sprite / window / colour math (ring patterns) |
| `krom_hello_world` | 10 | Basic BG & font rendering |
| `krom_bgmap_2bpp` | 10 | 2bpp background tilemap |
| `krom_bgmap_4bpp` | 10 | 4bpp background tilemap |
| `krom_bgmap_8bpp` | 10 | 8bpp background tilemap |
| `krom_blend_hicolor_3840` | 10 | Blend modes & hi-colour background |
| `krom_interlace_rpg` | 10 | Sprite rendering *(interlacing is noted as **not implemented**, but this ROM is the only available sprite test)* |

**Files per test** (example: `krom_rings`):
- `krom_rings.sfc` – ROM binary
- `krom_rings-framebuffer.png` – golden image

**Rust driver**:
```rust
run_framebuffer_test("krom_rings", 10);
```

---

### B. Manual Assembly Test ROMs

A few ROMs are hand-assembled specifically for this emulator.

#### `sprite_rendering`
*Source*: `sprite_rendering.asm`  
*Tool-chain*: `xa65` assembler (see top-level `AGENTS.md`).  
*What it tests*: Exercises **7 distinct sprite configurations**:
1. Basic 8×8 sprite (palette 0, red diagonal)
2. 8×8 sprite with alternate palette (palette 1, white)
3. 8×8 with horizontal flip
4. 8×8 with vertical flip
5. 16×16 large sprite (4 tile quadrants)
6. 8×8 from **nametable-1** (palette 2, cyan)
7. 8×8 with **priority 2**

The ROM:
- Switches to native mode, forces blank
- Uploads tile data to VRAM via DMA
- Uploads 3 palettes to CGRAM (sprite palettes start at index 128)
- Uploads a full 544-byte OAM table via DMA (512 main + 32 high)
- Enables OBJ on main screen (`TM=$10`)
- Disables force-blank, then loops forever

**Files**:
- `sprite_rendering.asm` – source (well-commented, educational)
- `sprite_rendering.sfc` – assembled ROM
- `sprite_rendering-framebuffer.png` – golden image

**Rust driver**:
```rust
run_framebuffer_test("sprite_rendering", 1);
```

> One frame is sufficient because the ROM halts in a stable display state.

---

### C. Colour-Math Iterative Test

#### `colourmath`
*What it tests*: **Colour-math hardware (color addition / subtraction)** across 5 different scenes.

**Workflow** (unique to this test):
1. Run ROM for 30 frames.
2. For `test_id = 0..5`:
   a. Capture framebuffer → compare to `colourmath-{test_id}.png`
   b. Simulate a joypad button press (`update_joypads(64, 0)`) to advance to the next scene.
   c. Run 1 + 5 frames to let the next scene settle.

**Files**:
- `colourmath.sfc` – ROM binary
- `colourmath-0.png` … `colourmath-4.png` – five golden images

---

### D. Debug Rendering Tests

These tests do **not** compare the final framebuffer.  Instead they exercise the *internal debug render helpers* on the `Ppu` struct.

#### `krom_interlace_rpg_debug_render`
After running the `krom_interlace_rpg` ROM for 10 frames, the test captures:

| Debug view | Saved golden | API call |
|---|---|---|
| Sprite #0 | `krom_interlace_rpg-sprite0.png` | `ppu.render_sprite(0)` |
| Background BG1 | `krom_interlace_rpg-bg0.png` | `ppu.render_background(BackgroundId::BG1)` |
| VRAM (BG1 tiles) | `krom_interlace_rpg-vram-bg1.png` | `ppu.render_vram(VramRenderSelection::Background(BG1))` |
| VRAM (Sprite 0 tile) | `krom_interlace_rpg-vram-sprite.png` | `ppu.render_vram(VramRenderSelection::Sprite0)` |

**Files**:
- `krom_interlace_rpg.sfc` – (shared ROM with the non-debug test)
- `krom_interlace_rpg-sprite0.png`
- `krom_interlace_rpg-bg0.png`
- `krom_interlace_rpg-vram-bg1.png`
- `krom_interlace_rpg-vram-sprite.png`

---

### E. Commercial-ROM Snapshot Tests (SMW, TLoZ)

Because the original commercial ROMs cannot be committed to the repo, the PPU is **decoupled from the CPU** via snapshot files.

#### Super Mario World (`smw`)

| Snapshot | Frame | Scene |
|---|---|---|
| `smw-titlescreen` | 480 | Title screen |
| `smw-map` | 1900 | World map |
| `smw-level` | 2700 | First level gameplay |

**Input recording**: `smw.input.json`  
A JSON map of `frame_number → joypad_word`.  This is replayed during snapshot generation so the emulator reaches deterministic points in-game.

#### The Legend of Zelda: A Link to the Past (`tloz`)

| Snapshot | Frame | Scene |
|---|---|---|
| `tloz-triforce` | 900 | Triforce intro sequence |
| `tloz-title` | 1800 | Title screen |
| `tloz-game` | 3000 | Early gameplay |

**Input recording**: `tloz.input.json` – same format as SMW.

**Files per snapshot** (example: `smw-titlescreen`):
- `smw-titlescreen.snapshot` – binary blob of PPU state (loaded via `Ppu::load_state`)
- `smw-titlescreen.png` – golden framebuffer image

> `.gitignore` explicitly ignores `smw.sfc` and `tloz.sfc` to prevent accidental commits of copyrighted ROMs.

---

## 4. File Naming Conventions

| Pattern | Meaning |
|---|---|
| `{name}.sfc` | SNES ROM binary (test input) |
| `{name}-framebuffer.png` | Golden image for framebuffer comparison |
| `{name}.snapshot` | Binary PPU state dump (for snapshot tests) |
| `{name}.png` (without `-framebuffer`) | Golden image for snapshot framebuffer comparison |
| `{rom_name}-{scene}.snapshot` | Snapshot for commercial-ROM tests |
| `{rom_name}-{scene}.png` | Golden image for commercial-ROM tests |
| `{rom_name}.input.json` | JSON `{ "frame": joypad_word }` input recording |
| `{rom_name}-{debug_type}.png` | Golden image for debug-render tests |

---

## 5. Special Fixtures / Helpers

All helpers live in `ppu_tests.rs`.

### `run_framebuffer_test(test_name: &str, frame: u64) -> System`
- Loads `{test_name}.sfc`
- Executes `frame` frames
- Captures framebuffer via `system.swap_video_frame`
- Compares with `{test_name}-framebuffer.png`

### `run_snapshot_framebuffer_test(snapshot_name: &str)`
- Loads `{snapshot_name}.snapshot` into a **new** `Ppu`
- Draws scanlines `0..256`
- Compares framebuffer with `{snapshot_name}.png`

### `generate_ppu_snapshots(rom_name, &[(scene, frame)])`
- **Ignored by default** (`#[ignore]`)
- Replays the ROM, injecting joypad inputs from `{rom_name}.input.json`
- At each target frame, writes `system.save_ppu_state()` to `{rom_name}-{scene}.snapshot`

### `compare_to_golden(image: &TestImageImpl, path_prefix: &Path)`
- The single source of truth for image comparison.
- Uses exact equality.  **No tolerance / no fuzzy matching**.

### `TestImageImpl`
- Thin adapter around `image::RgbaImage` implementing the crate-local `Image` trait.
- Converts the emulator’s `Rgba32` pixels into the `image` crate format.

---

## 6. Updating Golden Images & Snapshots

1. **Delete the old `.png`** (or run the test once without it to seed a new one).
2. **For snapshots**: run the ignored generator test, e.g.:
   ```bash
   cargo test --test ppu_tests generate_smw_ppu_snapshots -- --ignored
   ```
   Then run the normal snapshot test to verify:
   ```bash
   cargo test --test ppu_tests test_smw_titlescreen
   ```
3. Review the new golden images carefully (pixel-perfect correctness is the spec).
4. Commit both `.png` and `.snapshot` files.

---

## 7. How to add a new PPU test

### Option A – Small custom ROM (recommended for targeted behaviour)
1. Write a 65816 assembly file in this directory (see `sprite_rendering.asm` as template).
2. Assemble it with `xa65` ( handled by the build system or manually: `xa sprite_rendering.asm -o sprite_rendering.sfc` ).
3. Add a test in `ppu_tests.rs`:
   ```rust
   #[test]
   pub fn test_my_feature() {
       run_framebuffer_test("my_feature", 1);
   }
   ```
4. Run the test once (without the golden image) to generate `{my_feature}-framebuffer.png`.
5. Visually inspect the image, then commit it alongside the `.sfc` and the `.asm` source.

### Option B – Krom / public test ROM
1. Drop the `.sfc` in this directory.
2. Add:
   ```rust
   run_framebuffer_test("krom_new_test", 10);
   ```
3. Generate the golden image as above.

### Option C – Snapshot test for a commercial ROM
1. Obtain the ROM locally (never commit it).
2. Create an `{rom_name}.input.json` with the joypad events needed to reach the desired scene.
3. Add a generator test:
   ```rust
   #[test]
   #[ignore]
   fn generate_mypkg_ppu_snapshots() {
       generate_ppu_snapshots("mypkg", &[("scene1", 500)]);
   }
   ```
4. Run the generator (`-- --ignored`), verify the emitted `.snapshot` and `.png`.
5. Add the non-ignored reader test:
   ```rust
   #[test]
   fn test_mypkg_scene1() {
       run_snapshot_framebuffer_test("mypkg-scene1");
   }
   ```

---

## Quick Reference: File Inventory

### Source / ROM files
- `sprite_rendering.asm`
- `sprite_rendering.sfc`
- `colourmath.sfc`
- `krom_hdma_redspace.sfc`
- `krom_rings.sfc`
- `krom_hello_world.sfc`
- `krom_bgmap_2bpp.sfc`
- `krom_bgmap_4bpp.sfc`
- `krom_bgmap_8bpp.sfc`
- `krom_blend_hicolor_3840.sfc`
- `krom_interlace_rpg.sfc`

### Input recordings
- `smw.input.json`
- `tloz.input.json`

### Golden images (`.png`)
- `sprite_rendering-framebuffer.png`
- `colourmath-0.png` … `colourmath-4.png`
- `krom_*-framebuffer.png`
- `smw-titlescreen.png`, `smw-map.png`, `smw-level.png`
- `tloz-triforce.png`, `tloz-title.png`, `tloz-game.png`
- `krom_interlace_rpg-sprite0.png`, `-bg0.png`, `-vram-bg1.png`, `-vram-sprite.png`

### Snapshots (`.snapshot`)
- `smw-titlescreen.snapshot`, `smw-map.snapshot`, `smw-level.snapshot`
- `tloz-triforce.snapshot`, `tloz-title.snapshot`, `tloz-game.snapshot`

### Other
- `.gitignore` – ignores `smw.sfc` and `tloz.sfc`
- `SUMMARY.md` – this file
