How to create and build the Javascript component:

1. Create the `javascript_decoder` folder
2. `cd javascript_decoder`
3. Create `.gitignore` with content `node_modules`
4. `npm install @bytecodealliance/jco @bytecodealliance/componentize-js binaryen`
5. Create the `src` folder with the file `index.js`
6. Create the `wit` folder with the file `world.wit`
7. Update code as needed
8. `npx jco componentize src/index.js --wit wit/world.wit --out javascript_decoder.wasm && npx jco opt javascript_decoder.wasm --output javascript_decoder.wasm`
