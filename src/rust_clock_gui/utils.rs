use chrono::{
    DateTime, Datelike, Duration as ChronoDuration, Local, TimeDelta, TimeZone, Timelike,
};
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
