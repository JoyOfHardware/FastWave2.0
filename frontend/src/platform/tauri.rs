use zoon::*;

pub(super) async fn show_window() {
    tauri_glue::show_window().await
}

pub(super) async fn load_waveform(test_file_name: &'static str) {
    tauri_glue::load_waveform(test_file_name).await
}

pub(super) async fn get_hierarchy() -> wellen::Hierarchy {
    serde_wasm_bindgen::from_value(tauri_glue::get_hierarchy().await).unwrap_throw()
}

pub(super) async fn get_time_table() -> wellen::TimeTable {
    serde_wasm_bindgen::from_value(tauri_glue::get_time_table().await).unwrap_throw()
}

pub(super) async fn load_and_get_signal(signal_ref: wellen::SignalRef) -> wellen::Signal {
    serde_wasm_bindgen::from_value(tauri_glue::load_and_get_signal(signal_ref.index()).await)
        .unwrap_throw()
}

pub(super) async fn unload_signal(signal_ref: wellen::SignalRef) {
    tauri_glue::unload_signal(signal_ref.index()).await
}

mod tauri_glue {
    use zoon::*;

    // Note: Add all corresponding methods to `frontend/typescript/tauri_glue/tauri_glue.ts`
    #[wasm_bindgen(module = "/typescript/bundles/tauri_glue.js")]
    extern "C" {
        pub async fn show_window();

        pub async fn load_waveform(test_file_name: &str);

        pub async fn get_hierarchy() -> JsValue;

        pub async fn get_time_table() -> JsValue;

        pub async fn load_and_get_signal(signal_ref_index: usize) -> JsValue;

        pub async fn unload_signal(signal_ref_index: usize);
    }
}
