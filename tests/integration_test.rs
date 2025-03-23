use eframe::{egui, App};
use hello_world::gui_clock::ClockApp;
use std::time::{Duration, Instant};

#[test]
fn test_clock_update() {
    let mut app = ClockApp::default();
    let initial_time = app.get_current_time(); // Use getter

    // Simulate passage of time
    app.set_last_update(Instant::now() - Duration::from_secs(2));

    // Create an `egui::Context`
    let ctx = egui::Context::default();

    let mut frame = eframe::Frame::_new_kittest();

    // Call update
    app.update(&ctx, &mut frame);

    assert_ne!(initial_time, app.get_current_time()); // Ensure time updated
}
