use eframe::egui::{Vec2, ViewportBuilder};
use rust_clock_gui::rust_clock_gui::ClockApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_min_inner_size(Vec2::new(300.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(
        "ClockApp",
        options,
        Box::new(|_cc| Ok(Box::new(ClockApp::default()))),
    )
}
