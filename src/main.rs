use eframe::egui::viewport::IconData;
use eframe::egui::{Vec2, ViewportBuilder};
use rust_clock_gui::rust_clock_gui::ClockApp;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let icon = load_icon()
        .map(Arc::new)
        .expect("Failed to load embedded icon");

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

fn load_icon() -> Option<IconData> {
    let bytes = include_bytes!("icons/timer.png");

    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
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
