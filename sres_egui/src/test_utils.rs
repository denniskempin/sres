use std::path::PathBuf;

use egui_kittest::{Harness, SnapshotOptions};

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
    let mut harness = Harness::builder().build_ui(ui_fn);
    harness.fit_contents();
    harness.snapshot_options(name, &snapshot_options());
}
