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

pub(super) async fn get_time_table() -> wellen::TimeTable {
    let waveform = STORE.waveform.lock().unwrap_throw();
    let time_table = waveform.as_ref().unwrap_throw().time_table();
    // @TODO Wrap `time_table` in `Waveform` with `Rc/Arc` or add the method `take` / `clone` or refactor?
    serde_json::from_value(serde_json::to_value(time_table).unwrap_throw()).unwrap_throw()
}

pub(super) async fn load_and_get_signal(signal_ref: wellen::SignalRef) -> wellen::Signal {
    let mut waveform_lock = STORE.waveform.lock().unwrap_throw();
    let waveform = waveform_lock.as_mut().unwrap_throw();
    // @TODO maybe run it in a thread to not block the main one and then
    waveform.load_signals(&[signal_ref]);
    let signal = waveform.get_signal(signal_ref).unwrap_throw();
    // @TODO `clone` / `Rc/Arc` / refactor?
    serde_json::from_value(serde_json::to_value(signal).unwrap_throw()).unwrap_throw()
}

pub(super) async fn timeline(signal_ref: wellen::SignalRef, screen_width: u32) -> shared::Timeline {
    shared::Timeline { blocks: Vec::new() }
}

pub(super) async fn unload_signal(signal_ref: wellen::SignalRef) {
    let mut waveform_lock = STORE.waveform.lock().unwrap_throw();
    let waveform = waveform_lock.as_mut().unwrap_throw();
    waveform.unload_signals(&[signal_ref]);
}
