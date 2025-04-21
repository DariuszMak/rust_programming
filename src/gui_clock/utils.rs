use chrono::{Local, Timelike};
use eframe::egui;
use std::time::{Duration, Instant};
use std::{f32::consts::PI, ops::Add};

pub fn decompose_duration(diff_ms: Duration) -> Time {
    let diff_ms = diff_ms.as_millis() as u32;
    let hours = diff_ms / (1000 * 60 * 60);
    let remaining_ms_after_hours = diff_ms - hours * 60 * 60 * 1000;

    let minutes = remaining_ms_after_hours / (1000 * 60);
    let remaining_ms_after_minutes = remaining_ms_after_hours - minutes * 60 * 1000;

    let seconds = remaining_ms_after_minutes / 1000;
    let milliseconds = remaining_ms_after_minutes - seconds * 1000;

    Time::new(hours, minutes, seconds, milliseconds)
}

pub fn convert_instant_to_time(start_time: Instant) -> Time {
    let elapsed = Instant::now().duration_since(start_time);
    let recalculated_start = Local::now() - chrono::Duration::from_std(elapsed).unwrap();
    let t = recalculated_start.time();
    Time::new(t.hour(), t.minute(), t.second(), t.nanosecond() / 1_000_000)
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

pub struct ClockPid {
    pub pid_second: f32,
    pub pid_minute: f32,
    pub pid_hour: f32,
}

impl ClockPid {
    pub fn angles_in_radians(&self) -> (f32, f32, f32) {
        let second_angle = (self.pid_second / 60.0) * 2.0 * PI;
        let minute_angle = (self.pid_minute / 60.0) * 2.0 * PI;
        let hour_angle = (self.pid_hour / 12.0) * 2.0 * PI;
        (second_angle, minute_angle, hour_angle)
    }
}
pub struct Time {
    pub millisecond: u32,
    pub second: u32,
    pub minute: u32,
    pub hour: u32,
}

impl Time {
    pub fn new(hour: u32, minute: u32, second: u32, millisecond: u32) -> Self {
        Self {
            millisecond,
            second,
            minute,
            hour,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ClockAngles {
    pub second: f32,
    pub minute: f32,
    pub hour: f32,
}

impl Add for ClockAngles {
    type Output = ClockAngles;

    fn add(self, other: ClockAngles) -> ClockAngles {
        ClockAngles {
            second: self.second + other.second,
            minute: self.minute + other.minute,
            hour: self.hour + other.hour,
        }
    }
}
pub fn calculate_clock_angles(time: &Time) -> ClockAngles {
    let second_angle = time.second as f32 + time.millisecond as f32 / 1e3;
    let minute_angle = time.minute as f32 + second_angle / 60.0;
    let hour_angle = time.hour as f32 + minute_angle / 60.0;

    ClockAngles {
        second: second_angle,
        minute: minute_angle,
        hour: hour_angle,
    }
}
