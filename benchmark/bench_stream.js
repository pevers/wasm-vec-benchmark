const wasm = require("wasm-vec-benchmark");
console.log('wasm',wasm.__wasm.memory);

for (let i = 0; i < 1000; i++) {
    const ptr = wasm.get_content_stream();
    console.log('ptr',ptr);
    const out = new Uint16Array(new Uint16Array(wasm.__wasm.memory.buffer, ptr.offset, ptr.size));
    // If you do not free it you will see a rise in heap pointers!
    //ptr.free();
}