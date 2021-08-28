
use strsim::jaro_winkler;
use crate::types::predictive_response::*;


pub fn predictive_input_jaro_winkler( input: &str, list: Vec<String>) -> Result<PredictiveResults, &str> {
    if list.is_empty() || input.is_empty() {
        return Err("Neither input can be empty");
    }
    let mut top_result: Prediction = Prediction::new(0.0f64, list.get(0).unwrap());
    let mut result_list: Vec<Prediction> = Vec::new();
    for value in list {
        let score = jaro_winkler(input, &value);
        let prediction = Prediction::new(score, &value); 
    
        result_list.push(prediction.clone());
        if top_result.score < score {
            top_result = prediction;
        }
    }
    result_list.sort_by(| a, b| b.score.partial_cmp(&a.score).unwrap());
    Ok(PredictiveResults::new(Some(top_result), result_list))
}
