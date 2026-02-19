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
// These tests verify sprite rendering for each supported sprite size format.
// Each test sets up the PPU directly (without a ROM), writes tile data to VRAM,
// configures a palette in CGRAM, places a single sprite in OAM, and renders all
// scanlines. The resulting framebuffer is compared against a golden PNG image.
//
// On the first run (no golden file present) the test auto-creates the golden.
// On subsequent runs the golden is compared pixel-by-pixel.
//
// Tile pattern: each 8×8 sub-tile within the sprite receives a unique solid
// color (palette index 1-15) based on its (column, row) position, making it
// easy to visually verify correct size, layout, and ordering of tiles.

/// 16-entry palette written starting at CGRAM index 128 (sprite palette 0).
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

/// Builds a VRAM word array containing 4bpp tile data for a sprite made of
/// `coarse_w × coarse_h` 8×8 tiles.
///
/// Tile layout in VRAM follows SNES conventions:
/// - Tile index T is located at word address `T * 16`.
/// - Within a sprite, sub-tile (cx, cy) maps to tile index `cy * 16 + cx`.
/// - Each tile stores 8 rows; row R occupies two words at offsets R and R+8
///   within the tile block (planes 0/1 in the low word, planes 2/3 in the
///   high word).
///
/// Every sub-tile is filled with a single solid color chosen from 1-15 by
/// `(cy * coarse_w + cx) % 15 + 1`, so adjacent tiles are always different.
fn make_sprite_tile_data(coarse_w: u32, coarse_h: u32) -> Vec<u16> {
    // The highest tile index needed is (coarse_h-1)*16 + (coarse_w-1).
    let max_tile_idx = (coarse_h - 1) * 16 + (coarse_w - 1);
    let num_words = (max_tile_idx as usize + 1) * 16;
    let mut data = vec![0u16; num_words];

    for cy in 0..coarse_h {
        for cx in 0..coarse_w {
            let tile_idx = (cy * 16 + cx) as usize;
            // Unique color 1-15 for each sub-tile position.
            let color = ((cy * coarse_w + cx) % 15 + 1) as u8;

            // Decompose color into bitplanes (4bpp).
            let plane0: u8 = if color & 1 != 0 { 0xFF } else { 0x00 };
            let plane1: u8 = if color & 2 != 0 { 0xFF } else { 0x00 };
            let plane2: u8 = if color & 4 != 0 { 0xFF } else { 0x00 };
            let plane3: u8 = if color & 8 != 0 { 0xFF } else { 0x00 };

            // Each VRAM word packs two planes: low byte = plane N, high byte = plane N+1.
            let low_word = (plane1 as u16) << 8 | plane0 as u16; // planes 0 & 1
            let high_word = (plane3 as u16) << 8 | plane2 as u16; // planes 2 & 3

            let base = tile_idx * 16;
            for row in 0..8_usize {
                data[base + row] = low_word; // row word for planes 0/1
                data[base + 8 + row] = high_word; // row word for planes 2/3
            }
        }
    }

    data
}

/// Creates a `Ppu` configured with a single sprite ready for rendering.
///
/// Parameters:
/// - `objsel`: full OBJSEL byte ($2101); bits 7-5 select the size-pair mode,
///   bits 4-3 select the second nametable offset, bits 2-0 the base address.
/// - `oam_high_attr`: byte written to OAM high-table entry 0 ($200).
///   Bit 1 selects secondary size for sprite 0 (0 = small, 1 = large).
/// - `coarse_w`, `coarse_h`: sprite dimensions in 8×8 tile units, used to
///   generate the right amount of tile data.
fn setup_sprite_ppu(objsel: u8, oam_high_attr: u8, coarse_w: u32, coarse_h: u32) -> Ppu {
    let mut ppu = Ppu::new();

    // Helper: write a single byte to a PPU register via the bus interface.
    let wr = |ppu: &mut Ppu, offset: u16, value: u8| {
        ppu.write(AddressU24::new(0, offset), value);
    };

    // --- Object size and nametable configuration ---
    wr(&mut ppu, 0x2101, objsel); // OBJSEL

    // --- Write sprite tile data to VRAM ---
    // VMAIN = 0x80: address auto-increments after writing the high byte ($2119).
    wr(&mut ppu, 0x2115, 0x80);
    wr(&mut ppu, 0x2116, 0x00); // VMADDL: address low = 0
    wr(&mut ppu, 0x2117, 0x00); // VMADDH: address high = 0

    for word in make_sprite_tile_data(coarse_w, coarse_h) {
        wr(&mut ppu, 0x2118, (word & 0xFF) as u8); // VMDATAL (low byte, no increment)
        wr(&mut ppu, 0x2119, (word >> 8) as u8); // VMDATAH (high byte, then increment)
    }

    // --- Write sprite palette to CGRAM ---
    // Sprite palette 0 occupies CGRAM indices 128-143.
    wr(&mut ppu, 0x2121, 128); // CGADD: start at index 128
    for &color in &SPRITE_PALETTE {
        wr(&mut ppu, 0x2122, (color & 0xFF) as u8); // CGDATA low byte
        wr(&mut ppu, 0x2122, (color >> 8) as u8); //  CGDATA high byte
    }

    // --- Write sprite 0 to OAM (low table at byte address 0) ---
    wr(&mut ppu, 0x2102, 0x00); // OAMADDL: word address 0 → byte address 0
    wr(&mut ppu, 0x2103, 0x00); // OAMADDH: low OAM table
    wr(&mut ppu, 0x2104, 0x00); // Byte 0: X position = 0
    wr(&mut ppu, 0x2104, 0x00); // Byte 1: Y position = 0
    wr(&mut ppu, 0x2104, 0x00); // Byte 2: tile index = 0
    wr(&mut ppu, 0x2104, 0x30); // Byte 3: priority=3 (bits 4-5), palette=0, no flip

    // --- Write sprite 0 size selector to OAM high table (byte address 0x200) ---
    wr(&mut ppu, 0x2102, 0x00); // OAMADDL: reset address bits 0-8
    wr(&mut ppu, 0x2103, 0x01); // OAMADDH: bit 0=1 → address = 0x200
    wr(&mut ppu, 0x2104, oam_high_attr); // Bit 1 = size selector for sprite 0

    // --- Enable sprites on the main screen ---
    wr(&mut ppu, 0x212C, 0x10); // TM: bit 4 = OBJ (sprites)

    ppu
}

/// Renders all 256 scanlines of `ppu` and compares the framebuffer to a golden.
fn run_sprite_rendering_test(
    test_name: &str,
    objsel: u8,
    oam_high_attr: u8,
    coarse_w: u32,
    coarse_h: u32,
) {
    logging::test_init(true);
    let mut ppu = setup_sprite_ppu(objsel, oam_high_attr, coarse_w, coarse_h);
    for scanline in 0..256 {
        ppu.draw_scanline(scanline);
    }
    compare_to_golden(
        &ppu.framebuffer().to_rgba::<TestImageImpl>(),
        &test_dir().join(test_name),
    );
}

/// 8×8 sprite: OBJSEL size mode 0 selects {8×8, 16×16}; OAM high-table bit 1 = 0
/// picks the primary (8×8) size.
#[test]
fn test_sprite_8x8() {
    run_sprite_rendering_test("sprite-8x8", 0x00, 0x00, 1, 1);
}

/// 16×16 sprite: OBJSEL size mode 0 selects {8×8, 16×16}; OAM high-table bit 1 = 1
/// picks the secondary (16×16) size.
#[test]
fn test_sprite_16x16() {
    run_sprite_rendering_test("sprite-16x16", 0x00, 0x02, 2, 2);
}

/// 32×32 sprite: OBJSEL size mode 5 selects {32×32, 64×64}; OAM high-table bit 1 = 0
/// picks the primary (32×32) size.
#[test]
fn test_sprite_32x32() {
    run_sprite_rendering_test("sprite-32x32", 0xA0, 0x00, 4, 4);
}

/// 64×64 sprite: OBJSEL size mode 5 selects {32×32, 64×64}; OAM high-table bit 1 = 1
/// picks the secondary (64×64) size.
#[test]
fn test_sprite_64x64() {
    run_sprite_rendering_test("sprite-64x64", 0xA0, 0x02, 8, 8);
}

/// 16×32 sprite: OBJSEL size mode 6 selects {16×32, 32×64}; OAM high-table bit 1 = 0
/// picks the primary (16×32) size.
#[test]
fn test_sprite_16x32() {
    run_sprite_rendering_test("sprite-16x32", 0xC0, 0x00, 2, 4);
}

/// 32×64 sprite: OBJSEL size mode 6 selects {16×32, 32×64}; OAM high-table bit 1 = 1
/// picks the secondary (32×64) size.
#[test]
fn test_sprite_32x64() {
    run_sprite_rendering_test("sprite-32x64", 0xC0, 0x02, 4, 8);
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
