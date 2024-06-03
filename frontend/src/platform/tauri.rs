use zoon::*;

pub(super) async fn show_window() {
    tauri_glue::show_window().await.unwrap_throw()
}

pub(super) async fn pick_and_load_waveform() -> Option<super::Filename> {
    tauri_glue::pick_and_load_waveform()
        .await
        .unwrap_throw()
        .as_string()
}

pub(super) async fn get_hierarchy() -> wellen::Hierarchy {
    serde_wasm_bindgen::from_value(tauri_glue::get_hierarchy().await.unwrap_throw()).unwrap_throw()
}

pub(super) async fn get_time_table() -> wellen::TimeTable {
    serde_wasm_bindgen::from_value(tauri_glue::get_time_table().await.unwrap_throw()).unwrap_throw()
}

pub(super) async fn load_and_get_signal(signal_ref: wellen::SignalRef) -> wellen::Signal {
    serde_wasm_bindgen::from_value(
        tauri_glue::load_and_get_signal(signal_ref.index())
            .await
            .unwrap_throw(),
    )
    .unwrap_throw()
}

pub(super) async fn unload_signal(signal_ref: wellen::SignalRef) {
    tauri_glue::unload_signal(signal_ref.index())
        .await
        .unwrap_throw()
}

mod tauri_glue {
    use zoon::*;

    // Note: Add all corresponding methods to `frontend/typescript/tauri_glue/tauri_glue.ts`
    #[wasm_bindgen(module = "/typescript/bundles/tauri_glue.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn show_window() -> Result<(), JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn pick_and_load_waveform() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn get_hierarchy() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn get_time_table() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn load_and_get_signal(signal_ref_index: usize) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn unload_signal(signal_ref_index: usize) -> Result<(), JsValue>;
    }
}
