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

/// Tests sprite rendering by directly programming the PPU (no ROM required).
///
/// Exercises the following configurations in two rows of sprites:
///
/// Row 1 (Y=8):
///   Sprite 0: basic 8×8, palette 0
///   Sprite 1: horizontal flip
///   Sprite 2: vertical flip
///   Sprite 3: H+V flip
///   Sprite 4: large (16×16), composed of four distinct solid-colour tiles
///
/// Row 2 (Y=32):
///   Sprite 5: palette 1
///   Sprite 6: palette 2
///   Sprite 7: palette 3
///   Sprites 8+9: priority overlap (priority-0 sprite partially covered by priority-3)
///   Sprite 10: tile from nametable 1 (second sprite table)
#[test]
pub fn test_sprites() {
    logging::test_init(true);
    let mut ppu = Ppu::new();
    setup_sprite_test(&mut ppu);
    for scanline in 0..224 {
        ppu.draw_scanline(scanline);
    }
    compare_to_golden(
        &ppu.framebuffer().to_rgba::<TestImageImpl>(),
        &test_dir().join("sprites"),
    );
}

/// Writes a single byte to a PPU register.
fn ppu_write(ppu: &mut Ppu, reg: u16, value: u8) {
    ppu.write(AddressU24::new(0, reg), value);
}

/// Writes a complete 4bpp 8×8 tile to VRAM.
///
/// `word_addr` is the VRAM word address for tile row 0.
/// `planes[row]` = [plane0, plane1, plane2, plane3] byte values for that row.
///
/// Requires VMAIN = 0x80 (increment after writing the high byte) to already be set.
///
/// SNES 4bpp layout: 8 words of (plane0, plane1) at `word_addr + 0..7`, followed
/// by 8 words of (plane2, plane3) at `word_addr + 8..15`.
/// Bit 7 of each plane byte is the leftmost pixel; bit 0 is the rightmost.
fn write_tile_4bpp(ppu: &mut Ppu, word_addr: u16, planes: [[u8; 4]; 8]) {
    // Low bit-planes (0 & 1), rows 0-7
    ppu_write(ppu, 0x2116, (word_addr & 0xFF) as u8);
    ppu_write(ppu, 0x2117, (word_addr >> 8) as u8);
    for row in &planes {
        ppu_write(ppu, 0x2118, row[0]); // plane 0
        ppu_write(ppu, 0x2119, row[1]); // plane 1 – address increments after this
    }
    // High bit-planes (2 & 3) sit 8 words later in VRAM
    let hi_addr = word_addr + 8;
    ppu_write(ppu, 0x2116, (hi_addr & 0xFF) as u8);
    ppu_write(ppu, 0x2117, (hi_addr >> 8) as u8);
    for row in &planes {
        ppu_write(ppu, 0x2118, row[2]); // plane 2
        ppu_write(ppu, 0x2119, row[3]); // plane 3
    }
}

/// Returns a tile where every pixel has the given 4bpp colour index (1-15).
fn solid_tile(color: u8) -> [[u8; 4]; 8] {
    let row = [
        if color & 1 != 0 { 0xFF } else { 0x00 },
        if color & 2 != 0 { 0xFF } else { 0x00 },
        if color & 4 != 0 { 0xFF } else { 0x00 },
        if color & 8 != 0 { 0xFF } else { 0x00 },
    ];
    [row; 8]
}

/// Programs the PPU with sprite data for `test_sprites`.
///
/// Register formats are as documented by the SNES NESdev Wiki:
/// - OAM 4-byte sprite format: <https://snes.nesdev.org/wiki/Sprites>
/// - OBJSEL ($2101): <https://snes.nesdev.org/wiki/PPU_registers>
/// - Priority ordering (3=front … 0=back): <https://snes.nesdev.org/wiki/Sprites>
/// - 4bpp tile bit-plane layout (bit 7 = leftmost pixel):
///   <https://snes.nesdev.org/wiki/Tiles>
fn setup_sprite_test(ppu: &mut Ppu) {
    // Enable OBJ on the main screen (TM bit 4).
    ppu_write(ppu, 0x212C, 0x10);

    // OBJSEL ($2101): small = 8×8, large = 16×16.
    // Bits 5-7 (SSS) = 0 → 8×8 / 16×16 size pair.
    // Bits 3-4 (NN)  = 0 → name-select offset = (0+1) × 0x1000 = word 0x1000.
    // Bits 0-1 (bBB) = 0 → nametable 0 base = word 0x0000.
    // Therefore nametable 1 starts at word 0x0000 + 0x1000 = 0x1000.
    ppu_write(ppu, 0x2101, 0x00);

    // -----------------------------------------------------------------
    // CGRAM – four sprite palettes (CGRAM indices 128-191)
    // SNES colour format: 0b.BBBBB_GGGGG_RRRRR (15-bit BGR, little-endian)
    // -----------------------------------------------------------------

    // Backdrop (index 0): black
    ppu_write(ppu, 0x2121, 0);
    ppu_write(ppu, 0x2122, 0x00);
    ppu_write(ppu, 0x2122, 0x00);

    // Palette 0 (indices 128-132): transparent, red, green, blue, yellow
    ppu_write(ppu, 0x2121, 128);
    for (lo, hi) in [
        (0x00u8, 0x00u8), // colour 0: transparent
        (0x1F, 0x00),     // colour 1: red   (R=31)
        (0xE0, 0x03),     // colour 2: green (G=31)
        (0x00, 0x7C),     // colour 3: blue  (B=31)
        (0xFF, 0x03),     // colour 4: yellow (R=31, G=31)
    ] {
        ppu_write(ppu, 0x2122, lo);
        ppu_write(ppu, 0x2122, hi);
    }

    // Palette 1 (indices 144-148): transparent, cyan, magenta, white, gray
    ppu_write(ppu, 0x2121, 144);
    for (lo, hi) in [
        (0x00u8, 0x00u8), // colour 0: transparent
        (0xE0, 0x7F),     // colour 1: cyan    (G=31, B=31)
        (0x1F, 0x7C),     // colour 2: magenta (R=31, B=31)
        (0xFF, 0x7F),     // colour 3: white   (R=31, G=31, B=31)
        (0x10, 0x42),     // colour 4: gray    (R=16, G=16, B=16)
    ] {
        ppu_write(ppu, 0x2122, lo);
        ppu_write(ppu, 0x2122, hi);
    }

    // Palette 2 (indices 160-161): colour 0 = transparent, colour 1 = orange (R=31, G=16)
    ppu_write(ppu, 0x2121, 160);
    for (lo, hi) in [(0x00u8, 0x00u8), (0x1F, 0x02)] {
        ppu_write(ppu, 0x2122, lo);
        ppu_write(ppu, 0x2122, hi);
    }

    // Palette 3 (indices 176-177): colour 0 = transparent, colour 1 = pink (R=31, B=16)
    ppu_write(ppu, 0x2121, 176);
    for (lo, hi) in [(0x00u8, 0x00u8), (0x1F, 0x40)] {
        ppu_write(ppu, 0x2122, lo);
        ppu_write(ppu, 0x2122, hi);
    }

    // -----------------------------------------------------------------
    // VRAM – tile graphics
    // -----------------------------------------------------------------

    // VMAIN = 0x80: increment VRAM address after writing the high byte.
    ppu_write(ppu, 0x2115, 0x80);

    // Tile 0 at nametable 0 (word address 0x0000): asymmetric 4-quadrant pattern.
    //
    //   Top-left  4×4 px: colour 1   Top-right  4×4 px: colour 2
    //   Bot-left  4×4 px: colour 3   Bot-right  4×4 px: colour 4
    //
    // Bit 7 = leftmost pixel; bits 7-4 = left half (0xF0), bits 3-0 = right half (0x0F).
    //
    // Top rows: left = colour 1 (0b0001), right = colour 2 (0b0010)
    //   plane0: left bits set   → 0xF0
    //   plane1: right bits set  → 0x0F
    //   plane2/3: zero
    //
    // Bottom rows: left = colour 3 (0b0011), right = colour 4 (0b0100)
    //   plane0: left bits set   → 0xF0
    //   plane1: left bits set   → 0xF0
    //   plane2: right bits set  → 0x0F
    //   plane3: zero
    let top_row = [0xF0u8, 0x0F, 0x00, 0x00];
    let bot_row = [0xF0u8, 0xF0, 0x0F, 0x00];
    write_tile_4bpp(
        ppu,
        0x0000,
        [
            top_row, top_row, top_row, top_row, bot_row, bot_row, bot_row, bot_row,
        ],
    );

    // For the 16×16 large sprite (sprite 4, tile index 0):
    // The SNES engine maps a 16×16 sprite starting at tile T as:
    //   tile T    (top-left),   tile T+1  (top-right)
    //   tile T+16 (bot-left),   tile T+17 (bot-right)
    write_tile_4bpp(ppu, 0x0010, solid_tile(2)); // tile 1  → solid green
    write_tile_4bpp(ppu, 0x0100, solid_tile(3)); // tile 16 → solid blue
    write_tile_4bpp(ppu, 0x0110, solid_tile(4)); // tile 17 → solid yellow

    // Nametable 1, tile 0 (word address 0x1000): solid colour 1.
    // Used by sprite 10 to verify nametable selection.
    write_tile_4bpp(ppu, 0x1000, solid_tile(1));

    // -----------------------------------------------------------------
    // OAM – sprite attributes
    // -----------------------------------------------------------------

    // OAM attribute byte format (byte 3, VHPP CCCt per the SNES NESdev Wiki):
    //   vflip[7] | hflip[6] | priority[5:4] | palette[3:1] | name-select[0]
    // Priority: 3 = highest (front), 0 = lowest (back).

    // Initialise all 128 sprites to off-screen (Y=240) so unused slots are hidden.
    ppu_write(ppu, 0x2102, 0x00); // OAMADDL
    ppu_write(ppu, 0x2103, 0x00); // OAMADDH
    for _ in 0..128 {
        ppu_write(ppu, 0x2104, 0); // X
        ppu_write(ppu, 0x2104, 240); // Y = 240 (off-screen for 224-line display)
        ppu_write(ppu, 0x2104, 0); // tile
        ppu_write(ppu, 0x2104, 0); // attributes
    }

    // Reset OAM write address and write the test sprites.
    ppu_write(ppu, 0x2102, 0x00);
    ppu_write(ppu, 0x2103, 0x00);

    // -- Row 1 (Y=8): flip and size tests --------------------------------

    // Sprite 0: basic 8×8, palette 0, priority 3, no flip, at (8, 8)
    ppu_write(ppu, 0x2104, 8);
    ppu_write(ppu, 0x2104, 8);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0x30); // pri=3, pal=0

    // Sprite 1: horizontal flip, at (24, 8)
    ppu_write(ppu, 0x2104, 24);
    ppu_write(ppu, 0x2104, 8);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0x70); // hflip=1, pri=3, pal=0

    // Sprite 2: vertical flip, at (40, 8)
    ppu_write(ppu, 0x2104, 40);
    ppu_write(ppu, 0x2104, 8);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0xB0); // vflip=1, pri=3, pal=0

    // Sprite 3: both flips, at (56, 8)
    ppu_write(ppu, 0x2104, 56);
    ppu_write(ppu, 0x2104, 8);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0xF0); // hflip=vflip=1, pri=3, pal=0

    // Sprite 4: large (16×16), tile 0, palette 0, priority 3, at (80, 8)
    // Top-left tile = tile 0 (quadrant pattern), top-right = tile 1 (green),
    // bottom-left = tile 16 (blue), bottom-right = tile 17 (yellow).
    ppu_write(ppu, 0x2104, 80);
    ppu_write(ppu, 0x2104, 8);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0x30); // pri=3, pal=0

    // -- Row 2 (Y=32): palette, priority, and nametable tests -----------

    // Sprite 5: palette 1, at (8, 32)
    ppu_write(ppu, 0x2104, 8);
    ppu_write(ppu, 0x2104, 32);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0x32); // pri=3, pal=1

    // Sprite 6: palette 2, at (24, 32)
    ppu_write(ppu, 0x2104, 24);
    ppu_write(ppu, 0x2104, 32);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0x34); // pri=3, pal=2

    // Sprite 7: palette 3, at (40, 32)
    ppu_write(ppu, 0x2104, 40);
    ppu_write(ppu, 0x2104, 32);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0x36); // pri=3, pal=3

    // Priority overlap: sprite 8 (priority 0) is partially behind sprite 9 (priority 3).
    // Per the SNES NESdev Wiki, when sprites with different priority levels overlap the one
    // with the higher priority value (3 = front, 0 = back) wins, regardless of OAM index.

    // Sprite 8: priority 0, palette 0, at (56, 32)
    ppu_write(ppu, 0x2104, 56);
    ppu_write(ppu, 0x2104, 32);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0x00); // pri=0, pal=0

    // Sprite 9: priority 3, palette 1, at (60, 32) – overlaps sprite 8 by 4 pixels
    ppu_write(ppu, 0x2104, 60);
    ppu_write(ppu, 0x2104, 32);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0x32); // pri=3, pal=1

    // Sprite 10: nametable 1 tile 0 (solid colour 1), palette 0, priority 3, at (80, 32).
    // Tile data is at VRAM word 0x1000; nametable 0 tile 0 is at 0x0000 (quadrant pattern)
    // so the two will look different, confirming correct nametable selection.
    ppu_write(ppu, 0x2104, 80);
    ppu_write(ppu, 0x2104, 32);
    ppu_write(ppu, 0x2104, 0);
    ppu_write(ppu, 0x2104, 0x31); // pri=3, pal=0, nametable=1

    // OAM high table (1 byte per 4 sprites; bits: size3,x8_3, size2,x8_2, size1,x8_1, size0,x8_0)
    // Only sprite 4 needs the large-size bit set (high-table byte 1, bit 1).
    ppu_write(ppu, 0x2102, 0x00);
    ppu_write(ppu, 0x2103, 0x01); // select high table (OAM address 0x200)
    ppu_write(ppu, 0x2104, 0x00); // sprites 0-3: all small
    ppu_write(ppu, 0x2104, 0x02); // sprites 4-7: sprite 4 large (bit 1), rest small
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

    // Debug render VRAM
    let vram_bg0_path = test_dir().join("krom_interlace_rpg-vram-bg1");
    compare_to_golden(
        &ppu.render_vram(VramRenderSelection::Background(BackgroundId::BG1)),
        &vram_bg0_path,
    );
    let vram_sprite_path = test_dir().join("krom_interlace_rpg-vram-sprite");
    compare_to_golden(
        &ppu.render_vram(VramRenderSelection::Sprite0),
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
