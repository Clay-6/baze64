#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use baze64::{alphabet::Standard, Base64String};
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
struct App {
    input: String,
    output: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Baze64");
            let input_label = ui.label("Your input");
            ui.text_edit_multiline(&mut self.input)
                .labelled_by(input_label.id);
            ui.horizontal(|ui| {
                if ui.button("Encode").clicked() {
                    let encoded = Base64String::<Standard>::encode(self.input.as_bytes()).unwrap();
                    self.output = encoded.to_string();
                }
                if ui.button("Decode").clicked() {
                    let decoded = Base64String::<Standard>::from_encoded(&self.input)
                        .decode()
                        .unwrap();
                    self.output = String::from_utf8_lossy(&decoded).to_string();
                }
            });
            ui.heading(&self.output);
        });
    }
}
