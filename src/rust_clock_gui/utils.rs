use chrono::{DateTime, Datelike, Duration as ChronoDuration, Local, TimeDelta, Timelike};
use eframe::egui;

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

pub fn convert_system_time_to_time(start_time: DateTime<Local>) -> Time {
    let elapsed = Local::now().signed_duration_since(start_time);
    let recalculated_start = Local::now() - elapsed;

    Time::new(
        recalculated_start.year(),
        recalculated_start.month(),
        recalculated_start.day(),
        recalculated_start.hour(),
        recalculated_start.minute(),
        recalculated_start.second(),
        recalculated_start.nanosecond() / 1_000_000,
    )
}

pub fn decompose_duration(diff: TimeDelta, now: DateTime<Local>, to_seconds_only: bool) -> Time {
    let diff_ns = diff.num_nanoseconds().unwrap();
    let diff_ms = (diff_ns / 1_000_000) as u64;
    let milliseconds = (diff_ms % 1000) as u32;

    let chrono_diff = ChronoDuration::milliseconds(diff_ms as i64);
    let final_datetime = now + chrono_diff;

    if to_seconds_only {
        let total_seconds = (diff_ns / 1_000_000_000) as u32;
        return Time::new(
            final_datetime.year(),
            final_datetime.month(),
            final_datetime.day(),
            0,
            0,
            total_seconds,
            milliseconds,
        );
    }
    Time::new(
        final_datetime.year(),
        final_datetime.month(),
        final_datetime.day(),
        final_datetime.hour(),
        final_datetime.minute(),
        final_datetime.second(),
        final_datetime.timestamp_subsec_millis(),
    )
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

pub fn calculate_clock_angles(datetime: &DateTime<Local>, duration: &TimeDelta) -> HandAngles {
    let elapsed_ms = duration.num_milliseconds();

    let base_millis = datetime.timestamp_subsec_millis() as i64;
    let base_seconds = datetime.second() as i64;
    let base_minutes = datetime.minute() as i64;
    let base_hours = datetime.hour() as i64;

    let total_millis = base_millis + (elapsed_ms % 1000);
    let carry_seconds = total_millis / 1000;
    let millis = (total_millis % 1000) as f32;

    let total_seconds = base_seconds + (elapsed_ms / 1000) % 60 + carry_seconds;
    let carry_minutes = total_seconds / 60;
    let seconds = (total_seconds % 60) as f32;

    let total_minutes = base_minutes + (elapsed_ms / (1000 * 60)) % 60 + carry_minutes;
    let carry_hours = total_minutes / 60;
    let minutes = (total_minutes % 60) as f32;

    let hours = (base_hours + (elapsed_ms / (1000 * 60 * 60)) + carry_hours) as f32;

    let second_angle = seconds + millis / 1e3;
    let minute_angle = minutes + second_angle / 60.0;
    let hour_angle = hours + minute_angle / 60.0;

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
