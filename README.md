# nethelper_wasm
This library offers JavaScript-compatible wrappers for `netcalc` logic, enabling its use in Web Apps.  
It exposes only the safe functions that can be used in Javascript.

## How to compile to *.WASM
```
cargo install wasm-pack
wasm-pack build --target web
```

## How to use in React
After compiling with `wasm-pack build --target web`, go to your React Project and paste `"wasm": "file:~/nethelper_wasm/pkg"` in `package.json` under `"dependencies"`
