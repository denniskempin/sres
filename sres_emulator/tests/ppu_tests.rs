//! High level testing focused on the PPU
//!
//! Most tests execute roms and compare the rendered framebuffer against a previously stored
//! golden image.
//!
//! Some tests will use snapshots of the PPU state to run testing in isolation of the CPU
//! behavior and in absence of ROM files.
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use image::RgbaImage;
use sres_emulator::common::address::AddressU24;
use sres_emulator::common::bus::BusDeviceU24;
use sres_emulator::common::image::Image;
use sres_emulator::common::image::Rgba32;
use sres_emulator::common::logging;
use sres_emulator::components::cartridge::Cartridge;
use sres_emulator::components::ppu::BackgroundId;
use sres_emulator::components::ppu::Framebuffer;
use sres_emulator::components::ppu::Ppu;
use sres_emulator::components::ppu::VramRenderSelection;
use sres_emulator::System;

#[test]
pub fn test_krom_hdma_redspace() {
    run_framebuffer_test("krom_hdma_redspace", 10);
}

#[test]
pub fn test_krom_rings() {
    run_framebuffer_test("krom_rings", 10);
}

#[test]
pub fn test_krom_hello_world() {
    run_framebuffer_test("krom_hello_world", 10);
}

#[test]
pub fn test_krom_bgmap_2bpp() {
    run_framebuffer_test("krom_bgmap_2bpp", 10);
}

#[test]
pub fn test_krom_bgmap_4bpp() {
    run_framebuffer_test("krom_bgmap_4bpp", 10);
}

#[test]
pub fn test_krom_bgmap_8bpp() {
    run_framebuffer_test("krom_bgmap_8bpp", 10);
}

#[test]
pub fn test_krom_blend_hicolor_3840() {
    run_framebuffer_test("krom_blend_hicolor_3840", 10);
}

#[test]
pub fn test_krom_interlace_rpg() {
    // Note: Interlacing or high-res is not implemented and used by this test rom.
    // However it's the only test rom I have available to test sprite rendering.
    run_framebuffer_test("krom_interlace_rpg", 10);
}

#[test]
pub fn test_colourmath() {
    logging::test_init(true);

    let rom_path = test_dir().join(format!("colourmath.sfc"));
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());
    system.execute_frames(30);

    // The test rom shows 5 different scenes with different color math operations.
    for test_id in 0..5 {
        let framebuffer_path = test_dir().join(format!("colourmath-{test_id}"));

        let mut video_frame = Framebuffer::default();
        system.swap_video_frame(&mut video_frame);
        compare_to_golden(&video_frame.to_rgba::<TestImageImpl>(), &framebuffer_path);

        // Advance to next test by simulating a button press.
        system.update_joypads(64, 0);
        system.execute_frames(1);
        system.update_joypads(0, 0);
        system.execute_frames(5);
    }
}

/// Renders the framebuffer at `frame` and compares against previously stored golden image.
fn run_framebuffer_test(test_name: &str, frame: u64) -> System {
    logging::test_init(true);

    let rom_path = test_dir().join(format!("{test_name}.sfc"));
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());
    system.execute_frames(frame);
    let framebuffer_path = test_dir().join(format!("{test_name}-framebuffer"));

    let mut video_frame = Framebuffer::default();
    system.swap_video_frame(&mut video_frame);
    compare_to_golden(&video_frame.to_rgba::<TestImageImpl>(), &framebuffer_path);
    system
}

/// Renders debug views of PPU Sprites, Backgrounds and VRAM and comapres them against previously
/// stored golden images.
#[test]
pub fn test_krom_interlace_rpg_debug_render() {
    // Note: Interlacing or high-res is not implemented and used by this test rom.
    // However it's the only test rom I have available to test sprite rendering.
    logging::test_init(true);

    let rom_path = test_dir().join("krom_interlace_rpg.sfc");
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());
    system.execute_frames(10);
    let system_debug = system.debug();
    let ppu = system_debug.ppu();

    // Debug render sprite 0
    let sprite_path = test_dir().join("krom_interlace_rpg-sprite0");
    compare_to_golden(&ppu.render_sprite(0), &sprite_path);

    // Debug render BG0
    let background_path = test_dir().join("krom_interlace_rpg-bg0");
    compare_to_golden(&ppu.render_background(BackgroundId::BG1), &background_path);

    // Debug render portion of VRAM
    let vram_bg0_path = test_dir().join("krom_interlace_rpg-vram-bg1");
    compare_to_golden(
        &ppu.render_vram(32, 0, VramRenderSelection::Background(BackgroundId::BG1)),
        &vram_bg0_path,
    );
    let vram_sprite_path = test_dir().join("krom_interlace_rpg-vram-sprite");
    compare_to_golden(
        &ppu.render_vram(32, 0, VramRenderSelection::Sprite0),
        &vram_sprite_path,
    );
}

#[test]
#[ignore = "only run when snapshots need updating"]
fn generate_smw_ppu_snapshots() {
    generate_ppu_snapshots(
        "smw",
        &[("titlescreen", 480), ("map", 1900), ("level", 2700)],
    );
}

#[test]
fn test_smw_titlescreen() {
    run_snapshot_framebuffer_test("smw-titlescreen");
}

#[test]
fn test_smw_map() {
    run_snapshot_framebuffer_test("smw-map");
}

#[test]
fn test_smw_level() {
    run_snapshot_framebuffer_test("smw-level");
}

#[test]
#[ignore = "only run when snapshots need updating"]
fn generate_tloz_ppu_snapshots() {
    generate_ppu_snapshots(
        "tloz",
        &[("triforce", 900), ("title", 1800), ("game", 3000)],
    );
}

#[test]
fn test_tloz_triforce() {
    run_snapshot_framebuffer_test("tloz-triforce");
}

#[test]
fn test_tloz_title() {
    run_snapshot_framebuffer_test("tloz-title");
}

#[test]
fn test_tloz_game() {
    run_snapshot_framebuffer_test("tloz-game");
}

/// Loads the PPU memory / state from a snapshot file and compares the framebuffer rendering to
/// a previously stored golden image.
/// The snapshot files are generated by `generate_ppu_snapshots`.
fn run_snapshot_framebuffer_test(snapshot_name: &str) {
    logging::test_init(true);

    let mut ppu = Ppu::new();
    ppu.load_state(&std::fs::read(test_dir().join(format!("{snapshot_name}.snapshot"))).unwrap())
        .unwrap();
    for scanline in 0..256 {
        ppu.draw_scanline(scanline);
    }

    compare_to_golden(
        &ppu.framebuffer().to_rgba::<TestImageImpl>(),
        &test_dir().join(snapshot_name),
    );
}

/// Generates a PPU snapshot for each case listed in `snapshots` to be used with `run_snapshot_framebuffer_test`.
fn generate_ppu_snapshots(rom_name: &str, snapshots: &[(&str, u64)]) {
    let input_path = test_dir().join(format!("{rom_name}.input.json"));
    let input_recording: HashMap<u64, u16> = if input_path.exists() {
        serde_json::from_reader(std::fs::File::open(&input_path).unwrap()).unwrap()
    } else {
        HashMap::new()
    };

    let rom_path = test_dir().join(format!("{rom_name}.sfc"));
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());
    system.ppu().force_headless();

    let last_frame = snapshots.iter().map(|(_, frame)| frame).max().unwrap();
    for frame in 0..=*last_frame {
        if input_recording.contains_key(&frame) {
            system.update_joypads(input_recording[&frame], 0);
        }
        system.execute_frames(1);

        if let Some((test_name, _)) = snapshots
            .iter()
            .find(|(_, snapshot_frame)| *snapshot_frame == frame)
        {
            std::fs::write(
                test_dir().join(format!("{rom_name}-{test_name}.snapshot")),
                system.ppu().save_state(),
            )
            .unwrap();
        }
    }
}

// ============================================================================
// Sprite rendering tests
// ============================================================================
//
// A single test (`test_sprite_rendering`) covers all six SNES sprite size
// formats (8×8, 16×16, 32×32, 64×64, 16×32, 32×64) and produces one golden
// image showing all of them.
//
// Because the OBJSEL register selects exactly two sizes simultaneously,
// three render passes are needed (one per size-pair mode). Each pass places
// the small and large variants side-by-side. The top rows of each pass are
// cropped and stacked vertically into a single 256×144 composite.
//
// Tile graphics are stored in sprite_tiles.bin (checked into the repo).
// To regenerate it, run: cargo test generate_sprite_tiles_bin -- --ignored

/// 16-entry palette written to CGRAM starting at index 128 (sprite palette 0).
/// Color 0 is transparent; colors 1-15 are visually distinct hues.
/// SNES color format: 0bBBBBB_GGGGG_RRRRR (15-bit BGR).
const SPRITE_PALETTE: [u16; 16] = [
    0x0000, // 0: Transparent
    0x001F, // 1: Red
    0x03E0, // 2: Green
    0x03FF, // 3: Yellow
    0x7C00, // 4: Blue
    0x7C1F, // 5: Magenta
    0x7FE0, // 6: Cyan
    0x7FFF, // 7: White
    0x0010, // 8: Dark red
    0x0200, // 9: Dark green
    0x0210, // 10: Dark yellow
    0x4000, // 11: Dark blue
    0x4010, // 12: Dark magenta
    0x42E0, // 13: Teal
    0x4210, // 14: Gray
    0x2108, // 15: Light gray
];

/// Loads the pre-generated VRAM tile data from `sprite_tiles.bin`.
///
/// The file contains 1920 little-endian u16 words covering tile indices 0-119.
/// Each 8×8 tile at grid position (cx, cy) is filled with a solid color
/// `(cy * 8 + cx) % 15 + 1`, giving each sub-tile a distinct palette entry.
///
/// Regenerate the file with: cargo test generate_sprite_tiles_bin -- --ignored
fn load_sprite_tile_data() -> Vec<u16> {
    let path = test_dir().join("sprite_tiles.bin");
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|_| panic!("Missing {path:?} — run generate_sprite_tiles_bin first"));
    bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect()
}

/// Generates `sprite_tiles.bin` in the test data directory.
///
/// The file encodes a 8×8-tile grid (sufficient for the largest 64×64 sprite)
/// with each tile filled by a unique solid 4bpp colour derived from its
/// (column, row) position within the grid.
#[test]
#[ignore = "only run to regenerate sprite_tiles.bin"]
fn generate_sprite_tiles_bin() {
    // Tile index T = row*16 + col lives at VRAM word offset T*16.
    // The 8-wide grid uses columns 0-7; columns 8-15 are left at zero.
    let max_tile_idx: usize = 7 * 16 + 7; // tile 119
    let num_words = (max_tile_idx + 1) * 16; // 1920 words = 3840 bytes
    let mut words = vec![0u16; num_words];

    for cy in 0..8_usize {
        for cx in 0..8_usize {
            let tile_idx = cy * 16 + cx;
            let color = ((cy * 8 + cx) % 15 + 1) as u8;

            let plane0: u8 = if color & 1 != 0 { 0xFF } else { 0x00 };
            let plane1: u8 = if color & 2 != 0 { 0xFF } else { 0x00 };
            let plane2: u8 = if color & 4 != 0 { 0xFF } else { 0x00 };
            let plane3: u8 = if color & 8 != 0 { 0xFF } else { 0x00 };

            // Low word holds planes 0 & 1; high word (offset +8) holds planes 2 & 3.
            let low_word = (plane1 as u16) << 8 | plane0 as u16;
            let high_word = (plane3 as u16) << 8 | plane2 as u16;
            let base = tile_idx * 16;
            for row in 0..8_usize {
                words[base + row] = low_word;
                words[base + 8 + row] = high_word;
            }
        }
    }

    let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_le_bytes()).collect();
    std::fs::write(test_dir().join("sprite_tiles.bin"), bytes).unwrap();
}

/// Renders one OBJSEL size-pair mode with two sprites (small + large) placed
/// side-by-side and returns the full 256×224 framebuffer as an `RgbaImage`.
///
/// `objsel` encodes the size-pair in bits 7-5 (see OBJSEL register docs).
/// `small_pos` and `large_pos` are the (x, y) screen positions for each sprite.
/// OAM high-table byte 0x200 is set so sprite 0 = small, sprite 1 = large.
fn render_size_pair(
    vram_data: &[u16],
    objsel: u8,
    small_pos: (u8, u8),
    large_pos: (u8, u8),
) -> RgbaImage {
    let mut ppu = Ppu::new();
    let wr = |ppu: &mut Ppu, offset: u16, value: u8| {
        ppu.write(AddressU24::new(0, offset), value);
    };

    // OBJSEL: sprite size pair + nametable at address 0
    wr(&mut ppu, 0x2101, objsel);

    // Write tile data to VRAM (increment after high byte, $2119)
    wr(&mut ppu, 0x2115, 0x80);
    wr(&mut ppu, 0x2116, 0x00); // VMADDL
    wr(&mut ppu, 0x2117, 0x00); // VMADDH
    for &word in vram_data {
        wr(&mut ppu, 0x2118, (word & 0xFF) as u8);
        wr(&mut ppu, 0x2119, (word >> 8) as u8);
    }

    // Write sprite palette 0 to CGRAM (indices 128-143)
    wr(&mut ppu, 0x2121, 128);
    for &color in &SPRITE_PALETTE {
        wr(&mut ppu, 0x2122, (color & 0xFF) as u8);
        wr(&mut ppu, 0x2122, (color >> 8) as u8);
    }

    // Write sprites 0 (small) and 1 (large) to OAM low table
    wr(&mut ppu, 0x2102, 0x00); // OAMADDL
    wr(&mut ppu, 0x2103, 0x00); // OAMADDH: low table
    wr(&mut ppu, 0x2104, small_pos.0); // Sprite 0 X
    wr(&mut ppu, 0x2104, small_pos.1); // Sprite 0 Y
    wr(&mut ppu, 0x2104, 0x00); // Sprite 0 tile 0
    wr(&mut ppu, 0x2104, 0x30); // Sprite 0 priority=3, palette=0
    wr(&mut ppu, 0x2104, large_pos.0); // Sprite 1 X
    wr(&mut ppu, 0x2104, large_pos.1); // Sprite 1 Y
    wr(&mut ppu, 0x2104, 0x00); // Sprite 1 tile 0
    wr(&mut ppu, 0x2104, 0x30); // Sprite 1 priority=3, palette=0

    // OAM high table: sprite 0 = small (bit 1=0), sprite 1 = large (bit 3=1)
    wr(&mut ppu, 0x2102, 0x00);
    wr(&mut ppu, 0x2103, 0x01); // Select high OAM table (address 0x200)
    wr(&mut ppu, 0x2104, 0x08); // Bit 3 set → sprite 1 uses secondary (large) size

    // Enable sprites on the main screen
    wr(&mut ppu, 0x212C, 0x10);

    for scanline in 0..256 {
        ppu.draw_scanline(scanline);
    }
    ppu.framebuffer().to_rgba::<TestImageImpl>().inner
}

/// Renders all six SNES sprite size formats and validates them against a single
/// golden image.
///
/// Three render passes are performed (one per OBJSEL size-pair mode). Each pass
/// shows the small and large variants side-by-side. The top rows of each pass
/// are cropped to the height of the tallest sprite in that pair and stacked
/// vertically, producing a 256×144 composite:
///
///   rows   0- 15: 8×8  (left) and 16×16 (right)
///   rows  16- 79: 32×32 (left) and 64×64 (right)
///   rows  80-143: 16×32 (left) and 32×64 (right)
#[test]
fn test_sprite_rendering() {
    logging::test_init(true);
    let vram_data = load_sprite_tile_data();

    // (objsel, small_x, large_x, strip_height)
    // Sprites are placed at y=0; x chosen so neither overlaps the other.
    let passes: &[(u8, u8, u8, u32)] = &[
        (0x00, 0, 12, 16), // mode 0: 8×8  at x=0, 16×16 at x=12
        (0xA0, 0, 36, 64), // mode 5: 32×32 at x=0, 64×64 at x=36
        (0xC0, 0, 20, 64), // mode 6: 16×32 at x=0, 32×64 at x=20
    ];

    let total_h: u32 = passes.iter().map(|p| p.3).sum();
    let mut combined = RgbaImage::new(256, total_h);
    let mut y_offset = 0u32;
    for &(objsel, small_x, large_x, strip_h) in passes {
        let frame = render_size_pair(&vram_data, objsel, (small_x, 0), (large_x, 0));
        for y in 0..strip_h {
            for x in 0..256_u32 {
                combined.put_pixel(x, y_offset + y, *frame.get_pixel(x, y));
            }
        }
        y_offset += strip_h;
    }

    compare_to_golden(
        &TestImageImpl { inner: combined },
        &test_dir().join("sprite_rendering"),
    );
}

fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/ppu_tests")
}

fn compare_to_golden(image: &TestImageImpl, path_prefix: &Path) {
    let golden_path = path_prefix.with_extension("png");
    if golden_path.exists() {
        let golden: RgbaImage = image::open(&golden_path).unwrap().into_rgba8();
        if golden != image.inner {
            let actual_path = golden_path.with_extension("actual.png");
            image.inner.save(&actual_path).unwrap();
            panic!("Image does not match golden. See {:?}", actual_path);
        }
    } else {
        image.inner.save(golden_path).unwrap();
    }
}

struct TestImageImpl {
    inner: RgbaImage,
}

impl Image for TestImageImpl {
    fn new(width: u32, height: u32) -> Self {
        TestImageImpl {
            inner: RgbaImage::new(width, height),
        }
    }

    fn set_pixel(&mut self, index: (u32, u32), value: Rgba32) {
        self.inner[(index.0, index.1)] = image::Rgba::from(value.0);
    }
}
