use chrono::Local;
use chrono::Timelike;
use eframe::{egui, egui::Vec2, App};
use egui::Key;
use std::f32::consts::PI;
use std::time::Instant;

use super::polar_to_cartesian;

pub struct ClockApp {
    start_time: Instant,
    current_time: Instant,
    smooth_second: f32,
    smooth_minute: f32,
    smooth_hour: f32,

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
            smooth_second: 0.0,
            smooth_minute: 0.0,
            smooth_hour: 0.0,
            second_pid: PID {
                kp: 0.05,
                ki: 0.005,
                kd: 0.002,
                ..Default::default()
            },
            minute_pid: PID {
                kp: 0.15,
                ki: 0.015,
                kd: 0.01,
                ..Default::default()
            },
            hour_pid: PID {
                kp: 0.3,
                ki: 0.02,
                kd: 0.05,
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
            let now = Instant::now();
            self.start_time = now;
            self.current_time = now;
            self.smooth_second = 0.0;
            self.smooth_minute = 0.0;
            self.smooth_hour = 0.0;
        }

        let now = Local::now();
        let second = now.second() as f32 + now.nanosecond() as f32 / 1e9;
        let minute = now.minute() as f32 + second / 60.0;
        let hour = now.hour() as f32 + minute / 60.0;

        let second_error = second - self.smooth_second;
        let minute_error = minute - self.smooth_minute;
        let hour_error = hour - self.smooth_hour;

        self.smooth_second += self.second_pid.update(second_error);
        self.smooth_minute += self.minute_pid.update(minute_error);
        self.smooth_hour += self.hour_pid.update(hour_error);

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

                let (rect, _response) =
                    ui.allocate_exact_size(Vec2::splat(300.0), egui::Sense::hover());
                let painter = ui.painter();

                let mut center = rect.center();
                center.y += 100.0;
                let radius = rect.width().min(rect.height()) / 1.5;

                painter.circle_stroke(
                    center,
                    radius,
                    egui::Stroke::new(2.0, ui.visuals().text_color()),
                );

                let second_angle = (self.smooth_second / 60.0) * 2.0 * PI;
                let minute_angle = (self.smooth_minute / 60.0) * 2.0 * PI;
                let hour_angle = (self.smooth_hour / 12.0) * 2.0 * PI;

                let second_hand = polar_to_cartesian(center, radius * 0.9, second_angle);
                let minute_hand = polar_to_cartesian(center, radius * 0.7, minute_angle);
                let hour_hand = polar_to_cartesian(center, radius * 0.5, hour_angle);

                painter.line_segment(
                    [center, hour_hand],
                    egui::Stroke::new(4.0, egui::Color32::WHITE),
                );
                painter.line_segment(
                    [center, minute_hand],
                    egui::Stroke::new(3.0, egui::Color32::LIGHT_GRAY),
                );
                painter.line_segment(
                    [center, second_hand],
                    egui::Stroke::new(1.0, egui::Color32::RED),
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
