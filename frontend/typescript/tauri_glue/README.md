Init
- `npm install`

Watch & build (without typechecking)
-`- `node_modules/.bin/esbuild pixi_canvas.ts --bundle --outfile=../bundles/tauri_glue.js --format=esm --watch``

Watch & typecheck (without building)
- `node_modules/.bin/tsc tauri_glue.ts --watch -noEmit --preserveWatchOutput --target esnext --module esnext --moduleResolution bundler`

Created with commands:
- `npm i -E @tauri-apps/api`
- `npm i -D esbuild typescript`
