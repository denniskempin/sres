# PPU Tests

Directory: `sres_emulator/tests/ppu_tests`  
Test driver: `sres_emulator/tests/ppu_tests.rs`

## What This Is

Golden-image tests for the SNES Picture Processing Unit (PPU). Load a ROM, run frames, capture the framebuffer, compare against a `.png`. Pixel-perfect exact match; mismatches write `.actual.png` and panic.

## Test Types

### 1. ROM → Framebuffer
Load `.sfc`, run N frames, compare framebuffer against `{name}-framebuffer.png`.

**Krom test ROMs:**
- `krom_hdma_redspace` (10 frames) – HDMA colour gradient
- `krom_rings` (10 frames) – Sprites, windows, colour math
- `krom_hello_world` (10 frames) – Basic BG & font
- `krom_bgmap_2bpp/4bpp/8bpp` (10 frames) – BG tilemap modes
- `krom_blend_hicolor_3840` (10 frames) – Blending & hi-colour
- `krom_interlace_rpg` (10 frames) – Sprite rendering

**Hand-written ROMs:**
- `sprite_rendering` (1 frame) – 7 sprite configs (sizes, flips, palettes, priority, nametable)
- `colourmath` (30 frames + 5 scenes) – Colour addition/subtraction; advances scenes via simulated joypad input (`update_joypads(64, 0)`). Compares against `colourmath-0.png` … `colourmath-4.png`.

### 2. Snapshot → Framebuffer
For commercial ROMs (SMW, TLoZ). PPU state is saved to `.snapshot` at a specific frame using recorded inputs. Tests load the snapshot into a fresh `Ppu`, draw scanlines 0–255, and compare against `{snapshot}.png`. No ROM needed at test time.

**SMW:** `smw-titlescreen` (480), `smw-map` (1900), `smw-level` (2700)  
**TLoZ:** `tloz-triforce` (900), `tloz-title` (1800), `tloz-game` (3000)  
**Inputs:** `{rom}.input.json` — map of `frame_number → joypad_word`.

### 3. Debug Rendering
Not framebuffer tests. Exercise internal `Ppu` debug helpers.

After running `krom_interlace_rpg` for 10 frames:
- `render_sprite(0)` → `krom_interlace_rpg-sprite0.png`
- `render_background(BG1)` → `krom_interlace_rpg-bg0.png`
- `render_vram(Background(BG1))` → `krom_interlace_rpg-vram-bg1.png`
- `render_vram(Sprite0)` → `krom_interlace_rpg-vram-sprite.png`

## File Naming

| Pattern | Meaning |
|---|---|
| `{name}.sfc` | SNES ROM input |
| `{name}-framebuffer.png` | Golden image (ROM tests) |
| `{name}.snapshot` | Binary PPU state dump |
| `{name}.png` (no `-framebuffer`) | Golden image (snapshot tests) |
| `{rom}-{scene}.snapshot` | Commercial ROM snapshot |
| `{rom}-{scene}.png` | Golden image for snapshot |
| `{rom}.input.json` | Joypad input recording |
| `{rom}-{debug_type}.png` | Debug render golden image |

## Helpers (in `ppu_tests.rs`)

- `run_framebuffer_test(name, frames)` — Load `.sfc`, run frames, compare framebuffer.
- `run_snapshot_framebuffer_test(name)` — Load `.snapshot`, draw scanlines 0–255, compare.
- `generate_ppu_snapshots(rom, &[(scene, frame)])` — **Ignored by default.** Replay ROM with inputs, save PPU state at target frames.
- `compare_to_golden(image, path)` — Exact byte-wise comparison. Missing golden = auto-create.

## Updating Goldens / Snapshots

1. Delete old `.png` or run test once to seed.
2. For snapshots: `cargo test --test ppu_tests generate_smw_ppu_snapshots -- --ignored`
3. Verify visually. Commit `.png` and `.snapshot`.

## Adding a New PPU Test

**Custom ROM:**
1. Write `.asm` (see `sprite_rendering.asm`).
2. Assemble: `xa file.asm -o file.sfc`
3. Add test: `run_framebuffer_test("file", 1);`
4. Run once to generate `{file}-framebuffer.png`, verify, commit.

**Krom / public ROM:**
1. Drop `.sfc` in this directory.
2. Add `run_framebuffer_test("name", 10);`
3. Run to generate golden image.

**Snapshot (commercial ROM):**
1. Get ROM locally (never commit).
2. Create `{rom}.input.json`.
3. Add ignored generator:
   ```rust
   #[test]
   #[ignore]
   fn generate_mypkg_ppu_snapshots() {
       generate_ppu_snapshots("mypkg", &[("scene1", 500)]);
   }
   ```
4. Run generator (`-- --ignored`).
5. Add reader test:
   ```rust
   #[test]
   fn test_mypkg_scene1() {
       run_snapshot_framebuffer_test("mypkg-scene1");
   }
   ```

## Key Files

**ROMs / Source:**
- `sprite_rendering.asm`, `.sfc`
- `colourmath.sfc`
- `krom_*.sfc` (7 tests)

**Inputs:**
- `smw.input.json`
- `tloz.input.json`

**Goldens / Snapshots:**
- `sprite_rendering-framebuffer.png`
- `colourmath-{0..4}.png`
- `krom_*-framebuffer.png`
- `smw-{titlescreen,map,level}.{snapshot,png}`
- `tloz-{triforce,title,game}.{snapshot,png}`
- `krom_interlace_rpg-{sprite0,bg0,vram-bg1,vram-sprite}.png`

**Other:**
- `.gitignore` — ignores `smw.sfc`, `tloz.sfc`
