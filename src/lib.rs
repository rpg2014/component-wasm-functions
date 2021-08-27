mod utils;

extern crate strsim;

use strsim::jaro_winkler;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::{to_value};
use wasm_bindgen::JsValue;
use std::error::Error;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct Prediction {
    score: f64,
    text: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct PredictiveResults{
    top_result: Prediction,
    result_list: Vec<Prediction>
}

#[wasm_bindgen]
// (name: &str) means the 'str' variable is a pointer
pub fn greet(name: &str) {
    alert(&*("Hello, component-wasm-functions!".to_owned() + name));
}




#[wasm_bindgen]
pub fn predictive_input(input: &str, list: JsValue) -> Result<JsValue, JsValue> {
    let values: Vec<String> = serde_wasm_bindgen::from_value(list).unwrap();
    let results = predictive_input_jaro_winkler(input, values)?;
    return Ok(serde_wasm_bindgen::to_value(&results).unwrap())
}

pub fn predictive_input_jaro_winkler( input: &str, list: Vec<String>) -> Result<PredictiveResults, &str> {
    if list.is_empty() || input.is_empty() {
        return Err("Neither input can be empty");
    }
    let mut top_result: Prediction = Prediction {
        text: list.get(0).unwrap().to_owned(),
        score: 0.0f64
    };
    let mut result_list: Vec<Prediction> = Vec::new();
    for value in list {
        let score = jaro_winkler(input, &value);
        let prediction = Prediction {
            score,
            text: value
        };
        result_list.push(prediction.clone());
        if top_result.score < score {
            top_result = prediction;
        }
    }
    result_list.sort_by(| a, b| b.score.partial_cmp(&a.score).unwrap());
    Ok(PredictiveResults {
        top_result,
        result_list
    })
}