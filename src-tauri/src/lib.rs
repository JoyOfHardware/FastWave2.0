use std::sync::Mutex;
use tauri_plugin_dialog::DialogExt;
use wellen::simple::Waveform;

type Filename = String;

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
async fn get_hierarchy(store: tauri::State<'_, Store>) -> Result<serde_json::Value, ()> {
    let waveform = store.waveform.lock().unwrap();
    let hierarchy = waveform.as_ref().unwrap().hierarchy();
    Ok(serde_json::to_value(hierarchy).unwrap())
}

#[tauri::command(rename_all = "snake_case")]
async fn get_time_table(store: tauri::State<'_, Store>) -> Result<serde_json::Value, ()> {
    let waveform = store.waveform.lock().unwrap();
    let time_table = waveform.as_ref().unwrap().time_table();
    Ok(serde_json::to_value(time_table).unwrap())
}

#[tauri::command(rename_all = "snake_case")]
async fn load_and_get_signal(
    signal_ref_index: usize,
    store: tauri::State<'_, Store>,
) -> Result<serde_json::Value, ()> {
    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap();
    let mut waveform_lock = store.waveform.lock().unwrap();
    let waveform = waveform_lock.as_mut().unwrap();
    // @TODO maybe run it in a thread to not block the main one and then
    // make the command async or return the result through a Tauri channel
    waveform.load_signals_multi_threaded(&[signal_ref]);
    let signal = waveform.get_signal(signal_ref).unwrap();
    Ok(serde_json::to_value(signal).unwrap())
}

#[tauri::command(rename_all = "snake_case")]
async fn timeline(
    signal_ref_index: usize,
    screen_width: u32,
    store: tauri::State<'_, Store>,
) -> Result<serde_json::Value, ()> {
    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap();
    let mut waveform_lock = store.waveform.lock().unwrap();
    let waveform = waveform_lock.as_mut().unwrap();
    // @TODO maybe run it in a thread to not block the main one or return the result through a Tauri channel
    waveform.load_signals_multi_threaded(&[signal_ref]);
    let signal = waveform.get_signal(signal_ref).unwrap();

    // @TODO create Timeline
    let timeline = shared::Timeline { blocks: Vec::new() };

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
            get_hierarchy,
            get_time_table,
            load_and_get_signal,
            timeline,
            unload_signal,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
