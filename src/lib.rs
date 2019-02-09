extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
mod helper;
use self::helper::make_custom_pass;

#[wasm_bindgen]
pub extern "C" fn say_hi() -> std::string::String {
    "hi".to_string()
}

#[wasm_bindgen]
pub extern "C" fn make_password(
    low: usize,
    cap: usize,
    num: usize,
    sym: usize,
) -> std::string::String {
    make_custom_pass(low, cap, num, sym)
}
