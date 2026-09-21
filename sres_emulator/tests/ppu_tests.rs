//! Golden-image PPU tests (`System`): ROM framebuffer, PPU `.snapshot` plus `.writes`, and debug renders.
//! Snapshot tests `load_state`, replay mid-frame `$2100–$213F` writes, then `draw_scanline` with no ROM.
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use image::RgbaImage;
use sres_emulator::common::address::AddressU24;
use sres_emulator::common::bus::BusDeviceU24;
use sres_emulator::common::clock::ClockInfo;
use sres_emulator::common::image::Image;
use sres_emulator::common::image::Rgba32;
use sres_emulator::common::logging;
use sres_emulator::components::cartridge::Cartridge;
use sres_emulator::components::ppu::BackgroundId;
use sres_emulator::components::ppu::Framebuffer;
use sres_emulator::components::ppu::Ppu;
use sres_emulator::components::ppu::VramRenderSelection;
use sres_emulator::debugger::DebugEvent;
use sres_emulator::debugger::EventFilter;
use sres_emulator::main_bus::MainBusEvent;
use sres_emulator::ExecutionResult;
use sres_emulator::System;

#[test]
pub fn test_krom_hdma_redspace() {
    run_framebuffer_test("krom_hdma_redspace", 10);
}

#[test]
pub fn test_krom_hdma_redspace_snapshot_replay() {
    logging::test_init(true);

    let rom_path = test_dir().join("krom_hdma_redspace.sfc");
    let mut system = System::with_cartridge(&Cartridge::with_sfc_file(&rom_path).unwrap());
    // `run_framebuffer_test(..., 10)` swaps the vblank of frame 9. Capture that visible frame.
    system.execute_frames(9);
    execute_until_v0(&mut system);
    let state = system.save_ppu_state();
    let writes = capture_visible_ppu_writes(&mut system);

    let mut ppu = Ppu::new();
    ppu.load_state(&state).unwrap();
    replay_ppu_writes(&mut ppu, &writes);

    compare_to_golden(
        &ppu.framebuffer().to_rgba::<TestImageImpl>(),
        &test_dir().join("krom_hdma_redspace-framebuffer"),
    );
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

    let rom_path = test_dir().join("colourmath.sfc");
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

#[test]
#[ignore = "only run when snapshots need updating"]
fn generate_dkc_ppu_snapshots() {
    generate_ppu_snapshots("dkc", &[("jungle", 1800)]);
}

#[test]
fn test_dkc_jungle() {
    // Title-attract jungle. Missing HDMA freezes sky/mountain colors; not in-game Jungle Hijinxs.
    run_snapshot_framebuffer_test("dkc-jungle");
}

/// Loads the PPU memory / state from a snapshot file and compares the framebuffer rendering to
/// a previously stored golden image.
/// Mid-frame `$2100–$213F` writes from `{name}.writes` (if present) are replayed after each scanline.
/// The snapshot files are generated by `generate_ppu_snapshots`.
fn run_snapshot_framebuffer_test(snapshot_name: &str) {
    logging::test_init(true);

    let mut ppu = Ppu::new();
    ppu.load_state(&std::fs::read(test_dir().join(format!("{snapshot_name}.snapshot"))).unwrap())
        .unwrap();
    replay_ppu_writes(&mut ppu, &load_ppu_writes(snapshot_name));

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
    system.force_headless();

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
            execute_until_v0(&mut system);
            std::fs::write(
                test_dir().join(format!("{rom_name}-{test_name}.snapshot")),
                system.save_ppu_state(),
            )
            .unwrap();
            std::fs::write(
                test_dir().join(format!("{rom_name}-{test_name}.writes")),
                encode_ppu_writes(&capture_visible_ppu_writes(&mut system)),
            )
            .unwrap();
        }
    }
}

#[derive(Clone, Copy)]
struct PpuBusWrite {
    master_clock: u64,
    offset: u16,
    value: u8,
}

fn execute_until_v0(system: &mut System) {
    while system.clock_info().v != 0 {
        if matches!(system.execute_one_instruction(), ExecutionResult::Halt) {
            panic!("CPU halted before reaching v=0");
        }
    }
}

fn enable_ppu_write_log(system: &mut System) {
    let mut debugger = system.debugger();
    debugger.enable();
    debugger.clear_log_points();
    debugger.add_log_point(EventFilter::CpuMemoryWrite(0x2100..0x2140));
}

fn disable_ppu_write_log(system: &mut System) {
    let mut debugger = system.debugger();
    debugger.clear_log_points();
    debugger.disable();
}

fn capture_visible_ppu_writes(system: &mut System) -> Vec<PpuBusWrite> {
    enable_ppu_write_log(system);
    let start_f = system.clock_info().f;
    let mut writes = Vec::new();
    while system.clock_info().f == start_f && system.clock_info().v < 224 {
        system.execute_scanlines(1);
        let batch = drain_ppu_writes(system);
        assert!(
            batch.len() < sres_emulator::debugger::LOG_BUFFER_SIZE,
            "PPU write log filled the debugger ring; oldest HDMA writes were dropped"
        );
        writes.extend(batch);
    }
    disable_ppu_write_log(system);
    writes
}

fn drain_ppu_writes(system: &mut System) -> Vec<PpuBusWrite> {
    let mut writes = system.debugger().drain_events(|event| match event {
        DebugEvent::MainBus(MainBusEvent::Write(addr, value, master_clock)) => Some(PpuBusWrite {
            master_clock: *master_clock,
            offset: addr.offset,
            value: *value,
        }),
        _ => None,
    });
    // Ring buffer is newest-first.
    writes.reverse();
    writes.retain(|write| ClockInfo::from_master_clock(write.master_clock).v < 225);
    writes
}

const PPU_WRITE_RECORD_LEN: usize = 11;

fn encode_ppu_writes(writes: &[PpuBusWrite]) -> Vec<u8> {
    let mut out = Vec::with_capacity(writes.len() * PPU_WRITE_RECORD_LEN);
    for write in writes {
        out.extend_from_slice(&write.master_clock.to_le_bytes());
        out.extend_from_slice(&write.offset.to_le_bytes());
        out.push(write.value);
    }
    out
}

fn decode_ppu_writes(bytes: &[u8]) -> Vec<PpuBusWrite> {
    let (chunks, remainder) = bytes.as_chunks::<PPU_WRITE_RECORD_LEN>();
    assert!(remainder.is_empty(), "truncated PPU write log");
    chunks
        .iter()
        .map(|chunk| PpuBusWrite {
            master_clock: u64::from_le_bytes(chunk[0..8].try_into().unwrap()),
            offset: u16::from_le_bytes(chunk[8..10].try_into().unwrap()),
            value: chunk[10],
        })
        .collect()
}

fn load_ppu_writes(snapshot_name: &str) -> Vec<PpuBusWrite> {
    let path = test_dir().join(format!("{snapshot_name}.writes"));
    if !path.exists() {
        return Vec::new();
    }
    decode_ppu_writes(&std::fs::read(path).unwrap())
}

fn replay_ppu_writes(ppu: &mut Ppu, writes: &[PpuBusWrite]) {
    let mut by_scanline: [Vec<&PpuBusWrite>; 224] = [const { Vec::new() }; 224];
    for write in writes {
        let v = ClockInfo::from_master_clock(write.master_clock).v;
        if v < 224 {
            by_scanline[v as usize].push(write);
        }
    }
    for y in 0..224u32 {
        ppu.draw_scanline(y);
        for write in &by_scanline[y as usize] {
            ppu.write(AddressU24::new(0, write.offset), write.value);
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
///
/// The test ROM (`sprite_rendering.sfc`) programs the PPU via CPU+DMA and then loops
/// indefinitely, allowing the PPU to render the configured sprites each frame.
#[test]
pub fn test_sprite_rendering() {
    run_framebuffer_test("sprite_rendering", 1);
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
