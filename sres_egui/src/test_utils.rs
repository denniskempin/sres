//! Headless `egui_kittest` helpers backed by `egui_glow`.
//! `widget_snapshot` compares a widget to a PNG under `tests/snapshots/` (`UPDATE_SNAPSHOTS=1` to write).

use std::path::PathBuf;
use std::sync::Arc;

use eframe::egui_glow;
use eframe::glow;
use eframe::glow::HasContext as _;
use egui_kittest::Harness;
use egui_kittest::LazyRenderer;
use egui_kittest::SnapshotOptions;
use egui_kittest::TestRenderer;

/// Returns [`SnapshotOptions`] pointing to `sres_egui/tests/snapshots/`.
///
/// Using an absolute path derived from `CARGO_MANIFEST_DIR` ensures snapshots land in the
/// right place regardless of where `cargo test` is invoked from.
pub fn snapshot_options() -> SnapshotOptions {
    SnapshotOptions::default().output_path(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("snapshots"),
    )
}

/// Render a stateless UI widget in a headless [`Harness`], size the window to fit the widget,
/// and compare the result to a stored PNG snapshot.
///
/// # First run
/// When no snapshot exists yet, run with `UPDATE_SNAPSHOTS=1` to create the golden file:
///
/// ```sh
/// UPDATE_SNAPSHOTS=1 cargo test -p sres_egui
/// ```
///
/// Subsequent runs compare against the stored image. Re-run with `UPDATE_SNAPSHOTS=1` after
/// intentional visual changes to update the golden files.
///
/// # Panics
/// Panics if the rendered image does not match the stored snapshot.
pub fn widget_snapshot(name: &str, ui_fn: impl FnMut(&mut egui::Ui)) {
    let mut harness = Harness::builder()
        .renderer(LazyRenderer::new(GlowTestRenderer::new))
        .build_ui(ui_fn);
    harness.fit_contents();
    harness.snapshot_options(name, &snapshot_options());
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
}

impl Drop for GlowTestRenderer {
    fn drop(&mut self) {
        self._context.make_current();
        self.painter.destroy();
    }
}

impl TestRenderer for GlowTestRenderer {
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
