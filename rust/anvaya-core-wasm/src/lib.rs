use wasm_bindgen::prelude::*;
use anvaya_core;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello from ANVAYA Core, {}! The sum of 2+2 is {}.", name, anvaya_core::add(2, 2))
}
