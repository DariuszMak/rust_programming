use hello_world::gui_clock::ClockApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "ClockApp",
        options,
        Box::new(|_cc| Ok(Box::new(ClockApp::default()))),
    )
}
