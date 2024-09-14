use crate::{script_bridge, theme::*};
use zoon::*;

pub struct CommandPanel {}

impl CommandPanel {
    pub fn new() -> impl Element {
        Self {}.root()
    }

    fn root(&self) -> impl Element {
        let command_result: Mutable<Option<Result<JsValue, JsValue>>> = <_>::default();
        Row::new()
            .s(Align::new().top())
            .s(Gap::both(30))
            .s(Width::fill())
            .s(Padding::new().x(20).bottom(20))
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
            .s(Background::new().color(COLOR_SLATE_BLUE))
            .s(Padding::new().x(10).y(8))
            .s(RoundedCorners::all(15))
            .s(Height::default().min(50))
            .s(Width::fill().min(300))
            .s(Font::new()
                .tracking(1)
                .weight(FontWeight::Medium)
                .color(COLOR_WHITE)
                .family([FontFamily::new("Courier New"), FontFamily::Monospace]))
            .s(Shadows::new([Shadow::new()
                .inner()
                .color(COLOR_DARK_SLATE_BLUE)
                .blur(4)]))
            // @TODO `spellcheck` and `resize` to MZ API? (together with autocomplete and others?)
            .update_raw_el(|raw_el| {
                raw_el
                    .attr("spellcheck", "false")
                    .style("resize", "vertical")
            })
            .placeholder(Placeholder::new("FW.say_hello()").s(Font::new().color(COLOR_LIGHT_BLUE)))
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
                        Task::start(clone!((script, command_result) async move {
                            let result = script_bridge::strict_eval(&script.lock_ref()).await;
                            command_result.set(Some(result));
                        }));
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
                .color(COLOR_WHITE)
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
