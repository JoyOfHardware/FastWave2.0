use std::rc::Rc;
use zoon::*;

mod platform;
mod script_bridge;

mod controls_panel;
use controls_panel::ControlsPanel;

mod waveform_panel;
use waveform_panel::WaveformPanel;

#[derive(Clone, Copy, Default)]
enum Layout {
    Tree,
    #[default]
    Columns,
}

#[derive(Default)]
struct Store {
    selected_var_refs: MutableVec<wellen::VarRef>,
}

static STORE: Lazy<Store> = lazy::default();

fn main() {
    start_app("app", root);
    Task::start(async {
        // https://github.com/tauri-apps/tauri/issues/5170
        Timer::sleep(100).await;
        platform::show_window().await;
    });
}

fn root() -> impl Element {
    let hierarchy: Mutable<Option<Rc<wellen::Hierarchy>>> = <_>::default();
    let selected_var_refs = STORE.selected_var_refs.clone();
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
                    hierarchy.clone(),
                    selected_var_refs.clone(),
                    layout.clone(),
                ))
                .item_signal(
                    layout
                        .signal()
                        .map(|layout| matches!(layout, Layout::Tree))
                        .map_true(
                            clone!((hierarchy, selected_var_refs) move || WaveformPanel::new(
                                hierarchy.clone(),
                                selected_var_refs.clone(),
                            )),
                        ),
                ),
        )
        .item_signal(
            layout
                .signal()
                .map(|layout| matches!(layout, Layout::Columns))
                .map_true(move || WaveformPanel::new(hierarchy.clone(), selected_var_refs.clone())),
        )
}
