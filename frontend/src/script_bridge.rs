use zoon::{*, println};

#[wasm_bindgen(inline_js = r#"export function strict_eval(code) { return eval?.(`"use strict"; ${code};`) }"#)]
extern "C" {
    #[wasm_bindgen(catch)]
    pub fn strict_eval(code: &str) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
pub struct FW;

#[wasm_bindgen]
impl FW {
    pub fn do_something() {
        println!("Command result: {:#?}", strict_eval("FW.do_something_else();"));
    }

    pub fn do_something_else() {
        println!("ELSE!");
    }
}
