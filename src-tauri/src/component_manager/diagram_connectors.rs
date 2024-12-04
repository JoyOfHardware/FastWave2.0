use crate::{
    AddedDiagramConnectorsCount, ComponentId, DiagramConnectorName, DiagramConnectorPath,
    RemovedDiagramConnectorsCount, APP_HANDLE, WAVEFORM,
};
use once_cell::sync::Lazy;
use shared::{DiagramConnectorMessage, VarFormat};
use std::sync::Arc;
use tauri::async_runtime::{Mutex, RwLock};
use tauri::Emitter;
use wasmtime::component::{Component as WasmtimeComponent, *};
use wasmtime::{AsContextMut, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiView};
use wellen::GetItem;

bindgen!(in "wit/diagram_connector");

pub static DIAGRAM_CONNECTORS: Lazy<Arc<RwLock<Vec<Component>>>> = Lazy::new(<_>::default);
static ENGINE: Lazy<Engine> = Lazy::new(<_>::default);
static LINKER: Lazy<Linker<State>> = Lazy::new(|| {
    let mut linker = Linker::new(&ENGINE);
    wasmtime_wasi::add_to_linker_sync(&mut linker).unwrap();
    Component::add_to_linker(&mut linker, |state: &mut State| state).unwrap();
    linker
});
pub static STORE: Lazy<Arc<Mutex<Store<State>>>> = Lazy::new(|| {
    let store = Store::new(
        &ENGINE,
        State {
            ctx: WasiCtx::builder().build(),
            table: ResourceTable::new(),
        },
    );
    Arc::new(Mutex::new(store))
});

pub struct State {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for State {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

impl component::diagram_connector::host::Host for State {
    fn log(&mut self, message: String) {
        println!("Diagram Connector: {message}");
    }

    fn listen_for_component_text_changes(
        &mut self,
        diagram_connector_name: String,
        component_id: String,
    ) {
        let message = DiagramConnectorMessage::ListenForComponentTextChanges {
            diagram_connector_name,
            component_id,
        };
        APP_HANDLE
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .emit("diagram_connector_message", message)
            .unwrap();
    }

    fn set_component_text(&mut self, component_id: String, text: String) {
        let message = DiagramConnectorMessage::SetComponentText { component_id, text };
        APP_HANDLE
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .emit("diagram_connector_message", message)
            .unwrap();
    }

    // @TODO `resource` in WIT or async in the future
    // @TODO move business logic to the diagram connector
    fn address_and_way(&mut self, time_text: String) -> Result<(String, Option<u32>), ()> {
        let input_time = time_text.parse::<u64>().map_err(|error| {
            eprintln!("Failed to parse time_text '{time_text}', error: {error:#}");
        })?;

        let waveform_wrapper = WAVEFORM.read().unwrap();
        let mut maybe_waveform = waveform_wrapper.try_write().unwrap();
        let waveform = maybe_waveform.as_mut().unwrap();

        let hierarchy = waveform.hierarchy();

        // @TODO remove
        // let timescale = hierarchy.timescale().unwrap();
        // println!("Timescale: {timescale:#?}");

        let refill_valid_ref = hierarchy
            .lookup_var(
                &["TOP", "VexiiRiscv"],
                &"FetchL1Plugin_logic_refill_start_valid",
            )
            .unwrap();
        let refill_address_ref = hierarchy
            .lookup_var(
                &["TOP", "VexiiRiscv"],
                &"FetchL1Plugin_logic_refill_start_address",
            )
            .unwrap();
        let refill_way_ref = hierarchy
            .lookup_var(
                &["TOP", "VexiiRiscv"],
                &"FetchL1Plugin_logic_refill_start_wayToAllocate",
            )
            .unwrap();

        let refill_valid_var = hierarchy.get(refill_valid_ref);
        let refill_address_var = hierarchy.get(refill_address_ref);
        let refill_way_var = hierarchy.get(refill_way_ref);

        let refill_valid_signal_ref = refill_valid_var.signal_ref();
        let refill_address_signal_ref = refill_address_var.signal_ref();
        let refill_way_signal_ref = refill_way_var.signal_ref();

        let mut time_table_idx = None;
        for (idx, time) in waveform.time_table().iter().enumerate() {
            if *time >= input_time {
                time_table_idx = Some(idx as u32);
                break;
            }
        }
        let Some(time_table_idx) = time_table_idx else {
            eprintln!("time_table_idx is None");
            Err(())?
        };

        waveform.load_signals_multi_threaded(&[
            refill_valid_signal_ref,
            refill_address_signal_ref,
            refill_way_signal_ref,
        ]);

        let refill_valid_signal = waveform.get_signal(refill_valid_signal_ref).unwrap();
        let refill_valid_offset = refill_valid_signal.get_offset(time_table_idx).unwrap();
        let refill_valid_value = refill_valid_signal
            .get_value_at(&refill_valid_offset, 0)
            .to_string();

        let refill_address_signal = waveform.get_signal(refill_address_signal_ref).unwrap();
        let refill_address_offset = refill_address_signal.get_offset(time_table_idx).unwrap();
        let refill_address_value = refill_address_signal.get_value_at(&refill_address_offset, 0);
        let refill_address_value = VarFormat::Hexadecimal.format(refill_address_value);

        if refill_valid_value == "0" {
            return Ok((refill_address_value, None));
        }

        let refill_way_signal = waveform.get_signal(refill_way_signal_ref).unwrap();
        let refill_way_offset = refill_way_signal.get_offset(time_table_idx).unwrap();
        let refill_way_value = refill_way_signal.get_value_at(&refill_way_offset, 0);
        let refill_way_value = VarFormat::Unsigned.format(refill_way_value);

        Ok((
            refill_address_value,
            Some(refill_way_value.parse().unwrap()),
        ))
    }
}

pub async fn remove_all_diagram_connectors() -> RemovedDiagramConnectorsCount {
    let mut diagram_connectors = DIAGRAM_CONNECTORS.write().await;
    let diagram_connectors_count = diagram_connectors.len();
    diagram_connectors.clear();
    diagram_connectors_count
}

// @TODO Make println work on Windows in release mode?
// https://github.com/tauri-apps/tauri/discussions/8626

// @TODO Remove / improve comments below
// Testing
//
// Rust
// FW.add_diagram_connectors(["../test_files/components/rust_diagram_connector/rust_diagram_connector.wasm"])
//
// Remove all
// FW.remove_all_diagram_connectors()
//
// All Debug
// FW.add_diagram_connectors(["../test_files/components/rust_diagram_connector/rust_diagram_connector.wasm"])
//
// All Release
// FW.add_diagram_connectors(["../../test_files/components/rust_diagram_connector/rust_diagram_connector.wasm"])
pub async fn add_diagram_connectors(
    diagram_connector_paths: Vec<DiagramConnectorPath>,
) -> AddedDiagramConnectorsCount {
    println!("Diagram Connectors: {diagram_connector_paths:#?}");
    println!("Current dir: {:#?}", std::env::current_dir().unwrap());

    let mut added_diagram_connectors_count = 0;

    // @TODO (?) New thread to prevent "Cannot start a runtime from within a runtime."
    // when a call to a component fails / panics
    // std::thread::spawn(move || {
    // futures::executor::block_on(async move {
    for diagram_connector_path in diagram_connector_paths {
        if let Err(error) = add_diagram_connector(&diagram_connector_path).await {
            eprintln!("add_diagram_connectors error: {error:?}");
        } else {
            added_diagram_connectors_count += 1;
        }
    }
    // })
    // }).join().unwrap();

    added_diagram_connectors_count
}

async fn add_diagram_connector(path: &str) -> wasmtime::Result<()> {
    let wasmtime_component = WasmtimeComponent::from_file(&ENGINE, path)?;

    let mut store_lock = STORE.lock().await;
    let mut store = store_lock.as_context_mut();

    let component = Component::instantiate(&mut store, &wasmtime_component, &LINKER)?;

    println!(
        "Diagram Connector name: {}",
        component
            .component_diagram_connector_diagram_connector()
            .call_name(&mut store)?
    );
    component
        .component_diagram_connector_diagram_connector()
        .call_init(&mut store)?;

    DIAGRAM_CONNECTORS.write().await.push(component);

    Ok(())
}

// @TODO rename `ComponentId` everywhere to something like `DiagramElementId`?
// @TODO get rid of unwraps
pub async fn notify_diagram_connector_text_change(
    diagram_connector: DiagramConnectorName,
    component_id: ComponentId,
    text: String,
) {
    let mut store_lock = STORE.lock().await;
    let mut store = store_lock.as_context_mut();

    let diagram_connectors = DIAGRAM_CONNECTORS.read().await;

    // @TODO store diagram_collectors in a hashmap/btreemap?
    let diagram_connector = diagram_connectors
        .iter()
        .find(|diagram_collector| {
            let name = diagram_collector
                .component_diagram_connector_diagram_connector()
                .call_name(&mut store)
                .unwrap();
            name == diagram_connector
        })
        .unwrap();

    diagram_connector
        .component_diagram_connector_diagram_connector()
        .call_on_component_text_changed(&mut store, &component_id, &text)
        .unwrap();
}
