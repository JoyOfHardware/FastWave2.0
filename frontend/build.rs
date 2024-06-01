use std::env;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html

macro_rules! instruction {
    ($($arg: tt)*) => {
        println!($($arg)*)
    }
}

// https://github.com/rust-lang/cargo/issues/985
// macro_rules! warning {
//     ($($arg: tt)*) => {
//         instruction!("cargo:warning={}", format!($($arg)*))
//     }
// }

fn main() {
    let default_platform = "TAURI";
    let platform = env::var("FASTWAVE_PLATFORM").unwrap_or_else(|_| default_platform.to_owned());
    instruction!("cargo:rustc-cfg=FASTWAVE_PLATFORM=\"{platform}\"");
    instruction!("cargo:rerun-if-env-changed=FASTWAVE_PLATFORM");
}
