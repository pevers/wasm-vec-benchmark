const wasm = require("wasm-vec-benchmark");

for (let i = 0; i < 100000; i++) {
    wasm.get_content();
}