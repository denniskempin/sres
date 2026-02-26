# egui Documentation (version 0.33)

Source: context7 (/emilk/egui) — fetched 2026-02-21

This project (`sres_egui`) uses **egui 0.33** and **eframe 0.33** for its GUI frontend (native + WebAssembly).

---

## eframe App Setup

```rust
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // UI code here
        });
    }
}
```

---

## Panels

```rust
// Side panel (left or right)
egui::SidePanel::left("left_panel")
    .resizable(true)
    .default_width(200.0)
    .show(ctx, |ui| { ui.heading("Left Panel"); });

// Top/bottom panel
egui::TopBottomPanel::top("top_panel")
    .show(ctx, |ui| { ui.heading("Top Panel"); });

// Central panel (always add last)
egui::CentralPanel::default().show(ctx, |ui| {
    ui.heading("Main Content");
});
```

---

## Windows

```rust
use egui::Window;

// Basic window
Window::new("My Window").show(ctx, |ui| {
    ui.label("Window contents");
});

// Window with options
let mut window_open = true;
Window::new("Configurable Window")
    .open(&mut window_open)
    .resizable(true)
    .collapsible(true)
    .default_width(300.0)
    .vscroll(true)
    .show(ctx, |ui| { /* ... */ });

// Anchored window
Window::new("Anchored")
    .anchor(egui::Align2::RIGHT_TOP, [10.0, 10.0])
    .show(ctx, |ui| { ui.label("Anchored to top-right"); });
```

---

## Basic Widgets

```rust
// Labels & headings
ui.label("Simple text");
ui.heading("Large heading text");
ui.label(egui::RichText::new("Colored").color(egui::Color32::RED));

// Buttons
if ui.button("Click me!").clicked() { /* ... */ }
ui.hyperlink_to("egui docs", "https://docs.rs/egui");

// Text input
ui.text_edit_singleline(&mut my_string);
ui.text_edit_multiline(&mut my_string);
ui.add(egui::TextEdit::singleline(&mut text).hint_text("Enter text here..."));

// Checkboxes & radio buttons
ui.checkbox(&mut my_boolean, "Checkbox label");
ui.horizontal(|ui| {
    ui.radio_value(&mut my_enum, Enum::First, "First");
    ui.radio_value(&mut my_enum, Enum::Second, "Second");
});

// Sliders & drag values
ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0).text("value"));
ui.add(egui::DragValue::new(&mut my_f32).speed(0.1));

// Progress bar & spinner
ui.add(egui::ProgressBar::new(progress).show_percentage());
ui.add(egui::Spinner::new());

// Separator
ui.separator();

// Images
ui.image(egui::include_image!("path/to/image.png"));
ui.image((texture_id, egui::Vec2::new(640.0, 480.0)));

// Color picker
ui.color_edit_button_srgba(&mut color);
```

---

## Layouts

```rust
// Horizontal / vertical
ui.horizontal(|ui| { ui.label("A"); ui.label("B"); });
ui.vertical(|ui| { ui.label("Top"); ui.label("Bottom"); });
ui.vertical_centered(|ui| { ui.heading("Centered"); });

// Grid
use egui::Grid;
Grid::new("my_grid")
    .num_columns(2)
    .spacing([40.0, 4.0])
    .striped(true)
    .show(ui, |ui| {
        ui.label("Name:"); ui.text_edit_singleline(&mut name); ui.end_row();
        ui.label("Age:");  ui.add(egui::DragValue::new(&mut age)); ui.end_row();
    });
```

---

## Keyboard Input

```rust
use egui::Key;

// Key pressed (fires once per frame the key goes down)
if ctx.input(|i| i.key_pressed(Key::A)) { /* ... */ }

// Key held down (fires every frame)
if ctx.input(|i| i.key_down(Key::A)) {
    ctx.request_repaint(); // Keep repainting while held
}

// Key released
if ctx.input(|i| i.key_released(Key::A)) { /* ... */ }

// Modifier keys
if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(Key::S)) {
    save_file();
}
```

---

## Custom 2D Painting (Painter API)

```rust
use egui::{Painter, Color32, Stroke, pos2, vec2};

let (response, painter) = ui.allocate_painter(
    ui.available_size_before_wrap(),
    egui::Sense::click_and_drag(),
);

// Shapes
painter.circle_filled(pos2(100.0, 100.0), 50.0, Color32::RED);
painter.rect_stroke(
    egui::Rect::from_min_size(pos2(200.0, 100.0), vec2(100.0, 80.0)),
    5.0,
    Stroke::new(2.0, Color32::BLUE),
);
painter.line_segment([pos2(50.0, 50.0), pos2(150.0, 150.0)], Stroke::new(3.0, Color32::GREEN));
painter.text(pos2(300.0, 100.0), egui::Align2::LEFT_TOP, "Custom text",
             egui::FontId::default(), Color32::WHITE);

// Interactive painting
if let Some(ptr) = response.interact_pointer_pos() {
    painter.circle_filled(ptr, 5.0, Color32::YELLOW);
}
```

`Shape::Callback` lets you execute custom code during the painting phase (e.g., for raw GPU rendering via `glow` in eframe).

---

## Textures & Images

- To upload a texture from pixel data, use `ctx.load_texture(name, ColorImage, TextureOptions)`.
- To display using raw `TextureId`: `ui.image((texture_id, egui::Vec2::new(w, h)))`.
- For 3D scene render-to-texture: convert native texture to `egui::TextureId` using your integration.
- `egui_extras::install_image_loaders(ctx)` enables loading PNG/JPEG/etc. from file paths or URLs.

---

## Image Support (Cargo.toml)

```toml
egui_extras = { version = "0.33", features = ["all_loaders"] }
image = { version = "0.25", features = ["jpeg", "png"] }
```

---

## Immediate Mode Concept

egui is an **immediate mode** GUI: every frame you call widget functions directly (e.g., `if ui.button("Save").clicked() { save(); }`). There are no retained widget objects or callbacks to manage.
