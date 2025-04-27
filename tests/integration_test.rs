#[cfg(test)]
mod tests {
    use chrono::Local;
    use chrono::TimeDelta;
    use chrono::TimeZone;

    use eframe::egui::pos2;
    use rust_clock_gui::rust_clock_gui::calculate_clock_angles;
    use rust_clock_gui::rust_clock_gui::polar_to_cartesian;

    use rust_clock_gui::rust_clock_gui::utils::ClockPID;
    use rust_clock_gui::rust_clock_gui::ClockApp;
    use rust_clock_gui::rust_clock_gui::PID;
    use std::f32::consts::PI;
    use std::time::Duration;

    fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
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
    fn test_midnight_clock_angles() {
        let datetime = Local.with_ymd_and_hms(2025, 4, 27, 0, 0, 0).unwrap();
        let duration = TimeDelta::zero();

        let angles = calculate_clock_angles(&datetime, &duration);

        assert_eq!(angles.seconds, 0.0);
        assert_eq!(angles.minutes, 0.0);
        assert_eq!(angles.hours, 0.0);
    }

    #[test]
    fn test_noon_clock_angles() {
        let datetime = Local.with_ymd_and_hms(2025, 4, 27, 12, 0, 0).unwrap();
        let duration = TimeDelta::zero();

        let angles = calculate_clock_angles(&datetime, &duration);

        assert_eq!(angles.seconds, 0.0);
        assert_eq!(angles.minutes, 0.0);
        assert_eq!(angles.hours, 0.0);
    }

    #[test]
    fn test_noon_clock_angles_from_milliseconds() {
        let datetime = Local.with_ymd_and_hms(2025, 4, 27, 0, 0, 0).unwrap();
        let duration = TimeDelta::milliseconds(12 * 60 * 60 * 1000);

        let angles = calculate_clock_angles(&datetime, &duration);

        assert_eq!(angles.seconds, 43200.0);
        assert_eq!(angles.minutes, 720.0);
        assert_eq!(angles.hours, 12.0);
    }

    #[test]
    fn test_maximum_clock_angles() {
        let datetime = Local.with_ymd_and_hms(2025, 4, 27, 23, 59, 59).unwrap();
        let duration = TimeDelta::zero();

        let angles = calculate_clock_angles(&datetime, &duration);

        assert_eq!(angles.seconds, 59.0);
        assert_eq!(angles.minutes, 59.983334);
        assert_eq!(angles.hours, 11.9997225);
    }

    #[test]
    fn test_maximum_clock_angles_from_milliseconds() {
        let datetime = Local.with_ymd_and_hms(2025, 4, 27, 0, 0, 0).unwrap();
        let duration =
            TimeDelta::milliseconds(23 * 60 * 60 * 1000 + 59 * 60 * 1000 + 59 * 1000 + 999);

        let angles = calculate_clock_angles(&datetime, &duration);

        assert_eq!(angles.seconds, 86400.0);
        assert_eq!(angles.minutes, 1440.0);
        assert_eq!(angles.hours, 24.0);
    }

    #[test]
    fn test_half_past_three_clock_angles() {
        let datetime = Local.with_ymd_and_hms(2025, 4, 27, 3, 30, 0).unwrap();
        let duration = TimeDelta::zero();

        let angles = calculate_clock_angles(&datetime, &duration);

        assert_eq!(angles.seconds, 0.0);
        assert_eq!(angles.minutes, 30.0);
        assert_eq!(angles.hours, 3.5);
    }

    #[test]
    fn test_half_past_three_clock_angles_from_milliseconds() {
        let datetime = Local.with_ymd_and_hms(2025, 4, 27, 0, 00, 0).unwrap();
        let duration = TimeDelta::milliseconds(3 * 60 * 60 * 1000 + 30 * 60 * 1000);

        let angles = calculate_clock_angles(&datetime, &duration);

        assert_eq!(angles.seconds, 12600.0);
        assert_eq!(angles.minutes, 210.0);
        assert_eq!(angles.hours, 3.5);
    }

    #[test]
    fn test_circled_clock_angles() {
        let datetime = Local.with_ymd_and_hms(2025, 4, 27, 0, 0, 0).unwrap();
        let duration =
            TimeDelta::milliseconds(33 * 60 * 60 * 1000 + 65 * 60 * 1000 + 61 * 1000 + 2);

        let angles = calculate_clock_angles(&datetime, &duration);

        assert_eq!(angles.seconds, 122761.0);
        assert_eq!(angles.minutes, 2046.0167);
        assert_eq!(angles.hours, 34.100277);
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
        let clock_pid = ClockPID {
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
        let clock = ClockPID {
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
        let clock = ClockPID {
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
        let clock = ClockPID {
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
        let clock = ClockPID {
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
