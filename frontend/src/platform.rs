// @TODO maybe rewrite `FASTWAVE_PLATFORM` to features once it's possible to set them through env vars:
// https://github.com/rust-lang/cargo/issues/4829

// NOTE: `FASTWAVE_PLATFORM` is set in `Makefile.toml` tasks and then in `build.rs`

#[cfg(FASTWAVE_PLATFORM = "TAURI")]
mod tauri;
#[cfg(FASTWAVE_PLATFORM = "TAURI")]
use tauri as platform;

#[cfg(FASTWAVE_PLATFORM = "BROWSER")]
mod browser;
#[cfg(FASTWAVE_PLATFORM = "BROWSER")]
use browser as platform;

type Filename = String;
type JavascriptCode = String;

pub async fn show_window() {
    platform::show_window().await
}

// @TODO allow only supported file types by Wellen
// @TODO remove the `file` parameter once we don't have to use FileInput element
pub async fn pick_and_load_waveform(file: Option<gloo_file::File>) -> Option<Filename> {
    platform::pick_and_load_waveform(file).await
}

// @TODO allow only supported file type (*.fw.js)
// @TODO remove the `file` parameter once we don't have to use FileInput element
pub async fn load_file_with_selected_vars(file: Option<gloo_file::File>) -> Option<JavascriptCode> {
    platform::load_file_with_selected_vars(file).await
}

pub async fn get_hierarchy() -> wellen::Hierarchy {
    platform::get_hierarchy().await
}

pub async fn load_signal_and_get_timeline(
    signal_ref: wellen::SignalRef,
    timeline_zoom: f64,
    timeline_viewport_width: u32,
    timeline_viewport_x: i32,
    block_height: u32,
    var_format: shared::VarFormat,
) -> shared::Timeline {
    platform::load_signal_and_get_timeline(
        signal_ref,
        timeline_zoom,
        timeline_viewport_width,
        timeline_viewport_x,
        block_height,
        var_format,
    )
    .await
}

pub async fn unload_signal(signal_ref: wellen::SignalRef) {
    platform::unload_signal(signal_ref).await
}
