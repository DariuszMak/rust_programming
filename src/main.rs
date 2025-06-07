use eframe::egui::viewport::IconData;
use eframe::egui::{Vec2, ViewportBuilder};
use rust_clock_gui::rust_clock_gui::ClockApp;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let icon = load_icon("src/icons/timer.png")
        .map(Arc::new)
        .expect("Failed to load icon");

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(Vec2::new(500.0, 500.0))
            .with_min_inner_size(Vec2::new(300.0, 300.0))
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "ClockApp",
        options,
        Box::new(|_cc| Ok(Box::new(ClockApp::default()))),
    )
}

fn load_icon<P: AsRef<Path>>(path: P) -> Option<IconData> {
    let reader = BufReader::new(File::open(path).ok()?);
    let image = image::load(reader, image::ImageFormat::Png)
        .ok()?
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    Some(IconData {
        rgba,
        width,
        height,
    })
}
