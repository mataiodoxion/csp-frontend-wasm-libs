use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: isize, b: isize) -> isize {
    a + b
}
