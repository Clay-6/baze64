#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::fmt;

use baze64::{
    alphabet::{self, Standard, UrlSafe},
    Base64String,
};

use eframe::{egui, epaint::Vec2, NativeOptions};
use tracing::info;

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
    alphabet: Alphabet,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Baze64");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    let input_label = ui.label("Plaintext");
                    ui.text_edit_multiline(&mut self.plaintext)
                        .labelled_by(input_label.id);
                });

                ui.vertical(|ui| {
                    if ui.button("-> Encode").clicked() {
                        info!("Encode button clicked");
                        let encoded =
                            Base64String::encode_with(self.plaintext.as_bytes(), self.alphabet);
                        self.base64 = match encoded {
                            Ok(t) => t.to_string(),
                            Err(e) => format!("Error: {e}"),
                        };
                    }
                    if ui.button("Decode <-").clicked() {
                        info!("Decode button clicked");

                        let decoded =
                            Base64String::from_encoded_with(&self.base64, self.alphabet).decode();
                        self.plaintext = match decoded {
                            Ok(d) => String::from_utf8_lossy(&d).to_string(),
                            Err(e) => format!("Error: {e}"),
                        };
                    }
                });
                ui.vertical(|ui| {
                    let encoded_label = ui.label("Base64");
                    ui.text_edit_multiline(&mut self.base64)
                        .labelled_by(encoded_label.id);

                    egui::ComboBox::from_label("Base64 Alphabet")
                        .selected_text(self.alphabet.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.alphabet, Alphabet::Standard, "Standard");
                            ui.selectable_value(&mut self.alphabet, Alphabet::UrlSafe, "URL Safe");
                        });
                })
            });
        });
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Alphabet {
    #[default]
    Standard,
    UrlSafe,
}

impl alphabet::Alphabet for Alphabet {
    fn padding(&self) -> Option<char> {
        match self {
            Alphabet::Standard => Standard::new().padding(),
            Alphabet::UrlSafe => UrlSafe::new().padding(),
        }
    }

    fn encode_bits(&self, bits: u8) -> Result<char, baze64::B64Error> {
        match self {
            Alphabet::Standard => Standard::new().encode_bits(bits),
            Alphabet::UrlSafe => UrlSafe::new().encode_bits(bits),
        }
    }

    fn decode_char(&self, c: char) -> Result<u8, baze64::B64Error> {
        match self {
            Alphabet::Standard => Standard::new().decode_char(c),
            Alphabet::UrlSafe => UrlSafe::new().decode_char(c),
        }
    }
}

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Standard => "Standard",
                Self::UrlSafe => "URL Safe",
            }
        )
    }
}
