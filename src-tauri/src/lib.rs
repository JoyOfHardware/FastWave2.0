use std::rc::Rc;
use std::sync::Mutex;
use wellen::simple::Waveform;

mod wellen_helpers;

#[derive(Default)]
struct Store {
    waveform: Mutex<Option<Waveform>>,
}

#[tauri::command(rename_all = "snake_case")]
fn show_window(window: tauri::Window) {
    window.show().unwrap();
}

#[tauri::command(rename_all = "snake_case")]
fn load_waveform(test_file_name: Rc<String>, store: tauri::State<Store>) {
    static SIMPLE_VCD: &'static [u8; 311] = include_bytes!("../../test_files/simple.vcd");
    static WAVE_27_FST: &'static [u8; 28860652] = include_bytes!("../../test_files/wave_27.fst");
    let chosen_file = match test_file_name.as_str() {
        "simple.vcd" => SIMPLE_VCD.to_vec(),
        "wave_27.fst" => WAVE_27_FST.to_vec(),
        test_file_name => todo!("add {test_file_name} to the `test_files` folder"),
    };
    let waveform = wellen_helpers::read_from_bytes(chosen_file);
    let Ok(waveform) = waveform else {
        panic!("VCD file reading failed")
    };
    *store.waveform.lock().unwrap() = Some(waveform);
}

#[tauri::command(rename_all = "snake_case")]
fn get_hierarchy(store: tauri::State<Store>) -> serde_json::Value {
    let waveform = store.waveform.lock().unwrap();
    let hierarchy = waveform.as_ref().unwrap().hierarchy();
    serde_json::to_value(hierarchy).unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn get_time_table(store: tauri::State<Store>) -> serde_json::Value {
    let waveform = store.waveform.lock().unwrap();
    let time_table = waveform.as_ref().unwrap().time_table();
    serde_json::to_value(time_table).unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn load_and_get_signal(signal_ref_index: usize, store: tauri::State<Store>) -> serde_json::Value {
    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap();
    let mut waveform_lock = store.waveform.lock().unwrap();
    let waveform = waveform_lock.as_mut().unwrap();
    // @TODO maybe run it in a thread to not block the main one and then
    // make the command async or return the result through a Tauri channel
    waveform.load_signals_multi_threaded(&[signal_ref]);
    let signal = waveform.get_signal(signal_ref).unwrap();
    serde_json::to_value(signal).unwrap()
}

#[tauri::command(rename_all = "snake_case")]
fn unload_signal(signal_ref_index: usize, store: tauri::State<Store>) {
    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap();
    let mut waveform_lock = store.waveform.lock().unwrap();
    let waveform = waveform_lock.as_mut().unwrap();
    waveform.unload_signals(&[signal_ref]);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // https://github.com/tauri-apps/tauri/issues/8462
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    tauri::Builder::default()
        .manage(Store::default())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        // Npte: Add all handlers to `frontend/src/tauri_bridge.rs`
        .invoke_handler(tauri::generate_handler![
            show_window,
            load_waveform,
            get_hierarchy,
            get_time_table,
            load_and_get_signal,
            unload_signal,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
