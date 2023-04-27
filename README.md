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
5. Use the imported functions in your React components.
