mod utils;

use wasm_bindgen::prelude::*;
use compiler;
use std::fmt::Write;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

#[wasm_bindgen(module = "/src/interop.js")]
extern "C" {
    fn writeTermLn(s: &str) -> bool;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

pub struct WasmPrinter {
    chars: Vec<char>,
}

impl WasmPrinter {
    pub fn new() -> WasmPrinter {
        WasmPrinter { chars: Vec::new() }
    }
}
impl Write for WasmPrinter {
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        if c == '\n' {
            writeTermLn(&self.chars.iter().cloned().collect::<String>());
            self.chars.clear();
        } else {
            self.chars.push(c);
        }

        Ok(())
    }
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for s in s.chars() {
            let _ = self.write_char(s);
        }

        Ok(())
    }
}


#[wasm_bindgen]
pub fn evaluate(input: &str) {
    compiler::evaluate(input.to_string(), &mut WasmPrinter::new());
}

