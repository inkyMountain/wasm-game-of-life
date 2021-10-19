use std::{cell::RefCell, sync::Mutex};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
  static ref GLOBAL_STORAGE: Mutex<RefCell<HashMap<String, Vec<u8>>>> = {
    return Mutex::new(RefCell::new(HashMap::new()));
  };
}

#[wasm_bindgen(start)]
pub fn run() {}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
}

// 调用 js 中的 console.log
// fn using_web_sys() {
//     let result = String::from("xxx") + "yyy";
//     console::log_1(&"Hello using web-sys".into());
//     let bool_value = true;
//     console::log_1(&bool_value.into());
//     console::log_1(&1.into());

//     let js: JsValue = 4.into();
//     console::log_2(&"Logging arbitrary values looks like".into(), &js);
// }

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Person {
  name: String,
}

#[wasm_bindgen]
pub fn test(p: &JsValue) -> String {
  let person: Person = p.into_serde().unwrap();
  person.name
}

#[wasm_bindgen]
pub fn get_value_from_js(p: JsValue) -> Result<JsValue, JsValue> {
  let person: Person = serde_wasm_bindgen::from_value(p)?;
  serde_wasm_bindgen::to_value(&person).map_err(|err| err.into())

  // let hello = Person {
  //     name: String::from("hello"),
  // };
  // serde_wasm_bindgen::to_value(&hello).map_err(|err| err.into())
}

#[derive(Serialize, Deserialize)]
struct RustNode {
  value: String,
}

#[wasm_bindgen]
pub fn test_circular(node: JsValue) -> Result<JsValue, JsValue> {
  let node_rust: RustNode = serde_wasm_bindgen::from_value(node)?;
  serde_wasm_bindgen::to_value(&node_rust.value).map_err(|err| err.into())
}

#[wasm_bindgen]
pub fn new_buffer(key: String, len: usize) -> *const u8 {
  let mut buffer = vec![0; len];
  let ptr = buffer.as_ptr();
  let mut vec2: Vec<u8> = "<html><head></head></html>".as_bytes().to_vec();
  for byte in vec2 {
    
  }
  buffer.append(&mut vec2);
  GLOBAL_STORAGE
    .lock()
    .unwrap()
    .borrow_mut()
    .insert(key, buffer);
  ptr
}

#[wasm_bindgen]
pub fn append_buffer(key: String) {
  let buffer_ref = GLOBAL_STORAGE.lock().unwrap();
  let mut buffer = buffer_ref.borrow_mut();
  let to_append = "<html><head></head></html>".as_bytes().to_vec();

  // let old = buffer.entry(key).or_insert(vec![] as Vec<u8>);
  // old.append(&mut to_append);
  buffer.insert(key, to_append);
}
