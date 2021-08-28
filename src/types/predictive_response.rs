use wasm_bindgen::prelude::*; 
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Serialize, Deserialize,Debug, Clone)]
pub struct Prediction {
    pub score: f64,
    text: String,
}

impl Prediction {
    pub fn new(score: f64, text: &str) -> Prediction {
        Prediction {
            score,
            text: String::from(text)
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize,Debug)]
pub struct PredictiveResults{
    top_result: Option<Prediction>,
    result_list: Vec<Prediction>,
    error_message: Option<String>,
}
impl PredictiveResults {
    pub fn empty() -> PredictiveResults {
        PredictiveResults {
            result_list: Vec::new(),
            top_result: None,
            error_message: None
        }
    }

    pub fn new(top_result: Option<Prediction>, result_list: Vec<Prediction>) -> PredictiveResults{
        PredictiveResults {
            result_list,
            top_result,
            error_message: None,
        }
    }
}
