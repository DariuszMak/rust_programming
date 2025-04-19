use eframe::{egui, App};
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

// ✅ Re-add the getters for testing
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
        let elapsed = self.current_time.duration_since(self.start_time);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Clock App");
            ui.label(format!("Elapsed: {:.2?}", elapsed));
        });

        ctx.request_repaint();
    }
}
