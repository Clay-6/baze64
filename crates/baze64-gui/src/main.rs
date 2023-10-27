#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use baze64::{
    alphabet::{Alphabet, Standard, UrlSafe},
    Base64String,
};
use tracing::{debug, error, info};

slint::include_modules!();

fn main() {
    tracing_subscriber::fmt().init();

    let main_window = MainWindow::new().unwrap();
    debug!("main window created");

    let mw_weak = main_window.as_weak();
    main_window.on_encode_plaintext(move |text| {
        let mw = mw_weak.unwrap();
        let text = text.trim();
        let encoded = match mw.invoke_get_current_alphabet() {
            0 => Base64String::encode_with(text, Alpha::Standard),
            1 => Base64String::encode_with(text, Alpha::UrlSafe),
            _ => unreachable!(),
        };
        let encoded = encoded.to_string();
        info!(?text, ?encoded, "encoded plaintext");
        mw.invoke_set_base64(encoded.into());
        info!("set base64 text field");
    });

    let mw_weak = main_window.as_weak();
    main_window.on_decode_base64(move |base64| {
        let mw = mw_weak.unwrap();
        let decoded = match mw.invoke_get_current_alphabet() {
            0 => Base64String::from_encoded_with(&base64, Alpha::Standard),
            1 => Base64String::from_encoded_with(&base64, Alpha::UrlSafe),
            _ => unreachable!(),
        }
        .map_or_else(
            |e| {
                error!(?e);
                "[Invalid base64 input]".to_string()
            },
            |b64| {
                b64.decode_to_string().map_or_else(
                    |e| {
                        error!(?e);
                        "[Error decoding]".to_string()
                    },
                    |s| s.to_string(),
                )
            },
        );
        info!(?base64, ?decoded, "decoded base64");
        mw.invoke_set_plaintext(decoded.into());
        info!("set plaintext text field");
    });

    main_window.run().unwrap();
}

enum Alpha {
    Standard,
    UrlSafe,
}

impl Alphabet for Alpha {
    fn padding(&self) -> Option<char> {
        match self {
            Alpha::Standard => Standard::new().padding(),
            Alpha::UrlSafe => UrlSafe::new().padding(),
        }
    }

    fn encode_bits(&self, bits: u8) -> Result<char, baze64::B64Error> {
        match self {
            Alpha::Standard => Standard::new().encode_bits(bits),
            Alpha::UrlSafe => UrlSafe::new().encode_bits(bits),
        }
    }

    fn decode_char(&self, c: char) -> Result<u8, baze64::B64Error> {
        match self {
            Alpha::Standard => Standard::new().decode_char(c),
            Alpha::UrlSafe => UrlSafe::new().decode_char(c),
        }
    }

    fn is_valid(&self, c: char) -> bool {
        match self {
            Alpha::Standard => Standard::new().is_valid(c),
            Alpha::UrlSafe => UrlSafe::new().is_valid(c),
        }
    }
}
