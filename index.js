import wasm from "./Cargo.toml";

class Node {
  value;
  child;
  parent;
  constructor(value, child, parent) {
    this.value = value;
    this.child = child;
    this.parent = parent;
  }
}

wasm().then((exports) => {
  // const parent = new Node('parent');
  // const child = new Node('child');
  // parent.child = child;
  // child.parent = parent;
  // console.log('exports.test_circular(parent) ==========>', exports.test_circular(child))
  const key = "test";
  const ptr = exports.new_buffer(key, 10);
  exports.append_buffer(key);
});
