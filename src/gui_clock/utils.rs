use chrono::{DateTime, Local, Timelike};
use eframe::egui;

pub fn polar_to_cartesian(center: egui::Pos2, length: f32, angle: f32) -> egui::Pos2 {
    egui::pos2(
        center.x + angle.sin() * length,
        center.y - angle.cos() * length,
    )
}

use std::f32::consts::PI;

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

pub struct ClockAngles {
    pub second: f32,
    pub minute: f32,
    pub hour: f32,
}

pub fn calculate_clock_angles(time: DateTime<Local>) -> ClockAngles {
    let second = time.second() as f32 + time.nanosecond() as f32 / 1e9;
    let minute = time.minute() as f32 + second / 60.0;
    let hour = time.hour() as f32 + minute / 60.0;

    ClockAngles {
        second,
        minute,
        hour,
    }
}
