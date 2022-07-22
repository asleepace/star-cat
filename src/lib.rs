use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// import js console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// exported function
#[wasm_bindgen]
pub fn start() {
    log("Hello, world!")
}
