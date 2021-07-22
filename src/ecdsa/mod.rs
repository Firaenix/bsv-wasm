pub mod ecdh;
pub mod sign;
pub mod verify;

pub use ecdh::*;

use wasm_bindgen::prelude::*;

/**
 * Utility struct for low level ECDSA primitives
 */
#[wasm_bindgen]
pub struct ECDSA {}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SigningHash {
    Sha256,
    Sha256d,
}
