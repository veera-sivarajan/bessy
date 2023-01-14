use std::str;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

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

#[derive(Default)]
pub struct WasmPrinter(String);

impl std::io::Write for WasmPrinter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.push_str(str::from_utf8(buf).unwrap());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.clear();
        Ok(())
    }
}

#[wasm_bindgen]
pub fn evaluate(input: String) -> String {
    let mut output = WasmPrinter::default();
    core::evaluate(input, &mut output);
    output.0
}
