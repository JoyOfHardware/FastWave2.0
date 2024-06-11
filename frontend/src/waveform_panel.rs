use crate::platform;
use std::rc::Rc;
use wellen::GetItem;
use zoon::*;

mod pixi_canvas;
use pixi_canvas::{PixiCanvas, PixiController};

const ROW_HEIGHT: u32 = 40;
const ROW_GAP: u32 = 4;

#[derive(Clone)]
pub struct WaveformPanel {
    selected_var_refs: MutableVec<wellen::VarRef>,
    hierarchy: Mutable<Option<Rc<wellen::Hierarchy>>>,
    canvas_controller: Mutable<ReadOnlyMutable<Option<PixiController>>>,
}

impl WaveformPanel {
    pub fn new(
        hierarchy: Mutable<Option<Rc<wellen::Hierarchy>>>,
        selected_var_refs: MutableVec<wellen::VarRef>,
    ) -> impl Element {
        Self {
            selected_var_refs,
            hierarchy,
            canvas_controller: Mutable::new(Mutable::default().read_only()),
        }
        .root()
    }

    // @TODO autoscroll down
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
        let hierarchy = self.hierarchy.clone();
        let canvas_controller = self.canvas_controller.clone();
        PixiCanvas::new(ROW_HEIGHT, ROW_GAP)
            .s(Align::new().top())
            .s(Width::fill())
            .s(Height::exact_signal(selected_vars_panel_height.signal()))
            .task_with_controller(move |controller| {
                canvas_controller.set(controller.clone());
                selected_var_refs.signal_vec().delay_remove(clone!((hierarchy) move |var_ref| {
                    clone!((var_ref, hierarchy) async move {
                        if let Some(hierarchy) = hierarchy.get_cloned() {
                            // @TODO unload only when no other selected variable use it?
                            platform::unload_signal(hierarchy.get(var_ref).signal_ref()).await;
                        }
                    })
                })).for_each(clone!((controller, hierarchy) move |vec_diff| {
                    clone!((controller, hierarchy) async move {
                        match vec_diff {
                            VecDiff::Replace { values } => {
                                let controller = controller.wait_for_some_cloned().await;
                                controller.clear_vars();
                                for var_ref in values {
                                    Self::push_var(&controller, &hierarchy, var_ref).await;
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
                                    Self::push_var(controller, &hierarchy, var_ref).await;
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
        hierarchy: &Mutable<Option<Rc<wellen::Hierarchy>>>,
        var_ref: wellen::VarRef,
    ) {
        let hierarchy = hierarchy.get_cloned().unwrap();

        let var_format = shared::VarFormat::default();

        let var = hierarchy.get(var_ref);
        let signal_ref = var.signal_ref();
        let timeline = platform::load_signal_and_get_timeline(
            signal_ref,
            controller.get_timeline_zoom(),
            controller.get_timeline_viewport_width(),
            controller.get_timeline_viewport_x(),
            ROW_HEIGHT,
            var_format,
        )
        .await;

        let timescale = hierarchy.timescale();
        // @TODO remove
        zoon::println!("{timescale:?}");

        // Note: Sync `timeline`'s type with the `Timeline` in `frontend/typescript/pixi_canvas/pixi_canvas.ts'
        let timeline = serde_wasm_bindgen::to_value(&timeline).unwrap_throw();
        let signal_ref_index = signal_ref.index();
        let var_format = serde_wasm_bindgen::to_value(&var_format).unwrap_throw();
        controller.push_var(signal_ref_index, timeline, var_format);
    }

    fn selected_var_panel(
        &self,
        index: ReadOnlyMutable<Option<usize>>,
        var_ref: wellen::VarRef,
    ) -> Option<impl Element> {
        let Some(hierarchy) = self.hierarchy.get_cloned() else {
            None?
        };
        let var = hierarchy.get(var_ref);
        Row::new()
            .item(self.selected_var_name_button(var.name(&hierarchy), index.clone()))
            .item(self.selected_var_format_button(index))
            .apply(Some)
    }

    fn selected_var_name_button(
        &self,
        name: &str,
        index: ReadOnlyMutable<Option<usize>>,
    ) -> impl Element {
        let selected_var_refs = self.selected_var_refs.clone();
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Height::exact(ROW_HEIGHT))
            .s(Width::growable())
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| color!("SlateBlue"), || color!("SlateBlue", 0.8)),
            ))
            .s(RoundedCorners::new().left(15).right(5))
            .label(
                El::new()
                    .s(Align::new().left())
                    .s(Padding::new().left(20).right(17).y(10))
                    .child(name),
            )
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || {
                if let Some(index) = index.get() {
                    selected_var_refs.lock_mut().remove(index);
                }
            })
    }

    fn selected_var_format_button(&self, index: ReadOnlyMutable<Option<usize>>) -> impl Element {
        let var_format = Mutable::new(shared::VarFormat::default());
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let canvas_controller = self.canvas_controller.clone();
        Button::new()
            .s(Height::exact(ROW_HEIGHT))
            .s(Width::exact(70))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| color!("SlateBlue"), || color!("SlateBlue", 0.8)),
            ))
            .s(RoundedCorners::new().left(5))
            .label(
                El::new()
                    .s(Align::center())
                    .s(Padding::new().left(20).right(17).y(10))
                    .child_signal(var_format.signal().map(|format| format.as_static_str())),
            )
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || {
                let next_format = var_format.get().next();
                var_format.set(next_format);
                if let Some(canvas_controller) = canvas_controller.get_cloned().lock_ref().as_ref()
                {
                    if let Some(index) = index.get() {
                        canvas_controller.set_var_format(
                            index,
                            serde_wasm_bindgen::to_value(&next_format).unwrap_throw(),
                        );
                    }
                }
            })
    }
}
