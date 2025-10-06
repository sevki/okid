#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str;

fuzz_target!(|data: &[u8]| {
    // Try to parse as UTF-8 string and create OkId
    if let Ok(s) = str::from_utf8(data) {
        let _ = s.parse::<okid::OkId>();
        let _ = okid::OkId::from_display_safe(s);
    }

    // Try to create fingerprint from raw bytes
    let _ = okid::OkId::fingerprint(data);

    // Try to create bubblebabble from raw bytes
    if let Some(id) = okid::OkId::from_bubblebabble(data) {
        let _ = id.to_bubblebabble();
        let _ = id.to_string();
        let _ = id.to_path_safe();
    }
});
