use std::path::Path;
use std::path::PathBuf;

use image::RgbaImage;
use serde::Deserialize;
use serde::Serialize;
use sres_emulator::ppu::oam::SpriteSize;
use sres_emulator::ppu::Background;
use sres_emulator::ppu::BackgroundId;
use sres_emulator::ppu::BitDepth;
use sres_emulator::ppu::Ppu;
use sres_emulator::ppu::VramAddr;
use sres_emulator::util::image::Image;
use sres_emulator::util::image::Rgb15;
use sres_emulator::util::image::Rgba32;
use sres_emulator::util::logging;
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
pub fn test_krom_interlace_rpg() {
    // Note: Interlacing or high-res is not implemented and used by this test rom.
    // However it's the only test rom I have available to test sprite rendering.
    run_framebuffer_test("krom_interlace_rpg", 10);
}

/// Renders the framebuffer at `frame` and compares against previously stored golden image.
fn run_framebuffer_test(test_name: &str, frame: u32) -> System {
    logging::test_init(true);

    let rom_path = test_dir().join(format!("{test_name}.sfc"));
    let system = emulate_rom_until_frame(&rom_path, frame);
    let framebuffer_path = test_dir().join(format!("{test_name}-framebuffer"));
    compare_to_golden(
        &system.cpu.bus.ppu.get_rgba_framebuffer::<TestImageImpl>(),
        &framebuffer_path,
    );
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
    let system = emulate_rom_until_frame(&rom_path, 10);

    let ppu = system.cpu.bus.ppu;

    // Debug render sprite 0
    let sprite_path = test_dir().join("krom_interlace_rpg-sprite0");
    compare_to_golden(&ppu.debug_render_sprite(0), &sprite_path);

    // Debug render BG0
    let background_path = test_dir().join("krom_interlace_rpg-bg0");
    compare_to_golden(
        &ppu.debug_render_background(BackgroundId::BG0),
        &background_path,
    );

    // Debug render portion of VRAM
    let vram_path = test_dir().join("krom_interlace_rpg-vram");
    compare_to_golden(
        &ppu.debug_render_vram(VramAddr(0), 32, BitDepth::Bpp4, 0),
        &vram_path,
    );
}

#[test]
#[ignore = "Only run on demand to generate snapshot"]
fn smw_generate_snapshots() {
    generate_snapshot("smw", "titlescreen", 300);
}

#[test]
fn test_smw_titlescreen() {
    run_snapshot_framebuffer_test("smw", "titlescreen");
}

/// Renders the framebuffer of the previously generated PPU snapshot and compares against previously
/// stored golden image.
fn run_snapshot_framebuffer_test(test_name: &str, frame_name: &str) {
    let snapshot_path = test_dir().join(format!("{test_name}-{frame_name}.snapshot"));
    let mut ppu = PpuSnapshot::read_from_file(&snapshot_path).restore();
    for scanline in 0..256 {
        ppu.draw_scanline(scanline);
    }
    let framebuffer_path = test_dir().join(format!("{test_name}-{frame_name}"));
    compare_to_golden(
        &ppu.get_rgba_framebuffer::<TestImageImpl>(),
        &framebuffer_path,
    );
}

/// Runs the test rom until the given frame and stores a snapshot to be tested with
/// run_snapshot_framebuffer_test.
fn generate_snapshot(test_name: &str, frame_name: &str, frame: u32) {
    logging::test_init(true);

    let rom_path = test_dir().join(format!("{test_name}.sfc"));
    let system = emulate_rom_until_frame(&rom_path, frame);
    let snapshot_path = test_dir().join(format!("{test_name}-{frame_name}.snapshot"));
    PpuSnapshot::snapshot(&system.cpu.bus.ppu).write_to_file(&snapshot_path);
}

fn test_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/ppu_tests")
}

fn emulate_rom_until_frame(rom_file: &Path, frame: u32) -> System {
    let mut system = System::with_sfc(&rom_file).unwrap();
    system.cpu.reset();
    for _ in 0..frame {
        system.execute_one_frame();
    }
    system
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

/// Snapshot of PPU data. Can be saved from a running emulator session and then restored later
/// for testing PPU rendering in isolation of other emulator components.
#[derive(Serialize, Deserialize)]
struct PpuSnapshot {
    vram: Vec<u16>,
    cgram: Vec<Rgb15>,
    backgrounds: [Background; 4],
    oam: Vec<u8>,
    sprite_sizes: (SpriteSize, SpriteSize),
    nametables: (VramAddr, VramAddr),
}

impl PpuSnapshot {
    pub fn snapshot(ppu: &Ppu) -> Self {
        PpuSnapshot {
            vram: ppu.vram.memory.clone(),
            cgram: ppu.cgram.memory.clone(),
            backgrounds: [
                ppu.backgrounds[0].clone(),
                ppu.backgrounds[1].clone(),
                ppu.backgrounds[2].clone(),
                ppu.backgrounds[3].clone(),
            ],
            oam: ppu.oam.memory.clone(),
            sprite_sizes: ppu.oam.sprite_sizes,
            nametables: ppu.oam.nametables,
        }
    }

    pub fn write_to_file(&self, path: &Path) {
        bincode::serialize_into(std::fs::File::create(path).unwrap(), self).unwrap();
    }

    pub fn restore(self) -> Ppu {
        let mut ppu = Ppu::new();
        ppu.vram.memory = self.vram;
        ppu.cgram.memory = self.cgram;
        ppu.backgrounds[0] = self.backgrounds[0];
        ppu.backgrounds[1] = self.backgrounds[1];
        ppu.backgrounds[2] = self.backgrounds[2];
        ppu.backgrounds[3] = self.backgrounds[3];
        ppu.oam.memory = self.oam;
        ppu.oam.sprite_sizes = self.sprite_sizes;
        ppu.oam.nametables = self.nametables;
        ppu
    }

    pub fn read_from_file(path: &Path) -> Self {
        bincode::deserialize_from(std::fs::File::open(path).unwrap()).unwrap()
    }
}
