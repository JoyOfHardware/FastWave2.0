Init
- `npm install`

Watch & build (without typechecking)
- `node_modules/.bin/esbuild pixi_canvas.ts --bundle --minify --outfile=../bundles/pixi_canvas.js --format=esm --watch`

Watch & typecheck (without building)
- `node_modules/.bin/tsc pixi_canvas.ts --watch -noEmit --preserveWatchOutput --target esnext --module esnext --moduleResolution bundler`

Created with commands:
- `npm i -E pixi.js`
- `npm i -D esbuild typescript`


