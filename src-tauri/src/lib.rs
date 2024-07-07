use std::fs;
use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;
use wellen::simple::Waveform;

type Filename = String;
type JavascriptCode = String;
type AddedDecodersCount = usize;
type RemovedDecodersCount = usize;
type DecoderPath = String;

mod component_manager;

#[derive(Default)]
struct Store {
    waveform: Mutex<Option<Waveform>>,
}

#[tauri::command(rename_all = "snake_case")]
async fn show_window(window: tauri::Window) {
    window.show().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
async fn pick_and_load_waveform(
    store: tauri::State<'_, Store>,
    app: tauri::AppHandle,
) -> Result<Option<Filename>, ()> {
    let Some(file_response) = app.dialog().file().blocking_pick_file() else {
        return Ok(None);
    };
    let file_path = file_response.path.as_os_str().to_str().unwrap();
    // @TODO `read` should accept `Path` instead of `&str`
    let waveform = wellen::simple::read(file_path);
    let Ok(waveform) = waveform else {
        panic!("Waveform file reading failed")
    };
    *store.waveform.lock().unwrap() = Some(waveform);
    Ok(Some(file_response.name.unwrap()))
}

#[tauri::command(rename_all = "snake_case")]
async fn load_file_with_selected_vars(app: tauri::AppHandle) -> Result<Option<JavascriptCode>, ()> {
    let Some(file_response) = app.dialog().file().blocking_pick_file() else {
        return Ok(None);
    };
    // @TODO Tokio's `fs` or a Tauri `fs`?
    let Ok(javascript_code) = fs::read_to_string(file_response.path) else {
        panic!("Selected vars file reading failed")
    };
    Ok(Some(javascript_code))
}

#[tauri::command(rename_all = "snake_case")]
async fn get_hierarchy(store: tauri::State<'_, Store>) -> Result<serde_json::Value, ()> {
    let waveform = store.waveform.lock().unwrap();
    let hierarchy = waveform.as_ref().unwrap().hierarchy();
    Ok(serde_json::to_value(hierarchy).unwrap())
}

#[tauri::command(rename_all = "snake_case")]
async fn load_signal_and_get_timeline(
    signal_ref_index: usize,
    timeline_zoom: f64,
    timeline_viewport_width: u32,
    timeline_viewport_x: i32,
    block_height: u32,
    var_format: shared::VarFormat,
    store: tauri::State<'_, Store>,
) -> Result<serde_json::Value, ()> {
    // @TODO run (all?) in a blocking thread?
    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap();
    let mut waveform_lock = store.waveform.lock().unwrap();
    let waveform = waveform_lock.as_mut().unwrap();
    waveform.load_signals_multi_threaded(&[signal_ref]);
    let signal = waveform.get_signal(signal_ref).unwrap();
    let time_table = waveform.time_table();
    let timeline = shared::signal_to_timeline(
        signal,
        time_table,
        timeline_zoom,
        timeline_viewport_width,
        timeline_viewport_x,
        block_height,
        var_format,
    );
    Ok(serde_json::to_value(timeline).unwrap())
}

#[tauri::command(rename_all = "snake_case")]
async fn unload_signal(signal_ref_index: usize, store: tauri::State<'_, Store>) -> Result<(), ()> {
    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap();
    let mut waveform_lock = store.waveform.lock().unwrap();
    let waveform = waveform_lock.as_mut().unwrap();
    waveform.unload_signals(&[signal_ref]);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn add_decoders(decoder_paths: Vec<DecoderPath>) -> Result<AddedDecodersCount, ()> {
    Ok(component_manager::add_decoders(decoder_paths).await)
}

#[tauri::command(rename_all = "snake_case")]
async fn remove_all_decoders() -> Result<RemovedDecodersCount, ()> {
    Ok(component_manager::remove_all_decoders().await)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // https://github.com/tauri-apps/tauri/issues/8462
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .manage(Store::default())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        // Npte: Add all handlers to `frontend/src/tauri_bridge.rs`
        .invoke_handler(tauri::generate_handler![
            show_window,
            pick_and_load_waveform,
            load_file_with_selected_vars,
            get_hierarchy,
            load_signal_and_get_timeline,
            unload_signal,
            add_decoders,
            remove_all_decoders,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
