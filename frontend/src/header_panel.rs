use crate::{platform, script_bridge, Filename, Layout};
use std::sync::Arc;
use zoon::*;

pub struct HeaderPanel {
    hierarchy: Mutable<Option<Arc<wellen::Hierarchy>>>,
    layout: Mutable<Layout>,
    loaded_filename: Mutable<Option<Filename>>,
}

impl HeaderPanel {
    pub fn new(
        hierarchy: Mutable<Option<Arc<wellen::Hierarchy>>>,
        layout: Mutable<Layout>,
        loaded_filename: Mutable<Option<Filename>>,
    ) -> impl Element {
        Self {
            hierarchy,
            layout,
            loaded_filename,
        }
        .root()
    }

    fn root(&self) -> impl Element {
        Row::new()
            .s(Padding::new().x(20).y(15))
            .s(Gap::both(40))
            .item(
                Row::new()
                    .s(Align::new().top())
                    .s(Padding::new().top(5))
                    .s(Gap::both(15))
                    .item(self.load_button())
                    .item(self.layout_switcher()),
            )
            .item(self.command_panel())
    }

    #[cfg(FASTWAVE_PLATFORM = "TAURI")]
    fn load_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let hierarchy = self.hierarchy.clone();
        let loaded_filename = self.loaded_filename.clone();
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| color!("MediumSlateBlue"), || color!("SlateBlue")),
            ))
            .s(Align::new().left())
            .s(RoundedCorners::all(15))
            .label(El::new().s(Font::new().no_wrap()).child_signal(
                loaded_filename.signal_cloned().map_option(
                    |filename| format!("Unload {filename}"),
                    || format!("Load file.."),
                ),
            ))
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || {
                let mut hierarchy_lock = hierarchy.lock_mut();
                if hierarchy_lock.is_some() {
                    *hierarchy_lock = None;
                    return;
                }
                drop(hierarchy_lock);
                let hierarchy = hierarchy.clone();
                let loaded_filename = loaded_filename.clone();
                Task::start(async move {
                    if let Some(filename) = platform::pick_and_load_waveform(None).await {
                        loaded_filename.set_neq(Some(filename));
                        hierarchy.set(Some(Arc::new(platform::get_hierarchy().await)))
                    }
                })
            })
    }

    #[cfg(FASTWAVE_PLATFORM = "BROWSER")]
    fn load_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let hierarchy = self.hierarchy.clone();
        let loaded_filename = self.loaded_filename.clone();
        let file_input_id = "file_input";
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
                    .label(El::new().s(Font::new().no_wrap()).child_signal(
                        loaded_filename.signal_cloned().map_option(
                            |filename| format!("Unload {filename}"),
                            || format!("Load file.."),
                        ),
                    ))
                    .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                    .for_input(file_input_id)
                    .on_click_event_with_options(
                        EventOptions::new().preventable(),
                        clone!((hierarchy) move |event| {
                            let mut hierarchy_lock = hierarchy.lock_mut();
                            if hierarchy_lock.is_some() {
                                *hierarchy_lock = None;
                                if let RawMouseEvent::Click(raw_event) = event.raw_event {
                                    // @TODO Move to MoonZoon as a new API
                                    raw_event.prevent_default();
                                }
                                return;
                            }
                        }),
                    ),
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
                            let hierarchy = hierarchy.clone();
                            let loaded_filename = loaded_filename.clone();
                            Task::start(async move {
                                if let Some(filename) =
                                    platform::pick_and_load_waveform(Some(file)).await
                                {
                                    loaded_filename.set_neq(Some(filename));
                                    hierarchy.set(Some(Arc::new(platform::get_hierarchy().await)))
                                }
                            })
                        })
                }),
            )
    }

    fn layout_switcher(&self) -> impl Element {
        let layout = self.layout.clone();
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| color!("MediumSlateBlue"), || color!("SlateBlue")),
            ))
            .s(RoundedCorners::all(15))
            .label_signal(layout.signal().map(|layout| match layout {
                Layout::Tree => "Columns",
                Layout::Columns => "Tree",
            }))
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || {
                layout.update(|layout| match layout {
                    Layout::Tree => Layout::Columns,
                    Layout::Columns => Layout::Tree,
                })
            })
    }

    fn command_panel(&self) -> impl Element {
        let command_result: Mutable<Option<Result<JsValue, JsValue>>> = <_>::default();
        Row::new()
            .s(Align::new().top())
            .s(Gap::both(30))
            .s(Scrollbars::both())
            .s(Width::fill())
            .item(self.command_editor_panel(command_result.clone()))
            .item(self.command_result_panel(command_result.read_only()))
    }

    fn command_editor_panel(
        &self,
        command_result: Mutable<Option<Result<JsValue, JsValue>>>,
    ) -> impl Element {
        Column::new()
            .s(Align::new().top())
            .s(Gap::new().y(10))
            .s(Width::growable())
            .item(
                Row::new()
                    .s(Gap::new().x(15))
                    .s(Padding::new().x(5))
                    .item(El::new().child("Javascript commands"))
                    .item(El::new().s(Align::new().right()).child("Shift + Enter")),
            )
            .item(self.command_editor(command_result))
    }

    fn command_editor(
        &self,
        command_result: Mutable<Option<Result<JsValue, JsValue>>>,
    ) -> impl Element {
        let (script, script_signal) = Mutable::new_and_signal_cloned(String::new());
        // @TODO perhaps replace with an element with syntax highlighter like https://github.com/WebCoder49/code-input later
        TextArea::new()
            .s(Background::new().color(color!("SlateBlue")))
            .s(Padding::new().x(10).y(8))
            .s(RoundedCorners::all(15))
            .s(Height::default().min(50))
            .s(Width::fill().min(300))
            .s(Font::new()
                .tracking(1)
                .weight(FontWeight::Medium)
                .color(color!("White"))
                .family([FontFamily::new("Courier New"), FontFamily::Monospace]))
            .s(Shadows::new([Shadow::new()
                .inner()
                .color(color!("DarkSlateBlue"))
                .blur(4)]))
            // @TODO `spellcheck` and `resize` to MZ API? (together with autocomplete and others?)
            .update_raw_el(|raw_el| {
                raw_el
                    .attr("spellcheck", "false")
                    .style("resize", "vertical")
            })
            .placeholder(
                Placeholder::new("FW.say_hello()").s(Font::new().color(color!("LightBlue"))),
            )
            .label_hidden("command editor panel")
            .text_signal(script_signal)
            .on_change(clone!((script, command_result) move |text| {
                script.set_neq(text);
                command_result.set_neq(None);
            }))
            .on_key_down_event_with_options(EventOptions::new().preventable(), move |event| {
                if event.key() == &Key::Enter {
                    let RawKeyboardEvent::KeyDown(raw_event) = event.raw_event.clone();
                    if raw_event.shift_key() {
                        // @TODO move `prevent_default` to MZ API (next to the `pass_to_parent` method?)
                        raw_event.prevent_default();
                        let result = script_bridge::strict_eval(&script.lock_ref());
                        command_result.set(Some(result));
                    }
                }
            })
    }

    fn command_result_panel(
        &self,
        command_result: ReadOnlyMutable<Option<Result<JsValue, JsValue>>>,
    ) -> impl Element {
        Column::new()
            .s(Gap::new().y(10))
            .s(Align::new().top())
            .s(Scrollbars::both())
            .s(Padding::new().x(5))
            .s(Width::growable().max(750))
            .item(El::new().child("Command result"))
            .item(self.command_result_el(command_result))
    }

    fn command_result_el(
        &self,
        command_result: ReadOnlyMutable<Option<Result<JsValue, JsValue>>>,
    ) -> impl Element {
        El::new()
            .s(Font::new()
                .tracking(1)
                .weight(FontWeight::Medium)
                .color(color!("White"))
                .family([FontFamily::new("Courier New"), FontFamily::Monospace]))
            .s(Scrollbars::both())
            .s(Height::default().max(100))
            .child_signal(command_result.signal_ref(|result| {
                fn format_complex_js_value(js_value: &JsValue) -> String {
                    let value = format!("{js_value:?}");
                    let value = value.strip_prefix("JsValue(").unwrap_throw();
                    let value = value.strip_suffix(')').unwrap_throw();
                    value.to_owned()
                }
                match result {
                    Some(Ok(js_value)) => {
                        if let Some(string_value) = js_value.as_string() {
                            string_value
                        } else if let Some(number_value) = js_value.as_f64() {
                            number_value.to_string()
                        } else if let Some(bool_value) = js_value.as_bool() {
                            bool_value.to_string()
                        } else {
                            format_complex_js_value(js_value)
                        }
                    }
                    Some(Err(js_value)) => {
                        format!("ERROR: {}", format_complex_js_value(js_value))
                    }
                    None => "-".to_owned(),
                }
            }))
    }
}
