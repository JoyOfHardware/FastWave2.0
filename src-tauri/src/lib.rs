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
async fn load_signal_and_get_timeline(
    signal_ref_index: usize,
    screen_width: u32,
    block_height: u32,
    store: tauri::State<'_, Store>,
) -> Result<serde_json::Value, ()> {
    // @TODO run (all?) in a blocking thread
    let signal_ref = wellen::SignalRef::from_index(signal_ref_index).unwrap();
    let mut waveform_lock = store.waveform.lock().unwrap();
    let waveform = waveform_lock.as_mut().unwrap();
    waveform.load_signals_multi_threaded(&[signal_ref]);
    let signal = waveform.get_signal(signal_ref).unwrap();
    let time_table = waveform.time_table();
    let timeline = signal_to_timeline(signal, time_table, screen_width, block_height);
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
            load_signal_and_get_timeline,
            unload_signal,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn signal_to_timeline(
    signal: &wellen::Signal, 
    time_table: &[wellen::Time],
    screen_width: u32,
    block_height: u32,
) -> shared::Timeline {
    const MIN_BLOCK_WIDTH: u32 = 3;
    const LETTER_WIDTH: u32 = 15;
    const LETTER_HEIGHT: u32 = 21;
    const LABEL_X_PADDING: u32 = 10;

    let Some(last_time) = time_table.last().copied() else {
        return shared::Timeline::default()
    };

    let last_time = last_time as f64;
    let screen_width = screen_width as f64;

    let mut x_value_pairs = signal
        .iter_changes()
        .map(|(index, value)| {
            let index = index as usize;
            let time = time_table[index] as f64;
            let x = time / last_time * screen_width;
            (x, value)
        })
        .peekable();

    let mut blocks = Vec::new();

    while let Some((block_x, value)) = x_value_pairs.next() {
        let next_block_x = if let Some((next_block_x, _)) = x_value_pairs.peek() {
            *next_block_x
        } else {
            screen_width
        };

        let block_width = (next_block_x - block_x) as u32;
        if block_width < MIN_BLOCK_WIDTH {
            continue;
        } 

        let value = value.to_string();
        // @TODO dynamic formatter
        let value = u32::from_str_radix(&value, 2).unwrap();
        let value = format!("{value:x}");

        let value_width = value.chars().count() as u32 * LETTER_WIDTH;
        let label = if (value_width + (2 * LABEL_X_PADDING)) <= block_width {
            Some(shared::TimeLineBlockLabel {
                text: value,
                x: (block_width - value_width) / 2,
                y: (block_height - LETTER_HEIGHT) / 2,
            })
        } else {
            None
        };

        let block = shared::TimelineBlock {
            x: block_x as u32,
            width: block_width,
            height: block_height,
            label, 
        };
        blocks.push(block);
    }

    shared::Timeline { blocks }
}
