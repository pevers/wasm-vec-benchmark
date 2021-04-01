const wasm = require("wasm-vec-benchmark");

for (let i = 0; i < 1000; i++) {
    wasm.get_content();
}

console.log(wasm.get_content());