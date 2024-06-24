#[allow(warnings)]
mod bindings;

use bindings::component::rust_decoder::host;
use bindings::exports::component::rust_decoder::decoder;

macro_rules! log {
    ($($arg:tt)*) => (host::log(&format!($($arg)*)))
}

static NAME: &str = "Rust Test Decoder";

struct Component;

impl decoder::Guest for Component {
    fn init() {
        log!("'{NAME}' initialized")
    }

    fn name() -> String {
        NAME.to_string()
    }

    fn format_signal_value(mut value: String) -> String {
        value.push('!');
        value
    }
}

bindings::export!(Component with_types_in bindings);
