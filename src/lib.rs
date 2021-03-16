use wasm_bindgen::prelude::*;

pub mod keypair;
pub use keypair::*;

pub mod traits;
pub use traits::*;

pub mod errors;
pub use errors::*;

// use k256::ecdsa::signature::Signature;

// #[wasm_bindgen]
// pub fn add(a: i32, b: i32) -> i32 {
//   a + b
// }