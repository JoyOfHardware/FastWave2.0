use crate::{platform, theme::*, Filename, Layout, Mode};
use std::sync::Arc;
use zoon::*;
use crate::term::TERM_OPEN;

pub struct HeaderPanel {
    hierarchy: Mutable<Option<Arc<wellen::Hierarchy>>>,
    layout: Mutable<Layout>,
    mode: Mutable<Mode>,
    loaded_filename: Mutable<Option<Filename>>,
}

impl HeaderPanel {
    pub fn new(
        hierarchy: Mutable<Option<Arc<wellen::Hierarchy>>>,
        layout: Mutable<Layout>,
        mode: Mutable<Mode>,
        loaded_filename: Mutable<Option<Filename>>,
    ) -> impl Element {
        Self {
            hierarchy,
            layout,
            loaded_filename,
            mode,
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
                    .item(self.layout_switcher())
                    .item(self.mode_switcher())
                    .item(self.open_terminal())
                    .item(self.open_konata_file()),
            )
    }

    #[cfg(FASTWAVE_PLATFORM = "TAURI")]
    fn load_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let hierarchy = self.hierarchy.clone();
        let loaded_filename = self.loaded_filename.clone();
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| COLOR_MEDIUM_SLATE_BLUE, || COLOR_SLATE_BLUE),
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
        let file_input_id = "file_input_for_load_waveform_button";
        Row::new()
            .item(
                Label::new()
                    .s(Padding::new().x(20).y(10))
                    .s(Background::new().color_signal(
                        hovered_signal.map_bool(|| COLOR_MEDIUM_SLATE_BLUE, || COLOR_SLATE_BLUE),
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
                hovered_signal.map_bool(|| COLOR_MEDIUM_SLATE_BLUE, || COLOR_SLATE_BLUE),
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

    fn mode_switcher(&self) -> impl Element {
        let mode = self.mode.clone();
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| COLOR_MEDIUM_SLATE_BLUE, || COLOR_SLATE_BLUE),
            ))
            .s(RoundedCorners::all(15))
            .label_signal(mode.signal().map(|mode| match mode {
                Mode::Waves => "Diagrams",
                Mode::Diagrams => "Waves",
            }))
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || {
                mode.update(|mode| match mode {
                    Mode::Waves => Mode::Diagrams,
                    Mode::Diagrams => Mode::Waves,
                })
            })
    }

    fn open_konata_file(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| COLOR_MEDIUM_SLATE_BLUE, || COLOR_SLATE_BLUE),
            ))
            .s(Align::new().left())
            .s(RoundedCorners::all(15))
            .label(
                El::new()
                    .s(Font::new().no_wrap())
                    .child("Open Konata file.."),
            )
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || Task::start(platform::open_konata_file()))
    }

    fn open_terminal(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| COLOR_MEDIUM_SLATE_BLUE, || COLOR_SLATE_BLUE),
            ))
            .s(Align::new().left())
            .s(RoundedCorners::all(15))
            .label(
                El::new()
                    .s(Font::new().no_wrap())
                    .child("Open Terminal"),
            )
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || {
                let term_open = TERM_OPEN.get();
                TERM_OPEN.set(!term_open);

            })
    }
}
