#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use baze64::{
    alphabet::{Standard, UrlSafe},
    Base64String,
};
use tracing::{debug, info};

slint::include_modules!();

fn main() {
    tracing_subscriber::fmt().init();

    let main_window = MainWindow::new().unwrap();
    debug!("main window created");

    let mw_weak = main_window.as_weak();
    main_window.on_encode_plaintext(move |text| {
        let mw = mw_weak.unwrap();
        let encoded = match mw.invoke_get_current_alphabet().as_str() {
            "standard" => Base64String::<Standard>::encode(text.trim().as_bytes())
                .unwrap()
                .to_string(),
            "urlsafe" => Base64String::<UrlSafe>::encode(text.trim().as_bytes())
                .unwrap()
                .to_string(),
            other => panic!("How is the alphabet {other}"),
        };
        info!(?text, ?encoded, "encoded plaintext");
        mw.invoke_set_base64(encoded.into());
        info!("set base64 text field");
    });

    let mw_weak = main_window.as_weak();
    main_window.on_decode_base64(move |base64| {
        let mw = mw_weak.unwrap();
        let decoded = match mw.invoke_get_current_alphabet().as_str() {
            "standard" => Base64String::<Standard>::from_encoded(base64.trim())
                .decode_to_string()
                .unwrap(),
            "urlsafe" => Base64String::<UrlSafe>::from_encoded(base64.trim())
                .decode_to_string()
                .unwrap(),
            other => panic!("How is the alphabet {other}"),
        };
        info!(?base64, ?decoded, "decoded base64");
        mw.invoke_set_plaintext(decoded.into());
        info!("set plaintext text field");
    });

    main_window.run().unwrap();
}
