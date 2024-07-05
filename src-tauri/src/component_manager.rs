use crate::{AddedDecodersCount, DecoderPath};
use wasmtime::component::{Component as WasmtimeComponent, *};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiView};

bindgen!();

struct State {
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

// @TODO Make println work on Windows?
// https://github.com/tauri-apps/tauri/discussions/8626

// @TODO Remove / improve comments below
// Testing
// FW.add_decoders(["../test_files/components/rust_decoder/rust_decoder.wasm"])
// FW.add_decoders(["../test_files/components/javascript_decoder/javascript_decoder.wasm"])
// FW.add_decoders(["../test_files/components/python_decoder/python_decoder.wasm"])
pub fn add_decoders(decoder_paths: Vec<DecoderPath>) -> AddedDecodersCount {
    println!("decoders in Tauri: {decoder_paths:#?}");
    println!("Current dir: {:#?}", std::env::current_dir().unwrap());
    let decoder_paths_len = decoder_paths.len();

    // New thread to prevent panics caused by running runtime in runtime
    // @TODO replace with Tokio's spawn_blocking?
    std::thread::spawn(move || {
        if let Err(error) = add_decoder(&decoder_paths[0]) {
            eprintln!("add_decoders error: {error:?}");
        }
    })
    .join()
    .unwrap();

    decoder_paths_len
}

fn add_decoder(path: &str) -> wasmtime::Result<()> {
    let engine = Engine::default();

    let wasmtime_component = WasmtimeComponent::from_file(&engine, path)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    Component::add_to_linker(&mut linker, |state: &mut State| state)?;

    let mut store = Store::new(
        &engine,
        State {
            ctx: WasiCtx::builder().build(),
            table: ResourceTable::new(),
        },
    );

    let component = Component::instantiate(&mut store, &wasmtime_component, &linker)?;

    println!(
        "Decoder name: {}",
        component
            .component_decoder_decoder()
            .call_name(&mut store)?
    );
    component
        .component_decoder_decoder()
        .call_init(&mut store)?;

    Ok(())
}
