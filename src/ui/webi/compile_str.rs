use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_string_to_wat(code: &str) -> String {
    format!("compiled: {}", code)
}
