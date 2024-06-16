use crate::STORE;
use zoon::*;

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
    pub fn say_hello() -> String {
        "Hello!".to_owned()
    }

    pub fn clear_variables() -> String {
        let mut vars = STORE.selected_var_refs.lock_mut();
        let var_count = vars.len();
        vars.clear();
        format!("{var_count} variables cleared")
    }
}
