import * as rustWasm from "../pkg/wasm_game_of_life.js";

const NUMS = 50000;

require("./tinygo_wasm_exec.js");

// @ts-ignore
const go = new Go();
const fs = require("fs");
WebAssembly.instantiate(fs.readFileSync("./go.wasm"), go.importObject).then(
  function ({ instance, module }) {
    go.run(instance);

    console.time("rust");
    rustWasm.sum(NUMS);
    // console.log("result rust ===>", wasm.sum(NUMS));
    console.timeEnd("rust");

    console.time("go");
    // @ts-ignore
    const result = instance.exports.add(NUMS);
    // console.log("result go ===>", result);
    console.timeEnd("go");

    function add(a) {
      let sum = 0;
      for (let i = 0; i < a; i++) {
        sum += i;
      }
      return sum;
    }
    console.time("node");
    const nodeResult = add(NUMS);
    // console.log("result node ===>", nodeResult);
    console.timeEnd("node");
  }
);
