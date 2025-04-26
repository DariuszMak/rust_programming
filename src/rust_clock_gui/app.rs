use chrono::DateTime;
use chrono::Local;
use chrono::Timelike;
use eframe::{egui, egui::Vec2, App};
use egui::Key;
use std::f32::consts::PI;
use std::time::SystemTime;

use super::polar_to_cartesian;
use super::utils::calculate_clock_angles;
use super::utils::convert_system_time_to_time;
use super::utils::decompose_duration;
use super::utils::ClockPID;
use super::utils::Time;
use super::utils::PID;
use super::HandAngles;

pub struct ClockApp {
    start_time: SystemTime,
    current_time: SystemTime,
    pid_second: f32,
    pid_minute: f32,
    pid_hour: f32,

    second_pid: PID,
    minute_pid: PID,
    hour_pid: PID,
}

impl Default for ClockApp {
    fn default() -> Self {
        let now = SystemTime::now();
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
    pub fn get_current_time(&self) -> SystemTime {
        self.current_time
    }

    pub fn tick(&mut self) {
        self.current_time = SystemTime::now();
    }

    pub fn get_start_time(&self) -> SystemTime {
        self.start_time
    }

    pub fn set_start_time(&mut self, time: SystemTime) {
        self.start_time = time;
    }

    fn reset(&mut self) {
        self.start_time = SystemTime::now();
        self.current_time = self.start_time;
        self.pid_second = 0.0;
        self.pid_minute = 0.0;
        self.pid_hour = 0.0;
        self.second_pid.reset();
        self.minute_pid.reset();
        self.hour_pid.reset();
    }
}

impl App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(Key::R)) {
            self.reset();
        }

        self.tick();

        let start_time_converted = convert_system_time_to_time(self.start_time);

        let duration = self.current_time.duration_since(self.start_time);
        let current_datetime = Local::now();

        let duration_time: Time = decompose_duration(duration.unwrap(), current_datetime, true);

        let start_time_clock_angles: HandAngles = calculate_clock_angles(&start_time_converted);
        let duration_time_clock_angles: HandAngles = calculate_clock_angles(&duration_time);

        let calculated_angles = start_time_clock_angles + duration_time_clock_angles;

        let pid_second_error = calculated_angles.seconds - self.pid_second;
        let pid_minute_error = calculated_angles.minutes - self.pid_minute;
        let pid_hour_error = calculated_angles.hours - self.pid_hour;

        self.pid_second += self.second_pid.update(pid_second_error);
        self.pid_minute += self.minute_pid.update(pid_minute_error);
        self.pid_hour += self.hour_pid.update(pid_hour_error);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Analog Clock");

                let datetime: DateTime<Local> = self.current_time.into();
                let formatted_time = format!(
                    "{:02}:{:02}:{:02}.{:03}",
                    datetime.hour(),
                    datetime.minute(),
                    datetime.second(),
                    datetime.timestamp_subsec_millis()
                );
                ui.label(egui::RichText::new(formatted_time).monospace().size(24.0));
                ui.separator();

                let available_size = ui.available_size_before_wrap();
                let size = available_size.min_elem();
                let (rect, _response): (egui::Rect, egui::Response) =
                    ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
                let painter = ui.painter();

                let center = rect.center();
                let radius = size * 0.4;

                painter.circle_stroke(
                    center,
                    radius,
                    egui::Stroke::new(2.0, ui.visuals().text_color()),
                );

                let clock = ClockPID {
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
                        egui::Stroke::new(3.0, egui::Color32::LIGHT_GRAY),
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
