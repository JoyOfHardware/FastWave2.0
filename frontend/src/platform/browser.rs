use shared::wellen_helpers;
use std::sync::Mutex;
use wellen::simple::Waveform;
use zoon::*;

#[derive(Default)]
struct Store {
    waveform: Mutex<Option<Waveform>>,
}

static STORE: Lazy<Store> = lazy::default();

pub(super) async fn show_window() {}

pub(super) async fn pick_and_load_waveform(
    file: Option<gloo_file::File>,
) -> Option<super::Filename> {
    let file = file.unwrap_throw();

    let content = gloo_file::futures::read_as_bytes(&file)
        .await
        .unwrap_throw();

    let waveform = wellen_helpers::read_from_bytes(content);
    let Ok(waveform) = waveform else {
        panic!("Waveform file reading failed")
    };
    *STORE.waveform.lock().unwrap_throw() = Some(waveform);
    Some(file.name())
}

// @TODO Use this `pick_and_load_waveform` version once `showOpenFilePicker` is supported by Safari and Firefox
// https://caniuse.com/mdn-api_window_showopenfilepicker
// pub(super) async fn pick_and_load_waveform() -> Option<super::Filename> {
//     let file_handles_promise = window().show_open_file_picker().expect_throw(
//         "failed to open file picker (browser has to support `showOpenFilePicker` and use HTTPS",
//     );
//     let file_handles = JsFuture::from(file_handles_promise).await;
//     let file_handles = match file_handles {
//         Ok(file_handles) => file_handles.dyn_into::<js_sys::Array>().unwrap_throw(),
//         Err(error) => {
//             println!("file picker error: {error:?}");
//             return None;
//         }
//     };
//     let file_handle = file_handles
//         .at(0)
//         .dyn_into::<web_sys::FileSystemFileHandle>()
//         .unwrap_throw();

//     let file = JsFuture::from(file_handle.get_file())
//         .await
//         .unwrap_throw()
//         .dyn_into::<web_sys::File>()
//         .unwrap_throw();

//     let file = gloo_file::File::from(file);
//     let content = gloo_file::futures::read_as_bytes(&file)
//         .await
//         .unwrap_throw();

//     let waveform = wellen_helpers::read_from_bytes(content);
//     let Ok(waveform) = waveform else {
//         panic!("Waveform file reading failed")
//     };
//     *STORE.waveform.lock().unwrap_throw() = Some(waveform);
//     Some(file.name())
// }

pub(super) async fn get_hierarchy() -> wellen::Hierarchy {
    let waveform = STORE.waveform.lock().unwrap_throw();
    let hierarchy = waveform.as_ref().unwrap_throw().hierarchy();
    // @TODO Wrap `hierarchy` in `Waveform` with `Rc/Arc` or add the method `take` / `clone` or refactor?
    serde_json::from_value(serde_json::to_value(hierarchy).unwrap_throw()).unwrap_throw()
}

pub(super) async fn load_signal_and_get_timeline(
    signal_ref: wellen::SignalRef,
    screen_width: u32,
    block_height: u32,
) -> shared::Timeline {
    let mut waveform_lock = STORE.waveform.lock().unwrap();
    let waveform = waveform_lock.as_mut().unwrap();
    waveform.load_signals_multi_threaded(&[signal_ref]);
    let signal = waveform.get_signal(signal_ref).unwrap();
    let time_table = waveform.time_table();
    let timeline = signal_to_timeline(signal, time_table, screen_width, block_height);
    timeline
}

pub(super) async fn unload_signal(signal_ref: wellen::SignalRef) {
    let mut waveform_lock = STORE.waveform.lock().unwrap_throw();
    let waveform = waveform_lock.as_mut().unwrap_throw();
    waveform.unload_signals(&[signal_ref]);
}

// @TODO keep in sync with the same method in `src-tauri/src/lib.rs`
fn signal_to_timeline(
    signal: &wellen::Signal,
    time_table: &[wellen::Time],
    screen_width: u32,
    block_height: u32,
) -> shared::Timeline {
    const MIN_BLOCK_WIDTH: u32 = 3;
    // Courier New, 16px, sync with `label_style` in `pixi_canvas.rs`
    const LETTER_WIDTH: f64 = 9.61;
    const LETTER_HEIGHT: u32 = 21;
    const LABEL_X_PADDING: u32 = 10;

    let Some(last_time) = time_table.last().copied() else {
        return shared::Timeline::default();
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

    // @TODO parallelize?
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

        // @TODO cache?
        let value = shared::VarFormat::default().format(value);

        let value_width = (value.chars().count() as f64 * LETTER_WIDTH) as u32;
        // @TODO Ellipsis instead of hiding?
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
