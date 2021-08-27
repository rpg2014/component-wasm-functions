//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use component_wasm_functions::{predictive_input, predictive_input_jaro_winkler};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test()]
fn predictive_input_Should_return_results_when_called_with_things() {
    let list = vec![String::from("Alphavet"), String::from("able"),String::from("discussion"),String::from("father"),String::from("information"),String::from("memory")];
    let results = predictive_input_jaro_winkler("able", list).unwrap();
    console_log!("{:?}",results)
}
