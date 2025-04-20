use chrono::DateTime;
use chrono::Local;
use chrono::Timelike;
use eframe::{egui, egui::Vec2, App};
use egui::Key;
use std::f32::consts::PI;
use std::time::Instant;

use super::polar_to_cartesian;
use super::utils::calculate_clock_angles;
use super::utils::ClockPid;
use super::ClockAngles;

pub struct ClockApp {
    start_time: Instant,
    current_time: Instant,
    pid_second: f32,
    pid_minute: f32,
    pid_hour: f32,

    second_pid: PID,
    minute_pid: PID,
    hour_pid: PID,
}

#[derive(Default)]
pub struct PID {
    kp: f32,
    ki: f32,
    kd: f32,
    prev_error: f32,
    integral: f32,
}

impl PID {
    fn update(&mut self, error: f32) -> f32 {
        self.integral += error;
        let derivative = error - self.prev_error;
        self.prev_error = error;

        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}

impl Default for ClockApp {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            current_time: now,
            pid_second: 0.0,
            pid_minute: 0.0,
            pid_hour: 0.0,
            second_pid: PID {
                kp: 0.15,
                ki: 0.005,
                kd: 0.005,
                ..Default::default()
            },
            minute_pid: PID {
                kp: 0.08,
                ki: 0.004,
                kd: 0.004,
                ..Default::default()
            },
            hour_pid: PID {
                kp: 0.08,
                ki: 0.002,
                kd: 0.002,
                ..Default::default()
            },
        }
    }
}

impl ClockApp {
    pub fn get_current_time(&self) -> Instant {
        self.current_time
    }

    pub fn tick(&mut self) {
        self.current_time = Instant::now();
    }

    pub fn get_start_time(&self) -> Instant {
        self.start_time
    }

    pub fn set_start_time(&mut self, time: Instant) {
        self.start_time = time;
    }
}

impl App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.tick();

        if ctx.input(|i| i.key_pressed(Key::R)) {
            self.start_time = Instant::now();
            self.current_time = self.start_time;
            self.pid_second = 0.0;
            self.pid_minute = 0.0;
            self.pid_hour = 0.0;
        }

        let now: DateTime<Local> = Local::now();
        let clock_angles: ClockAngles = calculate_clock_angles(now);

        let second_error = clock_angles.second - self.pid_second;
        let minute_error = clock_angles.minute - self.pid_minute;
        let hour_error = clock_angles.hour - self.pid_hour;

        self.pid_second += self.second_pid.update(second_error);
        self.pid_minute += self.minute_pid.update(minute_error);
        self.pid_hour += self.hour_pid.update(hour_error);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Analog Clock");

                let formatted_time = format!(
                    "{:02}:{:02}:{:02}.{:03}",
                    now.hour(),
                    now.minute(),
                    now.second(),
                    now.nanosecond() / 1_000_000
                );
                ui.label(egui::RichText::new(formatted_time).monospace().size(24.0));

                let (rect, _response): (egui::Rect, egui::Response) =
                    ui.allocate_exact_size(Vec2::splat(300.0), egui::Sense::hover());
                let painter = ui.painter();

                let mut center: egui::Pos2 = rect.center();
                center.y += 100.0;
                let radius = rect.width().min(rect.height()) / 1.5;

                painter.circle_stroke(
                    center,
                    radius,
                    egui::Stroke::new(2.0, ui.visuals().text_color()),
                );

                let clock = ClockPid {
                    pid_second: self.pid_second,
                    pid_minute: self.pid_minute,
                    pid_hour: self.pid_hour,
                };

                let (pid_second_angle, pid_minute_angle, pid_hour_angle) =
                    clock.angles_in_radians();

                let second_hand = polar_to_cartesian(center, radius * 0.9, pid_second_angle);
                let minute_hand = polar_to_cartesian(center, radius * 0.7, pid_minute_angle);
                let hour_hand = polar_to_cartesian(center, radius * 0.5, pid_hour_angle);

                painter.line_segment(
                    [center, hour_hand],
                    egui::Stroke::new(8.0, egui::Color32::WHITE),
                );
                painter.line_segment(
                    [center, minute_hand],
                    egui::Stroke::new(6.0, egui::Color32::LIGHT_GRAY),
                );
                painter.line_segment(
                    [center, second_hand],
                    egui::Stroke::new(2.0, egui::Color32::RED),
                );

                for i in 0..60 {
                    let angle = (i as f32 / 60.0) * 2.0 * PI;
                    let outer = polar_to_cartesian(center, radius, angle);
                    let inner = if i % 5 == 0 {
                        polar_to_cartesian(center, radius - 10.0, angle)
                    } else {
                        polar_to_cartesian(center, radius - 5.0, angle)
                    };
                    painter.line_segment(
                        [inner, outer],
                        egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
                    );
                }

                for i in 0..12 {
                    let angle = (i as f32 / 12.0) * 2.0 * PI;
                    let outer = polar_to_cartesian(center, radius, angle);
                    let inner = polar_to_cartesian(center, radius - 10.0, angle);
                    painter
                        .line_segment([inner, outer], egui::Stroke::new(1.5, egui::Color32::GRAY));

                    let text_angle = (i as f32 / 12.0) * 2.0 * PI;
                    let text_position = polar_to_cartesian(center, radius - 25.0, text_angle);

                    painter.text(
                        text_position,
                        egui::Align2::CENTER_CENTER,
                        format!("{}", ((i + 12 - 1) % 12) + 1),
                        egui::TextStyle::Heading.resolve(ui.style()),
                        egui::Color32::WHITE,
                    );
                }
            });
        });

        ctx.request_repaint();
    }
}
