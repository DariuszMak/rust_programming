use chrono::{DateTime, Local, TimeDelta, TimeZone};
use eframe::egui;

use std::{f32::consts::PI, ops::Add};

pub struct HandAngles {
    pub seconds: f32,
    pub minutes: f32,
    pub hours: f32,
}

impl Add for HandAngles {
    type Output = HandAngles;

    fn add(self, other: HandAngles) -> HandAngles {
        HandAngles {
            seconds: self.seconds + other.seconds,
            minutes: self.minutes + other.minutes,
            hours: self.hours + other.hours,
        }
    }
}

pub fn calculate_clock_angles(datetime: &DateTime<Local>, duration: &TimeDelta) -> HandAngles {
    // Start from today's midnight (but in Local timezone)
    let midnight = datetime.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let midnight_local = Local.from_local_datetime(&midnight).unwrap();

    let start_ms = datetime
        .signed_duration_since(midnight_local)
        .num_milliseconds();
    let elapsed_ms = duration.num_milliseconds();
    let total_ms = start_ms + elapsed_ms;

    let total_seconds = total_ms as f64 / 1000.0;

    let seconds_angle = total_seconds;
    let minutes_angle = total_seconds / 60.0;
    let hours_angle = total_seconds / 3600.0;

    HandAngles {
        seconds: seconds_angle as f32,
        minutes: minutes_angle as f32,
        hours: hours_angle as f32,
    }
}

#[derive(Default)]
pub struct PID {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
    pub prev_error: f32,
    pub integral: f32,
}

impl PID {
    pub fn update(&mut self, error: f32) -> f32 {
        self.integral += error;
        let derivative = error - self.prev_error;
        self.prev_error = error;

        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
    pub fn reset(&mut self) {
        self.prev_error = 0.0;
        self.integral = 0.0;
    }
}

pub fn polar_to_cartesian(center: egui::Pos2, length: f32, angle: f32) -> egui::Pos2 {
    egui::pos2(
        center.x + angle.sin() * length,
        center.y - angle.cos() * length,
    )
}

pub struct ClockPID {
    pub pid_second: f32,
    pub pid_minute: f32,
    pub pid_hour: f32,
}

impl ClockPID {
    pub fn angles_in_radians(&self) -> (f32, f32, f32) {
        let second_angle = (self.pid_second / 60.0) * 2.0 * PI;
        let minute_angle = (self.pid_minute / 60.0) * 2.0 * PI;
        let hour_angle = (self.pid_hour / 12.0) * 2.0 * PI;
        (second_angle, minute_angle, hour_angle)
    }
}
