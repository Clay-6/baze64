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
        let encoded = Base64String::<Standard>::encode(text.as_bytes()).unwrap();
        debug!(?text, ?encoded, "encoded plaintext");
        let mw = mw_weak.unwrap();
        mw.set_base64(encoded.to_string().into());
        debug!("set base64 text field");
    });
    let mw_weak = main_window.as_weak();
    main_window.on_decode_base64(move |base64| {
        info!("on_decode_base64");
        let decoded = Base64String::<Standard>::from_encoded(&base64);
        debug!(?base64, ?decoded, "decoded base64");
        let mw = mw_weak.unwrap();
        mw.set_plaintext(decoded.decode_to_string().unwrap().into());
        debug!("set plaintext text field");
    });

    main_window.run().unwrap();
}
