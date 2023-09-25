use std::path::Path;
use std::path::PathBuf;

use image::RgbaImage;
use sres_emulator::util::image::Image;
use sres_emulator::util::image::Rgba32;
use sres_emulator::util::logging;
use sres_emulator::System;

#[test]
pub fn test_krom_hdma_redspace() {
    run_ppu_test("krom_hdma_redspace", &[10]);
}

#[test]
pub fn test_krom_rings() {
    run_ppu_test("krom_rings", &[10]);
}

#[test]
pub fn test_krom_hello_world() {
    run_ppu_test("krom_hello_world", &[10]);
}

#[test]
pub fn test_krom_bgmap_2bpp() {
    run_ppu_test("krom_bgmap_2bpp", &[10]);
}

#[test]
pub fn test_krom_bgmap_4bpp() {
    run_ppu_test("krom_bgmap_4bpp", &[10]);
}

#[test]
pub fn test_krom_interlace_rpg() {
    run_ppu_test("krom_interlace_rpg", &[10]);
}

fn run_ppu_test(test_name: &str, snapshot_frames: &[u32]) {
    logging::test_init(true);

    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let rom_path = root_dir.join(format!("tests/ppu_tests/{test_name}.sfc"));

    let mut system = System::with_sfc(&rom_path).unwrap();
    system.cpu.reset();
    for frame_count in snapshot_frames {
        for _ in 0..*frame_count {
            system.execute_one_frame();
        }
        let framebuffer_path = root_dir.join(format!("tests/ppu_tests/{test_name}-{frame_count}"));
        compare_to_golden(
            &system.cpu.bus.ppu.get_rgba_framebuffer::<TestImageImpl>(),
            &framebuffer_path,
        );
    }
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
