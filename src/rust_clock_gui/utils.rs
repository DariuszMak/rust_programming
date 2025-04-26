use chrono::{Datelike, Local, Timelike};
use eframe::egui;
use std::time::{Duration, SystemTime};
use std::{f32::consts::PI, ops::Add};

pub struct Time {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
}

impl Time {
    pub fn new(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
        millisecond: u32,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hours: hour,
            minutes: minute,
            seconds: second,
            milliseconds: millisecond,
        }
    }
}

pub fn convert_system_time_to_time(start_time: SystemTime) -> Time {
    let elapsed = SystemTime::now().duration_since(start_time).unwrap();
    let recalculated_start = Local::now() - chrono::Duration::from_std(elapsed).unwrap();
    let datetime = recalculated_start;

    Time::new(
        datetime.year(),
        datetime.month(),
        datetime.day(),
        datetime.hour(),
        datetime.minute(),
        datetime.second(),
        datetime.nanosecond() / 1_000_000,
    )
}

pub fn decompose_duration(diff_ms: Duration, to_seconds_only: bool) -> Time {
    let diff_ms = u32::try_from(diff_ms.as_millis()).unwrap();
    let hours = if to_seconds_only {
        0
    } else {
        diff_ms / (1000 * 60 * 60)
    };

    let remaining_ms_after_hours = diff_ms - hours * 60 * 60 * 1000;
    let minutes = if to_seconds_only {
        0
    } else {
        remaining_ms_after_hours / (1000 * 60)
    };

    let remaining_ms_after_minutes = remaining_ms_after_hours - minutes * 60 * 1000;
    let seconds = remaining_ms_after_minutes / 1000;
    let milliseconds = remaining_ms_after_minutes - seconds * 1000;

    Time::new(1970, 1, 1, hours, minutes, seconds, milliseconds)
}

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

pub fn calculate_clock_angles(time: &Time) -> HandAngles {
    let second_angle = time.seconds as f32 + time.milliseconds as f32 / 1e3;
    let minute_angle = time.minutes as f32 + second_angle / 60.0;
    let hour_angle = time.hours as f32 + minute_angle / 60.0;

    HandAngles {
        seconds: second_angle,
        minutes: minute_angle,
        hours: hour_angle,
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
