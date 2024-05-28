use crate::tauri_bridge;
use crate::HierarchyAndTimeTable;
use std::rc::Rc;
use wellen::GetItem;
use zoon::*;

#[derive(Clone, Copy)]
struct VarForUI<'a> {
    name: &'a str,
    var_type: wellen::VarType,
    var_direction: wellen::VarDirection,
    var_ref: wellen::VarRef,
    signal_type: wellen::SignalType,
}

#[derive(Clone)]
struct ScopeForUI<'a> {
    level: u32,
    name: &'a str,
    scope_ref: wellen::ScopeRef,
    has_children: bool,
    expanded: Mutable<bool>,
    parent_expanded: Option<ReadOnlyMutable<bool>>,
}

#[derive(Clone)]
pub struct ControlsPanel {
    selected_scope_ref: Mutable<Option<wellen::ScopeRef>>,
    hierarchy_and_time_table: Mutable<Option<HierarchyAndTimeTable>>,
    selected_var_refs: MutableVec<wellen::VarRef>,
}

impl ControlsPanel {
    pub fn new(
        hierarchy_and_time_table: Mutable<Option<HierarchyAndTimeTable>>,
        selected_var_refs: MutableVec<wellen::VarRef>,
    ) -> impl Element {
        Self {
            selected_scope_ref: <_>::default(),
            hierarchy_and_time_table,
            selected_var_refs,
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
        Column::new()
            .after_remove(move |_| drop(triggers))
            .s(Scrollbars::y_and_clip_x())
            .s(Height::fill())
            .s(Padding::all(20))
            .s(Gap::new().y(40))
            .s(Align::new().top())
            .item(
                Row::new()
                    .s(Gap::both(15))
                    .s(Align::new().left())
                    .item(self.load_button("simple.vcd"))
                    .item(self.load_button("wave_27.fst")),
            )
            .item_signal(
                self.hierarchy_and_time_table
                    .signal_cloned()
                    .map_some(clone!((self => s) move |(hierarchy, _)| s.scopes_panel(hierarchy))),
            )
            .item_signal(
                self.hierarchy_and_time_table
                    .signal_cloned()
                    .map_some(clone!((self => s) move |(hierarchy, _)| s.vars_panel(hierarchy))),
            )
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
                    let hierarchy = tauri_bridge::get_hierarchy().await;
                    let time_table = tauri_bridge::get_time_table().await;
                    hierarchy_and_time_table.set(Some((Rc::new(hierarchy), Rc::new(time_table))))
                })
            })
    }

    fn scopes_panel(&self, hierarchy: Rc<wellen::Hierarchy>) -> impl Element {
        Column::new()
            .s(Gap::new().y(20))
            .item(El::new().child("Scopes"))
            .item(self.scopes_list(hierarchy))
    }

    fn scopes_list(&self, hierarchy: Rc<wellen::Hierarchy>) -> impl Element {
        let mut scopes_for_ui = Vec::new();
        for scope_ref in hierarchy.scopes() {
            let mut scope_refs = Vec::new();
            scope_refs.push((0, scope_ref, None));
            while let Some((level, scope_ref, parent_expanded)) = scope_refs.pop() {
                let scope = hierarchy.get(scope_ref);
                let mut children = scope.scopes(&hierarchy).peekable();
                let has_children = children.peek().is_some();
                let expanded = Mutable::new(false);
                scopes_for_ui.push(ScopeForUI {
                    level,
                    name: scope.name(&hierarchy),
                    scope_ref,
                    has_children,
                    expanded: expanded.clone(),
                    parent_expanded,
                });
                for scope_ref in children {
                    scope_refs.push((level + 1, scope_ref, Some(expanded.read_only())));
                }
            }
        }
        Column::new()
            .s(Align::new().left())
            .s(Gap::new().y(10))
            .items(
                scopes_for_ui
                    .into_iter()
                    .map(clone!((self => s) move |scope_for_ui| s.scope_button_row(scope_for_ui))),
            )
    }

    fn scope_button_row(&self, scope_for_ui: ScopeForUI) -> impl Element {
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
        let task_collapse = {
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
        let display = signal::option(
            scope_for_ui
                .parent_expanded
                .clone()
                .map(|parent_expanded| parent_expanded.signal().map_false(|| "none")),
        )
        .map(Option::flatten);
        El::new()
            // @TODO Add `Display` Style to MoonZoon? Merge with `Visible` Style?
            .update_raw_el(|raw_el| raw_el.style_signal("display", display))
            .s(Padding::new().left(scope_for_ui.level * 30))
            .after_remove(move |_| drop(task_collapse))
            .child(
                Row::new()
                    .s(Background::new().color_signal(background_color))
                    .s(RoundedCorners::all(15))
                    .s(Clip::both())
                    .s(Align::new().left())
                    .item(scope_for_ui.has_children.then(clone!((self => s, scope_for_ui.expanded => expanded) move || s.scope_toggle(expanded))))
                    .item(self.scope_button(scope_for_ui, button_hovered))
            )
    }

    fn scope_toggle(&self, expanded: Mutable<bool>) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Padding::new().left(10))
            .s(Height::fill())
            .s(Font::new().color_signal(hovered_signal.map_true(|| color!("LightBlue"))))
            .label(
                El::new()
                    .s(Transform::with_signal_self(
                        expanded.signal().map_false(|| Transform::new().rotate(-90)),
                    ))
                    .child("▼"),
            )
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(move || expanded.update(not))
    }

    fn scope_button(
        &self,
        scope_for_ui: ScopeForUI,
        button_hovered: Mutable<bool>,
    ) -> impl Element {
        Button::new()
            .s(Padding::new().x(15).y(5))
            .on_hovered_change(move |is_hovered| button_hovered.set_neq(is_hovered))
            .on_press(clone!((self.selected_scope_ref => selected_scope_ref, scope_for_ui.scope_ref => scope_ref) move || selected_scope_ref.set_neq(Some(scope_ref))))
            .label(scope_for_ui.name)
    }

    fn vars_panel(&self, hierarchy: Rc<wellen::Hierarchy>) -> impl Element {
        let selected_scope_ref = self.selected_scope_ref.clone();
        Column::new()
            .s(Gap::new().y(20))
            .item(El::new().child("Variables"))
            .item_signal(selected_scope_ref.signal().map_some(
                clone!((self => s) move |scope_ref| s.vars_list(scope_ref, hierarchy.clone())),
            ))
    }

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
                    name: var.name(&hierarchy),
                    var_type: var.var_type(),
                    var_direction: var.direction(),
                    var_ref,
                    signal_type: var.signal_tpe(),
                }
            });
        Column::new()
            .s(Align::new().left())
            .s(Gap::new().y(10))
            .items(vars_for_ui.map(clone!((self => s) move |var_for_ui| s.var_row(var_for_ui))))
    }

    fn var_row(&self, var_for_ui: VarForUI) -> impl Element {
        Row::new()
            .s(Gap::new().x(10))
            .item(self.var_button(var_for_ui))
            .item(self.var_tag_type(var_for_ui))
            .item(self.var_tag_index(var_for_ui))
            .item(self.var_tag_bit(var_for_ui))
            .item(self.var_tag_direction(var_for_ui))
    }

    fn var_button(&self, var_for_ui: VarForUI) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let selected_var_ref = self.selected_var_refs.clone();
        El::new().child(
            Button::new()
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
