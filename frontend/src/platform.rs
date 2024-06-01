#[cfg(feature = "platform_tauri")]
mod tauri;
#[cfg(feature = "platform_tauri")]
use tauri as platform;

#[cfg(feature = "platform_browser")]
mod browser;
#[cfg(feature = "platform_browser")]
use browser as platform;

pub async fn show_window() {
    platform::show_window().await
}

pub async fn load_waveform(test_file_name: &'static str) {
    platform::load_waveform(test_file_name).await
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

pub async fn unload_signal(signal_ref: wellen::SignalRef) {
    platform::unload_signal(signal_ref).await
}
