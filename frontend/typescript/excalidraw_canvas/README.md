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
