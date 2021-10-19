const rustWasm = require("./pkg/wasm_game_of_life");
// const name = rustWasm.test({name: "jf"})
// console.log('name ==========>', name)
// const ptr = rustWasm.new_buffer('test', 100)
// console.log('ptr ==========>', ptr)
const key = "test";
console.clear();
const ptr = rustWasm.new_buffer(key, 100);
const wasmBuffer = new Uint8Array(rustWasm.__wasm.memory.buffer);
// rustWasm.append_buffer(key);
console.log("ptr ==========>", ptr);
const decoder = new TextDecoder();
// console.log('wasmBuffer.slice ==========>', decoder.decode(wasmBuffer.slice(ptr)))
console.log(
  "wasmBuffer.slice ==========>",
  "--" + decoder.decode(wasmBuffer.slice(ptr))
) + "--";
// const result = decoder.decode(wasmBuffer.slice(ptr))
// console.log('result ==========>', result)
