# FastWave
> Cross-Platform Wave Viewer 

---

<p align="center">
  <img width="800" src="docs/screenshot_firefox.png" alt="fastwave_screenshot_firefox" />
</p>

<p align="center">
  <img width="800" src="docs/video_desktop.gif" alt="fastwave_video_desktop" />
</p>

---

### Install requirements:

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Node.js](https://nodejs.org/)
3. `cargo install cargo-make`
4. `makers install`

___

### Start the desktop version:

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

### Production build of the desktop version:

1. `makers bundle`
2. Runnable executable is in `target/release`
3. Installable bundles specific for the platform are in `target/release/bundle`

---

### Start in a browser:

1. `makers start_browser`
2. Ctrl + Click the server URL mentioned in the terminal log 

---

### Start in a browser in the release mode:

1. `makers start_browser_release`
2. Ctrl + Click the server URL mentioned in the terminal log 

---

### Steps before pushing:

1. `makers format`



### Test files

See the folder `test_files`.
