use zoon::*;

pub(super) async fn show_window() {
    tauri_glue::show_window().await.unwrap_throw()
}

pub(super) async fn pick_and_load_waveform(
    _file: Option<gloo_file::File>,
) -> Option<super::Filename> {
    tauri_glue::pick_and_load_waveform()
        .await
        .unwrap_throw()
        .as_string()
}

pub(super) async fn load_file_with_selected_vars(
    _file: Option<gloo_file::File>,
) -> Option<super::JavascriptCode> {
    tauri_glue::load_file_with_selected_vars()
        .await
        .unwrap_throw()
        .as_string()
}

pub(super) async fn get_hierarchy() -> wellen::Hierarchy {
    serde_wasm_bindgen::from_value(tauri_glue::get_hierarchy().await.unwrap_throw()).unwrap_throw()
}

pub(super) async fn load_signal_and_get_timeline(
    signal_ref: wellen::SignalRef,
    timeline_zoom: f64,
    timeline_viewport_width: u32,
    timeline_viewport_x: i32,
    block_height: u32,
    var_format: shared::VarFormat,
) -> shared::Timeline {
    let var_format = serde_wasm_bindgen::to_value(&var_format).unwrap_throw();
    serde_wasm_bindgen::from_value(
        tauri_glue::load_signal_and_get_timeline(
            signal_ref.index(),
            timeline_zoom,
            timeline_viewport_width,
            timeline_viewport_x,
            block_height,
            var_format,
        )
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

pub(super) async fn add_decoders(
    decoder_paths: Vec<super::DecoderPath>,
) -> super::AddedDecodersCount {
    serde_wasm_bindgen::from_value(tauri_glue::add_decoders(decoder_paths).await.unwrap_throw())
        .unwrap_throw()
}

pub(super) async fn remove_all_decoders() -> super::RemovedDecodersCount {
    serde_wasm_bindgen::from_value(tauri_glue::remove_all_decoders().await.unwrap_throw())
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
        pub async fn load_file_with_selected_vars() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn get_hierarchy() -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn load_signal_and_get_timeline(
            signal_ref_index: usize,
            timeline_zoom: f64,
            timeline_viewport_width: u32,
            timeline_viewport_x: i32,
            block_height: u32,
            var_format: JsValue,
        ) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn unload_signal(signal_ref_index: usize) -> Result<(), JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn add_decoders(
            decoder_paths: Vec<super::super::DecoderPath>,
        ) -> Result<JsValue, JsValue>;

        #[wasm_bindgen(catch)]
        pub async fn remove_all_decoders() -> Result<JsValue, JsValue>;
    }
}
