Init
- `npm install && cp -r locales node_modules/@excalidraw/excalidraw/types/`

Watch & build (without typechecking)
- `node_modules/.bin/esbuild excalidraw_canvas.ts --bundle --outfile=../bundles/excalidraw_canvas.js --format=esm --watch`

Watch & typecheck (without building)
- `node_modules/.bin/tsc excalidraw_canvas.ts --watch -noEmit --preserveWatchOutput --target esnext --module esnext --moduleResolution bundler`

Created with commands:
- `npm i -E react react-dom @excalidraw/excalidraw`
- `npm i -E @types/react @types/react-dom`
- `npm i -E roughjs @excalidraw/laser-pointer jotai browser-fs-access`
- `npm i -D esbuild typescript`
- `locales/en.json` downloaded from `https://raw.githubusercontent.com/excalidraw/excalidraw/refs/tags/v0.17.6/src/locales/en.json`
- `excalidraw-assets-dev` and `excalidraw-assets` from `FastWave2.0\frontend\typescript\excalidraw_canvas\node_modules\@excalidraw\excalidraw\dist` copied into `FastWave2.0\public\excalidraw`
- Lines added to `FastWave2.0\backend\globals.js`:
    ```js
    // -- Excalidraw settings --
    // @TODO replace with "true" once Preact is integrated into ExcalidrawCanvas
    var process = { env: { IS_PREACT: "false" } };
    // @TODO probably remove or update once Preact is integrated into ExcalidrawCanvas
    window.__REACT_DEVTOOLS_GLOBAL_HOOK__ = { isDisabled: true };
    window.EXCALIDRAW_ASSET_PATH = "/_api/public/excalidraw/";
    ```
