#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use baze64::{alphabet::Standard, Base64String};
use eframe::{egui, epaint::Vec2, NativeOptions};

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let options = NativeOptions {
        initial_window_size: Some(Vec2::new(770.0, 200.0)),
        ..Default::default()
    };

    eframe::run_native("Baze64", options, Box::new(|_cc| Box::<App>::default()))
}

#[derive(Debug, Default)]
struct App {
    plaintext: String,
    base64: String,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Baze64");

            ui.horizontal(|ui| {
                let input_label = ui.label("Plaintext");
                ui.text_edit_multiline(&mut self.plaintext)
                    .labelled_by(input_label.id);

                ui.vertical(|ui| {
                    if ui.button("-> Encode").clicked() {
                        let encoded = Base64String::<Standard>::encode(self.plaintext.as_bytes());
                        self.base64 = match encoded {
                            Ok(t) => t.to_string(),
                            Err(e) => format!("Error: {e}"),
                        };
                    }
                    if ui.button("Decode <-").clicked() {
                        let decoded = Base64String::<Standard>::from_encoded(&self.base64).decode();
                        self.plaintext = match decoded {
                            Ok(d) => String::from_utf8_lossy(&d).to_string(),
                            Err(e) => format!("Error: {e}"),
                        };
                    }
                });

                let encoded_label = ui.label("Base64");
                ui.text_edit_multiline(&mut self.base64)
                    .labelled_by(encoded_label.id);
            });
        });
    }
}