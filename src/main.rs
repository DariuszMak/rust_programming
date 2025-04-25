use rust_clock_gui::rust_clock_gui::ClockApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ClockApp",
        options,
        Box::new(|_cc| Ok(Box::new(ClockApp::default()))),
    )
}
