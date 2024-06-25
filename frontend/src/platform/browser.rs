use shared::wellen_helpers;
use std::sync::Mutex;
use wellen::simple::Waveform;
use zoon::{*, eprintln};

#[derive(Default)]
struct BrowserPlatformStore {
    waveform: Mutex<Option<Waveform>>,
}

static BROWSER_PLATFORM_STORE: Lazy<BrowserPlatformStore> = lazy::default();

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
    *BROWSER_PLATFORM_STORE.waveform.lock().unwrap_throw() = Some(waveform);
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
//     *BROWSER_PLATFORM_STORE.waveform.lock().unwrap_throw() = Some(waveform);
//     Some(file.name())
// }

// @TODO allow only supported file type (*.fw.js)
// @TODO remove the `file` parameter once we don't have to use FileInput element
pub async fn load_file_with_selected_vars(
    file: Option<gloo_file::File>,
) -> Option<super::JavascriptCode> {
    let file = file.unwrap_throw();

    let javascript_code = gloo_file::futures::read_as_text(&file).await.unwrap_throw();

    Some(javascript_code)
}

// @TODO Use alternative `load_file_with_selected_vars` version once `showOpenFilePicker` is supported by Safari and Firefox
// https://caniuse.com/mdn-api_window_showopenfilepicker
// (see the `pick_and_load_waveform` method above)

pub(super) async fn get_hierarchy() -> wellen::Hierarchy {
    let waveform = BROWSER_PLATFORM_STORE.waveform.lock().unwrap_throw();
    let hierarchy = waveform.as_ref().unwrap_throw().hierarchy();
    // @TODO Wrap `hierarchy` in `Waveform` with `Rc/Arc` or add the method `take` / `clone` or refactor?
    serde_json::from_value(serde_json::to_value(hierarchy).unwrap_throw()).unwrap_throw()
}

pub(super) async fn load_signal_and_get_timeline(
    signal_ref: wellen::SignalRef,
    timeline_zoom: f64,
    timeline_viewport_width: u32,
    timeline_viewport_x: i32,
    block_height: u32,
    var_format: shared::VarFormat,
) -> shared::Timeline {
    let mut waveform_lock = BROWSER_PLATFORM_STORE.waveform.lock().unwrap();
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
    timeline
}

pub(super) async fn unload_signal(signal_ref: wellen::SignalRef) {
    let mut waveform_lock = BROWSER_PLATFORM_STORE.waveform.lock().unwrap_throw();
    let waveform = waveform_lock.as_mut().unwrap_throw();
    waveform.unload_signals(&[signal_ref]);
}

pub(super) async fn add_decoders(
    _decoder_paths: Vec<super::DecoderPath>,
) -> super::AddedDecodersCount {
    // @TODO error message for user
    eprintln!("Adding decoders is not supported in the browser.");
    0
}
