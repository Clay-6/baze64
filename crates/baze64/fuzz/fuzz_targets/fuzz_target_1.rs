#![no_main]

use libfuzzer_sys::fuzz_target;

use base64::{engine::general_purpose, Engine as _};
use baze64::{alphabet::Standard, Base64String};

fuzz_target!(|data: &[u8]| {
    let baze = Base64String::<Standard>::encode(data).to_string();
    let reference = general_purpose::STANDARD.encode(data);

    assert_eq!(baze, reference);
});
