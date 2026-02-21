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

/// Tests sprite rendering with one of each major sprite configuration:
/// - 8x8 basic sprite
/// - 8x8 sprite with a different palette
/// - 8x8 sprite with horizontal flip
/// - 8x8 sprite with vertical flip
/// - 16x16 large sprite
/// - 8x8 sprite from the second sprite nametable
/// - 8x8 sprite with a non-default priority
#[test]
pub fn test_sprite_rendering() {
    logging::test_init(true);
    let mut ppu = Ppu::new();

    // Enable sprites on main screen (TM bit 4 = OBJ)
    ppu_write(&mut ppu, 0x212C, 0x10);

    // OBJSEL: 8x8 / 16x16 sizes, nametable 0 at word 0x0000, nametable 1 at word 0x1000
    ppu_write(&mut ppu, 0x2101, 0x00);

    // === VRAM: Write sprite tile data ===

    // Tile 0 (word addr 0x0000): diagonal pattern – clearly distinct under all flip combinations
    let diagonal_tile = encode_4bpp_tile(&[
        [1, 1, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0, 0, 0, 0],
        [0, 0, 1, 1, 1, 0, 0, 0],
        [0, 0, 0, 1, 1, 1, 0, 0],
        [0, 0, 0, 0, 1, 1, 1, 0],
        [0, 0, 0, 0, 0, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 1, 1],
    ]);
    write_vram_seq(&mut ppu, 0x0000, &diagonal_tile);

    // Tile 1 (word addr 0x0010): solid color 2 – 16x16 sprite top-right quadrant
    write_vram_seq(&mut ppu, 0x0010, &encode_4bpp_tile(&[[2u8; 8]; 8]));

    // Tile 16 (word addr 0x0100): solid color 3 – 16x16 sprite bottom-left quadrant
    write_vram_seq(&mut ppu, 0x0100, &encode_4bpp_tile(&[[3u8; 8]; 8]));

    // Tile 17 (word addr 0x0110): solid color 4 – 16x16 sprite bottom-right quadrant
    write_vram_seq(&mut ppu, 0x0110, &encode_4bpp_tile(&[[4u8; 8]; 8]));

    // NT1 tile 0 (word addr 0x1000): same diagonal pattern, rendered with palette 2 (cyan)
    write_vram_seq(&mut ppu, 0x1000, &diagonal_tile);

    // === CGRAM: Write sprite palette data ===
    // Sprite palettes start at CGRAM index 128 (palette 0 = indices 128–143, etc.)
    ppu_write(&mut ppu, 0x2121, 128); // CGADD

    // Palette 0: color 1=red, 2=green, 3=blue, 4=yellow
    write_cgdata_word(&mut ppu, 0x0000); // [128] color 0: transparent
    write_cgdata_word(&mut ppu, 0x001F); // [129] color 1: red   (R=31)
    write_cgdata_word(&mut ppu, 0x03E0); // [130] color 2: green (G=31)
    write_cgdata_word(&mut ppu, 0x7C00); // [131] color 3: blue  (B=31)
    write_cgdata_word(&mut ppu, 0x03FF); // [132] color 4: yellow (R=31,G=31)
    for _ in 5..16 {
        write_cgdata_word(&mut ppu, 0x0000);
    }

    // Palette 1: color 1=white
    write_cgdata_word(&mut ppu, 0x0000); // [144] color 0: transparent
    write_cgdata_word(&mut ppu, 0x7FFF); // [145] color 1: white
    for _ in 2..16 {
        write_cgdata_word(&mut ppu, 0x0000);
    }

    // Palette 2: color 1=cyan
    write_cgdata_word(&mut ppu, 0x0000); // [160] color 0: transparent
    write_cgdata_word(&mut ppu, 0x7FE0); // [161] color 1: cyan (G=31,B=31)
    for _ in 2..16 {
        write_cgdata_word(&mut ppu, 0x0000);
    }

    // === OAM: Write sprite attributes ===
    ppu_write(&mut ppu, 0x2102, 0x00); // OAMADDL = 0
    ppu_write(&mut ppu, 0x2103, 0x00); // OAMADDH = 0

    // Sprite 0: basic 8x8 (palette 0 = red L-shape), x=8, y=10
    oam_write_sprite(&mut ppu, 8, 10, 0, 0x00);

    // Sprite 1: palette 1 (white L-shape), x=28, y=10
    // attr bits 1-3 = palette → 0b000_0_010 = 0x02
    oam_write_sprite(&mut ppu, 28, 10, 0, 0x02);

    // Sprite 2: horizontal flip, x=48, y=10
    // attr bit 6 = hflip → 0x40
    oam_write_sprite(&mut ppu, 48, 10, 0, 0x40);

    // Sprite 3: vertical flip, x=68, y=10
    // attr bit 7 = vflip → 0x80
    oam_write_sprite(&mut ppu, 68, 10, 0, 0x80);

    // Sprite 4: 16x16 large (size toggle set via high table), x=88, y=10
    oam_write_sprite(&mut ppu, 88, 10, 0, 0x00);

    // Sprite 5: nametable 1, palette 2 (cyan L-shape), x=128, y=10
    // attr bit 0 = nametable select, bits 1-3 = palette 2 → 0b000_0_101 = 0x05
    oam_write_sprite(&mut ppu, 128, 10, 0, 0x05);

    // Sprite 6: priority 2, x=150, y=10
    // attr bits 4-5 = priority → 0b0010_0000 = 0x20
    oam_write_sprite(&mut ppu, 150, 10, 0, 0x20);

    // Sprites 7–127: hidden off-screen at y=224
    for _ in 7..128 {
        oam_write_sprite(&mut ppu, 0, 224, 0, 0x00);
    }

    // OAM high table (32 bytes at OAM address 0x200).
    // Sprite 4 (id=4): high table byte at 0x201, bit 1 = size toggle → large (16x16)
    ppu_write(&mut ppu, 0x2102, 0x00); // OAMADDL = 0
    ppu_write(&mut ppu, 0x2103, 0x01); // OAMADDH = 1  →  addr = 0x200
    ppu_write(&mut ppu, 0x2104, 0x00); // byte 0 (sprites 0-3): all small, X-high=0
    ppu_write(&mut ppu, 0x2104, 0x02); // byte 1 (sprites 4-7): sprite 4 large (bit 1)
    for _ in 2..32 {
        ppu_write(&mut ppu, 0x2104, 0x00);
    }

    // === Render all scanlines ===
    for scanline in 0..256 {
        ppu.draw_scanline(scanline);
    }

    compare_to_golden(
        &ppu.framebuffer().to_rgba::<TestImageImpl>(),
        &test_dir().join("sprite_rendering"),
    );
}

// ---------------------------------------------------------------------------
// PPU programming helpers used by test_sprite_rendering
// ---------------------------------------------------------------------------

/// Write a single byte to a PPU register via the bus interface.
fn ppu_write(ppu: &mut Ppu, reg: u16, value: u8) {
    ppu.write(AddressU24 { bank: 0, offset: reg }, value);
}

/// Write a sequence of 16-bit words to VRAM starting at `start_word_addr`.
/// Configures VMAIN to increment after VMDATAH (0x2119) writes.
fn write_vram_seq(ppu: &mut Ppu, start_word_addr: u16, words: &[u16]) {
    ppu_write(ppu, 0x2115, 0x80); // VMAIN: increment after VMDATAH
    ppu_write(ppu, 0x2116, (start_word_addr & 0xFF) as u8); // VMADDL
    ppu_write(ppu, 0x2117, (start_word_addr >> 8) as u8); // VMADDH
    for &word in words {
        ppu_write(ppu, 0x2118, word as u8); // VMDATAL (low byte, no increment)
        ppu_write(ppu, 0x2119, (word >> 8) as u8); // VMDATAH (high byte, increments addr)
    }
}

/// Encode an 8×8 tile from pixel color indices (0–15) into 16 VRAM words (4bpp format).
///
/// SNES 4bpp tile layout: 8 words for planes 0+1, then 8 words for planes 2+3.
/// Within each word the MSB holds the leftmost pixel's plane bit.
///
/// For each pixel `px` at column `i`, bit `(7-i)` of each plane byte carries the
/// corresponding bit of `px`: `plane0[bit] = px & 1`, `plane1[bit] = (px>>1) & 1`, etc.
/// Each word is stored as `[plane_low_byte, plane_high_byte]` in little-endian order.
fn encode_4bpp_tile(rows: &[[u8; 8]; 8]) -> Vec<u16> {
    let mut words = Vec::with_capacity(16);
    // Planes 0+1 (words 0–7)
    for row in rows {
        let (mut p0, mut p1) = (0u8, 0u8);
        for (i, &px) in row.iter().enumerate() {
            p0 |= ((px & 1) << (7 - i)) as u8;
            p1 |= (((px >> 1) & 1) << (7 - i)) as u8;
        }
        words.push(u16::from_le_bytes([p0, p1]));
    }
    // Planes 2+3 (words 8–15)
    for row in rows {
        let (mut p2, mut p3) = (0u8, 0u8);
        for (i, &px) in row.iter().enumerate() {
            p2 |= (((px >> 2) & 1) << (7 - i)) as u8;
            p3 |= (((px >> 3) & 1) << (7 - i)) as u8;
        }
        words.push(u16::from_le_bytes([p2, p3]));
    }
    words
}

/// Write a 15-bit BGR color to CGRAM (two consecutive CGDATA writes).
fn write_cgdata_word(ppu: &mut Ppu, color: u16) {
    ppu_write(ppu, 0x2122, color as u8);
    ppu_write(ppu, 0x2122, (color >> 8) as u8);
}

/// Write a single sprite entry to OAM via four sequential OAMDATA writes.
fn oam_write_sprite(ppu: &mut Ppu, x: u8, y: u8, tile: u8, attr: u8) {
    ppu_write(ppu, 0x2104, x); // X low  (latched at even address)
    ppu_write(ppu, 0x2104, y); // Y      (flushes [X,Y] at odd address)
    ppu_write(ppu, 0x2104, tile); // Tile (latched at even address)
    ppu_write(ppu, 0x2104, attr); // Attr (flushes [tile,attr] at odd address)
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
