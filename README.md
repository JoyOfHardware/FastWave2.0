# FastWave
> Cross-Platform Wave Viewer 

---

<p align="center">
  <img width="800" src="https://github.com/JoyOfHardware/FastWave2.0/assets/18517402/87b7cafb-ccdf-4968-8057-3a19632f227f" alt="fastwave_screenshot" />
</p>

---

### Install requirements:

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Node.js](https://nodejs.org/)
3. `cargo install cargo-make`
4. `makers install`

___

### Start:

1. `makers start`

Troubleshooting:
- In case of Tauri compilation errors, install system dependencies: https://beta.tauri.app/guides/prerequisites/

- Possible Tauri runtime errors in terminal of VSCode installed from Linux Snap package manager:
    ```
    Failed to load module "colorreload-gtk-module"

    /usr/lib/x86_64-linux-gnu/webkit2gtk-4.1/WebKitNetworkProcess: symbol lookup error: /snap/core20/current/lib/x86_64-linux-gnu/libpthread.so.0: undefined symbol: __libc_pthread_init, version GLIBC_PRIVATE
    ```
    Fix it by installing VSCode directly from official `.deb` bundle or try to unset multiple env variables - more info in https://stackoverflow.com/questions/75921414/java-symbol-lookup-error-snap-core20-current-lib-x86-64-linux-gnu-libpthread

---

### Start in the browser:

1. `makers start_browser`
2. Ctrl + Click the server URL mentioned in the terminal log 

---

### Steps before pushing:

1. `makers format`

---

### Production build:

1. `makers bundle`
2. Runnable executable is in `target/release`
3. Installable bundles specific for the platform are in `target/release/bundle`
