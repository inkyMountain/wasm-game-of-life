import { wasm } from "@rollup/plugin-wasm";

export default {
  input: "index.js",
  output: {
    file: "bundle.js",
    format: "cjs",
  },
  plugins: [wasm()],
};
