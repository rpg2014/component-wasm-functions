use wasm_bindgen::prelude::*; 
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Serialize, Deserialize,Debug)]
pub enum AlgorithmTypes {
    JaroWinkler,
    DatFastShit,
}