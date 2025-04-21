#[cfg(test)]
mod tests {
    use chrono::Local;
    use chrono::Timelike;
    use eframe::egui::pos2;
    use gui_clock::gui_clock::calculate_clock_angles;
    use gui_clock::gui_clock::polar_to_cartesian;
    use gui_clock::gui_clock::utils::convert_instant_to_time;
    use gui_clock::gui_clock::utils::ClockPid;
    use gui_clock::gui_clock::utils::Time;
    use gui_clock::gui_clock::ClockApp;
    use gui_clock::gui_clock::PID;
    use std::f32::consts::PI;
    use std::thread;
    use std::time::Duration;
    use std::time::Instant;

    fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }

    fn make_time(hour: u32, minute: u32, second: u32) -> Time {
        Time::new(hour, minute, second, 0)
    }

    #[test]
    fn test_convert_instant_to_time_recent() {
        let start_time = Instant::now();
        thread::sleep(Duration::from_millis(10));
        let result = convert_instant_to_time(start_time);
        let now = Local::now().time();
        let expected = Time::new(
            now.hour(),
            now.minute(),
            now.second(),
            now.nanosecond() / 1_000_000,
        );

        assert_eq!(result.hour, expected.hour);
        assert_eq!(result.minute, expected.minute);
        assert_eq!(result.second, expected.second);
    }

    #[test]
    fn test_convert_instant_to_time_after_delay() {
        let start_time = Instant::now();
        thread::sleep(Duration::from_secs(1));
        let result = convert_instant_to_time(start_time);
        let now = Local::now().time();
        let expected = Time::new(
            now.hour(),
            now.minute(),
            now.second(),
            now.nanosecond() / 1_000_000,
        );

        assert_eq!(result.hour, expected.hour);
        assert_eq!(result.minute, expected.minute);
        assert!(result.second == expected.second || result.second == expected.second - 1.0);
    }
    
    #[test]
    fn test_pid_update() {
        let mut pid = PID {
            kp: 1.0,
            ki: 0.1,
            kd: 0.5,
            ..Default::default()
        };

        let output1 = pid.update(1.0);
        assert!((output1 - (1.0 + 0.1 + 0.5)).abs() < f32::EPSILON);

        let output2 = pid.update(0.5);
        let expected = 0.5 * pid.kp + (1.0 + 0.5) * pid.ki + (0.5 - 1.0) * pid.kd;
        assert!((output2 - expected).abs() < f32::EPSILON);
    }

    #[test]
    fn test_clock_pid_angles_in_radians() {
        let clock_pid = ClockPid {
            pid_second: 15.0,
            pid_minute: 30.0,
            pid_hour: 6.0,
        };

        let (sec_rad, min_rad, hour_rad) = clock_pid.angles_in_radians();

        assert!((sec_rad - std::f32::consts::PI / 2.0).abs() < 1e-6);
        assert!((min_rad - std::f32::consts::PI).abs() < 1e-6);
        assert!((hour_rad - std::f32::consts::PI).abs() < 1e-6);
    }

    #[test]
    fn test_angles_at_zero() {
        let clock = ClockPid {
            pid_second: 0.0,
            pid_minute: 0.0,
            pid_hour: 0.0,
        };
        let (s, m, h) = clock.angles_in_radians();
        assert!(approx_eq(s, 0.0, 1e-10));
        assert!(approx_eq(m, 0.0, 1e-10));
        assert!(approx_eq(h, 0.0, 1e-10));
    }

    #[test]
    fn test_angles_at_halfway() {
        let clock = ClockPid {
            pid_second: 30.0,
            pid_minute: 30.0,
            pid_hour: 6.0,
        };
        let (s, m, h) = clock.angles_in_radians();
        assert!(approx_eq(s, PI, 1e-10));
        assert!(approx_eq(m, PI, 1e-10));
        assert!(approx_eq(h, PI, 1e-10));
    }

    #[test]
    fn test_angles_at_full() {
        let clock = ClockPid {
            pid_second: 60.0,
            pid_minute: 60.0,
            pid_hour: 12.0,
        };
        let (s, m, h) = clock.angles_in_radians();
        assert!(approx_eq(s, 2.0 * PI, 1e-10));
        assert!(approx_eq(m, 2.0 * PI, 1e-10));
        assert!(approx_eq(h, 2.0 * PI, 1e-10));
    }

    #[test]
    fn test_quarter_angles() {
        let clock = ClockPid {
            pid_second: 15.0,
            pid_minute: 15.0,
            pid_hour: 3.0,
        };
        let (s, m, h) = clock.angles_in_radians();
        assert!(approx_eq(s, 0.25 * 2.0 * PI, 1e-10));
        assert!(approx_eq(m, 0.25 * 2.0 * PI, 1e-10));
        assert!(approx_eq(h, 0.25 * 2.0 * PI, 1e-10));
    }

    #[test]
    fn test_midnight_angles() {
        let time = make_time(0, 0, 0);
        let angles = calculate_clock_angles(&time);
        assert_eq!(angles.second, 0.0);
        assert_eq!(angles.minute, 0.0);
        assert_eq!(angles.hour, 0.0);
    }

    #[test]
    fn test_noon_angles() {
        let time = make_time(12, 0, 0);
        let angles = calculate_clock_angles(&time);
        assert_eq!(angles.second, 0.0);
        assert_eq!(angles.minute, 0.0);
        assert_eq!(angles.hour, 12.0);
    }

    #[test]
    fn test_maximum_angles() {
        let time = make_time(23, 59, 59);
        let angles = calculate_clock_angles(&time);
        assert_eq!(angles.second, 59.0);
        assert_eq!(angles.minute, 59.983334);
        assert_eq!(angles.hour, 23.999722);
    }

    #[test]
    fn test_half_past_three() {
        let time = make_time(3, 30, 0);
        let angles = calculate_clock_angles(&time);
        assert_eq!(angles.second, 0.0);
        assert_eq!(angles.minute, 30.0);
        assert_eq!(angles.hour, 3.5);
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
        let center = pos2(100.0, 100.0);
        let result = polar_to_cartesian(center, 50.0, 0.0);
        assert!((result.x - center.x).abs() < f32::EPSILON);
        assert!((result.y - (center.y - 50.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn test_polar_to_cartesian_quarter_angle() {
        let center = pos2(0.0, 0.0);
        let result = polar_to_cartesian(center, 1.0, std::f32::consts::FRAC_PI_2);
        assert!((result.x - 1.0).abs() < f32::EPSILON);
        assert!(result.y.abs() < f32::EPSILON);
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
