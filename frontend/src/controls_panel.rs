use crate::tauri_bridge;
use crate::{HierarchyAndTimeTable, Layout};
use futures_util::join;
use std::mem;
use std::ops::Not;
use std::rc::Rc;
use wellen::GetItem;
use zoon::*;

const SCOPE_VAR_ROW_MAX_WIDTH: u32 = 480;
const MILLER_COLUMN_MAX_HEIGHT: u32 = 500;

#[derive(Clone)]
struct VarForUI {
    name: Rc<String>,
    var_type: wellen::VarType,
    var_direction: wellen::VarDirection,
    var_ref: wellen::VarRef,
    signal_type: wellen::SignalType,
}

#[derive(Clone)]
struct ScopeForUI {
    level: usize,
    name: Rc<String>,
    scope_ref: wellen::ScopeRef,
    has_children: bool,
    expanded: Mutable<bool>,
    parent_expanded: Option<ReadOnlyMutable<bool>>,
    selected_scope_in_level: Mutable<Option<wellen::ScopeRef>>,
}

#[derive(Clone)]
pub struct ControlsPanel {
    selected_scope_ref: Mutable<Option<wellen::ScopeRef>>,
    hierarchy_and_time_table: Mutable<Option<HierarchyAndTimeTable>>,
    selected_var_refs: MutableVec<wellen::VarRef>,
    layout: Mutable<Layout>,
}

impl ControlsPanel {
    pub fn new(
        hierarchy_and_time_table: Mutable<Option<HierarchyAndTimeTable>>,
        selected_var_refs: MutableVec<wellen::VarRef>,
        layout: Mutable<Layout>,
    ) -> impl Element {
        Self {
            selected_scope_ref: <_>::default(),
            hierarchy_and_time_table,
            selected_var_refs,
            layout,
        }
        .root()
    }

    fn triggers(&self) -> Vec<TaskHandle> {
        vec![Task::start_droppable(
            self.hierarchy_and_time_table
                .signal_ref(Option::is_none)
                .for_each_sync(clone!((self => s) move |_| {
                    s.selected_scope_ref.set(None);
                    s.selected_var_refs.lock_mut().clear();
                })),
        )]
    }

    fn root(&self) -> impl Element {
        let triggers = self.triggers();
        let layout = self.layout.clone();
        let layout_and_hierarchy_signal = map_ref! {
            let layout = layout.signal(),
            let hierarchy_and_time_table = self.hierarchy_and_time_table.signal_cloned() => {
                (*layout, hierarchy_and_time_table.clone().map(|(hierarchy, _)| hierarchy))
            }
        };
        Column::new()
            .after_remove(move |_| drop(triggers))
            .s(Width::with_signal_self(
                self.layout
                    .signal()
                    .map(|layout| matches!(layout, Layout::Columns))
                    .map_true(|| Width::fill()),
            ))
            .s(Height::with_signal_self(layout.signal().map(
                move |layout| match layout {
                    Layout::Tree => Height::fill(),
                    Layout::Columns => Height::fill().max(MILLER_COLUMN_MAX_HEIGHT),
                },
            )))
            .s(Scrollbars::both())
            .s(Padding::all(20))
            .s(Gap::new().y(40))
            .s(Align::new().top())
            .item(
                Row::new()
                    .s(Gap::both(15))
                    .s(Align::new().left())
                    .item(self.load_button("simple.vcd"))
                    .item(self.load_button("wave_27.fst"))
                    .item(self.layout_switcher()),
            )
            .item_signal(
                self.hierarchy_and_time_table
                    .signal_cloned()
                    .map_some(clone!((self => s) move |(hierarchy, _)| s.scopes_panel(hierarchy))),
            )
            .item_signal(layout_and_hierarchy_signal.map(
                clone!((self => s) move |(layout, hierarchy)| {
                    hierarchy.and_then(clone!((s) move |hierarchy| {
                        matches!(layout, Layout::Tree).then(move || s.vars_panel(hierarchy))
                    }))
                }),
            ))
    }

    fn load_button(&self, test_file_name: &'static str) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let hierarchy_and_time_table = self.hierarchy_and_time_table.clone();
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| color!("MediumSlateBlue"), || color!("SlateBlue")),
            ))
            .s(Align::new().left())
            .s(RoundedCorners::all(15))
            .label(
                El::new().s(Font::new().no_wrap()).child_signal(
                    hierarchy_and_time_table
                        .signal_ref(Option::is_some)
                        .map_bool(
                            || format!("Unload test file"),
                            move || format!("Load {test_file_name}"),
                        ),
                ),
            )
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || {
                let mut hierarchy_and_time_table_lock = hierarchy_and_time_table.lock_mut();
                if hierarchy_and_time_table_lock.is_some() {
                    *hierarchy_and_time_table_lock = None;
                    return;
                }
                drop(hierarchy_and_time_table_lock);
                let hierarchy_and_time_table = hierarchy_and_time_table.clone();
                Task::start(async move {
                    tauri_bridge::load_waveform(test_file_name).await;
                    let (hierarchy, time_table) = join!(
                        tauri_bridge::get_hierarchy(),
                        tauri_bridge::get_time_table()
                    );
                    hierarchy_and_time_table.set(Some((Rc::new(hierarchy), Rc::new(time_table))))
                })
            })
    }

    fn layout_switcher(&self) -> impl Element {
        let layout = self.layout.clone();
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Padding::new().x(20).y(10))
            .s(Background::new().color_signal(
                hovered_signal.map_bool(|| color!("MediumSlateBlue"), || color!("SlateBlue")),
            ))
            .s(Align::new().left())
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

    fn scopes_panel(&self, hierarchy: Rc<wellen::Hierarchy>) -> impl Element {
        Column::new()
            .s(Height::fill().min(150))
            .s(Scrollbars::y_and_clip_x())
            .s(Gap::new().y(20))
            .s(Width::fill())
            .item_signal(
                self.layout
                    .signal()
                    .map(|layout| matches!(layout, Layout::Tree))
                    .map_true(|| El::new().child("Scopes")),
            )
            .item(self.scopes_list(hierarchy))
    }

    fn scopes_list(&self, hierarchy: Rc<wellen::Hierarchy>) -> impl Element {
        let layout = self.layout.clone();
        let mut scopes_for_ui = Vec::new();
        let mut max_level_index: usize = 0;
        for scope_ref in hierarchy.scopes() {
            let mut scope_refs = Vec::new();
            scope_refs.push((0, scope_ref, None));
            let mut selected_scope_in_levels: Vec<Mutable<Option<wellen::ScopeRef>>> =
                vec![<_>::default()];
            while let Some((level, scope_ref, parent_expanded)) = scope_refs.pop() {
                let scope = hierarchy.get(scope_ref);
                let mut children = scope.scopes(&hierarchy).peekable();
                let has_children = children.peek().is_some();
                let expanded = Mutable::new(false);
                if level > max_level_index {
                    max_level_index = level;
                    selected_scope_in_levels.push(<_>::default());
                }
                scopes_for_ui.push(ScopeForUI {
                    level,
                    name: Rc::new(scope.name(&hierarchy).to_owned()),
                    scope_ref,
                    has_children,
                    expanded: expanded.clone(),
                    parent_expanded,
                    selected_scope_in_level: selected_scope_in_levels[level].clone(),
                });
                for scope_ref in children {
                    scope_refs.push((level + 1, scope_ref, Some(expanded.read_only())));
                }
            }
        }
        let scopes_for_ui = Rc::new(scopes_for_ui);
        let s = self.clone();
        El::new()
            .s(Height::fill())
            .s(Scrollbars::both())
            .s(Width::fill())
            .child_signal(layout.signal().map(move |layout| match layout {
                Layout::Tree => {
                    Column::new()
                        .s(Align::new().left())
                        .s(Gap::new().y(10))
                        .s(Height::fill())
                        .s(Scrollbars::y_and_clip_x())
                        .s(Padding::new().right(15))
                        .items(
                            scopes_for_ui
                                .iter()
                                .map(clone!((s) move |scope_for_ui| s.scope_button_row(scope_for_ui.clone()))),
                        ).unify()
                }
                Layout::Columns => {
                    let mut scopes_for_ui_in_levels: Vec<Vec<ScopeForUI>> = vec![Vec::new(); max_level_index + 1];
                    for scope_for_ui in scopes_for_ui.iter() {
                        scopes_for_ui_in_levels[scope_for_ui.level].push(scope_for_ui.clone());
                    }
                    let viewport_x = Mutable::new(0);
                    El::new()
                        .s(Height::fill())
                        .s(Scrollbars::x_and_clip_y())
                        .s(Padding::new().bottom(15))
                        .s(Width::fill())
                        .viewport_x_signal(viewport_x.signal())
                        .child(
                            Row::new()
                                .s(Height::fill())
                                // @TODO add `width: max-content` to MoonZoon's `Width`?
                                .update_raw_el(|raw_el| raw_el.style("width", "max-content"))
                                .on_viewport_size_change(move |_, _| viewport_x.set(i32::MAX))
                                .items(scopes_for_ui_in_levels.into_iter().map(|scopes_in_level| {
                                    Column::new()
                                        .s(Height::fill())
                                        .s(Scrollbars::y_and_clip_x())
                                        // @TODO `Width::default` add the class `exact_width` with `flex-shrink: 0;`
                                        // We should make it more explicit / discoverable in MoonZoon
                                        .s(Width::default())
                                        .s(Gap::new().y(10))
                                        .s(Padding::new().x(10))
                                        .items(
                                            scopes_in_level
                                                .into_iter()
                                                .map(clone!((s) move |scope_for_ui| s.scope_button_row(scope_for_ui)))
                                        )
                                }))
                                .item(s.vars_panel(hierarchy.clone()))
                        ).unify()
                }
            }))
    }

    fn scope_button_row(&self, scope_for_ui: ScopeForUI) -> impl Element {
        let layout = self.layout.clone();
        let (button_hovered, button_hovered_signal) = Mutable::new_and_signal(false);
        let selected_scope_ref = self.selected_scope_ref.clone();
        let is_selected = selected_scope_ref
            .signal()
            .map(move |selected_scope_ref| selected_scope_ref == Some(scope_for_ui.scope_ref));
        let background_color = map_ref! {
            let is_selected = is_selected,
            let is_hovered = button_hovered_signal => match (*is_selected, *is_hovered) {
                (true, _) => color!("BlueViolet"),
                (false, true) => color!("MediumSlateBlue"),
                (false, false) => color!("SlateBlue"),
            }
        };
        let task_collapse_on_parent_collapse = {
            let expanded = scope_for_ui.expanded.clone();
            scope_for_ui.parent_expanded.clone().map(|parent_expanded| {
                Task::start_droppable(parent_expanded.signal().for_each_sync(
                    move |parent_expanded| {
                        if not(parent_expanded) {
                            expanded.set_neq(false);
                        }
                    },
                ))
            })
        };
        let task_expand_or_collapse_on_selected_scope_in_level_change = {
            let expanded = scope_for_ui.expanded.clone();
            let scope_ref = scope_for_ui.scope_ref;
            let layout = layout.clone();
            Task::start_droppable(scope_for_ui.selected_scope_in_level.signal().for_each_sync(
                move |selected_scope_in_level| {
                    if matches!(layout.get(), Layout::Columns) {
                        if let Some(selected_scope) = selected_scope_in_level {
                            if selected_scope == scope_ref {
                                return expanded.set(true);
                            }
                        }
                        expanded.set(false);
                    }
                },
            ))
        };
        let display = signal::option(
            scope_for_ui
                .parent_expanded
                .clone()
                .map(|parent_expanded| parent_expanded.signal().map_false(|| "none")),
        )
        .map(Option::flatten);
        let level = scope_for_ui.level as u32;
        El::new()
            // @TODO Add `Display` Style to MoonZoon? Merge with `Visible` Style?
            .update_raw_el(|raw_el| raw_el.style_signal("display", display))
            .s(Padding::new().left_signal(layout.signal().map(move |layout| match layout {
                Layout::Tree => level * 30,
                Layout::Columns => 0,
            })))
            .s(Width::default().max(SCOPE_VAR_ROW_MAX_WIDTH))
            .after_remove(move |_| {
                drop(task_collapse_on_parent_collapse);
                drop(task_expand_or_collapse_on_selected_scope_in_level_change);
            })
            .child(
                Row::new()
                    .s(Background::new().color_signal(background_color))
                    .s(RoundedCorners::all(15))
                    .s(Clip::both())
                    .s(Align::new().left())
                    .items_signal_vec(layout.signal().map(clone!((self => s, scope_for_ui) move |layout| {
                        let toggle = scope_for_ui.has_children.then(clone!((s, scope_for_ui) move || s.scope_toggle(scope_for_ui)));
                        let button = s.scope_button(scope_for_ui.clone(), button_hovered.clone());
                        match layout {
                            Layout::Tree => element_vec![toggle, button],
                            Layout::Columns => element_vec![button, toggle],
                        }
                    })).to_signal_vec())
            )
    }

    fn scope_toggle(&self, scope_for_ui: ScopeForUI) -> impl Element {
        let layout = self.layout.clone();
        let expanded = scope_for_ui.expanded.clone();
        let layout_and_expanded = map_ref! {
            let layout = layout.signal(),
            let expanded = expanded.signal() => (*layout, *expanded)
        };
        let selected_scope_ref: Mutable<Option<wellen::ScopeRef>> = self.selected_scope_ref.clone();
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Padding::new()
                .left_signal(
                    self.layout
                        .signal()
                        .map(|layout| matches!(layout, Layout::Tree))
                        .map_true(|| 10),
                )
                .right_signal(
                    self.layout
                        .signal()
                        .map(|layout| matches!(layout, Layout::Columns))
                        .map_true(|| 10),
                ))
            .s(Height::fill())
            .s(Font::new().color_signal(hovered_signal.map_true(|| color!("LightBlue"))))
            .label(
                El::new()
                    .s(Transform::with_signal_self(layout_and_expanded.map(
                        |(layout, expanded)| match layout {
                            Layout::Tree => expanded.not().then(|| Transform::new().rotate(-90)),
                            Layout::Columns => {
                                Some(Transform::new().rotate(if expanded { -90 } else { 90 }))
                            }
                        },
                    )))
                    .child("▼"),
            )
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || match layout.get() {
                Layout::Tree => {
                    if scope_for_ui.expanded.get() {
                        scope_for_ui.selected_scope_in_level.set(None);
                    } else {
                        scope_for_ui
                            .selected_scope_in_level
                            .set(Some(scope_for_ui.scope_ref));
                    }
                    scope_for_ui.expanded.update(not)
                }
                Layout::Columns => {
                    selected_scope_ref.set_neq(None);
                    if scope_for_ui.expanded.get() {
                        scope_for_ui.selected_scope_in_level.set(None);
                    } else {
                        scope_for_ui
                            .selected_scope_in_level
                            .set(Some(scope_for_ui.scope_ref));
                    }
                }
            })
    }

    fn scope_button(
        &self,
        scope_for_ui: ScopeForUI,
        button_hovered: Mutable<bool>,
    ) -> impl Element {
        Button::new()
            .s(Padding::new().x(15).y(5))
            .s(Font::new().wrap_anywhere())
            .on_hovered_change(move |is_hovered| button_hovered.set_neq(is_hovered))
            .on_press(
                clone!((self.selected_scope_ref => selected_scope_ref, scope_for_ui) move || {
                    selected_scope_ref.set_neq(Some(scope_for_ui.scope_ref));
                    scope_for_ui.selected_scope_in_level.set_neq(None);
                }),
            )
            .label(scope_for_ui.name)
    }

    fn vars_panel(&self, hierarchy: Rc<wellen::Hierarchy>) -> impl Element {
        let selected_scope_ref = self.selected_scope_ref.clone();
        Column::new()
            .s(Gap::new().y(20))
            .s(Height::fill().min(150))
            .s(Scrollbars::y_and_clip_x())
            .item_signal(
                self.layout
                    .signal()
                    .map(|layout| matches!(layout, Layout::Tree))
                    .map_true(|| El::new().child("Variables")),
            )
            .item_signal(selected_scope_ref.signal().map_some(
                clone!((self => s) move |scope_ref| s.vars_list(scope_ref, hierarchy.clone())),
            ))
    }

    // @TODO Group variables?
    fn vars_list(
        &self,
        selected_scope_ref: wellen::ScopeRef,
        hierarchy: Rc<wellen::Hierarchy>,
    ) -> impl Element {
        let vars_for_ui = hierarchy
            .get(selected_scope_ref)
            .vars(&hierarchy)
            .map(|var_ref| {
                let var = hierarchy.get(var_ref);
                VarForUI {
                    name: Rc::new(var.name(&hierarchy).to_owned()),
                    var_type: var.var_type(),
                    var_direction: var.direction(),
                    var_ref,
                    signal_type: var.signal_tpe(),
                }
            });

        // Lazy loading to not freeze the main thread
        const CHUNK_SIZE: usize = 50;
        let mut chunked_vars_for_ui: Vec<Vec<VarForUI>> = <_>::default();
        let mut chunk = Vec::with_capacity(CHUNK_SIZE);
        for (index, var_for_ui) in vars_for_ui.enumerate() {
            chunk.push(var_for_ui);
            if index % CHUNK_SIZE == 0 {
                chunked_vars_for_ui.push(mem::take(&mut chunk));
            }
        }
        if not(chunk.is_empty()) {
            chunked_vars_for_ui.push(chunk);
        }
        let vars_for_ui_mutable_vec = MutableVec::<VarForUI>::new();
        let append_vars_for_ui_task =
            Task::start_droppable(clone!((vars_for_ui_mutable_vec) async move {
                for chunk in chunked_vars_for_ui {
                    Task::next_macro_tick().await;
                    vars_for_ui_mutable_vec.lock_mut().extend(chunk);
                }
            }));

        Column::new()
            .s(Width::with_signal_self(
                self.layout
                    .signal()
                    .map(|layout| matches!(layout, Layout::Columns))
                    .map_true(|| Width::default().min(SCOPE_VAR_ROW_MAX_WIDTH)),
            ))
            .s(Align::new().left())
            .s(Gap::new().y(10))
            .s(Height::fill())
            .s(Scrollbars::y_and_clip_x())
            .items_signal_vec(
                vars_for_ui_mutable_vec
                    .signal_vec_cloned()
                    .map(clone!((self => s) move |var_for_ui| s.var_row(var_for_ui))),
            )
            .after_remove(move |_| drop(append_vars_for_ui_task))
    }

    fn var_row(&self, var_for_ui: VarForUI) -> impl Element {
        Row::new()
            .s(Gap::new().x(10))
            .s(Padding::new().right(15))
            .s(Width::default().max(SCOPE_VAR_ROW_MAX_WIDTH))
            .item(self.var_button(var_for_ui.clone()))
            .item(self.var_tag_type(var_for_ui.clone()))
            .item(self.var_tag_index(var_for_ui.clone()))
            .item(self.var_tag_bit(var_for_ui.clone()))
            .item(self.var_tag_direction(var_for_ui))
    }

    fn var_button(&self, var_for_ui: VarForUI) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let selected_var_ref = self.selected_var_refs.clone();
        El::new().child(
            Button::new()
                .s(Font::new().wrap_anywhere())
                .s(Padding::new().x(15).y(5))
                .s(Background::new().color_signal(
                    hovered_signal.map_bool(|| color!("MediumSlateBlue"), || color!("SlateBlue")),
                ))
                .s(RoundedCorners::all(15))
                .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                .on_press(move || selected_var_ref.lock_mut().push(var_for_ui.var_ref))
                .label(var_for_ui.name),
        )
    }

    fn var_tag_type(&self, var_for_ui: VarForUI) -> impl Element {
        let var_type = var_for_ui.var_type;
        El::new().child(format!("{var_type:?}"))
    }

    fn var_tag_index(&self, var_for_ui: VarForUI) -> Option<impl Element> {
        let wellen::SignalType::BitVector(_, Some(index)) = var_for_ui.signal_type else {
            None?
        };
        let msb = index.msb();
        let lsb = index.lsb();
        El::new().child(format!("[{msb}:{lsb}]")).apply(Some)
    }

    fn var_tag_bit(&self, var_for_ui: VarForUI) -> Option<impl Element> {
        let wellen::SignalType::BitVector(length, _) = var_for_ui.signal_type else {
            None?
        };
        El::new()
            .s(Font::new().no_wrap())
            .child(format!("{length}-bit"))
            .apply(Some)
    }

    fn var_tag_direction(&self, var_for_ui: VarForUI) -> impl Element {
        let direction = match var_for_ui.var_direction {
            wellen::VarDirection::Unknown => String::new(),
            direction => format!("{direction:?}"),
        };
        El::new().child(direction)
    }
}
