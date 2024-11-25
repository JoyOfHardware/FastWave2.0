use std::borrow::Cow;
use std::cell::RefCell;

#[allow(warnings)]
mod bindings;

use bindings::component::diagram_connector::host;
use bindings::exports::component::diagram_connector::diagram_connector;

macro_rules! log {
    ($($arg:tt)*) => (host::log(&format!($($arg)*)))
}

static NAME: &str = "Rust Test Diagram Connector";

// Note: Ids from `test_files/cache_diagram.excalidraw`
const ADDRESS_COMPONENT_ID: &str = "ITxhJ7NtZ74YFd9JQ0_pl";
const TIME_COMPONENT_ID: &str = "afXu8_6Kqfq-q2IsjtAcP";
const STATUS_COMPONENT_ID: &str = "0iH5yRbH4IEseV3mnof3A";

thread_local! {
    static TIME_TEXT: RefCell<String> = <_>::default();
}

struct Component;

impl diagram_connector::Guest for Component {
    fn init() {
        host::listen_for_component_text_changes(NAME, TIME_COMPONENT_ID);
        log!("'{NAME}' initialized")
    }

    fn name() -> String {
        NAME.to_string()
    }

    fn on_component_text_changed(component_id: String, text: String) {
        match component_id.as_str() {
            TIME_COMPONENT_ID => {
                TIME_TEXT.set(text);
                refresh_fields();
            }
            _ => (),
        }
    }
}

fn refresh_fields() {
    let way = TIME_TEXT.with_borrow(|time_text| host::address_and_way(&time_text));
    let Ok((address, way)) = way else {
        return;
    };
    host::set_component_text(ADDRESS_COMPONENT_ID, &address);

    let status_text: Cow<str> = if let Some(way) = way {
        format!("VALID, WAY: {way}").into()
    } else {
        "NOT VALID".into()
    };
    host::set_component_text(STATUS_COMPONENT_ID, &status_text);
}

bindings::export!(Component with_types_in bindings);
