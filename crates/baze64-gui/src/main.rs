#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, epaint::Vec2, NativeOptions};

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let options = NativeOptions {
        initial_window_size: Some(Vec2::new(320.0, 230.0)),
        ..Default::default()
    };

    eframe::run_native("Baze64", options, Box::new(|_cc| Box::<App>::default()))
}

#[derive(Debug, Default)]
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.heading("Baze64"));
    }
}
