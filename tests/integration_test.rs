#[cfg(test)]
mod tests {
    use chrono::Local;
    use chrono::Timelike;
    use eframe::egui::pos2;
    use gui_clock::gui_clock::calculate_clock_angles;
    use gui_clock::gui_clock::polar_to_cartesian;
    use gui_clock::gui_clock::utils::convert_instant_to_time;
    use gui_clock::gui_clock::utils::decompose_duration;
    use gui_clock::gui_clock::utils::ClockPID;
    use gui_clock::gui_clock::utils::Time;
    use gui_clock::gui_clock::ClockApp;
    use gui_clock::gui_clock::PID;
    use std::f32::consts::PI;
    use std::time::Duration;
    use std::time::Instant;

    fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }

    #[test]
    fn test_decompose_from_milliseconds() {
        let duration = Duration::from_millis(1001);

        let components = decompose_duration(duration, false);
        assert_eq!(components.hours, 0);
        assert_eq!(components.minutes, 0);
        assert_eq!(components.seconds, 1);
        assert_eq!(components.milliseconds, 1);
    }

    #[test]
    fn test_decompose_one_minute_and_a_second() {
        let duration = Duration::from_secs(61);

        let components = decompose_duration(duration, false);
        assert_eq!(components.hours, 0);
        assert_eq!(components.minutes, 1);
        assert_eq!(components.seconds, 1);
        assert_eq!(components.milliseconds, 0);
    }

    #[test]
    fn test_decompose_exact_one_hour() {
        let duration = Duration::from_secs(3600);

        let components = decompose_duration(duration, false);
        assert_eq!(components.hours, 1);
        assert_eq!(components.minutes, 0);
        assert_eq!(components.seconds, 0);
        assert_eq!(components.milliseconds, 0);
    }

    #[test]
    fn test_decompose_to_seconds_only() {
        let duration = Duration::from_millis(2 * 60 * 60 * 1000 + 34 * 60 * 1000 + 56 * 1000 + 789);

        let components = decompose_duration(duration, true);
        assert_eq!(components.hours, 0);
        assert_eq!(components.minutes, 0);
        assert_eq!(components.seconds, 9296);
        assert_eq!(components.milliseconds, 789);
    }

    #[test]
    fn test_decompose_hours_minutes_seconds_millis() {
        let duration = Duration::from_millis(2 * 60 * 60 * 1000 + 34 * 60 * 1000 + 56 * 1000 + 789);

        let components = decompose_duration(duration, false);
        assert_eq!(components.hours, 2);
        assert_eq!(components.minutes, 34);
        assert_eq!(components.seconds, 56);
        assert_eq!(components.milliseconds, 789);
    }

    #[test]
    fn test_convert_instant_to_time_simulated_delay() {
        let simulated_delay = Duration::from_secs(1);
        let start_time = Instant::now() - simulated_delay;
        let result = convert_instant_to_time(start_time);

        let now = Local::now().time();
        let expected = Time::new(
            now.hour(),
            now.minute(),
            now.second().saturating_sub(1),
            now.nanosecond() / 1_000_000,
        );

        assert_eq!(result.hours, expected.hours);
        assert_eq!(result.minutes, expected.minutes);
        assert!(result.seconds == expected.seconds || result.seconds == expected.seconds + 1);
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
    fn test_midnight_clock_angles() {
        let time: Time = Time::new(0, 0, 0, 0);
        let angles = calculate_clock_angles(&time);
        assert_eq!(angles.seconds, 0.0);
        assert_eq!(angles.minutes, 0.0);
        assert_eq!(angles.hours, 0.0);
    }

    #[test]
    fn test_noon_clock_angles() {
        let time: Time = Time::new(12, 0, 0, 0);
        let angles = calculate_clock_angles(&time);
        assert_eq!(angles.seconds, 0.0);
        assert_eq!(angles.minutes, 0.0);
        assert_eq!(angles.hours, 12.0);
    }

    #[test]
    fn test_maximum_clock_angles() {
        let time: Time = Time::new(23, 59, 59, 0);
        let angles = calculate_clock_angles(&time);
        assert_eq!(angles.seconds, 59.0);
        assert_eq!(angles.minutes, 59.983334);
        assert_eq!(angles.hours, 23.999722);
    }

    #[test]
    fn test_half_past_three_clock_angles() {
        let time: Time = Time::new(3, 30, 0, 0);
        let angles = calculate_clock_angles(&time);
        assert_eq!(angles.seconds, 0.0);
        assert_eq!(angles.minutes, 30.0);
        assert_eq!(angles.hours, 3.5);
    }

    #[test]
    fn test_circled_clock_angles() {
        let time: Time = Time::new(33, 65, 61, 2);
        let angles = calculate_clock_angles(&time);
        assert_eq!(angles.seconds, 61.002);
        assert_eq!(angles.minutes, 66.0167);
        assert_eq!(angles.hours, 34.100277);
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
