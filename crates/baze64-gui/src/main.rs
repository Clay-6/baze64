use baze64::{alphabet::Standard, Base64String};
use tracing::{debug, info};

slint::include_modules!();

fn main() {
    tracing_subscriber::fmt().init();

    let main_window = MainWindow::new().unwrap();
    debug!("main window created");

    let mw_weak = main_window.as_weak();
    main_window.on_encode_plaintext(move |text| {
        info!("on_decode_plaintext");
        let encoded = Base64String::<Standard>::encode(text.as_bytes())
            .unwrap()
            .to_string();
        info!(?text, ?encoded, "encoded plaintext");
        let mw = mw_weak.unwrap();
        mw.invoke_set_base64(encoded.into());
        info!("set base64 text field");
    });
    let mw_weak = main_window.as_weak();
    main_window.on_decode_base64(move |base64| {
        info!("on_decode_base64");
        let decoded = Base64String::<Standard>::from_encoded(&base64)
            .decode_to_string()
            .unwrap();
        info!(?base64, ?decoded, "decoded base64");
        let mw = mw_weak.unwrap();
        mw.invoke_set_plaintext(decoded.into());
        info!("set plaintext text field");
    });

    main_window.run().unwrap();
}
