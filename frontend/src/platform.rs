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

pub async fn show_window() {
    platform::show_window().await
}

// @TODO allow only supported file types by Wellen
// @TODO remove the `file` parameter once we don't have to use FileInput element
pub async fn pick_and_load_waveform(file: Option<gloo_file::File>) -> Option<Filename> {
    platform::pick_and_load_waveform(file).await
}

pub async fn get_hierarchy() -> wellen::Hierarchy {
    platform::get_hierarchy().await
}

pub async fn get_time_table() -> wellen::TimeTable {
    platform::get_time_table().await
}

pub async fn load_and_get_signal(signal_ref: wellen::SignalRef) -> wellen::Signal {
    platform::load_and_get_signal(signal_ref).await
}

pub async fn timeline(signal_ref: wellen::SignalRef, screen_width: u32) -> shared::Timeline {
    platform::timeline(signal_ref, screen_width).await
}

pub async fn unload_signal(signal_ref: wellen::SignalRef) {
    platform::unload_signal(signal_ref).await
}
