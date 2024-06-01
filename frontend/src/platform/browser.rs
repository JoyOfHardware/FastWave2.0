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

pub(super) async fn load_waveform(test_file_name: &'static str) {
    static SIMPLE_VCD: &'static [u8; 311] = include_bytes!("../../../test_files/simple.vcd");
    // static WAVE_27_FST: &'static [u8; 28860652] = include_bytes!("../../../test_files/wave_27.fst");
    let chosen_file = match test_file_name {
        "simple.vcd" => SIMPLE_VCD.to_vec(),
        // "wave_27.fst" => WAVE_27_FST.to_vec(),
        test_file_name => todo!("add {test_file_name} to the `test_files` folder"),
    };
    let waveform = wellen_helpers::read_from_bytes(chosen_file);
    let Ok(waveform) = waveform else {
        panic!("VCD file reading failed")
    };
    *STORE.waveform.lock().unwrap_throw() = Some(waveform);
}

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

pub(super) async fn unload_signal(signal_ref: wellen::SignalRef) {
    let mut waveform_lock = STORE.waveform.lock().unwrap_throw();
    let waveform = waveform_lock.as_mut().unwrap_throw();
    waveform.unload_signals(&[signal_ref]);
}
