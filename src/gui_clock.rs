use eframe::{egui, App};
use std::time::Instant;

pub struct ClockApp {
    current_time: Instant,
    last_update: Instant,
}

impl Default for ClockApp {
    fn default() -> Self {
        Self {
            current_time: Instant::now(),
            last_update: Instant::now(),
        }
    }
}

impl ClockApp {
    pub fn get_current_time(&self) -> Instant {
        self.current_time
    }

    pub fn get_last_update(&self) -> Instant {
        self.last_update
    }

    pub fn set_last_update(&mut self, time: Instant) {
        self.last_update = time;
    }
}

impl App for ClockApp {
    fn update(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.current_time = Instant::now();
    }
}
