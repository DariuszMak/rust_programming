use hello_world::gui_clock::ClockApp;
use std::time::Duration;

#[test]
fn test_clock_tick_updates_time() {
    let mut app = ClockApp::default();
    let initial_time = app.get_current_time();

    std::thread::sleep(Duration::from_millis(10));
    app.tick();

    assert!(app.get_current_time() > initial_time);
}
