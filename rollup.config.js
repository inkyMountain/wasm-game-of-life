import rust from "@wasm-tool/rollup-plugin-rust";

export default {
  input: "index.js",
  output: {
    file: "wasm.js",
    format: "cjs",
  },
  plugins: [
    rust({
      nodejs: true,
      inlineWasm: true,
    }),
  ],
};
