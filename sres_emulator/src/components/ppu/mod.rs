//! Implementation of the Picture Processing Unit
mod cgram;
mod clock;
mod debug;
mod oam;
mod vram;

use std::marker::PhantomData;

use bitcode::Decode;
use bitcode::Encode;
use intbits::Bits;

use self::cgram::CgRam;
// Public to allow for access in benches
pub use self::clock::Clock;
pub use self::debug::PpuDebug;
pub use self::debug::VramRenderSelection;
use self::oam::Oam;
use self::vram::Vram;
use crate::common::address::AddressU15;
use crate::common::address::AddressU24;
use crate::common::clock::ClockInfo;
use crate::common::image::Image;
use crate::common::image::Rgb15;
use crate::common::uint::U16Ext;
use crate::common::uint::U32Ext;
use crate::common::uint::U8Ext;

#[derive(Default, Copy, Clone, Debug, PartialEq, Encode, Decode, strum::Display)]
pub enum BackgroundId {
    #[default]
    BG1 = 0,
    BG2 = 1,
    BG3 = 2,
    BG4 = 3,
}

pub struct Ppu {
    disabled: bool,
    headless: bool,
    state: PpuState,
}

#[derive(Encode, Decode)]
pub struct PpuState {
    timer: Clock,
    vram: Vram,
    bgmode: BgMode,
    bg3_priority: bool,
    backgrounds: [Background; 4],

    framebuffer: Framebuffer,
    cgram: CgRam,
    oam: Oam,
    last_drawn_scanline: u64,

    bgofs_latch: u8,
    bghofs_latch: u8,

    color_math_backdrop_enabled: bool,
    color_math_operation: ColorMathOperation,
    color_math_half: bool,
    fixed_color: Rgb15,

    mode7_latch: u8,
    m7a_mul: i16,
    m7b_mul: i8,

    counter_latch: bool,
    h_counter: u16,
    h_counter_latch: bool,
    v_counter: u16,
    v_counter_latch: bool,
}

impl Default for PpuState {
    fn default() -> Self {
        Self {
            timer: Clock::default(),
            vram: Vram::new(),
            bgmode: BgMode::Mode0,
            bg3_priority: false,
            backgrounds: [Background::default(); 4],
            framebuffer: Framebuffer::default(),
            cgram: CgRam::new(),
            oam: Oam::new(),
            last_drawn_scanline: 0,
            bgofs_latch: 0,
            bghofs_latch: 0,
            color_math_backdrop_enabled: false,
            color_math_operation: ColorMathOperation::Add,
            color_math_half: false,
            mode7_latch: 0,
            m7a_mul: 0,
            m7b_mul: 0,
            counter_latch: false,
            h_counter: 0,
            h_counter_latch: false,
            v_counter: 0,
            v_counter_latch: false,
            fixed_color: Rgb15::default(),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Layer {
    Background(BackgroundId, bool),
    Object(u8),
}

impl Ppu {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            disabled: false,
            headless: false,
            state: PpuState::default(),
        }
    }

    /// Only used for benchmarks, runs full PPU emulation but does not render
    pub fn force_headless(&mut self) {
        self.headless = true;
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        &self.state.framebuffer
    }

    pub fn reset(&mut self) {
        self.state = PpuState::default();
    }

    pub fn load_state(&mut self, encoded: &[u8]) -> anyhow::Result<()> {
        self.state = bitcode::decode(encoded)?;
        Ok(())
    }

    pub fn save_state(&self) -> Vec<u8> {
        bitcode::encode(&self.state)
    }

    pub fn clock_info(&self) -> ClockInfo {
        self.state.timer.clock_info()
    }

    pub fn debug(&self) -> PpuDebug<'_> {
        PpuDebug(self)
    }

    pub fn bus_read(&mut self, addr: AddressU24) -> u8 {
        match addr.offset {
            0x2138 => self.state.oam.read_oamdataread(),
            0x2139 => self.state.vram.read_vmdatalread(),
            0x213A => self.state.vram.read_vmdatahread(),
            0x213B => self.state.cgram.read_cgdataread(),
            0x2134..=0x2136 => self.read_mpy(addr),
            0x2137 => self.read_shvl(),
            0x213C => self.read_ophct(),
            0x213D => self.read_opvct(),
            0x213E => self.peek_stat77(),
            0x213F => self.read_stat78(),
            0x4200 | 0x4207..=0x420A | 0x4210..=0x4212 => self.state.timer.bus_read(addr),
            _ => {
                log::warn!("PPU: Unhandled read from {:04X}", addr.offset);
                0
            }
        }
    }

    pub fn bus_peek(&self, addr: AddressU24) -> Option<u8> {
        match addr.offset {
            0x2138 => Some(self.state.oam.peek_oamdataread()),
            0x2139 => Some(self.state.vram.peek_vmdatalread()),
            0x213A => Some(self.state.vram.peek_vmdatahread()),
            0x213B => Some(self.state.cgram.peek_cgdataread()),
            0x2134..=0x2136 => Some(self.read_mpy(addr)),
            0x2137 => Some(self.peek_shvl()),
            0x213C => Some(self.peek_ophct()),
            0x213D => Some(self.peek_opvct()),
            0x213E => Some(self.peek_stat77()),
            0x213F => Some(self.peek_stat78()),
            0x4200 | 0x4207..=0x420A | 0x4210..=0x4212 => self.state.timer.bus_peek(addr),
            _ => None,
        }
    }

    pub fn bus_write(&mut self, addr: AddressU24, value: u8) {
        match addr.offset {
            0x2100 => self.write_inidisp(value),
            0x2101 => self.state.oam.write_objsel(value),
            0x2102 => self.state.oam.write_oamaddl(value),
            0x2103 => self.state.oam.write_oamaddh(value),
            0x2104 => self.state.oam.write_oamdata(value),
            0x2105 => self.write_bgmode(value),
            0x2107..=0x210A => self.write_bgnsc(addr, value),
            0x210B => self.write_bg12nba(value),
            0x210C => self.write_bg34nba(value),
            0x210D | 0x210F | 0x2111 | 0x2113 => self.write_bgnhofs(addr, value),
            0x210E | 0x2110 | 0x2112 | 0x2114 => self.write_bgnvofs(addr, value),
            0x2115 => self.state.vram.write_vmain(value),
            0x2116 => self.state.vram.write_vmaddl(value),
            0x2117 => self.state.vram.write_vmaddh(value),
            0x2118 => self.state.vram.write_vmdatal(value),
            0x2119 => self.state.vram.write_vmdatah(value),
            0x2121 => self.state.cgram.write_cgadd(value),
            0x2122 => self.state.cgram.write_cgdata(value),
            0x212C => self.write_tm(value),
            0x212D => self.write_ts(value),
            0x2131 => self.write_cdadsub(value),
            0x2132 => self.write_coldata(value),
            0x211B => self.write_m7a(value),
            0x211C => self.write_m7b(value),
            0x4200 | 0x4207..=0x420A | 0x4210..=0x4212 => self.state.timer.bus_write(addr, value),
            _ => log::warn!(
                "PPU: Unhandled write to {:04X} = {:02X}",
                addr.offset,
                value
            ),
        }
    }

    pub fn advance_master_clock(&mut self, cycles: u64) {
        self.state.timer.advance_master_clock(cycles);
        if self.disabled {
            return;
        }
        if self.clock_info().v != self.state.last_drawn_scanline {
            if !self.headless {
                self.draw_scanline(self.clock_info().v as u32);
            }
            self.state.last_drawn_scanline = self.clock_info().v;
        }
    }

    pub fn consume_nmi_interrupt(&mut self) -> bool {
        self.state.timer.consume_nmi_interrupt()
    }

    pub fn consume_timer_interrupt(&mut self) -> bool {
        self.state.timer.consume_timer_interrupt()
    }

    pub fn draw_scanline(&mut self, screen_y: u32) {
        if screen_y >= 224 {
            return;
        }

        let mut bg_data: [[(u8, bool); 256]; 4] = [
            [(0, false); 256],
            [(0, false); 256],
            [(0, false); 256],
            [(0, false); 256],
        ];
        let layers = self.decode_bgmode(screen_y, &mut bg_data);

        let mut obj_data: [(u8, u8); 256] = [(0, 0); 256];
        self.decode_obj(screen_y, &mut obj_data);

        // Render sub screen first, it'll be used for blending while rendering the main screen.
        let mut raw_sub = [self.state.fixed_color; 256];
        for layer in layers.iter().rev() {
            match layer {
                Layer::Background(id, layer_priority) => {
                    let bg = self.state.backgrounds[*id as usize];
                    if bg.bit_depth == BitDepth::Disabled || !bg.subscreen_enabled {
                        continue;
                    }
                    for (x, (pixel, priority)) in bg_data[*id as usize].iter().enumerate() {
                        if layer_priority != priority {
                            continue;
                        }
                        if *pixel > 0 {
                            raw_sub[x] = self.state.cgram[bg.palette_addr + pixel];
                        }
                    }
                }
                Layer::Object(layer_priority) => {
                    if !self.state.oam.sub_enabled {
                        continue;
                    }
                    for (x, (pixel, priority)) in obj_data.iter().enumerate() {
                        if layer_priority != priority {
                            continue;
                        }
                        if *pixel > 0 {
                            raw_sub[x] = self.state.cgram[*pixel];
                        }
                    }
                }
            }
        }

        // Pre-invert subscreen so we don't have to branch for each pixel
        let sub = match self.state.color_math_operation {
            ColorMathOperation::Add => {
                raw_sub.map(|pixel| (pixel.r() as i16, pixel.g() as i16, pixel.b() as i16))
            }
            ColorMathOperation::Subtract => raw_sub.map(|pixel| {
                (
                    -(pixel.r() as i16),
                    -(pixel.g() as i16),
                    -(pixel.b() as i16),
                )
            }),
        };
        let div_factor = if self.state.color_math_half { 2 } else { 1 };

        // Render main screen
        let mut scanline = if self.state.color_math_backdrop_enabled {
            sub.map(|pixel| (self.state.cgram[0] + pixel) / div_factor)
        } else {
            [self.state.cgram[0]; 256]
        };

        for layer in layers.iter().rev() {
            match layer {
                Layer::Background(id, layer_priority) => {
                    let bg = self.state.backgrounds[*id as usize];
                    if bg.bit_depth == BitDepth::Disabled || !bg.main_enabled {
                        continue;
                    }
                    if bg.color_math_enabled {
                        for (x, (pixel, priority)) in bg_data[*id as usize].iter().enumerate() {
                            if layer_priority != priority {
                                continue;
                            }
                            if *pixel > 0 {
                                scanline[x] = (self.state.cgram[bg.palette_addr + pixel] + sub[x])
                                    / div_factor;
                            }
                        }
                    } else {
                        for (x, (pixel, priority)) in bg_data[*id as usize].iter().enumerate() {
                            if layer_priority != priority {
                                continue;
                            }
                            if *pixel > 0 {
                                scanline[x] = self.state.cgram[bg.palette_addr + pixel];
                            }
                        }
                    }
                }
                Layer::Object(layer_priority) => {
                    if !self.state.oam.main_enabled {
                        continue;
                    }
                    for (x, (pixel, priority)) in obj_data.iter().enumerate() {
                        if layer_priority != priority {
                            continue;
                        }
                        if *pixel > 0 {
                            scanline[x] = self.state.cgram[*pixel];
                        }
                    }
                }
            }
        }
        for x in 0..256 {
            self.state.framebuffer[(x, screen_y)] = scanline[x as usize];
        }
    }

    /// Decodes background data and determines layer priorities
    ///
    /// Follows the following table from snes.nesdev.org:
    ///
    /// Mode| BG bit depth  |Offsets |     Priorities (front -> back)
    ///     |BG1 BG2 BG3 BG4|per tile|
    ///  0  | 2   2   2   2 |   No   |   S3 H1 H2 S2 L1 L2 S1 H3 H4 S0 L3 L4
    ///  1  | 4   4   2     |   No   |   S3 H1 H2 S2 L1 L2 S1 H3    S0 L3
    ///     |               |        |H3 S3 H1 H2 S2 L1 L2 S1       S0 L3
    ///  2  | 4   4         |  Yes   |   S3 H1    S2 H2    S1 L1    S0 L2
    ///  3  | 8   4         |   No   |   S3 H1    S2 H2    S1 L1    S0 L2
    ///  4  | 8   2         |  Yes   |   S3 H1    S2 H2    S1 L1    S0 L2
    ///  5  | 4   2         |   No   |   S3 H1    S2 H2    S1 L1    S0 L2
    ///  6  | 4             |  Yes   |   S3 H1    S2       S1 L1    S0
    ///  7  | 8             |   No   |   S3       S2       S1 L1    S0
    /// 7EXT| 8   7         |   No   |   S3       S2 H2    S1 L1    S0 L2
    fn decode_bgmode(&self, screen_y: u32, bg_data: &mut [[(u8, bool); 256]; 4]) -> &[Layer] {
        use BackgroundId::*;
        use Layer::*;

        const S0: Layer = Object(0);
        const S1: Layer = Object(1);
        const S2: Layer = Object(2);
        const S3: Layer = Object(3);
        const L1: Layer = Background(BG1, false);
        const L2: Layer = Background(BG2, false);
        const L3: Layer = Background(BG3, false);
        const L4: Layer = Background(BG4, false);
        const H1: Layer = Background(BG1, true);
        const H2: Layer = Background(BG2, true);
        const H3: Layer = Background(BG3, true);
        const H4: Layer = Background(BG4, true);

        match self.state.bgmode {
            BgMode::Mode0 => {
                self.decode_bg::<Bpp2Decoder>(screen_y, BG1, &mut (*bg_data)[0]);
                self.decode_bg::<Bpp2Decoder>(screen_y, BG2, &mut (*bg_data)[1]);
                self.decode_bg::<Bpp2Decoder>(screen_y, BG3, &mut (*bg_data)[2]);
                self.decode_bg::<Bpp2Decoder>(screen_y, BG4, &mut (*bg_data)[3]);
                &[S3, H1, H2, S2, L1, L2, S1, H3, H4, S0, L3, L4]
            }
            BgMode::Mode1 => {
                self.decode_bg::<Bpp4Decoder>(screen_y, BG1, &mut (*bg_data)[0]);
                self.decode_bg::<Bpp4Decoder>(screen_y, BG2, &mut (*bg_data)[1]);
                self.decode_bg::<Bpp2Decoder>(screen_y, BG3, &mut (*bg_data)[2]);
                if self.state.bg3_priority {
                    &[H3, S3, H1, H2, S2, L1, L2, S1, S0, L3]
                } else {
                    &[S3, H1, H2, S2, L1, L2, S1, H3, S0, L3]
                }
            }
            BgMode::Mode2 => {
                self.decode_bg::<Bpp4Decoder>(screen_y, BG1, &mut (*bg_data)[0]);
                self.decode_bg::<Bpp4Decoder>(screen_y, BG2, &mut (*bg_data)[1]);
                &[S3, H1, S2, H2, S1, L1, S0, L2]
            }
            BgMode::Mode3 => {
                self.decode_bg::<Bpp8Decoder>(screen_y, BG1, &mut (*bg_data)[0]);
                self.decode_bg::<Bpp4Decoder>(screen_y, BG2, &mut (*bg_data)[1]);
                &[S3, H1, S2, H2, S1, L1, S0, L2]
            }
            BgMode::Mode5 => {
                self.decode_bg::<Bpp4Decoder>(screen_y, BG1, &mut (*bg_data)[0]);
                self.decode_bg::<Bpp2Decoder>(screen_y, BG2, &mut (*bg_data)[1]);
                &[S3, H1, S2, H2, S1, L1, S0, L2]
            }
            _ => panic!("Unsupported BG mode: {}", self.state.bgmode),
        }
    }

    fn decode_bg<TileDecoderT: TileDecoder>(
        &self,
        screen_y: u32,
        background_id: BackgroundId,
        data: &mut [(u8, bool); 256],
    ) {
        let bg = self.state.backgrounds[background_id as usize];
        if bg.bit_depth == BitDepth::Disabled || !(bg.main_enabled || bg.subscreen_enabled) {
            return;
        }

        let y = screen_y + bg.v_offset;
        for screen_x in 0..256 {
            let x = screen_x + bg.h_offset;

            let tile = bg.get_tile::<TileDecoderT>(x / 8, y / 8, &self.state.vram);
            let pixel = tile.row(y % 8, &self.state.vram).pixel(x % 8);
            data[screen_x as usize] = (pixel, tile.priority);
        }
    }

    fn decode_obj(&self, screen_y: u32, obj_data: &mut [(u8, u8); 256]) {
        let sprites = self.state.oam.get_all_sprites_on_scanline(screen_y);
        for (sprite, row) in sprites {
            let row_coarse = row / 8;
            let row_fine = row % 8;
            if sprite.x >= 256 {
                continue;
            }

            for coarse_x in 0..sprite.coarse_width() {
                if sprite.x + coarse_x * 8 >= 256 {
                    continue;
                }
                let tile_x = if sprite.hflip {
                    sprite.coarse_width() - coarse_x - 1
                } else {
                    coarse_x
                };
                let tile_y = if sprite.vflip {
                    sprite.coarse_height() - row_coarse - 1
                } else {
                    row_coarse
                };
                let tile_idx = sprite.tile + tile_x + tile_y * 16;
                let tile = Tile::<Bpp4Decoder>::from_tileset_index(
                    sprite.nametable,
                    tile_idx,
                    sprite.hflip,
                    sprite.vflip,
                );
                for (fine_x, pixel) in tile.row(row_fine, &self.state.vram).pixels() {
                    if sprite.x + coarse_x * 8 + fine_x >= 256 {
                        continue;
                    }
                    if pixel > 0 {
                        obj_data[(sprite.x + coarse_x * 8 + fine_x) as usize] =
                            (sprite.palette_addr() + pixel, sprite.priority);
                    }
                }
            }
        }
    }

    /// Register 2100: INIDISP
    /// 7  bit  0
    /// ---- ----
    /// F... BBBB
    /// |    ||||
    /// |    ++++- Screen brightness (linear steps from 0 = none to $F = full)
    /// +--------- Force blanking
    fn write_inidisp(&mut self, value: u8) {
        log::info!("INIDISP = {:08b}", value);
        self.disabled = value.bit(7);
    }

    /// Register 2105: BGMODE
    /// 7  bit  0
    /// ---- ----
    /// 4321 PMMM
    /// |||| ||||
    /// |||| |+++- BG mode
    /// |||| +---- Mode 1 BG3 priority (0 = normal, 1 = high)
    /// |||+------ BG1 character size (0 = 8x8, 1 = 16x16)
    /// ||+------- BG2 character size (0 = 8x8, 1 = 16x16)
    /// |+-------- BG3 character size (0 = 8x8, 1 = 16x16)
    /// +--------- BG4 character size (0 = 8x8, 1 = 16x16)
    fn write_bgmode(&mut self, value: u8) {
        self.state.bgmode = match value.bits(0..=2) {
            0 => BgMode::Mode0,
            1 => BgMode::Mode1,
            2 => BgMode::Mode2,
            3 => BgMode::Mode3,
            4 => BgMode::Mode4,
            5 => BgMode::Mode5,
            6 => BgMode::Mode6,
            7 => BgMode::Mode7,
            _ => unreachable!(),
        };
        self.state.bg3_priority = value.bit(3);

        use BitDepth::*;
        let bit_depths = match self.state.bgmode {
            BgMode::Mode0 => (Bpp2, Bpp2, Bpp2, Bpp2),
            BgMode::Mode1 => (Bpp4, Bpp4, Bpp2, Disabled),
            BgMode::Mode2 => (Bpp4, Bpp4, Opt, Disabled),
            BgMode::Mode3 => (Bpp8, Bpp4, Disabled, Disabled),
            BgMode::Mode4 => (Bpp8, Bpp2, Opt, Disabled),
            BgMode::Mode5 => (Bpp4, Bpp2, Disabled, Disabled),
            BgMode::Mode6 => (Bpp4, Disabled, Opt, Disabled),
            BgMode::Mode7 => (Bpp8, Disabled, Disabled, Disabled),
        };
        self.state.backgrounds[0].bit_depth = bit_depths.0;
        self.state.backgrounds[1].bit_depth = bit_depths.1;
        self.state.backgrounds[2].bit_depth = bit_depths.2;
        self.state.backgrounds[3].bit_depth = bit_depths.3;

        let palette_addr = match self.state.bgmode {
            BgMode::Mode0 => (0, 32, 64, 96),
            _ => (0, 0, 0, 0),
        };
        self.state.backgrounds[0].palette_addr = palette_addr.0;
        self.state.backgrounds[1].palette_addr = palette_addr.1;
        self.state.backgrounds[2].palette_addr = palette_addr.2;
        self.state.backgrounds[3].palette_addr = palette_addr.3;

        fn to_tile_size(value: bool) -> TileSize {
            if value {
                TileSize::Size16x16
            } else {
                TileSize::Size8x8
            }
        }
        for i in 0..4 {
            self.state.backgrounds[i].tile_size = to_tile_size(value.bit(4 + i));
        }
    }

    /// Register 2107..210A: BGNSC - BG1..BG4 tilemap base address
    /// 7  bit  0
    /// ---- ----
    /// AAAA AAYX
    /// |||| ||||
    /// |||| |||+- Horizontal tilemap count (0 = 1 tilemap, 1 = 2 tilemaps)
    /// |||| ||+-- Vertical tilemap count (0 = 1 tilemap, 1 = 2 tilemaps)
    /// ++++-++--- Tilemap VRAM address (word address = AAAAAA << 10)
    fn write_bgnsc(&mut self, addr: AddressU24, value: u8) {
        let bg_id = (addr.offset - 0x2107) as usize;
        self.state.backgrounds[bg_id].tilemap_addr = AddressU15((value.bits(2..=7) as u16) << 10);
        self.state.backgrounds[bg_id].tilemap_size = match value.bits(0..=1) {
            0 => TilemapSize::Size32x32,
            1 => TilemapSize::Size64x32,
            2 => TilemapSize::Size32x64,
            3 => TilemapSize::Size64x64,
            _ => unreachable!(),
        }
    }

    /// Register 210B: BG12NBA - Tileset base address for BG1 and BG2
    /// 7  bit  0
    /// ---- ----
    /// BBBB AAAA
    /// |||| ||||
    /// |||| ++++- BG1 CHR word base address (word address = AAAA << 12)
    /// ++++------ BG2 CHR word base address (word address = BBBB << 12)
    fn write_bg12nba(&mut self, value: u8) {
        self.state.backgrounds[0].tileset_addr = AddressU15((value.low_nibble() as u16) << 12);
        self.state.backgrounds[1].tileset_addr = AddressU15((value.high_nibble() as u16) << 12);
    }

    /// Register 210C: BG34NBA - Tileset base address for BG3 and BG4
    /// 7  bit  0
    /// ---- ----
    /// DDDD CCCC
    /// |||| ||||
    /// |||| ++++- BG3 CHR word base address (word address = CCCC << 12)
    /// ++++------ BG4 CHR word base address (word address = DDDD << 12)
    fn write_bg34nba(&mut self, value: u8) {
        self.state.backgrounds[2].tileset_addr = AddressU15((value.low_nibble() as u16) << 12);
        self.state.backgrounds[3].tileset_addr = AddressU15((value.high_nibble() as u16) << 12);
    }

    /// Register 210D, 210F, 2111, 2113: BGNHOFS - Background N horizontal scroll
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  .... ..XX   XXXX XXXX
    ///         ||   |||| ||||
    ///         ++---++++-++++- BGn horizontal scroll
    ///
    /// On write: BGnHOFS = (value << 8) | (bgofs_latch & ~7) | (bghofs_latch & 7)
    ///           bgofs_latch = value
    ///           bghofs_latch = value
    fn write_bgnhofs(&mut self, addr: AddressU24, value: u8) {
        let bg_id = ((addr.offset - 0x210D) / 2) as usize;
        self.state.backgrounds[bg_id].h_offset = ((value as u32) << 8)
            | ((self.state.bgofs_latch as u32) & !7)
            | ((self.state.bghofs_latch as u32) & 7);
        self.state.bgofs_latch = value;
        self.state.bghofs_latch = value;
    }

    /// Register 210E, 2110, 2112, 2114: BGNVOFS - Background N vertical scroll
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  .... ..YY   YYYY YYYY
    ///         ||   |||| ||||
    ///         ++---++++-++++- BGn vertical scroll
    ///
    /// On write: BGnVOFS = (value << 8) | bgofs_latch
    ///           bgofs_latch = value
    ///
    /// Note: BG1VOFS uses the same address as M7VOFS
    fn write_bgnvofs(&mut self, addr: AddressU24, value: u8) {
        let bg_id = ((addr.offset - 0x210E) / 2) as usize;
        self.state.backgrounds[bg_id].v_offset =
            ((value as u32) << 8) | (self.state.bgofs_latch as u32);
        self.state.bgofs_latch = value;
    }

    /// Register 212C: TM - Main screen layer enable
    /// 7  bit  0
    /// ---- ----
    /// ...O 4321
    ///    | ||||
    ///    | |||+- Enable BG1 on main screen
    ///    | ||+-- Enable BG2 on main screen
    ///    | |+--- Enable BG3 on main screen
    ///    | +---- Enable BG4 on main screen
    ///    +------ Enable OBJ on main screen
    fn write_tm(&mut self, value: u8) {
        for i in 0..4 {
            self.state.backgrounds[i].main_enabled = value.bit(i);
        }
        self.state.oam.main_enabled = value.bit(4);
    }

    /// Register 212D: TS - Subscreen layer enable
    /// 7  bit  0
    /// ---- ----
    /// ...O 4321
    ///    | ||||
    ///    | |||+- Enable BG1 on subscreen
    ///    | ||+-- Enable BG2 on subscreen
    ///    | |+--- Enable BG3 on subscreen
    ///    | +---- Enable BG4 on subscreen
    ///    +------ Enable OBJ on subscreen
    fn write_ts(&mut self, value: u8) {
        for i in 0..4 {
            self.state.backgrounds[i].subscreen_enabled = value.bit(i);
        }
        self.state.oam.sub_enabled = value.bit(4);
    }

    /// Register 2131: CGADSUB - Color math control
    /// 7  bit  0
    /// ---- ----
    /// MHBO 4321
    /// |||| ||||
    /// |||| |||+- BG1 color math enable
    /// |||| ||+-- BG2 color math enable
    /// |||| |+--- BG3 color math enable
    /// |||| +---- BG4 color math enable
    /// |||+------ OBJ color math enable (palettes 4-7 only)
    /// ||+------- Backdrop color math enable
    /// |+-------- Half color math
    /// +--------- Operator type (0 = add, 1 = subtract)
    fn write_cdadsub(&mut self, value: u8) {
        for i in 0..4 {
            self.state.backgrounds[i].color_math_enabled = value.bit(i);
        }
        self.state.oam.color_math_enabled = value.bit(4);
        self.state.color_math_backdrop_enabled = value.bit(5);
        self.state.color_math_half = value.bit(6);
        self.state.color_math_operation = match value.bit(7) {
            false => ColorMathOperation::Add,
            true => ColorMathOperation::Subtract,
        };
    }

    /// Register 2132: COLDATA - Fixed color for color math
    /// 7  bit  0
    /// ---- ----
    /// BGRC CCCC
    /// |||| ||||
    /// |||+-++++- Color value
    /// ||+------- Write color value to blue channel
    /// |+-------- Write color value to green channel
    /// +--------- Write color value to red channel
    fn write_coldata(&mut self, value: u8) {
        let color_value = value.bits(0..=4);
        if value.bit(7) {
            self.state.fixed_color.set_b(color_value);
        }
        if value.bit(6) {
            self.state.fixed_color.set_g(color_value);
        }
        if value.bit(5) {
            self.state.fixed_color.set_r(color_value);
        }
    }

    /// Register 211B: M7A - Mode 7 Matrix A
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  DDDD DDDD   dddd dddd
    ///  |||| ||||   |||| ||||
    ///  ++++-++++---++++-++++- Mode 7 matrix A (8.8 fixed point)
    ///  ++++-++++---++++-++++- 16-bit multiplication factor (signed)
    ///
    /// On write: M7A = (value << 8) | mode7_latch
    ///           mode7_latch = value
    fn write_m7a(&mut self, value: u8) {
        self.state.m7a_mul = (((value as u16) << 8) | self.state.mode7_latch as u16) as i16;
        self.state.mode7_latch = value;
    }

    /// Register 211C: M7A - Mode 7 Matrix B
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  DDDD DDDD   dddd dddd
    ///  |||| ||||   |||| ||||
    ///  ++++-++++---++++-++++- Mode 7 matrix B (8.8 fixed point)
    ///              ++++-++++- 8-bit multiplication factor (signed)
    ///
    /// On write: M7B = (value << 8) | mode7_latch
    ///           mode7_latch = value
    fn write_m7b(&mut self, value: u8) {
        self.state.m7b_mul = value as i8;
        self.state.mode7_latch = value;
    }

    /// Register 2134-6: MPYL/M/H - 24 Bit Multipliction result
    ///   MPYH        MPYM        MPYL
    ///   $2136       $2135       $2134
    /// 7  bit  0   7  bit  0   7  bit  0
    /// ---- ----   ---- ----   ---- ----
    /// HHHH HHHH   MMMM MMMM   LLLL LLLL
    /// |||| ||||   |||| ||||   |||| ||||
    /// ++++-++++---++++-++++---++++-++++- 24-bit multiplication result (signed)
    fn read_mpy(&self, addr: AddressU24) -> u8 {
        let mpy = (self.state.m7a_mul as i32 * self.state.m7b_mul as i32) as u32;
        match addr.offset {
            0x2134 => mpy.low_word().low_byte(),
            0x2135 => mpy.low_word().high_byte(),
            0x2136 => mpy.high_word().low_byte(),
            _ => unreachable!(),
        }
    }

    /// Register 2137: SHVL - Software latch for H/V counters
    /// 7  bit  0
    /// ---- ----
    /// xxxx xxxx
    /// |||| ||||
    /// ++++-++++- Open bus
    ///
    /// On read: counter_latch = 1
    fn read_shvl(&mut self) -> u8 {
        if !self.state.counter_latch {
            self.state.h_counter = self.clock_info().hdot() as u16;
            self.state.v_counter = self.clock_info().v as u16;
        }
        self.state.counter_latch = true;
        0
    }

    fn peek_shvl(&self) -> u8 {
        0
    }

    /// Register 213C: OPHCT - Output horizontal counter
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  xxxx xxxH   HHHH HHHH
    ///  |||| ||||   |||| ||||
    ///  |||| |||+---++++-++++- Horizontal counter value
    ///  ++++-+++-------------- PPU2 open bus
    ///
    /// On read: If ophct_byte == 0, value = OPHCT.low
    ///          If ophct_byte == 1, value = OPHCT.high
    ///          ophct_byte = ~ophct_byte
    fn read_ophct(&mut self) -> u8 {
        if self.state.h_counter_latch {
            self.state.h_counter_latch = false;
            self.state.h_counter.high_byte()
        } else {
            self.state.h_counter_latch = true;
            self.state.h_counter.low_byte()
        }
    }

    fn peek_ophct(&self) -> u8 {
        if self.state.h_counter_latch {
            self.state.h_counter.high_byte()
        } else {
            self.state.h_counter.low_byte()
        }
    }

    /// Register 213D: OPVCT - Output vertical counter
    /// 15  bit  8   7  bit  0
    ///  ---- ----   ---- ----
    ///  xxxx xxxH   HHHH HHHH
    ///  |||| ||||   |||| ||||
    ///  |||| |||+---++++-++++- Vertical counter value
    ///  ++++-+++-------------- PPU2 open bus
    ///
    /// On read: If opvct_byte == 0, value = OPVCT.low
    ///          If opvct_byte == 1, value = OPVCT.high
    ///          opvct_byte = ~opvct_byte
    fn read_opvct(&mut self) -> u8 {
        if self.state.v_counter_latch {
            self.state.v_counter_latch = false;
            self.state.v_counter.high_byte()
        } else {
            self.state.v_counter_latch = true;
            self.state.v_counter.low_byte()
        }
    }

    fn peek_opvct(&self) -> u8 {
        if self.state.v_counter_latch {
            self.state.v_counter.high_byte()
        } else {
            self.state.v_counter.low_byte()
        }
    }

    /// Register 213E: STAT77 - PPU1 status and version number
    /// 7  bit  0
    /// ---- ----
    /// TRMx VVVV
    /// |||| ||||
    /// |||| ++++- PPU1 version
    /// |||+------ PPU1 open bus
    /// ||+------- Master/slave mode (PPU1 pin 25)
    /// |+-------- Range over flag (sprite tile overflow)
    /// +--------- Time over flag (sprite overflow)
    fn peek_stat77(&self) -> u8 {
        log::warn!("STAT77 not implemented");
        0
    }

    /// Register 213F: STAT78 - PPU2 status and version number
    /// 7  bit  0
    /// ---- ----
    /// FLxM VVVV
    /// |||| ||||
    /// |||| ++++- PPU2 version
    /// |||+------ 0: 262 or 525i lines = 60Hz, 1: 312 or 625i lines = 50Hz (PPU2 pin 30)
    /// ||+------- PPU2 open bus
    /// |+-------- Counter latch value
    /// +--------- Interlace field
    ///
    /// On read: counter_latch = 0
    ///          ophct_byte = 0
    ///          opvct_byte = 0
    fn read_stat78(&mut self) -> u8 {
        self.state.counter_latch = false;
        self.state.h_counter_latch = false;
        self.state.v_counter_latch = false;
        self.peek_stat78()
    }

    fn peek_stat78(&self) -> u8 {
        log::warn!("STAT78 not implemented");
        0
    }
}

#[derive(Encode, Decode, Clone)]
pub struct Framebuffer(Vec<Rgb15>);

impl Framebuffer {
    fn iter(&self) -> impl Iterator<Item = (u32, u32, &Rgb15)> {
        self.0
            .iter()
            .enumerate()
            .map(|(idx, pixel)| (idx as u32 % 256, idx as u32 / 256, pixel))
    }

    pub fn to_rgba<ImageT: Image>(&self) -> ImageT {
        let mut image = ImageT::new(256, 224);
        for (x, y, pixel) in self.iter() {
            image.set_pixel((x, y), (*pixel).into());
        }
        image
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self(vec![Rgb15(0); 256 * 224])
    }
}

impl std::ops::Index<(u32, u32)> for Framebuffer {
    type Output = Rgb15;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        &self.0[index.0 as usize + index.1 as usize * 256]
    }
}

impl std::ops::IndexMut<(u32, u32)> for Framebuffer {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        &mut self.0[index.0 as usize + index.1 as usize * 256]
    }
}

#[derive(Default, Copy, Clone, Debug, Encode, Decode, strum::Display)]
enum BgMode {
    #[default]
    Mode0,
    Mode1,
    Mode2,
    Mode3,
    Mode4,
    Mode5,
    Mode6,
    Mode7,
}

#[derive(Encode, Decode, Copy, Clone)]
enum ColorMathOperation {
    Add,
    Subtract,
}

#[derive(Default, Copy, Clone, Debug, Encode, Decode)]
struct Background {
    main_enabled: bool,
    subscreen_enabled: bool,
    color_math_enabled: bool,
    bit_depth: BitDepth,
    palette_addr: u8,
    tile_size: TileSize,
    tilemap_addr: AddressU15,
    tileset_addr: AddressU15,
    tilemap_size: TilemapSize,
    h_offset: u32,
    v_offset: u32,
}

impl Background {
    fn get_tile<TileDecoderT: TileDecoder>(
        &self,
        coarse_x: u32,
        coarse_y: u32,
        vram: &Vram,
    ) -> Tile<TileDecoderT> {
        let tilemap_idx = match self.tilemap_size {
            TilemapSize::Size32x32 => 0,
            TilemapSize::Size64x32 => (coarse_x / 32) % 2,
            TilemapSize::Size32x64 => (coarse_y / 32) % 2,
            TilemapSize::Size64x64 => (coarse_x / 32) % 2 + ((coarse_y / 32) % 2) * 2,
        };
        let tile_idx = tilemap_idx * 1024 + (coarse_y % 32) * 32 + (coarse_x % 32);
        let tilemap_entry = vram[self.tilemap_addr + tile_idx];
        Tile::from_tilemap_entry(self.tileset_addr, tilemap_entry)
    }

    fn coarse_width(&self) -> u32 {
        match self.tilemap_size {
            TilemapSize::Size32x32 => 32,
            TilemapSize::Size64x32 => 64,
            TilemapSize::Size32x64 => 32,
            TilemapSize::Size64x64 => 64,
        }
    }

    fn coarse_height(&self) -> u32 {
        match self.tilemap_size {
            TilemapSize::Size32x32 => 32,
            TilemapSize::Size64x32 => 32,
            TilemapSize::Size32x64 => 64,
            TilemapSize::Size64x64 => 64,
        }
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Encode, Decode, strum::Display)]
enum BitDepth {
    #[default]
    Disabled,
    Bpp2,
    Bpp4,
    Bpp8,
    Opt,
}

#[derive(Default, Copy, Clone, Debug, Encode, Decode, strum::Display)]
enum TileSize {
    #[default]
    Size8x8,
    Size16x16,
}

#[derive(Default, Copy, Clone, Debug, Encode, Decode, strum::Display)]
enum TilemapSize {
    #[default]
    Size32x32,
    Size64x32,
    Size32x64,
    Size64x64,
}

struct Tile<TileDecoderT: TileDecoder> {
    tile_addr: AddressU15,
    palette: u8,
    priority: bool,
    flip_v: bool,
    flip_h: bool,
    _decoder: PhantomData<TileDecoderT>,
}

impl<TileDecoderT: TileDecoder> Tile<TileDecoderT> {
    fn from_tilemap_entry(tileset_addr: AddressU15, tilemap_entry: u16) -> Self {
        let tile_idx = tilemap_entry.bits(0..=9);
        Self {
            tile_addr: tileset_addr + tile_idx * TileDecoderT::WORDS_PER_ROW as u16 * 8,
            palette: tilemap_entry.bits(10..=12) as u8,
            priority: tilemap_entry.bit(13),
            flip_v: tilemap_entry.bit(15),
            flip_h: tilemap_entry.bit(14),
            _decoder: PhantomData,
        }
    }

    fn from_tileset_index(
        tileset_addr: AddressU15,
        tile_idx: u32,
        flip_h: bool,
        flip_v: bool,
    ) -> Self {
        Self {
            tile_addr: tileset_addr + tile_idx * TileDecoderT::WORDS_PER_ROW * 8,
            palette: 0,
            priority: false,
            flip_v,
            flip_h,
            _decoder: PhantomData,
        }
    }

    fn row(&self, row_idx: u32, vram: &Vram) -> TileRow<TileDecoderT> {
        let flipped_idx = if self.flip_v { 7 - row_idx } else { row_idx };
        TileRow::new(
            TileDecoderT::new(self.tile_addr + flipped_idx, vram),
            self.palette,
            self.flip_h,
        )
    }

    fn rows<'a>(
        &'a self,
        vram: &'a Vram,
    ) -> impl Iterator<Item = (u32, TileRow<TileDecoderT>)> + 'a {
        (0..8).map(move |row_idx| (row_idx, self.row(row_idx, vram)))
    }
}

struct TileRow<TileDecoderT: TileDecoder> {
    decoder: TileDecoderT,
    palette: u8,
    flip: bool,
}

impl<TileDecoderT: TileDecoder> TileRow<TileDecoderT> {
    fn new(decoder: TileDecoderT, palette: u8, flip: bool) -> Self {
        Self {
            decoder,
            palette,
            flip,
        }
    }

    fn pixel(&self, pixel_idx: u32) -> u8 {
        let flipped_idx = if self.flip { pixel_idx } else { 7 - pixel_idx };
        let raw_pixel = self.decoder.pixel(flipped_idx);
        if raw_pixel == 0 {
            0
        } else {
            raw_pixel.saturating_add(self.palette.saturating_mul(TileDecoderT::NUM_COLORS))
        }
    }

    fn pixels(&self) -> impl Iterator<Item = (u32, u8)> + '_ {
        (0..8).map(|pixel_idx| (pixel_idx, self.pixel(pixel_idx)))
    }
}

trait TileDecoder {
    const WORDS_PER_ROW: u32;
    const NUM_COLORS: u8;

    fn new(tile_addr: AddressU15, vram: &Vram) -> Self;
    fn pixel(&self, pixel_idx: u32) -> u8;
}

struct Bpp2Decoder {
    planes: [u8; 2],
}

impl TileDecoder for Bpp2Decoder {
    const WORDS_PER_ROW: u32 = 1;
    const NUM_COLORS: u8 = 4;

    fn new(row_addr: AddressU15, vram: &Vram) -> Self {
        let data = vram[row_addr];
        Self {
            planes: [data.low_byte(), data.high_byte()],
        }
    }

    fn pixel(&self, pixel_idx: u32) -> u8 {
        self.planes[0].bit(pixel_idx) as u8 + ((self.planes[1].bit(pixel_idx) as u8) << 1)
    }
}

struct Bpp4Decoder {
    planes: [u8; 4],
}

impl TileDecoder for Bpp4Decoder {
    const WORDS_PER_ROW: u32 = 2;
    const NUM_COLORS: u8 = 16;

    fn new(row_addr: AddressU15, vram: &Vram) -> Self {
        let low_word = vram[row_addr];
        let high_word = vram[row_addr + 8_u16];
        Self {
            planes: [
                low_word.low_byte(),
                low_word.high_byte(),
                high_word.low_byte(),
                high_word.high_byte(),
            ],
        }
    }

    fn pixel(&self, pixel_idx: u32) -> u8 {
        self.planes[0].bit(pixel_idx) as u8
            + ((self.planes[1].bit(pixel_idx) as u8) << 1)
            + ((self.planes[2].bit(pixel_idx) as u8) << 2)
            + ((self.planes[3].bit(pixel_idx) as u8) << 3)
    }
}

struct Bpp8Decoder {
    planes: [u8; 8],
}

impl TileDecoder for Bpp8Decoder {
    const WORDS_PER_ROW: u32 = 4;
    const NUM_COLORS: u8 = 255;

    fn new(row_addr: AddressU15, vram: &Vram) -> Self {
        let word0 = vram[row_addr];
        let word1 = vram[row_addr + 8_u16];
        let word2 = vram[row_addr + 16_u16];
        let word3 = vram[row_addr + 24_u16];
        Self {
            planes: [
                word0.low_byte(),
                word0.high_byte(),
                word1.low_byte(),
                word1.high_byte(),
                word2.low_byte(),
                word2.high_byte(),
                word3.low_byte(),
                word3.high_byte(),
            ],
        }
    }

    fn pixel(&self, pixel_idx: u32) -> u8 {
        self.planes[0].bit(pixel_idx) as u8
            + ((self.planes[1].bit(pixel_idx) as u8) << 1)
            + ((self.planes[2].bit(pixel_idx) as u8) << 2)
            + ((self.planes[3].bit(pixel_idx) as u8) << 3)
            + ((self.planes[4].bit(pixel_idx) as u8) << 4)
            + ((self.planes[5].bit(pixel_idx) as u8) << 5)
            + ((self.planes[6].bit(pixel_idx) as u8) << 6)
            + ((self.planes[7].bit(pixel_idx) as u8) << 7)
    }
}
