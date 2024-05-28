use std::rc::Rc;
use zoon::*;

mod tauri_bridge;

mod controls_panel;
use controls_panel::ControlsPanel;

mod waveform_panel;
use waveform_panel::WaveformPanel;

type HierarchyAndTimeTable = (Rc<wellen::Hierarchy>, Rc<wellen::TimeTable>);

// @TODO REMOVE
const SIMULATE_CLICKS: bool = false;

fn main() {
    start_app("app", root);
    Task::start(async {
        // https://github.com/tauri-apps/tauri/issues/5170
        Timer::sleep(100).await;
        tauri_bridge::show_window().await;
    });
}

fn root() -> impl Element {
    let hierarchy_and_time_table: Mutable<Option<HierarchyAndTimeTable>> = <_>::default();
    let selected_var_refs: MutableVec<wellen::VarRef> = <_>::default();
    Row::new()
        .s(Height::fill())
        .s(Font::new().color(color!("Lavender")))
        .item(ControlsPanel::new(
            hierarchy_and_time_table.clone(),
            selected_var_refs.clone(),
        ))
        .item(WaveformPanel::new(
            hierarchy_and_time_table,
            selected_var_refs,
        ))
}
