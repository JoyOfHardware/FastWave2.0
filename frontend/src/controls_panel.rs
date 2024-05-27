use crate::tauri_bridge;
use crate::HierarchyAndTimeTable;
use std::collections::VecDeque;
use std::rc::Rc;
use wellen::GetItem;
use zoon::{println, *};

#[derive(Clone, Copy)]
struct VarForUI<'a> {
    name: &'a str,
    var_type: wellen::VarType,
    var_direction: wellen::VarDirection,
    var_ref: wellen::VarRef,
    signal_type: wellen::SignalType,
}

#[derive(Clone, Copy)]
struct ScopeForUI<'a> {
    level: u32,
    name: &'a str,
    scope_ref: wellen::ScopeRef,
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
            .item(self.load_button())
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

    fn load_button(&self) -> impl Element {
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
                        .map_bool(|| "Unload simple.vcd", || "Load simple.vcd"),
                ),
            )
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            // @TODO REMOVE
            .after_insert(clone!((hierarchy_and_time_table) move |_| {
                if crate::SIMULATE_CLICKS {
                    let mut hierarchy_and_time_table_lock = hierarchy_and_time_table.lock_mut();
                    if hierarchy_and_time_table_lock.is_some() {
                        *hierarchy_and_time_table_lock = None;
                        return;
                    }
                    drop(hierarchy_and_time_table_lock);
                    let hierarchy_and_time_table = hierarchy_and_time_table.clone();
                    Task::start(async move {
                        tauri_bridge::load_waveform().await;
                        let hierarchy = tauri_bridge::get_hierarchy().await;
                        for variable in hierarchy.iter_vars() {
                            println!("{variable:?}");
                        }
                        for scope in hierarchy.iter_scopes() {
                            println!("{scope:?}");
                        }
                        let time_table = tauri_bridge::get_time_table().await;
                        println!("{time_table:?}");
                        hierarchy_and_time_table.set(Some((Rc::new(hierarchy), Rc::new(time_table))))
                    })
                }
            }))
            .on_press(move || {
                let mut hierarchy_and_time_table_lock = hierarchy_and_time_table.lock_mut();
                if hierarchy_and_time_table_lock.is_some() {
                    *hierarchy_and_time_table_lock = None;
                    return;
                }
                drop(hierarchy_and_time_table_lock);
                let hierarchy_and_time_table = hierarchy_and_time_table.clone();
                Task::start(async move {
                    tauri_bridge::load_waveform().await;
                    let hierarchy = tauri_bridge::get_hierarchy().await;
                    for variable in hierarchy.iter_vars() {
                        println!("{variable:?}");
                    }
                    for scope in hierarchy.iter_scopes() {
                        println!("{scope:?}");
                    }
                    let time_table = tauri_bridge::get_time_table().await;
                    println!("{time_table:?}");
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
            let mut scope_refs = VecDeque::new();
            scope_refs.push_back((0, scope_ref));
            while let Some((level, scope_ref)) = scope_refs.pop_front() {
                let scope = hierarchy.get(scope_ref);
                scopes_for_ui.push(ScopeForUI {
                    level,
                    name: scope.name(&hierarchy),
                    scope_ref,
                });
                for scope_ref in scope.scopes(&hierarchy) {
                    scope_refs.push_back((level + 1, scope_ref));
                }
            }
        }
        Column::new()
            .s(Align::new().left())
            .s(Gap::new().y(10))
            .items(
                scopes_for_ui
                    .into_iter()
                    .map(clone!((self => s) move |scope_for_ui| s.scope_button(scope_for_ui))),
            )
    }

    fn scope_button(&self, scope_for_ui: ScopeForUI) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        let selected_scope_ref = self.selected_scope_ref.clone();
        let is_selected = selected_scope_ref
            .signal()
            .map(move |selected_scope_ref| selected_scope_ref == Some(scope_for_ui.scope_ref));
        let background_color = map_ref! {
            let is_selected = is_selected,
            let is_hovered = hovered_signal => match (*is_selected, *is_hovered) {
                (true, _) => color!("BlueViolet"),
                (false, true) => color!("MediumSlateBlue"),
                (false, false) => color!("SlateBlue"),
            }
        };
        El::new()
            // @TODO REMOVE
            .after_insert(
                clone!((selected_scope_ref, scope_for_ui.scope_ref => scope_ref) move |_| {
                    if crate::SIMULATE_CLICKS {
                        selected_scope_ref.set_neq(Some(scope_ref));
                    }
                }),
            )
            .s(Padding::new().left(scope_for_ui.level * 30))
            .child(
                Button::new()
                    .s(Padding::new().x(15).y(5))
                    .s(Background::new().color_signal(background_color))
                    .s(RoundedCorners::all(15))
                    .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                    .on_press(move || selected_scope_ref.set_neq(Some(scope_for_ui.scope_ref)))
                    .label(scope_for_ui.name),
            )
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
                // @TODO REMOVE
                .after_insert(
                    clone!((selected_var_ref, var_for_ui.var_ref => var_ref) move |_| {
                        if crate::SIMULATE_CLICKS {
                            selected_var_ref.lock_mut().extend([var_ref, var_ref]);
                        }
                    }),
                )
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
