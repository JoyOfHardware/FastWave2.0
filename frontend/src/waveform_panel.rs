use crate::{platform, script_bridge, Filename};
use std::sync::Arc;
use wellen::GetItem;
use zoon::*;

mod pixi_canvas;
use pixi_canvas::{PixiCanvas, PixiController};

const ROW_HEIGHT: u32 = 40;
const ROW_GAP: u32 = 4;

#[derive(Clone)]
pub struct WaveformPanel {
    selected_var_refs: MutableVec<wellen::VarRef>,
    hierarchy: Mutable<Option<Arc<wellen::Hierarchy>>>,
    loaded_filename: Mutable<Option<Filename>>,
    canvas_controller: Mutable<ReadOnlyMutable<Option<PixiController>>>,
}

impl WaveformPanel {
    pub fn new(
        hierarchy: Mutable<Option<Arc<wellen::Hierarchy>>>,
        selected_var_refs: MutableVec<wellen::VarRef>,
        loaded_filename: Mutable<Option<Filename>>,
    ) -> impl Element {
        Self {
            selected_var_refs,
            hierarchy,
            loaded_filename,
            canvas_controller: Mutable::new(Mutable::default().read_only()),
        }
        .root()
    }

    fn root(&self) -> impl Element {
        Column::new()
            .s(Padding::all(20))
            .s(Scrollbars::y_and_clip_x())
            .s(Width::fill())
            .s(Height::fill())
            .s(Gap::new().y(20))
            .item(self.selected_vars_controls())
            .item(self.vars_and_timelines_panel())
    }

    fn selected_vars_controls(&self) -> impl Element {
        Row::new()
            .s(Align::center())
            .s(Gap::new().x(20))
            .s(Width::fill())
            .item(Spacer::fill())
            .item(self.load_save_selected_vars_buttons())
            .item(self.keys_info())
    }

    fn keys_info(&self) -> impl Element {
        El::new().s(Width::fill()).child(
            Row::new()
                .s(Align::new().center_x())
                .s(Gap::new().x(15))
                .item(El::new().s(Font::new().no_wrap()).child("Zoom: Wheel"))
                .item(
                    El::new()
                        .s(Font::new().no_wrap())
                        .child("Pan: Shift + Wheel"),
                ),
        )
    }

    fn load_save_selected_vars_buttons(&self) -> impl Element {
        Row::new()
            .s(Gap::new().x(20))
            .item(self.load_selected_vars_button())
            .item(
                El::new()
                    .s(Font::new().no_wrap())
                    .child("Selected Variables"),
            )
            .item(self.save_selected_vars_button())
    }

    #[cfg(FASTWAVE_PLATFORM = "TAURI")]
    fn load_selected_vars_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| color!("MediumSlateBlue"), || color!("SlateBlue")),
            ))
            .s(Align::new().left())
            .s(RoundedCorners::all(15))
            .label("Load")
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(|| {
                Task::start(async move {
                    if let Some(javascript_code) =
                        platform::load_file_with_selected_vars(None).await
                    {
                        match script_bridge::strict_eval(&javascript_code) {
                            Ok(js_value) => {
                                zoon::println!("File with selected vars loaded: {js_value:?}")
                            }
                            Err(js_value) => {
                                zoon::eprintln!(
                                    "Failed to load file with selected vars: {js_value:?}"
                                )
                            }
                        }
                    }
                })
            })
    }

    #[cfg(FASTWAVE_PLATFORM = "BROWSER")]
    fn load_selected_vars_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let file_input_id = "file_input_for_load_selected_vars_button";
        Row::new()
            .item(
                Label::new()
                    .s(Padding::new().x(20).y(10))
                    .s(Background::new().color_signal(
                        hovered_signal
                            .map_bool(|| color!("MediumSlateBlue"), || color!("SlateBlue")),
                    ))
                    .s(Align::new().left())
                    .s(RoundedCorners::all(15))
                    .s(Cursor::new(CursorIcon::Pointer))
                    .label("Load")
                    .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                    .for_input(file_input_id),
            )
            .item(
                // @TODO https://github.com/MoonZoon/MoonZoon/issues/39
                // + https://developer.mozilla.org/en-US/docs/Web/API/File_API/Using_files_from_web_applications#using_hidden_file_input_elements_using_the_click_method
                TextInput::new().id(file_input_id).update_raw_el(|raw_el| {
                    let dom_element = raw_el.dom_element();
                    raw_el
                        .style("display", "none")
                        .attr("type", "file")
                        .event_handler(move |_: events::Input| {
                            let Some(file_list) =
                                dom_element.files().map(gloo_file::FileList::from)
                            else {
                                zoon::println!("file list is `None`");
                                return;
                            };
                            let Some(file) = file_list.first().cloned() else {
                                zoon::println!("file list is empty");
                                return;
                            };
                            Task::start(async move {
                                if let Some(javascript_code) =
                                    platform::load_file_with_selected_vars(Some(file)).await
                                {
                                    match script_bridge::strict_eval(&javascript_code) {
                                        Ok(js_value) => zoon::println!(
                                            "File with selected vars loaded: {js_value:?}"
                                        ),
                                        Err(js_value) => zoon::eprintln!(
                                            "Failed to load file with selected vars: {js_value:?}"
                                        ),
                                    }
                                }
                            })
                        })
                }),
            )
    }

    fn save_selected_vars_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let loaded_filename = self.loaded_filename.clone();
        let selected_var_refs = self.selected_var_refs.clone();
        let hierarchy = self.hierarchy.clone();
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| color!("MediumSlateBlue"), || color!("SlateBlue")),
            ))
            .s(RoundedCorners::all(15))
            .label("Save")
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || {
                let loaded_filename = loaded_filename.get_cloned().unwrap_throw();
                let file_name = format!("{}_vars.fw.js", loaded_filename.replace('.', "_"));

                let hierarchy = hierarchy.get_cloned().unwrap_throw();
                let mut full_var_names = Vec::new();
                for var_ref in selected_var_refs.lock_ref().as_slice() {
                    let var = hierarchy.get(*var_ref);
                    let var_name = var.full_name(&hierarchy);
                    full_var_names.push(format!("\"{var_name}\""));
                }
                let full_var_names_string = full_var_names.join(",\n\t\t");
                let file_content = include_str!("waveform_panel/template_vars.px.js")
                    .replacen("{LOADED_FILENAME}", &loaded_filename, 1)
                    .replacen("{FULL_VAR_NAMES}", &full_var_names_string, 1);

                // @TODO we need to use ugly code with temp anchor element until (if ever)
                // `showSaveFilePicker` is supported in Safari and Firefox (https://caniuse.com/?search=showSaveFilePicker)
                let file = gloo_file::File::new(&file_name, file_content.as_str());
                let file_object_url = gloo_file::ObjectUrl::from(file);
                let a = document().create_element("a").unwrap_throw();
                a.set_attribute("href", &file_object_url).unwrap_throw();
                a.set_attribute("download", &file_name).unwrap_throw();
                a.set_attribute("style", "display: none;").unwrap_throw();
                dom::body().append_child(&a).unwrap_throw();
                a.unchecked_ref::<web_sys::HtmlElement>().click();
                a.remove();
            })
    }

    // @TODO autoscroll down
    fn vars_and_timelines_panel(&self) -> impl Element {
        let selected_vars_panel_height_getter: Mutable<u32> = <_>::default();
        Row::new()
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
        hierarchy: &Mutable<Option<Arc<wellen::Hierarchy>>>,
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

        // @TODO render timeline with time units
        // let timescale = hierarchy.timescale();

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
                    .update_raw_el(|raw_el| {
                        raw_el
                            // @TODO move `title` to MZ API? (as `native_tooltip`?)
                            .attr("title", name)
                            // Note: `text-overflow` / ellipsis` doesn't work with flex and dynamic sizes
                            .style("text-overflow", "ellipsis")
                            .style("display", "inline-block")
                    })
                    .s(Scrollbars::both().visible(false))
                    .s(Width::default().max(400))
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
