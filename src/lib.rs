mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    println!("hello")
}

#[wasm_bindgen]
pub fn add(a: i8, b: i8) -> i8{
    return a + b
}

#[wasm_bindgen]
pub fn sum(a: i8) -> i8{
    let mut total  = 0;
    let mut i = 0;
    while i <= a {
      total += i;
      i = i + 1;
    }
    return total
}
