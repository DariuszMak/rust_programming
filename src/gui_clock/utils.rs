use eframe::egui;
use std::{f32::consts::PI, ops::Add};

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
    pub milisecond: f32,
    pub second: f32,
    pub minute: f32,
    pub hour: f32,
}

impl Time {
    pub fn new(hour: u32, minute: u32, second: u32, milisecond: u32) -> Self {
        Self {
            milisecond: milisecond as f32,
            second: second as f32,
            minute: minute as f32,
            hour: hour as f32,
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
    let second_angle = time.second + time.milisecond / 1e3;
    let minute_angle = time.minute + second_angle / 60.0;
    let hour_angle = time.hour + minute_angle / 60.0;

    ClockAngles {
        second: second_angle,
        minute: minute_angle,
        hour: hour_angle,
    }
}
