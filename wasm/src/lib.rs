mod utils;

use wasm_bindgen::prelude::*;
use compiler;

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
pub fn greet(input: &str) {
    // let input = String::from("for (var i = 0; i < 10; i = i + 1) { print i; }");
    let output = compiler::evaluate(input.to_string());
    alert(&output);
}

