#[cfg(test)]
mod tests {
    use eframe::egui::pos2;
    use hello_world::gui_clock::polar_to_cartesian;
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

    #[test]
    fn test_polar_to_cartesian_zero_angle() {
        let center = pos2(0.0, 0.0);
        let length = 10.0;
        let angle = 0.0;
        let result = polar_to_cartesian(center, length, angle);
        let expected = pos2(0.0, -10.0);
        assert!((result.x - expected.x).abs() < 1e-5);
        assert!((result.y - expected.y).abs() < 1e-5);
    }

    #[test]
    fn test_polar_to_cartesian_90_degrees() {
        let center = pos2(0.0, 0.0);
        let length = 10.0;
        let angle = std::f32::consts::FRAC_PI_2;
        let result = polar_to_cartesian(center, length, angle);
        let expected = pos2(10.0, 0.0);
        assert!((result.x - expected.x).abs() < 1e-5);
        assert!((result.y - expected.y).abs() < 1e-5);
    }

    #[test]
    fn test_polar_to_cartesian_180_degrees() {
        let center = pos2(0.0, 0.0);
        let length = 10.0;
        let angle = std::f32::consts::PI;
        let result = polar_to_cartesian(center, length, angle);
        let expected = pos2(0.0, 10.0);
        assert!((result.x - expected.x).abs() < 1e-5);
        assert!((result.y - expected.y).abs() < 1e-5);
    }
}
