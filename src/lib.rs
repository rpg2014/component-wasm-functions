mod utils;
mod algorithms;
mod types;

extern crate strsim;



use wasm_bindgen::prelude::*;
use serde_wasm_bindgen;
use wasm_bindgen::JsValue;
use algorithms::jaro_winkler::predictive_input_jaro_winkler;
use types::predictive_response::PredictiveResults;
use types::algorithm_types::AlgorithmTypes;

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
pub fn predictive_input(input: &str, list: JsValue, algorithm: AlgorithmTypes) -> Result<JsValue, JsValue> {
    let values: Vec<String> = serde_wasm_bindgen::from_value(list).unwrap();
    let results = match algorithm {
        AlgorithmTypes::JaroWinkler => predictive_input_jaro_winkler(input, values)?,
        _ => PredictiveResults::empty()
    } ;
    return Ok(serde_wasm_bindgen::to_value(&results).unwrap())
}

