use chrono::Local;
use chrono::Timelike;
use eframe::{egui, egui::Vec2, App};
use std::f32::consts::PI;
use std::time::Instant;

pub struct ClockApp {
    start_time: Instant,
    current_time: Instant,
}

impl Default for ClockApp {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            current_time: now,
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Analog Clock");
                let (rect, _response) =
                    ui.allocate_exact_size(Vec2::splat(300.0), egui::Sense::hover());
                let painter = ui.painter();

                let center = rect.center();
                let radius = rect.width().min(rect.height()) / 2.0 - 10.0;

                painter.circle_stroke(
                    center,
                    radius,
                    egui::Stroke::new(2.0, ui.visuals().text_color()),
                );

                let now = Local::now();
                let second = now.second() as f32;
                let minute = now.minute() as f32 + second / 60.0;
                let hour = now.hour() as f32 + minute / 60.0;

                let second_angle = (second / 60.0) * 2.0 * PI;
                let minute_angle = (minute / 60.0) * 2.0 * PI;
                let hour_angle = (hour / 12.0) * 2.0 * PI;

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

                for i in 0..12 {
                    let angle = (i as f32 / 12.0) * 2.0 * PI;
                    let outer = polar_to_cartesian(center, radius, angle);
                    let inner = polar_to_cartesian(center, radius - 10.0, angle);
                    painter
                        .line_segment([inner, outer], egui::Stroke::new(1.5, egui::Color32::GRAY));
                }
            });
        });

        ctx.request_repaint();
    }
}

pub fn polar_to_cartesian(center: egui::Pos2, length: f32, angle: f32) -> egui::Pos2 {
    egui::pos2(
        center.x + angle.sin() * length,
        center.y - angle.cos() * length,
    )
}
