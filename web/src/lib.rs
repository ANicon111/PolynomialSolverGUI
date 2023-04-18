mod logic;
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
include!("logic.rs");

#[wasm_bindgen]
pub fn get_results(name: &str) -> Vec<JsValue> {
    let mut vals: Vec<JsValue> = vec![];
    let solutions = get_solutions(name.to_string());
    for val in solutions {
        vals.push(serde_wasm_bindgen::to_value(&val).unwrap());
    }
    vals
}
