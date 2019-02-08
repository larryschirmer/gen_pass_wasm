extern crate wasm_bindgen;
mod make_custom_pass;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub extern "C" fn make_password(
    low: usize,
    cap: usize,
    num: usize,
    sym: usize,
) -> std::string::String {
    make_custom_pass::make_custom_pass(low, cap, num, sym)
}
