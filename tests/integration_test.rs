#[cfg(test)]
mod tests {
    use chrono::{Local, TimeZone};
    use eframe::egui::pos2;
    use gui_clock::gui_clock::calculate_clock_angles;
    use gui_clock::gui_clock::polar_to_cartesian;
    use gui_clock::gui_clock::ClockApp;
    use std::time::Duration;

    fn round_f32(v: f32, decimals: u32) -> f32 {
        let factor = 10f32.powi(decimals as i32);
        (v * factor).round() / factor
    }

    #[test]
    fn test_midnight_angles() {
        let time = Local.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let angles = calculate_clock_angles(time);
        assert_eq!(angles.second, 0.0);
        assert_eq!(angles.minute, 0.0);
        assert_eq!(angles.hour, 0.0);
    }

    #[test]
    fn test_noon_angles() {
        let time = Local.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let angles = calculate_clock_angles(time);
        assert_eq!(angles.second, 0.0);
        assert_eq!(angles.minute, 0.0);
        assert_eq!(angles.hour, 12.0);
    }

    #[test]
    fn test_half_past_three() {
        let time = Local.with_ymd_and_hms(2023, 1, 1, 3, 30, 0).unwrap();
        let angles = calculate_clock_angles(time);
        assert_eq!(round_f32(angles.second, 2), 0.0);
        assert_eq!(round_f32(angles.minute, 2), 30.0);
        assert_eq!(round_f32(angles.hour, 2), 3.5);
    }

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
