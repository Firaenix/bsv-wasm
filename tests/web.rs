//! Test suite for the Web and headless browsers.

// #![cfg(target_arch = "wasm32")]
extern crate wasm_bindgen_test;
use bsv_rs::Hex;
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!();
// wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
  let string = bsv_rs::PrivateKey::from_hex("Hello hex");
  println!("{:#?}", string);

  assert_eq!(0, 0)
}