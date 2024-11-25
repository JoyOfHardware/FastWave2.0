use crate::{AddedDecodersCount, DecoderPath, RemovedDecodersCount};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tauri::async_runtime::{Mutex, RwLock};
use wasmtime::component::{Component as WasmtimeComponent, *};
use wasmtime::{AsContextMut, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiView};

bindgen!(in "wit/decoder");

pub static DECODERS: Lazy<Arc<RwLock<Vec<Component>>>> = Lazy::new(<_>::default);
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

impl component::decoder::host::Host for State {
    fn log(&mut self, message: String) {
        println!("Decoder: {message}");
    }
}

pub async fn remove_all_decoders() -> RemovedDecodersCount {
    let mut decoders = DECODERS.write().await;
    let decoders_count = decoders.len();
    decoders.clear();
    decoders_count
}

// @TODO Make println work on Windows in release mode?
// https://github.com/tauri-apps/tauri/discussions/8626

// @TODO Remove / improve comments below
// Testing
//
// Rust
// FW.add_decoders(["../test_files/components/rust_decoder/rust_decoder.wasm"])
// FW.add_decoders(["../test_files/components/rust_decoder/rust_decoder.wasm", "../test_files/components/rust_decoder/rust_decoder.wasm"])
//
// JS
// FW.add_decoders(["../test_files/components/javascript_decoder/javascript_decoder.wasm"])
//
// Python
// FW.add_decoders(["../test_files/components/python_decoder/python_decoder.wasm"])
//
// Remove all
// FW.remove_all_decoders()
//
// All Debug
// FW.add_decoders(["../test_files/components/rust_decoder/rust_decoder.wasm", "../test_files/components/javascript_decoder/javascript_decoder.wasm", "../test_files/components/python_decoder/python_decoder.wasm"])
//
// All Release
// FW.add_decoders(["../../test_files/components/rust_decoder/rust_decoder.wasm", "../../test_files/components/javascript_decoder/javascript_decoder.wasm", "../../test_files/components/python_decoder/python_decoder.wasm"])
pub async fn add_decoders(decoder_paths: Vec<DecoderPath>) -> AddedDecodersCount {
    println!("Decoders: {decoder_paths:#?}");
    println!("Current dir: {:#?}", std::env::current_dir().unwrap());

    let mut added_decoders_count = 0;

    // @TODO (?) New thread to prevent "Cannot start a runtime from within a runtime."
    // when a call to a component fails / panics
    // std::thread::spawn(move || {
    // futures::executor::block_on(async move {
    for decoder_path in decoder_paths {
        if let Err(error) = add_decoder(&decoder_path).await {
            eprintln!("add_decoders error: {error:?}");
        } else {
            added_decoders_count += 1;
        }
    }
    // })
    // }).join().unwrap();

    added_decoders_count
}

async fn add_decoder(path: &str) -> wasmtime::Result<()> {
    let wasmtime_component = WasmtimeComponent::from_file(&ENGINE, path)?;

    let mut store_lock = STORE.lock().await;
    let mut store = store_lock.as_context_mut();

    let component = Component::instantiate(&mut store, &wasmtime_component, &LINKER)?;

    println!(
        "Decoder name: {}",
        component
            .component_decoder_decoder()
            .call_name(&mut store)?
    );
    component
        .component_decoder_decoder()
        .call_init(&mut store)?;

    DECODERS.write().await.push(component);

    Ok(())
}
