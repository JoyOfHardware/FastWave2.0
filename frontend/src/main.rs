use std::rc::Rc;
use zoon::*;

mod platform;

mod controls_panel;
use controls_panel::ControlsPanel;

mod waveform_panel;
use waveform_panel::WaveformPanel;

type HierarchyAndTimeTable = (Rc<wellen::Hierarchy>, Rc<wellen::TimeTable>);

#[derive(Clone, Copy, Default)]
enum Layout {
    Tree,
    #[default]
    Columns,
}

fn main() {
    start_app("app", root);
    Task::start(async {
        // https://github.com/tauri-apps/tauri/issues/5170
        Timer::sleep(100).await;
        platform::show_window().await;
    });
}

fn root() -> impl Element {
    let hierarchy_and_time_table: Mutable<Option<HierarchyAndTimeTable>> = <_>::default();
    let selected_var_refs: MutableVec<wellen::VarRef> = <_>::default();
    let layout: Mutable<Layout> = <_>::default();
    Column::new()
        .s(Height::fill())
        .s(Scrollbars::y_and_clip_x())
        .s(Font::new().color(color!("Lavender")))
        .item(
            Row::new()
                .s(Height::fill())
                .s(Gap::new().x(15))
                .item(ControlsPanel::new(
                    hierarchy_and_time_table.clone(),
                    selected_var_refs.clone(),
                    layout.clone(),
                ))
                .item_signal(layout.signal().map(|layout| matches!(layout, Layout::Tree)).map_true(clone!((hierarchy_and_time_table, selected_var_refs) move || WaveformPanel::new(
                    hierarchy_and_time_table.clone(),
                    selected_var_refs.clone(),
                ))))
        )
        .item_signal(layout.signal().map(|layout| matches!(layout, Layout::Columns)).map_true(move || WaveformPanel::new(
            hierarchy_and_time_table.clone(),
            selected_var_refs.clone(),
        )))
}
