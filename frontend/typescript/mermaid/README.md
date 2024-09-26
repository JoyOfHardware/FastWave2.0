Init
- `npm install`

Watch & build (without typechecking)
- `node_modules/.bin/esbuild mermaid.ts --bundle --outfile=../bundles/mermaid.js --format=esm --watch`

Watch & typecheck (without building)
- `node_modules/.bin/tsc mermaid.ts --watch -noEmit --preserveWatchOutput --target esnext --module esnext --moduleResolution bundler`

Created with commands:
- `npm i -E mermaid`
- `npm i -D esbuild typescript`
