//! Headless `egui_glow` widget snapshot helpers.
//! `widget_snapshot` compares a widget to a PNG under `tests/snapshots/` (`UPDATE_SNAPSHOTS=1` to write).

use std::path::PathBuf;
use std::sync::Arc;

use eframe::egui_glow;
use eframe::glow;
use eframe::glow::HasContext as _;
use egui::style::ScrollAnimation;
use egui::Pos2;
use egui::Rect;
use egui::Vec2;
use egui::ViewportId;

const DEFAULT_YIQ_THRESHOLD: f32 = 0.6;

/// Render a stateless UI widget in a headless `egui::Context`, size the window to fit the widget,
/// and compare the result to a stored PNG under `sres_egui/tests/snapshots/`.
///
/// # First run
/// When no snapshot exists yet (or after intentional visual changes), run with `UPDATE_SNAPSHOTS=1`:
///
/// ```sh
/// UPDATE_SNAPSHOTS=1 cargo test -p sres_egui
/// ```
///
/// # Panics
/// Panics if the rendered image does not match the stored snapshot.
pub fn widget_snapshot(name: &str, mut ui_fn: impl FnMut(&mut egui::Ui)) {
    let ctx = egui::Context::default();
    ctx.set_theme(egui::Theme::Dark);
    ctx.set_os(egui::os::OperatingSystem::Nix);
    ctx.all_styles_mut(|style| {
        style.visuals.text_cursor.blink = false;
        style.scroll_animation = ScrollAnimation::none();
        style.animation_time = 0.0;
    });

    let mut renderer = GlowTestRenderer::new();
    let mut screen_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 600.0));

    let mut run_step = |renderer: &mut GlowTestRenderer, screen_rect: Rect, sizing_pass: bool| {
        let mut input = egui::RawInput {
            screen_rect: Some(screen_rect),
            predicted_dt: 0.25,
            ..Default::default()
        };
        if let Some(vp) = input.viewports.get_mut(&ViewportId::ROOT) {
            vp.native_pixels_per_point = Some(1.0);
        }
        let mut panel_rect = None;
        let mut output = ctx.run_ui(input, |ui| {
            let mut builder = egui::UiBuilder::new();
            builder.sizing_pass = sizing_pass;
            let resp = ui.scope_builder(builder, |ui| {
                egui::Frame::central_panel(ui.style())
                    .outer_margin(8.0)
                    .inner_margin(0.0)
                    .show(ui, &mut ui_fn);
            });
            panel_rect = Some(resp.response.rect);
        });
        renderer.handle_delta(&mut output.textures_delta);
        (output, panel_rect)
    };

    // Initial pass + sizing pass to match egui_kittest::Harness::fit_contents().
    let _ = run_step(&mut renderer, screen_rect, false);
    let (_, panel_rect) = run_step(&mut renderer, screen_rect, true);
    if let Some(mut used) = panel_rect {
        ctx.memory(|mem| {
            mem.areas()
                .visible_layer_ids()
                .into_iter()
                .filter(|layer_id| layer_id.order != egui::Order::Background)
                .filter_map(|layer_id| mem.area_rect(layer_id.id))
                .for_each(|area_rect| used |= area_rect);
        });
        screen_rect = Rect::from_min_size(Pos2::ZERO, used.size());
    }

    let mut last_output = None;
    for _ in 0..4 {
        let (output, _) = run_step(&mut renderer, screen_rect, false);
        let repaint_now = output
            .viewport_output
            .get(&ViewportId::ROOT)
            .is_some_and(|v| v.repaint_delay.is_zero());
        last_output = Some(output);
        if !repaint_now {
            break;
        }
    }

    let actual = renderer
        .render(&ctx, last_output.as_ref().expect("at least one pass"))
        .expect("render widget snapshot");

    compare_snapshot(name, &actual);
}

fn compare_snapshot(name: &str, actual: &image::RgbaImage) {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
        .join(name);
    let snapshot_path = base.with_extension("png");
    let update = std::env::var_os("UPDATE_SNAPSHOTS").is_some_and(|v| v == "1" || v == "true");

    if update || !snapshot_path.exists() {
        if let Some(parent) = snapshot_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        actual.save(&snapshot_path).unwrap();
        if !update {
            panic!(
                "Snapshot {snapshot_path:?} did not exist; wrote new snapshot. Re-run to verify."
            );
        }
        return;
    }

    let expected = image::open(&snapshot_path).unwrap().to_rgba8();
    if expected.dimensions() != actual.dimensions() {
        let new_path = base.with_extension("new.png");
        actual.save(&new_path).unwrap();
        panic!(
            "Snapshot '{name}' dimensions mismatch: expected {:?}, got {:?}. Wrote {new_path:?}",
            expected.dimensions(),
            actual.dimensions()
        );
    }

    let (w, h) = expected.dimensions();
    let mut diff_pixels = 0usize;
    for y in 0..h {
        for x in 0..w {
            let e = expected.get_pixel(x, y);
            let a = actual.get_pixel(x, y);
            let idx = (y as usize) * (w as usize) + (x as usize);
            if e != a
                && yiq_squared_distance(e, a, idx) > DEFAULT_YIQ_THRESHOLD
                && !is_antialiased(&expected, x, y, w, h, actual)
                && !is_antialiased(actual, x, y, w, h, &expected)
            {
                diff_pixels += 1;
            }
        }
    }
    if diff_pixels > 0 {
        let new_path = base.with_extension("new.png");
        actual.save(&new_path).unwrap();
        panic!("Snapshot '{name}' ({w}x{h}) differed by {diff_pixels} pixels. Wrote {new_path:?}");
    }
}

fn yiq_squared_distance(left: &image::Rgba<u8>, right: &image::Rgba<u8>, pos: usize) -> f32 {
    let (ly, li, lq) = rgba_to_yiq(left, pos);
    let (ry, ri, rq) = rgba_to_yiq(right, pos);
    let dy = ly - ry;
    let di = li - ri;
    let dq = lq - rq;
    0.5053 * dy * dy + 0.299 * di * di + 0.1957 * dq * dq
}

#[allow(clippy::excessive_precision)]
fn rgba_to_yiq(px: &image::Rgba<u8>, pos: usize) -> (f32, f32, f32) {
    let [r, g, b, a] = px.0.map(f32::from);
    let (r, g, b) = if a < 255.0 {
        let alpha = a / 255.0;
        let bg_r = 48.0 + 159.0 * ((pos % 2) as f32);
        let bg_g = 48.0
            + 159.0 * ((pos as f32 / std::f32::consts::GOLDEN_RATIO).floor() as u32 % 2) as f32;
        let bg_b = 48.0 + 159.0 * ((pos as f32 / 2.618033988749895).floor() as u32 % 2) as f32;
        (
            bg_r + (r - bg_r) * alpha,
            bg_g + (g - bg_g) * alpha,
            bg_b + (b - bg_b) * alpha,
        )
    } else {
        (r, g, b)
    };
    (
        0.298_895_31 * r + 0.586_622_47 * g + 0.114_482_23 * b,
        0.595_977_99 * r - 0.274_171_6 * g - 0.321_801_89 * b,
        0.211_470_19 * r - 0.522_617_11 * g + 0.311_146_94 * b,
    )
}

fn neighbor_bounds(x: u32, y: u32, width: u32, height: u32) -> (u32, u32, u32, u32) {
    (
        x.saturating_sub(1),
        y.saturating_sub(1),
        (x.saturating_add(1)).min(width.saturating_sub(1)),
        (y.saturating_add(1)).min(height.saturating_sub(1)),
    )
}

fn has_many_siblings(image: &image::RgbaImage, x1: u32, y1: u32, width: u32, height: u32) -> bool {
    let (x0, y0, x2, y2) = neighbor_bounds(x1, y1, width, height);
    let on_edge = x1 == x0 || x1 == x2 || y1 == y0 || y1 == y2;
    let mut same = if on_edge { 0u32 } else { 1 };
    let center = image.get_pixel(x1, y1);
    for x in x0..=x2 {
        for y in y0..=y2 {
            if (x != x1 || y != y1) && center == image.get_pixel(x, y) {
                same += 1;
                if same > 2 {
                    return true;
                }
            }
        }
    }
    false
}

#[allow(clippy::excessive_precision)]
fn rgb_y(px: [u8; 4]) -> f32 {
    0.298_895_31 * f32::from(px[0])
        + 0.586_622_47 * f32::from(px[1])
        + 0.114_482_23 * f32::from(px[2])
}

fn is_antialiased(
    left: &image::RgbaImage,
    x1: u32,
    y1: u32,
    width: u32,
    height: u32,
    right: &image::RgbaImage,
) -> bool {
    let (x0, y0, x2, y2) = neighbor_bounds(x1, y1, width, height);
    let on_edge = x1 == x0 || x1 == x2 || y1 == y0 || y1 == y2;
    let mut zeros = if on_edge { 1u32 } else { 0 };
    let mut min = 0.0f32;
    let mut max = 0.0f32;
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (0, 0, 0, 0);

    let center_y = rgb_y(left.get_pixel(x1, y1).0);

    for x in x0..=x2 {
        for y in y0..=y2 {
            if x == x1 && y == y1 {
                continue;
            }
            let delta = center_y - rgb_y(left.get_pixel(x, y).0);
            if delta == 0.0 {
                zeros += 1;
                if zeros > 2 {
                    return false;
                }
            } else if delta < min {
                min = delta;
                min_x = x;
                min_y = y;
            } else if delta > max {
                max = delta;
                max_x = x;
                max_y = y;
            }
        }
    }
    if min == 0.0 || max == 0.0 {
        return false;
    }
    (has_many_siblings(left, min_x, min_y, width, height)
        && has_many_siblings(right, min_x, min_y, width, height))
        || (has_many_siblings(left, max_x, max_y, width, height)
            && has_many_siblings(right, max_x, max_y, width, height))
}

struct GlowTestRenderer {
    _context: HeadlessGlContext,
    gl: Arc<glow::Context>,
    painter: egui_glow::Painter,
}

impl GlowTestRenderer {
    fn new() -> Self {
        let (context, gl) = HeadlessGlContext::new();
        let gl = Arc::new(gl);
        let painter =
            egui_glow::Painter::new(gl.clone(), "", None, false).expect("create egui_glow Painter");
        Self {
            _context: context,
            gl,
            painter,
        }
    }

    fn handle_delta(&mut self, delta: &mut egui::TexturesDelta) {
        self._context.make_current();
        for (id, images) in delta.set.drain() {
            for image in images {
                self.painter.set_texture(id, &image);
            }
        }
        for id in delta.free.drain() {
            self.painter.free_texture(id);
        }
    }

    fn render(
        &mut self,
        ctx: &egui::Context,
        output: &egui::FullOutput,
    ) -> Result<image::RgbaImage, String> {
        self._context.make_current();

        let pixels_per_point = ctx.pixels_per_point();
        let size = ctx.content_rect().size() * pixels_per_point;
        let width = size.x.round().max(1.0) as u32;
        let height = size.y.round().max(1.0) as u32;
        let size_in_pixels = [width, height];

        let tessellated = ctx.tessellate(output.shapes.clone(), pixels_per_point);

        unsafe {
            let fbo = self.gl.create_framebuffer()?;
            let color_tex = self.gl.create_texture()?;

            self.gl.bind_texture(glow::TEXTURE_2D, Some(color_tex));
            self.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(None),
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            self.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );

            self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
            self.gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(color_tex),
                0,
            );

            let status = self.gl.check_framebuffer_status(glow::FRAMEBUFFER);
            if status != glow::FRAMEBUFFER_COMPLETE {
                self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                self.gl.delete_texture(color_tex);
                self.gl.delete_framebuffer(fbo);
                return Err(format!("Incomplete framebuffer: 0x{status:x}"));
            }

            self.painter.clear(size_in_pixels, [0.0, 0.0, 0.0, 0.0]);
            self.painter
                .paint_primitives(size_in_pixels, pixels_per_point, &tessellated);

            let mut pixels = vec![0u8; (width * height * 4) as usize];
            self.gl.pixel_store_i32(glow::PACK_ALIGNMENT, 1);
            self.gl.read_pixels(
                0,
                0,
                width as i32,
                height as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelPackData::Slice(Some(&mut pixels)),
            );

            self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            self.gl.delete_texture(color_tex);
            self.gl.delete_framebuffer(fbo);

            // OpenGL origin is bottom-left; flip rows vertically to top-left.
            let row_bytes = (width * 4) as usize;
            let mut flipped = vec![0u8; pixels.len()];
            for y in 0..height as usize {
                let src = (height as usize - 1 - y) * row_bytes;
                let dst = y * row_bytes;
                flipped[dst..dst + row_bytes].copy_from_slice(&pixels[src..src + row_bytes]);
            }

            image::RgbaImage::from_raw(width, height, flipped)
                .ok_or_else(|| "Failed to build RgbaImage from GL framebuffer".to_owned())
        }
    }
}

#[cfg(target_os = "macos")]
struct HeadlessGlContext {
    ctx: cgl::CGLContextObj,
    _lib: libloading::Library,
}

#[cfg(target_os = "macos")]
impl HeadlessGlContext {
    fn new() -> (Self, glow::Context) {
        const CGL_OGLP_VERSION_3_2_CORE: cgl::CGLPixelFormatAttribute = 0x3200;

        let attribs = [
            cgl::kCGLPFAOpenGLProfile,
            CGL_OGLP_VERSION_3_2_CORE,
            cgl::kCGLPFAColorSize,
            24,
            cgl::kCGLPFAAlphaSize,
            8,
            cgl::kCGLPFAAllowOfflineRenderers,
            0,
        ];

        unsafe {
            let mut pix: cgl::CGLPixelFormatObj = std::ptr::null_mut();
            let mut npix = 0;
            let err = cgl::CGLChoosePixelFormat(attribs.as_ptr(), &mut pix, &mut npix);
            assert_eq!(err, cgl::kCGLNoError, "CGLChoosePixelFormat failed: {err}");
            assert!(!pix.is_null(), "No CGL pixel format available");

            let mut ctx: cgl::CGLContextObj = std::ptr::null_mut();
            let err = cgl::CGLCreateContext(pix, std::ptr::null_mut(), &mut ctx);
            cgl::CGLDestroyPixelFormat(pix);
            assert_eq!(err, cgl::kCGLNoError, "CGLCreateContext failed: {err}");

            let err = cgl::CGLSetCurrentContext(ctx);
            assert_eq!(err, cgl::kCGLNoError, "CGLSetCurrentContext failed: {err}");

            let lib =
                libloading::Library::new("/System/Library/Frameworks/OpenGL.framework/OpenGL")
                    .expect("load OpenGL.framework");
            let gl = glow::Context::from_loader_function(|name| {
                lib.get::<*const std::ffi::c_void>(name.as_bytes())
                    .map(|sym| *sym)
                    .unwrap_or(std::ptr::null())
            });
            (Self { ctx, _lib: lib }, gl)
        }
    }

    fn make_current(&self) {
        unsafe {
            let err = cgl::CGLSetCurrentContext(self.ctx);
            assert_eq!(err, cgl::kCGLNoError, "CGLSetCurrentContext failed: {err}");
        }
    }
}

#[cfg(target_os = "macos")]
impl Drop for HeadlessGlContext {
    fn drop(&mut self) {
        unsafe {
            cgl::CGLSetCurrentContext(std::ptr::null_mut());
            cgl::CGLDestroyContext(self.ctx);
        }
    }
}

#[cfg(target_os = "linux")]
struct HeadlessGlContext {
    context: glutin::context::PossiblyCurrentContext,
}

#[cfg(target_os = "linux")]
impl HeadlessGlContext {
    fn new() -> (Self, glow::Context) {
        use glutin::api::egl::device::Device;
        use glutin::api::egl::display::Display as EglDisplay;
        use glutin::config::ConfigTemplateBuilder;
        use glutin::context::ContextAttributesBuilder;
        use glutin::prelude::*;

        let devices: Vec<_> = Device::query_devices()
            .expect("query EGL devices")
            .collect();
        let device = devices
            .iter()
            .find(|d| d.name().is_some_and(|n| n.contains("llvmpipe")))
            .or_else(|| devices.first())
            .expect("no EGL device available");

        let display =
            unsafe { EglDisplay::with_device(device, None) }.expect("create EGL device display");
        let template = ConfigTemplateBuilder::new().build();
        let config = unsafe { display.find_configs(template) }
            .expect("find EGL configs")
            .next()
            .expect("no matching EGL config");

        let attrs = ContextAttributesBuilder::new().build(None);
        let not_current =
            unsafe { display.create_context(&config, &attrs) }.expect("create EGL context");
        let context = not_current
            .make_current_surfaceless()
            .expect("make EGL context current");

        let gl = unsafe {
            glow::Context::from_loader_function_cstr(|name| display.get_proc_address(name))
        };
        (Self { context }, gl)
    }

    fn make_current(&self) {
        self.context
            .make_current_surfaceless()
            .expect("make EGL context current");
    }
}
