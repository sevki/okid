use cfg_aliases::cfg_aliases;

fn main() {
    // Setup cfg aliases
    cfg_aliases! {
        // Target architectures
        wasm32: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },

        // wasm-bindgen: JS bindings enabled via the "wasm" feature
        // This brings in wasm-bindgen, js-sys, web-sys dependencies
        wasm_bindgen: { feature = "wasm" },

        // Operating systems
        android: { target_os = "android" },
        macos: { target_os = "macos" },
        linux: { target_os = "linux" },
        windows: { target_os = "windows" },
        ios: { target_os = "ios" },
        unix_like: { any(target_os = "linux", target_os = "macos", target_os = "android", target_os = "ios") },

        // Cloudflare Workers: wasm32 + worker feature
        worker: { all(target_arch = "wasm32", feature = "worker") },
    }
}
