use crate::{platform, HierarchyAndTimeTable};
use wellen::GetItem;
use zoon::{eprintln, *};

mod pixi_canvas;
use pixi_canvas::{PixiCanvas, PixiController};

const ROW_HEIGHT: u32 = 40;
const ROW_GAP: u32 = 4;

#[derive(Clone)]
pub struct WaveformPanel {
    selected_var_refs: MutableVec<wellen::VarRef>,
    hierarchy_and_time_table: Mutable<Option<HierarchyAndTimeTable>>,
}

impl WaveformPanel {
    pub fn new(
        hierarchy_and_time_table: Mutable<Option<HierarchyAndTimeTable>>,
        selected_var_refs: MutableVec<wellen::VarRef>,
    ) -> impl Element {
        Self {
            selected_var_refs,
            hierarchy_and_time_table,
        }
        .root()
    }

    fn root(&self) -> impl Element {
        let selected_vars_panel_height_getter: Mutable<u32> = <_>::default();
        Row::new()
            .s(Padding::all(20))
            .s(Scrollbars::y_and_clip_x())
            .s(Width::growable())
            .s(Height::fill())
            .item(self.selected_vars_panel(selected_vars_panel_height_getter.clone()))
            .item(self.canvas(selected_vars_panel_height_getter.read_only()))
    }

    fn selected_vars_panel(&self, height_getter: Mutable<u32>) -> impl Element {
        Column::new()
            .s(Gap::new().y(ROW_GAP))
            .s(Align::new().top())
            .on_viewport_size_change(move |_, height| height_getter.set_neq(height))
            .items_signal_vec(self.selected_var_refs.signal_vec().enumerate().map(
                clone!((self => s) move |(index, var_ref)| {
                    s.selected_var_panel(index, var_ref)
                }),
            ))
    }

    fn canvas(&self, selected_vars_panel_height: ReadOnlyMutable<u32>) -> impl Element {
        let selected_var_refs = self.selected_var_refs.clone();
        let hierarchy_and_time_table = self.hierarchy_and_time_table.clone();
        PixiCanvas::new(ROW_HEIGHT, ROW_GAP)
            .s(Align::new().top())
            .s(Width::fill())
            .s(Height::exact_signal(selected_vars_panel_height.signal()))
            .s(RoundedCorners::new().right(15))
            .task_with_controller(move |controller| {
                selected_var_refs.signal_vec().delay_remove(clone!((hierarchy_and_time_table) move |var_ref| {
                    clone!((var_ref, hierarchy_and_time_table) async move {
                        if let Some(hierarchy_and_time_table) = hierarchy_and_time_table.get_cloned() {
                            platform::unload_signal(hierarchy_and_time_table.0.get(var_ref).signal_ref()).await;
                        }
                    })
                })).for_each(clone!((controller, hierarchy_and_time_table) move |vec_diff| {
                    clone!((controller, hierarchy_and_time_table) async move {
                        match vec_diff {
                            VecDiff::Replace { values } => {
                                let controller = controller.wait_for_some_cloned().await;
                                controller.clear_vars();
                                for var_ref in values {
                                    Self::push_var(&controller, &hierarchy_and_time_table, var_ref).await;
                                }
                            },
                            VecDiff::InsertAt { index: _, value: _ } => { todo!("`task_with_controller` + `InsertAt`") }
                            VecDiff::UpdateAt { index: _, value: _ } => { todo!("`task_with_controller` + `UpdateAt`") }
                            VecDiff::RemoveAt { index } => {
                                if let Some(controller) = controller.lock_ref().as_ref() {
                                    controller.remove_var(index);
                                }
                            }
                            VecDiff::Move { old_index: _, new_index: _ } => { todo!("`task_with_controller` + `Move`") }
                            VecDiff::Push { value: var_ref } => {
                                if let Some(controller) = controller.lock_ref().as_ref() {
                                    Self::push_var(controller, &hierarchy_and_time_table, var_ref).await;
                                }
                            }
                            VecDiff::Pop {} => {
                                if let Some(controller) = controller.lock_ref().as_ref() {
                                    controller.pop_var();
                                }
                            }
                            VecDiff::Clear {} => {
                                if let Some(controller) = controller.lock_ref().as_ref() {
                                    controller.clear_vars();
                                }
                            }
                        }
                    })
                }))
            })
    }

    async fn push_var(
        controller: &PixiController,
        hierarchy_and_time_table: &Mutable<Option<HierarchyAndTimeTable>>,
        var_ref: wellen::VarRef,
    ) {
        let (hierarchy, time_table) = hierarchy_and_time_table.get_cloned().unwrap();
        if time_table.is_empty() {
            eprintln!("timetable is empty");
            return;
        }
        let last_time = time_table.last().copied().unwrap_throw();

        let var = hierarchy.get(var_ref);
        let signal_ref = var.signal_ref();
        let timeline = platform::timeline(signal_ref, controller.screen_width()).await;

        // @TODO remove
        zoon::println!("Timeline in Rust: {timeline:#?}");

        let timescale = hierarchy.timescale();
        // @TODO remove
        zoon::println!("{timescale:?}");

        if timeline.blocks.is_empty() {
            eprintln!("timeline is empty");
            return;
        }
        // Note: Sync `timeline`'s type with the `Timeline` in `frontend/typescript/pixi_canvas/pixi_canvas.ts'
        controller.push_var(serde_wasm_bindgen::to_value(&timeline).unwrap_throw());
    }

    fn selected_var_panel(
        &self,
        index: ReadOnlyMutable<Option<usize>>,
        var_ref: wellen::VarRef,
    ) -> Option<impl Element> {
        let Some((hierarchy, _)) = self.hierarchy_and_time_table.get_cloned() else {
            None?
        };
        let var = hierarchy.get(var_ref);
        let name: &str = var.name(&hierarchy);
        let selected_var_refs = self.selected_var_refs.clone();
        Button::new()
            .s(Height::exact(ROW_HEIGHT))
            .s(Background::new().color(color!("SlateBlue", 0.8)))
            .s(RoundedCorners::new().left(15))
            .label(
                El::new()
                    .s(Align::center())
                    .s(Padding::new().left(20).right(17).y(10))
                    .child(name),
            )
            .on_press(move || {
                if let Some(index) = index.get() {
                    selected_var_refs.lock_mut().remove(index);
                }
            })
            .apply(Some)
    }
}
