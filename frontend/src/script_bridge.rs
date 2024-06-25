use crate::STORE;
use wellen::GetItem;
use zoon::*;

type FullVarName = String;
type DecoderPath = String;

#[wasm_bindgen(
    inline_js = r#"export function strict_eval(code) { "use strict"; return eval?.(`${code}`) }"#
)]
extern "C" {
    #[wasm_bindgen(catch)]
    pub fn strict_eval(code: &str) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
pub struct FW;

#[wasm_bindgen]
impl FW {
    /// JS: `FW.say_hello()` -> `Hello!`
    pub fn say_hello() -> String {
        "Hello!".to_owned()
    }

    /// JS: `FW.clear_selected_vars()` -> `4`
    pub fn clear_selected_vars() -> usize {
        let mut vars = STORE.selected_var_refs.lock_mut();
        let var_count = vars.len();
        vars.clear();
        var_count
    }

    /// JS: `FW.select_vars(["simple_tb.s.A", "simple_tb.s.B"])` -> `2`
    pub fn select_vars(full_var_names: Vec<FullVarName>) -> usize {
        if let Some(hierarchy) = STORE.hierarchy.get_cloned() {
            let mut new_var_refs = Vec::new();
            for full_var_name in full_var_names {
                let path_with_name = full_var_name.split_terminator('.').collect::<Vec<_>>();
                if let Some((name, path)) = path_with_name.split_last() {
                    if let Some(var_ref) = hierarchy.lookup_var(path, name) {
                        new_var_refs.push(var_ref);
                    }
                }
            }
            let var_ref_count = new_var_refs.len();
            STORE.selected_var_refs.lock_mut().replace(new_var_refs);
            return var_ref_count;
        }
        0
    }

    /// JS: `FW.loaded_filename()` -> `simple.vcd`
    pub fn loaded_filename() -> Option<String> {
        STORE.loaded_filename.get_cloned()
    }

    /// JS: `FW.selected_vars()` -> `["simple_tb.s.A", "simple_tb.s.B"]`
    pub fn selected_vars() -> Vec<FullVarName> {
        if let Some(hierarchy) = STORE.hierarchy.get_cloned() {
            let mut full_var_names = Vec::new();
            for var_ref in STORE.selected_var_refs.lock_ref().as_slice() {
                let var = hierarchy.get(*var_ref);
                let var_name = var.full_name(&hierarchy);
                full_var_names.push(var_name);
            }
            return full_var_names;
        }
        Vec::new()
    }

    /// JS: `FW.add_decoders(["test_files/components/rust_decoder/rust_decoder.wasm"])` -> `1`
    pub fn add_decoders(decoder_paths: Vec<DecoderPath>) -> usize {
        zoon::println!("decoders: {decoder_paths:#?}");
        0
    } 
}
