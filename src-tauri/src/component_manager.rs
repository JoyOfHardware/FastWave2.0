use crate::{AddedDecodersCount, DecoderPath};
use wasmtime::component::{*, Component as WasmtimeComponent};
use wasmtime::{Engine, Store};

bindgen!();

struct LinkedState;

impl component::decoder::host::Host for LinkedState {
    fn log(&mut self, message: String) -> () {
        println!("Decoder: {message}");
    }
}

// FW.add_decoders(["../test_files/components/rust_decoder/rust_decoder.wasm"])
// FW.add_decoders(["../test_files/components/javascript_decoder/javascript_decoder.wasm"])
// FW.add_decoders(["../test_files/components/python_decoder/python_decoder.wasm"])
pub fn add_decoders(decoder_paths: Vec<DecoderPath>) -> AddedDecodersCount {
    println!("decoders in Tauri: {decoder_paths:#?}");
    println!("{:#?}", std::env::current_dir());

    if let Err(error) = add_decoder(&decoder_paths[0]) {
        eprintln!("add_decoders error: {error:?}");
    }

    decoder_paths.len()
}

fn add_decoder(path: &str) -> wasmtime::Result<()> {
    let engine = Engine::default();
    let wasmtime_component = WasmtimeComponent::from_file(&engine, path)?;
    
    let mut linker = Linker::new(&engine);
    Component::add_to_linker(&mut linker, |state: &mut LinkedState| state)?;

    let mut store = Store::new(&engine, LinkedState);

    let (component, _instance) = Component::instantiate(&mut store, &wasmtime_component, &linker)?; 

    component.component_decoder_decoder().call_init(&mut store)?;

    Ok(())
}
