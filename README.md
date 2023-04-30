# nethelper_wasm
nethelper_wasm is a library that provides JavaScript-compatible wrappers for `netcalc` logic, enabling its use in web apps.  
This library exposes only the safe functions that can be used in JavaScript.

## How to compile to *.WASM
1. Install `wasm-pack` by running the following command: `cargo install wasm-pack`
2. Run `wasm-pack build --target web` to compile the library to WebAssembly.

## How to use WASM in React
After compiling the library with `wasm-pack build --target web`:
1. Navigate to your React project and open `package.json`
2. Under `"dependencies"`, add `"wasm": "file:~/nethelper_wasm/pkg"`
3. In the root directory of your React project, run `npm install`
4. Import the functions you need in your React components with `import { function_name } from "wasm"`
5. Use the imported functions in your React components


## FAQ
1. The file `package.json` is missing inside `/pkg/`:  
Follow this [fix](https://github.com/rustwasm/wasm-pack/issues/965#issuecomment-767687015) or manually create it.

Example `package.json`
```json
{
  "name": "nethelper_wasm",
  "collaborators": [
    "name <email>"
  ],
  "version": "0.1.0",
  "files": [
    "nethelper_wasm_bg.wasm",
    "nethelper_wasm.js",
    "nethelper_wasm.d.ts"
  ],
  "module": "nethelper_wasm.js",
  "types": "nethelper_wasm.d.ts",
  "sideEffects": false
}
```
