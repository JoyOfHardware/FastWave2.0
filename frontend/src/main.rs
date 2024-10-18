use std::sync::Arc;
use zoon::*;

mod platform;
mod script_bridge;

mod controls_panel;
use controls_panel::ControlsPanel;

mod diagram_panel;
use diagram_panel::{DiagramPanel, ExcalidrawController};

mod waveform_panel;
use waveform_panel::{PixiController, WaveformPanel};

mod header_panel;
use header_panel::HeaderPanel;

mod command_panel;
use command_panel::CommandPanel;

pub mod theme;
use theme::*;

#[derive(Clone, Copy, Default)]
enum Layout {
    Tree,
    #[default]
    Columns,
}

#[derive(Clone, Copy, Default)]
enum Mode {
    // @TODO make default
    // #[default]
    Waves,
    #[default]
    Diagrams,
}

type Filename = String;

#[derive(Default)]
struct Store {
    selected_var_refs: MutableVec<wellen::VarRef>,
    hierarchy: Mutable<Option<Arc<wellen::Hierarchy>>>,
    loaded_filename: Mutable<Option<Filename>>,
    pixi_canvas_controller: Mutable<Mutable<Option<SendWrapper<PixiController>>>>,
    excalidraw_canvas_controller: Mutable<Mutable<Option<SendWrapper<ExcalidrawController>>>>,
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
    let hierarchy = STORE.hierarchy.clone();
    let selected_var_refs = STORE.selected_var_refs.clone();
    let layout: Mutable<Layout> = <_>::default();
    let mode: Mutable<Mode> = <_>::default();
    let loaded_filename = STORE.loaded_filename.clone();
    let pixi_canvas_controller = STORE.pixi_canvas_controller.clone();
    let excalidraw_canvas_controller = STORE.excalidraw_canvas_controller.clone();
    Column::new()
        .s(Height::fill())
        .s(Scrollbars::y_and_clip_x())
        .s(Font::new().color(COLOR_LAVENDER))
        .item(HeaderPanel::new(
            hierarchy.clone(),
            layout.clone(),
            mode.clone(),
            loaded_filename.clone(),
        ))
        .item_signal(mode.signal().map(clone!((hierarchy, selected_var_refs, loaded_filename, pixi_canvas_controller) move |mode| match mode {
            Mode::Waves => {
                Column::new()
                    .s(Height::fill())
                    .s(Scrollbars::y_and_clip_x())
                    .item(
                        Row::new()
                            .s(Scrollbars::y_and_clip_x())
                            .s(Gap::new().x(15))
                            .s(Height::growable().min(150))
                            .item(ControlsPanel::new(
                                hierarchy.clone(),
                                selected_var_refs.clone(),
                                layout.clone(),
                                loaded_filename.clone(),
                            ))
                            .item_signal({
                                let hierarchy = hierarchy.clone();
                                let selected_var_refs = selected_var_refs.clone();
                                let loaded_filename = loaded_filename.clone();
                                let pixi_canvas_controller = pixi_canvas_controller.clone();
                                map_ref!{
                                    let layout = layout.signal(),
                                    let hierarchy_is_some = hierarchy.signal_ref(Option::is_some) => {
                                        (*hierarchy_is_some && matches!(layout, Layout::Tree)).then(clone!((hierarchy, selected_var_refs, loaded_filename, pixi_canvas_controller) move || WaveformPanel::new(
                                            hierarchy.clone(),
                                            selected_var_refs.clone(),
                                            loaded_filename.clone(),
                                            pixi_canvas_controller.clone(),
                                        )))
                                    }
                                }
                            }),
                    )
                    .item_signal({
                        let hierarchy = hierarchy.clone();
                        let selected_var_refs = selected_var_refs.clone();
                        let loaded_filename = loaded_filename.clone();
                        let pixi_canvas_controller = pixi_canvas_controller.clone();
                        map_ref!{
                            let layout = layout.signal(),
                            let hierarchy_is_some = hierarchy.signal_ref(Option::is_some) => {
                                (*hierarchy_is_some && matches!(layout, Layout::Columns)).then(clone!((hierarchy, selected_var_refs, loaded_filename, pixi_canvas_controller) move || WaveformPanel::new(
                                    hierarchy.clone(),
                                    selected_var_refs.clone(),
                                    loaded_filename.clone(),
                                    pixi_canvas_controller.clone(),
                                )))
                            }
                        }
                    })
            }
            Mode::Diagrams => {
                Column::new()
                    .s(Height::fill())
                    .s(Scrollbars::y_and_clip_x())
                    .item(DiagramPanel::new(excalidraw_canvas_controller.clone()))
            }
        })))
        .item(CommandPanel::new())
}
